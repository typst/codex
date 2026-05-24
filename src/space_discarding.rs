//! Whether to keep or discard space characters that are inferred due to
//! newlines in markup.
//!
//! These definitions allow Chinese and Japanese text to be broken across lines
//! in Typst markup without producing spaces.
//!
//! This is included in Codex for use by other markup systems such as static
//! site generators, Markdown implementations, or HTML renderers.
//!
//! ## Rationale
//!
//! Typst markup should be an ergonomic system to use regardless of which script
//! you are writing in. If there are behaviors Typst applies by default which
//! are obviously wrong in common cases, we endeavor to fix those defaults
//! instead of requiring every user to manually specify an alternative.
//!
//! A common obviously wrong behavior is adding space characters between Chinese
//! and Japanese text split across lines. Chinese and Japanese don't use spaces
//! in writing, but it's still reasonable for authors to format their text files
//! with maximum widths and split text across lines. Applying the rule for Latin
//! text to infer a space from newlines for Chinese and Japanese is incorrect.
//!
//! The straightforward alternative would be to determine whether to keep or
//! discard spaces based solely on a user-provided language tag. But we do not
//! expect authors to tag every individual use of other languages inserted into
//! a main text, which is common in Chinese and Japanese. We may use language
//! tags to improve the current definitions in the future, but we do not
//! consider tags to be a valid replacement for correct default behavior.
//!
//! ## Note on "Writing Systems"
//!
//! Comments in this file use "writing system" as a colloquial term to describe
//! individual scripts or commonly understood collections of scripts. This is
//! because writing out the "Han, Bopomofo, Hiragana, Katakana, and Yi" scripts
//! and distinguishing correctly between Traditional Chinese, Simplified
//! Chinese, and Kanji is a lot of effort. And while this discussion mainly
//! cites Chinese and Japanese for clarity, we endeavor to have ergonomic
//! behavior for all writing systems, scripts, and languages.
//!
//! Much of the research on writing systems used for this feature was based on
//! the wonderfully detailed [Orthography Descriptions][scriptnotes] by Richard
//! Ishida. They proved invaluable here, and are certainly helping his mission
//! of making the World Wide Web worldwide :)
//!
//! [scriptnotes]: https://r12a.github.io/scripts/index.html#scriptnotes

use icu_properties::props::{EastAsianWidth, Emoji, Script};
use icu_properties::{CodePointMapDataBorrowed, CodePointSetDataBorrowed};

/// Whether to discard an inferred space between two strings. This is `true`
/// when the inner character of _either_ string is unambiguously from a writing
/// system which does not use space characters. Otherwise the inferred space
/// should be kept.
///
/// Currently this check includes characters which we determine to be from the
/// Chinese, Japanese, or Yi writing systems plus ideographic punctuation. Note
/// that Korean does use spaces between words and predominantly uses Latin
/// punctuation instead of ideographic punctuation.
///
/// We currently do not discard inferred space characters from writing systems
/// which use spaces only between phrases or sentences. This is because there
/// are no clear distinctions for where phrases or sentences may or may not
/// start/end, so we fall back to keeping all spaces instead of trying to make
/// smart inferences that authors may not expect.
///
/// We may extend this definition in the future by adding an argument for a
/// language tag to improve the behavior around ambiguous characters. We may
/// also analyze grapheme clusters instead of single characters, but we would
/// only check one grapheme cluster from each string.
#[inline]
pub fn discard_space_between(before: &str, after: &str) -> bool {
    // Get the inner characters first since we often need to check both.
    let before_c = before.chars().next_back();
    let after_c = after.chars().next();
    // Discard spaces if either character's writing system does not use spaces.
    // Keep spaces if both character's writing system uses spaces.
    before_c.is_some_and(|c| writing_system_spacing(c) == WritingSystemSpacing::No)
        || after_c.is_some_and(|c| writing_system_spacing(c) == WritingSystemSpacing::No)
}

/// Whether a character is part of a writing system that uses space characters.
///
/// The only common writing systems which do not use space characters in any
/// respect are Chinese, Japanese, and Yi.
///
/// These systems do use empty space in writing, but it is empty due to the lack
/// of ink in ideographic characters typeset on a grid (like the ideographic
/// fullstop `。`), not due to explicit space characters in text.
///
/// Some ideographic punctuation characters primarily used by Chinese, Japanese,
/// and Yi are also used in Korean writing (which uses spaces between words).
/// But these are often used in vertical writing whereas modern Korean
/// predominantly uses Latin punctuation when written horizontally. So this
/// definition treats these shared characters as part of the writing systems
/// which do not use space characters.
///
/// The term "ideographic punctuation" is meant colloquially, and is not related
/// to the "Ideographic Symbols and Punctuation" Unicode block. This is
/// discussed more in `test_ideographic_punctuation_spacing` below.
///
/// ## How Writing Systems use Space Characters
///
/// Beside Chinese, Japanese, and Yi, most writing systems in active use today
/// do use space characters to separate text. Some writing systems use spaces to
/// separate words, some to separate syllables, and some to separate only
/// phrases or sentences.
///
/// Using spaces between **words** is the most common and includes writing
/// systems such as Latin, Arabic, Cyrillic, Korean, and Hebrew. It also
/// includes many Brahmic (or Indic) scripts such as Devanagari, Bengali,
/// Telugu, Tamil, Gujarati, and Kannada.
///
/// However, many active writing systems use spaces not between words, but
/// between **phrases and sentences**, sometimes with other punctuation,
/// sometimes without. This includes Thai, Javanese, Burmese, Khmer, Tibetan,
/// and Lao. The Balinese script is likely in this category, but it is hard to
/// find good examples of its use online.
///
/// Of course, space character frequency for writing systems differs greatly
/// both between systems and within systems when used for different languages.
///
/// Kinds of space usage do not fall along the common distinction of alphabets,
/// abjads, abugidas, or syllabaries that are normally used to distinguish
/// writing systems. Examples of both word spacing and phrase/sentence spacing
/// are present in both alphabets (Latin/Lao) and abugidas (Devanagari/Thai),
/// although abugidas dominate in phrase/sentence spacing.
///
/// Finally, one writing system uses spaces uniquely and deserves its own
/// section:
///
/// ### Ethiopic script (Ge'ez Script)
/// - <https://r12a.github.io/scripts/ethi/am.html#word>
/// - <https://en.wikipedia.org/wiki/Ge%CA%BDez_script>
///
/// The Ethiopic script, or Ge'ez script, is primarily used in the Amharic,
/// Tigrinya, and Tigre languages, and has historically used `፡` U+1361 ETHIOPIC
/// WORDSPACE as a separator between words instead of space characters, but
/// modern text is increasingly using normal spaces instead of the wordspace
/// character. Ethiopic also has several native punctuation characters like `፣`
/// U+1363 ETHIOPIC COMMA and `።` U+1362 ETHIOPIC FULL STOP. A notable
/// complication is that writers very often insert the full stop by writing two
/// wordspace characters instead of using the dedicated codepoint.
///
/// The wordspace character generally attaches to the end of words when spacing
/// is added (such as for justification), and linebreaks should never separate
/// the wordspace from its preceding character. When using the wordspace
/// character, justified text can insert whitespace either on both sides of the
/// wordspace, centering it between words, or immediately after the wordspace,
/// leaving it connected to the previous word.
///
/// We should avoid introducing space characters that the author didn't intend
/// after the wordspace character, but the situation is tricky. A simple rule
/// could be to discard an inferred space which comes after the wordspace
/// character (under the assumption that the wordspace is being used as a
/// space). But this is insufficient as the writer may be using normal spaces
/// instead of wordspaces generally, but still be using pairs of wordspace
/// characters in place of the Ethiopic full stop, after which they would likely
/// expect an inferred space from a newline.
///
/// Given this complexity, we shouldn't try to automatically discard spaces
/// around Ethiopic script characters, so we currently treat it like a writing
/// system which does use spaces.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum WritingSystemSpacing {
    /// Characters whose writing systems do not use space characters to separate
    /// text.
    ///
    /// Only Chinese, Japanese, and Yi do not use spaces. This also includes
    /// common ideographic punctuation characters.
    ///
    /// Amusingly, this also includes U+3000 `　` IDEOGRAPHIC SPACE.
    No,
    /// Characters whose writing system does use space characters, or characters
    /// whose writing system is ambiguous between writing systems which do or
    /// don't use spaces (such as the curly quotes U+201C “ and U+201D ” which
    /// are used in Latin and in Chinese/Japanese).
    ///
    /// That includes writing systems which use spaces in any way, such as
    /// between syllables, words, phrases, or sentences.
    YesOrAmbiguous,
}

/// Determine the [`WritingSystemSpacing`] for a character.
///
/// It was very useful to consult the `UnicodeSet` utility when defining this
/// function: <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp>.
#[inline]
fn writing_system_spacing(c: char) -> WritingSystemSpacing {
    const SCRIPT_MAP: CodePointMapDataBorrowed<Script> = CodePointMapDataBorrowed::new();
    const EAW_MAP: CodePointMapDataBorrowed<EastAsianWidth> =
        CodePointMapDataBorrowed::new();
    const EMOJI_SET: CodePointSetDataBorrowed = CodePointSetDataBorrowed::new::<Emoji>();

    // Just checking the `Script` property gets us most of the way to
    // determining the character's writing system spacing.
    match SCRIPT_MAP.get(c) {
        Script::Han
        | Script::Bopomofo
        | Script::Hiragana
        | Script::Katakana
        | Script::Yi => WritingSystemSpacing::No,

        // Chinese combining ideographic tone marks (U+302A-U+302D) and Japanese
        // voicing marks (U+3099-U+309A).
        Script::Inherited
            if matches!(
                c,
                '\u{302A}'..='\u{302D}' | '\u{3099}'..='\u{309A}',
            ) =>
        {
            WritingSystemSpacing::No
        }

        // We check for an East Asian Width of `Fullwidth`, `Halfwidth`, or
        // `Wide` to determine the `Script=Common` characters that are likely to
        // be used in East Asian typography. Unfortunately even among those
        // there are several exceptions we want to consider as `YesOrAmbiguous`,
        // notably that most emoji characters are `Wide`. More edge cases are
        // discussed in detail in `test_ideographic_punctuation_spacing` and
        // `test_emoji_presentation_spacing` below.
        //
        // This applies `No` to roughly 700 characters (mainly symbols and
        // punctuation).
        Script::Common => match EAW_MAP.get(c) {
            EastAsianWidth::Fullwidth => WritingSystemSpacing::No,

            // We exclude `₩` as it is specifically Korean and to match the
            // behavior of `¥`.
            // <https://www.unicode.org/reports/tr11/tr11-44.html#ED3>
            EastAsianWidth::Halfwidth if c != '₩' => WritingSystemSpacing::No,

            // Emoji are used in many writing systems (no emoji are `Fullwidth`
            // or `Halfwidth`).
            EastAsianWidth::Wide if !EMOJI_SET.contains(c) => WritingSystemSpacing::No,

            // The circled numbers on black squares likely shouldn't have been
            // given `EA=Ambiguous`.
            // <https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-22/#G37527>
            EastAsianWidth::Ambiguous if matches!(c, '㉈'..='㉏') => {
                WritingSystemSpacing::No
            }

            _ => WritingSystemSpacing::YesOrAmbiguous,
        },

        // Everything else is from a writing system using space characters or is
        // ambiguous in its writing system.
        _ => WritingSystemSpacing::YesOrAmbiguous,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use WritingSystemSpacing::{No, YesOrAmbiguous};

    /// This is easier to read than an exclamation mark inside the assertions.
    fn keep_space_between(before: &str, after: &str) -> bool {
        !discard_space_between(before, after)
    }

    /// Assert the [`WritingSystemSpacing`] for a character matches the expected
    /// value.
    #[track_caller]
    fn check_spacing(c: char, expected: WritingSystemSpacing) {
        let actual = writing_system_spacing(c);
        assert_eq!(actual, expected, "Mismatch for U+{:X} '{c}'", c as u32);
    }

    #[test]
    fn test_discard_space_between() {
        // Discard
        assert!(discard_space_between("漢", "漢"));
        assert!(discard_space_between("1", "漢"));
        assert!(discard_space_between("漢", "A"));
        assert!(discard_space_between("漢。", "A"));
        assert!(discard_space_between("A", "（漢"));
        assert!(discard_space_between("A。", "B"));
        assert!(discard_space_between("A", "（B"));

        // Keep
        assert!(keep_space_between("A", "B"));
        assert!(keep_space_between("1", "2"));
        assert!(keep_space_between("end.", "Begin"));
        assert!(keep_space_between("가", "각"));

        // Empty strings
        // Discard
        assert!(discard_space_between("漢", ""));
        assert!(discard_space_between("", "漢"));
        // Keep
        assert!(keep_space_between("", ""));
        assert!(keep_space_between("A", ""));
        assert!(keep_space_between("", "A"));

        // Spaces themselves don't get special treatment
        assert!(discard_space_between("漢", " "));
        assert!(discard_space_between(" ", "漢"));
        assert!(keep_space_between(" ", " "));
    }

    /// Test the [`WritingSystemSpacing`] for basic characters from multiple
    /// scripts.
    #[test]
    fn test_writing_system_spacing() {
        // Characters from different writing systems:
        check_spacing('A', YesOrAmbiguous); // Latin uses spaces
        check_spacing('1', YesOrAmbiguous);
        check_spacing('Ａ', YesOrAmbiguous); // Including the fullwidth variants
        check_spacing('ａ', YesOrAmbiguous);
        check_spacing('가', YesOrAmbiguous); // Modern Korean _does_ use spaces
        check_spacing('ힰ', YesOrAmbiguous);
        check_spacing('한', YesOrAmbiguous);
        check_spacing('漢', No); // Chinese does not use spaces
        check_spacing('汉', No);
        check_spacing('ㄅ', No); // Including bopomofo
        check_spacing('あ', No); // Japanese does not use spaces
        check_spacing('ア', No);
        check_spacing('ｱ', No);
        check_spacing('ꀀ', No); // Yi does not use spaces
        check_spacing('꒐', No);
        // Emoji do not discard spaces (more edge cases are tested below):
        check_spacing('😀', YesOrAmbiguous);
    }

    /// Test the [`WritingSystemSpacing`] for ideographic punctuation
    /// characters.
    ///
    /// The term "ideographic punctuation" is used to refer to `Common` script
    /// Unicode characters which are generally used only in Chinese, Japanese,
    /// or Yi.
    ///
    /// We currently draw a hard line based on the `EastAsianWidth` property,
    /// but that may be worth changing.
    ///
    /// The tested characters are inspired by clreq and jlreq:
    /// - <https://www.w3.org/TR/clreq>
    /// - <https://www.w3.org/TR/jlreq>
    #[test]
    fn test_ideographic_punctuation_spacing() {
        // Punctuation with `EastAsianWidth=Wide`.
        check_spacing('　', No);
        check_spacing('ー', No);
        check_spacing('。', No);
        check_spacing('．', No);
        check_spacing('，', No);
        check_spacing('、', No);
        check_spacing('：', No);
        check_spacing('；', No);
        check_spacing('！', No);
        check_spacing('？', No);
        check_spacing('～', No);
        check_spacing('・', No);
        check_spacing('／', No);
        check_spacing('「', No);
        check_spacing('」', No);
        check_spacing('『', No);
        check_spacing('』', No);
        check_spacing('（', No);
        check_spacing('）', No);
        check_spacing('《', No);
        check_spacing('》', No);
        check_spacing('〈', No);
        check_spacing('〉', No);
        check_spacing('【', No);
        check_spacing('】', No);
        check_spacing('〖', No);
        check_spacing('〗', No);
        check_spacing('〔', No);
        check_spacing('〕', No);
        check_spacing('［', No);
        check_spacing('］', No);
        check_spacing('｛', No);
        check_spacing('｝', No);
        check_spacing('＿', No);
        check_spacing('﹏', No);

        // Punctuation with `EastAsianWidth=Halfwidth`.
        check_spacing('｡', No);
        check_spacing('｢', No);
        check_spacing('｣', No);
        check_spacing('､', No);
        check_spacing('･', No);
        check_spacing('ﾞ', No);
        check_spacing('￨', No);
        check_spacing('￩', No);
        check_spacing('￪', No);
        check_spacing('￫', No);
        check_spacing('￬', No);
        check_spacing('￭', No);
        check_spacing('￮', No);

        // These punctuation are listed in clreq or jlreq, but do not have
        // `EastAsianWidth=Fullwidth|Halfwidth|Wide`.
        check_spacing('#', YesOrAmbiguous);
        check_spacing('*', YesOrAmbiguous);
        check_spacing('/', YesOrAmbiguous);
        check_spacing('-', YesOrAmbiguous); // hyphen
        check_spacing('–', YesOrAmbiguous); // en dash
        check_spacing('—', YesOrAmbiguous); // em dash
        check_spacing('⸺', YesOrAmbiguous); // two-em dash
        check_spacing('‼', YesOrAmbiguous);
        check_spacing('⁇', YesOrAmbiguous);
        check_spacing('·', YesOrAmbiguous); // middle dot
        check_spacing('‧', YesOrAmbiguous); // hyphenation point
        check_spacing('●', YesOrAmbiguous); // black circle
        check_spacing('•', YesOrAmbiguous); // bullet
        check_spacing('“', YesOrAmbiguous);
        check_spacing('”', YesOrAmbiguous);
        check_spacing('‘', YesOrAmbiguous);
        check_spacing('’', YesOrAmbiguous);
        check_spacing('…', YesOrAmbiguous);
        check_spacing('⋯', YesOrAmbiguous);
        check_spacing('①', YesOrAmbiguous);
        check_spacing('⊕', YesOrAmbiguous);

        // Misc Common/Inherited Script edge cases.
        check_spacing('\u{302B}', No);
        check_spacing('\u{3099}', No);
        check_spacing('㉈', No);
        check_spacing('꜀', YesOrAmbiguous);
        // Despite its name, in usage 〿 U+303F Ideographic Half Fill Space is
        // closer to the block element ▓ U+2593 Dark Shade than anything else.
        // They even used to share a code page in some CJK DOS systems.
        // See: <https://en.wikipedia.org/wiki/JIS_X_0201#Variants_and_extensions>
        // Or search `SP500000` in: <https://ccsids.net/3-3220-055/gcgids_S.txt>
        check_spacing('〿', YesOrAmbiguous);
        check_spacing('▓', YesOrAmbiguous);
        // Like Firefox, we treat the Korean won sign as ambiguous, although not
        // the fullwidth variant. This matches the behavior of the Yen/Yuan sign
        // and its fullwidth variant, although Chinese and Japanese usually use
        // different characters instead of `¥`. Note that Latin text uses `¥`
        // for currency from both Chinese and Japanese, which often requires
        // disambiguation. What fun!
        check_spacing('₩', YesOrAmbiguous);
        check_spacing('￦', No);
        check_spacing('¥', YesOrAmbiguous);
        check_spacing('￥', No);
    }

    /// Test [`WritingSystemSpacing`] for emoji with `Emoji_Presentation=No` and
    /// test [`discard_space_between`] with text and emoji presentation
    /// selectors.
    ///
    /// Unicode offers two glyph styles for some emoji characters that are
    /// selectable by adding the text (U+FE0E) or emoji (U+FE0F) presentation
    /// selectors. The default style is defined by the `Emoji_Presentation`
    /// property (`EPres`) and there is an argument to be made that codepoints
    /// with `EPres=No` should be treated as normal text when lacking a
    /// presentation selector. An example is the trademark sign, U+2122 ™, which
    /// is an emoji but has `EPres=No`.
    ///
    /// Note that any emoji _can_ be followed by the text presentation selector.
    /// Most don't seem to have alternate textual glyphs in common fonts, but
    /// many emoji such as the common smiley faces do have alternate glyphs.
    ///
    /// This distinction was probably a mistake by Unicode (it appears they are
    /// not considering default text presentation at all for new characters) but
    /// we are stuck with it for the existing characters.
    ///
    /// This matters for us because we are trying to use the `East_Asian_Width`
    /// property (`EA`) to determine `Script=Common` characters which are used
    /// in Chinese and Japanese. Unfortunately most emoji have `EA=Wide`, so we
    /// currently filter out emoji specifically, but it may be worth treating
    /// some textual presentations of certain emoji as Chinese/Japanese in the
    /// future. Especially so when doing grapheme cluster analysis.
    ///
    /// Here are some relevant facts:
    /// - All emoji have `Script=Common`
    /// - Only emoji have `EPres=Yes`
    /// - 219 (of >1400 total) emoji have `EPres=No`
    /// - Twelve ASCII codepoints are emoji: `[#*0-9]`, all have `EPres=No` and
    ///   `EA=Narrow`
    /// - All emoji have an `East_Asian_Width` of either `Wide` (most emoji),
    ///   `Neutral` (194), `Ambiguous` (33), or `Narrow` (12: `[#*0-9]`)
    /// - Six emoji have `East_Asian_Width=Wide` and `EPres=No`:
    ///   - 〰 U+3030 WAVY DASH
    ///   - 〽 U+303D PART ALTERNATION MARK
    ///   - ㊗ U+3297 CIRCLED IDEOGRAPH CONGRATULATION
    ///   - ㊙ U+3299 CIRCLED IDEOGRAPH SECRET
    ///   - 🈂 U+1F202 SQUARED KATAKANA SA
    ///   - 🈷 U+1F237 SQUARED CJK UNIFIED IDEOGRAPH-6708
    /// - Six emoji have unique scripts in their `Script_Extensions` property:
    ///   - 〰 U+3030 has `Bopomofo|Hangul|Han|Hiragana|Katakana`
    ///   - 〽 U+303D has `Han|Hiragana|Katakana`
    ///   - ㊗ U+3297 and U+3299 ㊙ have `Han`
    ///   - 🉐 U+1F250 and U+1F251 🉑 have `Han`
    ///
    /// These can be checked at
    /// <https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp> by entering a
    /// `UnicodeSet` syntax like `\p{Emoji} & \p{EPres=No} - \p{EA=Neutral}`.
    #[test]
    fn test_emoji_presentation_spacing() {
        // Emoji with `Emoji_Presentation=No` and `East_Asian_Width=W`
        check_spacing('〰', YesOrAmbiguous);
        check_spacing('〽', YesOrAmbiguous);
        check_spacing('㊗', YesOrAmbiguous); // Tested below
        check_spacing('㊙', YesOrAmbiguous);
        check_spacing('🈂', YesOrAmbiguous); // Tested below
        check_spacing('🈷', YesOrAmbiguous);
        // These have `EPres=Yes`, but also `Scx=Han`
        check_spacing('🉐', YesOrAmbiguous);
        check_spacing('🉑', YesOrAmbiguous);

        // ㊗ U+3297 CIRCLED IDEOGRAPH CONGRATULATION (`EPres=No`)
        // Arguably 1-4 should discard spaces.
        assert!(keep_space_between("1", "㊗"));
        assert!(keep_space_between("㊗", "2"));
        assert!(keep_space_between("3", "㊗\u{FE0E}"));
        assert!(keep_space_between("㊗\u{FE0E}", "4"));
        assert!(keep_space_between("5", "㊗\u{FE0F}"));
        assert!(keep_space_between("㊗\u{FE0F}", "6"));

        // ㊕ U+3295 CIRCLED IDEOGRAPH SPECIAL (this is not an emoji)
        // This already discards spaces.
        assert!(discard_space_between("1", "㊕"));
        assert!(discard_space_between("㊕", "2"));
        // I do not think it would be correct to test a non-emoji with the emoji
        // or text presentation selectors.

        // That these two characters have different `Emoji_Presentation` is
        // actually horrible, and a counterpoint against us trying to consider
        // the `Emoji_Presentation` property at all.

        // 🈂 U+1F202 SQUARED KATAKANA SA (`EPres=No`)
        // Arguably 1-4 should discard spaces.
        assert!(keep_space_between("1", "🈂"));
        assert!(keep_space_between("🈂", "2"));
        assert!(keep_space_between("3", "🈂\u{FE0E}"));
        assert!(keep_space_between("🈂\u{FE0E}", "4"));
        assert!(keep_space_between("5", "🈂\u{FE0F}"));
        assert!(keep_space_between("🈂\u{FE0F}", "6"));

        // 🈁 U+1F201 SQUARED KATAKANA KOKO (`EPres=Yes`)
        // Arguably 3 and 4 should discard spaces (perhaps based on lang tag?).
        assert!(keep_space_between("1", "🈁"));
        assert!(keep_space_between("🈁", "2"));
        assert!(keep_space_between("3", "🈁\u{FE0E}"));
        assert!(keep_space_between("🈁\u{FE0E}", "4"));
        assert!(keep_space_between("5", "🈁\u{FE0F}"));
        assert!(keep_space_between("🈁\u{FE0F}", "6"));
    }
}
