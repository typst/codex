//! Style mathematical symbols in Unicode.

use std::fmt::{self, Write};
use std::iter::FusedIterator;

/// The version of [Unicode](https://www.unicode.org/) that this version of the
/// styling module is based on.
pub const UNICODE_VERSION: (u8, u8, u8) = (16, 0, 0);

/// A style for mathematical symbols.
///
/// Notation in mathematics uses a basic set of characters which can be styled.
/// The following groupings are used in the documentation:
/// - digits: The basic Latin digits 0–9 (U+0030..U+0039).
/// - latin: The basic uppercase and lowercase Latin letters, a–z
///   (U+0061..U+007A) and A–Z (U+0041..U+005A).
/// - greek: The uppercase Greek letters Α–Ω (U+0391..U+03A9), plus nabla ∇
///   (U+2207) and theta ϴ (U+03F4). The lowercase Greek letters α–ω
///   (U+03B1..U+03C9), plus the partial differential sign ∂ (U+2202), and the
///   glyph variants ϵ (U+03F5), ϑ (U+03D1), ϰ (U+03F0), ϕ (U+03D5), ϱ
///   (U+03F1), ϖ (U+03D6).
/// - arabic: The Arabic letters ا (U+0627), ب (U+0628), ت–غ (U+062A..U+063A),
///   ف–و (U+0641..U+0648), ي (U+064A).
/// - arabic-dotless: The dotless Arabic letter variants ٮ (U+066E), ٯ
///   (U+066F), ڡ (U+06A1), ں (U+06BA)
/// - digamma: The uppercase and lowercase digamma, Ϝ (U+03DC) and ϝ (U+03DD).
/// - dotless: The dotless variants of the lowercase Latin letters i and j, ı
///   (U+0131) and ȷ (U+0237).
///
/// Note that some styles support only a subset of a group. The characters each
/// style supports are given in their documentation.
///
/// # Script style variants
///
/// There are two widely recognized variants of the script style: chancery and
/// roundhand. They can be distinguished with variation sequences, by using the
/// variation selectors U+FE00 and U+FE01 for chancery and roundhand
/// respectively. These are specified in the [StandardizedVariants.txt] file
/// from the Unicode Character Database.
///
/// Only the uppercase Latin letters are standardized variation sequences, but
/// the [`Chancery`](MathStyle::Chancery) and
/// [`Roundhand`](MathStyle::Roundhand) styles also support the lowercase Latin
/// letters. In addition, the bold styles
/// [`BoldChancery`](MathStyle::BoldChancery) and
/// [`BoldRoundhand`](MathStyle::BoldRoundhand) are provided, which support
/// both the uppercase and lowercase Latin letters despite not being specified
/// as standardized variation sequences by Unicode.
///
/// # Shaping
///
/// The Arabic styles (including those from the
/// [`DoubleStruck`](MathStyle::DoubleStruck) style) are not subject to
/// shaping. However, [`Plain`](MathStyle::Plain) should still be shaped, as
/// the characters are Arabic letters in the Arabic block (U+0600..U+06FF).
///
/// [StandardizedVariants.txt]: <https://www.unicode.org/Public/UNIDATA/StandardizedVariants.txt>
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum MathStyle {
    /// Unstyled default. May be serif or sans-serif depending on the font.
    #[default]
    Plain,
    /// Bold style. May be serif or sans-serif depending on the font.
    ///
    /// Supported characters: digits, latin, greek, digamma.
    Bold,
    /// Italic style. May be serif or sans-serif depending on the font.
    ///
    /// Supported characters: latin, greek, dotless, and the extra ħ (U+0127).
    Italic,
    /// Bold italic style. May be serif or sans-serif depending on the font.
    ///
    /// Supported characters: latin, greek.
    BoldItalic,
    /// Script style. May be chancery or roundhand depending on the font.
    ///
    /// Supported characters: latin.
    Script,
    /// Bold script style. May be chancery or roundhand depending on the font.
    ///
    /// Supported characters: latin.
    BoldScript,
    /// Fraktur style. Also known as black-letter style.
    ///
    /// Supported characters: latin.
    Fraktur,
    /// Bold fraktur style. Also known as bold black-letter style.
    ///
    /// Supported characters: latin.
    BoldFraktur,
    /// Sans-serif style.
    ///
    /// Supported characters: digits, latin.
    SansSerif,
    /// Bold sans-serif style.
    ///
    /// Supported characters: digits, latin, greek.
    SansSerifBold,
    /// Italic sans-serif style.
    ///
    /// Supported characters: latin.
    SansSerifItalic,
    /// Bold italic sans-serif style.
    ///
    /// Supported characters: latin, greek.
    SansSerifBoldItalic,
    /// Monospace style.
    ///
    /// Supported characters: digits, latin.
    Monospace,
    /// Isolated style.
    ///
    /// Supported characters: arabic excluding ه (U+0647), arabic-dotless.
    Isolated,
    /// Initial style.
    ///
    /// Supported characters: arabic excluding ا (U+0627), د–ز
    /// (U+062F..U+0632), ط (U+0637), ظ (U+0638), و (U+0648).
    Initial,
    /// Tailed style.
    ///
    /// Supported characters: arabic excluding ا (U+0627), ب (U+0628),
    /// ت (U+062A), ث (U+062B),  د–ز (U+062F..U+0632), ط (U+0637), ظ (U+0638),
    /// ف (U+0641), ك (U+0643), م (U+0645), ه (U+0647), و (U+0648), and
    /// arabic-dotless excluding ٮ (U+066E), ڡ (U+06A1).
    Tailed,
    /// Stretched style.
    ///
    /// Supported characters: arabic excluding ا (U+0627), د–ز
    /// (U+062F..U+0632), ل (U+0644), و (U+0648), and arabic-dotless excluding
    /// ٯ (U+066F), ں (U+06BA).
    Stretched,
    /// Looped style.
    ///
    /// Supported characters: arabic excluding ك (U+0643).
    Looped,
    /// Double-struck style. Also known as open-face style or blackboard-bold
    /// style.
    ///
    /// Supported characters: digits, latin, arabic excluding ا (U+0627),
    /// ك (U+0643), ه (U+0647), and the extras ∑ (U+2211), Γ (U+0393), Π
    /// (U+03A0), γ (U+03B3), π (U+03C0).
    DoubleStruck,
    /// Italic double-struck style. Also known as italic open-face style or
    /// italic blackboard-bold style.
    ///
    /// This is an exceptional style as only the following Latin letters are
    /// supported: D (U+0044), d (U+0064), e (U+0065), i (U+0069), j (U+006A).
    DoubleStruckItalic,
    /// Chancery variant of script style.
    ///
    /// Supported characters: latin.
    Chancery,
    /// Chancery variant of bold script style.
    ///
    /// Supported characters: latin.
    BoldChancery,
    /// Roundhand variant of script style.
    ///
    /// Supported characters: latin.
    Roundhand,
    /// Roundhand variant of bold script style.
    ///
    /// Supported characters: latin.
    BoldRoundhand,
}

/// Returns an iterator that yields the styled equivalent of a `char`.
///
/// This `struct` is created by the [`to_style`] function. See its
/// documentation for more.
#[derive(Debug, Clone)]
pub struct ToStyle(core::array::IntoIter<char, 2>);

impl ToStyle {
    #[inline]
    fn new(chars: [char; 2]) -> ToStyle {
        let mut iter = chars.into_iter();
        if chars[1] == '\0' {
            iter.next_back();
        }
        ToStyle(iter)
    }
}

impl Iterator for ToStyle {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn fold<Acc, Fold>(self, init: Acc, fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.0.fold(init, fold)
    }

    fn count(self) -> usize {
        self.0.count()
    }

    fn last(self) -> Option<Self::Item> {
        self.0.last()
    }
}

impl DoubleEndedIterator for ToStyle {
    fn next_back(&mut self) -> Option<char> {
        self.0.next_back()
    }

    fn rfold<Acc, Fold>(self, init: Acc, rfold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        self.0.rfold(init, rfold)
    }
}

impl ExactSizeIterator for ToStyle {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl FusedIterator for ToStyle {}

impl fmt::Display for ToStyle {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in self.0.clone() {
            f.write_char(c)?;
        }
        Ok(())
    }
}

/// Returns an iterator that yields the styled conversion of a `char`, as
/// specified by `style`, as one or more `char`s.
///
/// # Examples
///
/// ```
/// use codex::styling::{to_style, MathStyle};
///
/// assert_eq!("𞺚", to_style('ظ', MathStyle::Looped).to_string());
/// assert_eq!("𝒬\u{fe00}", to_style('Q', MathStyle::Chancery).to_string());
///
/// let s = "xγΩAذح1∑س"
///     .chars()
///     .flat_map(|c| to_style(c, MathStyle::DoubleStruck))
///     .collect::<String>();
/// assert_eq!("𝕩ℽΩ𝔸𞺸𞺧𝟙⅀𞺮", s);
/// ```
pub fn to_style(c: char, style: MathStyle) -> ToStyle {
    use conversions::*;
    use MathStyle::*;
    let styled = match style {
        Plain => [c, '\0'],
        Bold => [to_bold(c), '\0'],
        Italic => [to_italic(c), '\0'],
        BoldItalic => [to_bold_italic(c), '\0'],
        Script => [to_script(c), '\0'],
        BoldScript => [to_bold_script(c), '\0'],
        Fraktur => [to_fraktur(c), '\0'],
        BoldFraktur => [to_bold_fraktur(c), '\0'],
        SansSerif => [to_sans_serif(c), '\0'],
        SansSerifBold => [to_sans_serif_bold(c), '\0'],
        SansSerifItalic => [to_sans_serif_italic(c), '\0'],
        SansSerifBoldItalic => [to_sans_serif_bold_italic(c), '\0'],
        Monospace => [to_monospace(c), '\0'],
        Isolated => [to_isolated(c), '\0'],
        Initial => [to_initial(c), '\0'],
        Tailed => [to_tailed(c), '\0'],
        Stretched => [to_stretched(c), '\0'],
        Looped => [to_looped(c), '\0'],
        DoubleStruck => [to_double_struck(c), '\0'],
        DoubleStruckItalic => [to_double_struck_italic(c), '\0'],
        Chancery => to_chancery(c),
        BoldChancery => to_bold_chancery(c),
        Roundhand => to_roundhand(c),
        BoldRoundhand => to_bold_roundhand(c),
    };
    ToStyle::new(styled)
}

/// Functions which convert a `char` to its specified styled form.
///
/// Sourced from:
/// - [Unicode Core Specification - Section 22.2, Letterlike Symbols]
/// - [Letterlike Symbols]
/// - [Mathematical Alphanumeric Symbols]
/// - [Arabic Mathematical Alphabetic Symbols]
///
/// [Unicode Core Specification - Section 22.2, Letterlike Symbols]: <https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-22/#G14143>
/// [Letterlike Symbols]: <https://unicode.org/charts/PDF/U2100.pdf>
/// [Mathematical Alphanumeric Symbols]: <https://unicode.org/charts/PDF/U1D400.pdf>
/// [Arabic Mathematical Alphabetic Symbols]: <https://unicode.org/charts/PDF/U1EE00.pdf>
mod conversions {
    const VARIATION_SELECTOR_1: char = '\u{FE00}';
    const VARIATION_SELECTOR_2: char = '\u{FE01}';

    /// The character given by adding `delta` to the codepoint of `c`.
    #[inline]
    fn apply_delta(c: char, delta: u32) -> char {
        std::char::from_u32((c as u32) + delta).unwrap()
    }

    pub fn to_bold(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Bold symbols (U+1D400..U+1D433)
            'A'..='Z' => 0x1D3BF,
            'a'..='z' => 0x1D3B9,
            // Bold Greek symbols (U+1D6A8..U+1D6DA)
            'Α'..='Ρ' => 0x1D317,
            'ϴ' => 0x1D2C5,
            'Σ'..='Ω' => 0x1D317,
            '∇' => 0x1B4BA,
            'α'..='ω' => 0x1D311,
            // Additional bold Greek symbols (U+1D6DB..U+1D6E1)
            '∂' => 0x1B4D9,
            'ϵ' => 0x1D2E7,
            'ϑ' => 0x1D30C,
            'ϰ' => 0x1D2EE,
            'ϕ' => 0x1D30A,
            'ϱ' => 0x1D2EF,
            'ϖ' => 0x1D30B,
            // Additional bold Greek symbols (U+1D7CA..U+1D7CB)
            'Ϝ'..='ϝ' => 0x1D3EE,
            // Bold digits (U+1D7CE..U+1D7D7)
            '0'..='9' => 0x1D79E,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_italic(c: char) -> char {
        let delta = match c {
            // Letterlike Symbols Block (U+2100..U+214F)
            // Letterlike symbols (U+2100..U+2134)
            'h' => 0x20A6,
            'ħ' => 0x1FE8,

            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Italic symbols (U+1D434..U+1D467)
            'A'..='Z' => 0x1D3F3,
            'a'..='z' => 0x1D3ED,
            // Dotless symbols (U+1D6A4..U+1D6A5)
            'ı' => 0x1D573,
            'ȷ' => 0x1D46E,
            // Italic Greek symbols (U+1D6E2..U+1D714)
            'Α'..='Ρ' => 0x1D351,
            'ϴ' => 0x1D2FF,
            'Σ'..='Ω' => 0x1D351,
            '∇' => 0x1B4F4,
            'α'..='ω' => 0x1D34B,
            // Additional italic Greek symbols (U+1D715..U+1D71B)
            '∂' => 0x1B513,
            'ϵ' => 0x1D321,
            'ϑ' => 0x1D346,
            'ϰ' => 0x1D328,
            'ϕ' => 0x1D344,
            'ϱ' => 0x1D329,
            'ϖ' => 0x1D345,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_bold_italic(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Bold italic symbols (U+1D468..U+1D49B)
            'A'..='Z' => 0x1D427,
            'a'..='z' => 0x1D421,
            // Bold italic Greek symbols (U+1D71C..U+1D74E)
            'Α'..='Ρ' => 0x1D38B,
            'ϴ' => 0x1D339,
            'Σ'..='Ω' => 0x1D38B,
            '∇' => 0x1B52E,
            'α'..='ω' => 0x1D385,
            // Additional bold italic Greek symbols (U+1D74F..U+1D755)
            '∂' => 0x1B54D,
            'ϵ' => 0x1D35B,
            'ϑ' => 0x1D380,
            'ϰ' => 0x1D362,
            'ϕ' => 0x1D37E,
            'ϱ' => 0x1D363,
            'ϖ' => 0x1D37F,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_script(c: char) -> char {
        let delta = match c {
            // Letterlike Symbols Block (U+2100..U+214F)
            // Letterlike symbols (U+2100..U+2134)
            'g' => 0x20A3,
            'H' => 0x20C3,
            'I' => 0x20C7,
            'L' => 0x20C6,
            'R' => 0x20C9,
            'B' => 0x20EA,
            'e' => 0x20CA,
            'E'..='F' => 0x20EB,
            'M' => 0x20E6,
            'o' => 0x20C5,

            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Script symbols (U+1D49C..U+1D4CF)
            'A'..='Z' => 0x1D45B,
            'a'..='z' => 0x1D455,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_bold_script(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Bold script symbols (U+1D4D0..U+1D503)
            'A'..='Z' => 0x1D48F,
            'a'..='z' => 0x1D489,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_fraktur(c: char) -> char {
        let delta = match c {
            // Letterlike Symbols Block (U+2100..U+214F)
            // Letterlike symbols (U+2100..U+2134)
            'H' => 0x20C4,
            'I' => 0x20C8,
            'R' => 0x20CA,
            'Z' => 0x20CE,
            'C' => 0x20EA,

            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Fraktur symbols (U+1D504..U+1D537)
            'A'..='Z' => 0x1D4C3,
            'a'..='z' => 0x1D4BD,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_bold_fraktur(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Bold Fraktur symbols (U+1D56C..U+1D59F)
            'A'..='Z' => 0x1D52B,
            'a'..='z' => 0x1D525,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_sans_serif(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Sans-serif symbols (U+1D5A0..U+1D5D3)
            'A'..='Z' => 0x1D55F,
            'a'..='z' => 0x1D559,
            // Sans-serif digits (U+1D7E2..U+1D7EB)
            '0'..='9' => 0x1D7B2,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_sans_serif_bold(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Sans-serif bold symbols (U+1D5D4..U+1D607)
            'A'..='Z' => 0x1D593,
            'a'..='z' => 0x1D58D,
            // Sans-serif bold Greek symbols (U+1D756..U+1D788)
            'Α'..='Ρ' => 0x1D3C5,
            'ϴ' => 0x1D373,
            'Σ'..='Ω' => 0x1D3C5,
            '∇' => 0x1B568,
            'α'..='ω' => 0x1D3BF,
            // Additional sans-serif bold Greek symbols (U+1D789..U+1D78F)
            '∂' => 0x1B587,
            'ϵ' => 0x1D395,
            'ϑ' => 0x1D3BA,
            'ϰ' => 0x1D39C,
            'ϕ' => 0x1D3B8,
            'ϱ' => 0x1D39D,
            'ϖ' => 0x1D3B9,
            // Sans-serif bold digits (U+1D7EC..U+1D7F5)
            '0'..='9' => 0x1D7BC,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_sans_serif_italic(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Sans-serif italic symbols (U+1D608..U+1D63B)
            'A'..='Z' => 0x1D5C7,
            'a'..='z' => 0x1D5C1,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_sans_serif_bold_italic(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Sans-serif bold italic symbols (U+1D63C..U+1D66F)
            'A'..='Z' => 0x1D5FB,
            'a'..='z' => 0x1D5F5,
            // Sans-serif bold italic Greek symbols (U+1D790..U+1D7C2)
            'Α'..='Ρ' => 0x1D3FF,
            'ϴ' => 0x1D3AD,
            'Σ'..='Ω' => 0x1D3FF,
            '∇' => 0x1B5A2,
            'α'..='ω' => 0x1D3F9,
            // Additional sans-serif bold italic Greek symbols (U+1D7C3..U+1D7C9)
            '∂' => 0x1B5C1,
            'ϵ' => 0x1D3CF,
            'ϑ' => 0x1D3F4,
            'ϰ' => 0x1D3D6,
            'ϕ' => 0x1D3F2,
            'ϱ' => 0x1D3D7,
            'ϖ' => 0x1D3F3,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_monospace(c: char) -> char {
        let delta = match c {
            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Monospace symbols (U+1D670..U+1D6A3)
            'A'..='Z' => 0x1D62F,
            'a'..='z' => 0x1D629,
            // Monospace digits (U+1D7F6..U+1D7FF)
            '0'..='9' => 0x1D7C6,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_isolated(c: char) -> char {
        let delta = match c {
            // Arabic Mathematical Alphabetic Symbols Block (U+1EE00..U+1EEFF)
            // Isolated symbols (U+1EE00..U+1EE1F)
            'ا'..='ب' => 0x1E7D9,
            'ج' => 0x1E7D6,
            'د' => 0x1E7D4,
            'و' => 0x1E7BD,
            'ز' => 0x1E7D4,
            'ح' => 0x1E7DA,
            'ط' => 0x1E7D1,
            'ي' => 0x1E7BF,
            'ك'..='ن' => 0x1E7C7,
            'س' => 0x1E7DB,
            'ع' => 0x1E7D6,
            'ف' => 0x1E7CF,
            'ص' => 0x1E7DC,
            'ق' => 0x1E7D0,
            'ر' => 0x1E7E2,
            'ش' => 0x1E7E0,
            'ت'..='ث' => 0x1E7EB,
            'خ' => 0x1E7E9,
            'ذ' => 0x1E7E8,
            'ض' => 0x1E7E3,
            'ظ' => 0x1E7E2,
            'غ' => 0x1E7E1,
            'ٮ' => 0x1E7AE,
            'ں' => 0x1E763,
            'ڡ' => 0x1E77D,
            'ٯ' => 0x1E7B0,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_initial(c: char) -> char {
        let delta = match c {
            // Arabic Mathematical Alphabetic Symbols Block (U+1EE00..U+1EEFF)
            // Initial symbols (U+1EE21..U+1EE3B)
            'ب' => 0x1E7F9,
            'ج' => 0x1E7F6,
            'ه' => 0x1E7DD,
            'ح' => 0x1E7FA,
            'ي' => 0x1E7DF,
            'ك'..='ن' => 0x1E7E7,
            'س' => 0x1E7FB,
            'ع' => 0x1E7F6,
            'ف' => 0x1E7EF,
            'ص' => 0x1E7FC,
            'ق' => 0x1E7F0,
            'ش' => 0x1E800,
            'ت'..='ث' => 0x1E80B,
            'خ' => 0x1E809,
            'ض' => 0x1E803,
            'غ' => 0x1E801,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_tailed(c: char) -> char {
        let delta = match c {
            // Arabic Mathematical Alphabetic Symbols Block (U+1EE00..U+1EEFF)
            // Tailed symbols (U+1EE42..U+1EE5F)
            'ج' => 0x1E816,
            'ح' => 0x1E81A,
            'ي' => 0x1E7FF,
            'ل' => 0x1E807,
            'ن' => 0x1E807,
            'س' => 0x1E81B,
            'ع' => 0x1E816,
            'ص' => 0x1E81C,
            'ق' => 0x1E810,
            'ش' => 0x1E820,
            'خ' => 0x1E829,
            'ض' => 0x1E823,
            'غ' => 0x1E821,
            'ں' => 0x1E7A3,
            'ٯ' => 0x1E7F0,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_stretched(c: char) -> char {
        let delta = match c {
            // Arabic Mathematical Alphabetic Symbols Block (U+1EE00..U+1EEFF)
            // Stretched symbols (U+1EE61..U+1EE7E)
            'ب' => 0x1E839,
            'ج' => 0x1E836,
            'ه' => 0x1E81D,
            'ح' => 0x1E83A,
            'ط' => 0x1E831,
            'ي' => 0x1E81F,
            'ك' => 0x1E827,
            'م'..='ن' => 0x1E827,
            'س' => 0x1E83B,
            'ع' => 0x1E836,
            'ف' => 0x1E82F,
            'ص' => 0x1E83C,
            'ق' => 0x1E830,
            'ش' => 0x1E840,
            'ت'..='ث' => 0x1E84B,
            'خ' => 0x1E849,
            'ض' => 0x1E843,
            'ظ' => 0x1E842,
            'غ' => 0x1E841,
            'ٮ' => 0x1E80E,
            'ڡ' => 0x1E7DD,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_looped(c: char) -> char {
        let delta = match c {
            // Arabic Mathematical Alphabetic Symbols Block (U+1EE00..U+1EEFF)
            // Looped symbols (U+1EE80..U+1EE9B)
            'ا'..='ب' => 0x1E859,
            'ج' => 0x1E856,
            'د' => 0x1E854,
            'ه'..'و' => 0x1E83D,
            'ز' => 0x1E854,
            'ح' => 0x1E85A,
            'ط' => 0x1E851,
            'ي' => 0x1E83F,
            'ل'..='ن' => 0x1E847,
            'س' => 0x1E85B,
            'ع' => 0x1E856,
            'ف' => 0x1E84F,
            'ص' => 0x1E85C,
            'ق' => 0x1E850,
            'ر' => 0x1E862,
            'ش' => 0x1E860,
            'ت'..='ث' => 0x1E86B,
            'خ' => 0x1E869,
            'ذ' => 0x1E868,
            'ض' => 0x1E863,
            'ظ' => 0x1E862,
            'غ' => 0x1E861,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_double_struck(c: char) -> char {
        let delta = match c {
            // Letterlike Symbols Block (U+2100..U+214F)
            // Letterlike symbols (U+2100..U+2134)
            'C' => 0x20BF,
            'H' => 0x20C5,
            'N' => 0x20C7,
            'P'..='Q' => 0x20C9,
            'R' => 0x20CB,
            'Z' => 0x20CA,
            // Additional letterlike symbols (U+2139..U+213F)
            'π' => 0x1D7C,
            'γ' => 0x1D8A,
            'Γ' => 0x1DAB,
            'Π' => 0x1D9F,
            // Double-struck large operator (U+2140)
            '∑' => return '⅀', // delta is negative

            // Mathematical Alphanumeric Symbols Block (U+1D400..U+1D7FF)
            // Double-struck symbols (U+1D538..U+1D56B)
            'A'..='Z' => 0x1D4F7,
            'a'..='z' => 0x1D4F1,
            // Double-struck digits (U+1D7D8..U+1D7E1)
            '0'..='9' => 0x1D7A8,

            // Arabic Mathematical Alphabetic Symbols Block (U+1EE00..U+1EEFF)
            // Double-struck symbols (U+1EEA1..U+1EEBB)
            'ب' => 0x1E879,
            'ج' => 0x1E876,
            'د' => 0x1E874,
            'و' => 0x1E85D,
            'ز' => 0x1E874,
            'ح' => 0x1E87A,
            'ط' => 0x1E871,
            'ي' => 0x1E85F,
            'ل'..='ن' => 0x1E867,
            'س' => 0x1E87B,
            'ع' => 0x1E876,
            'ف' => 0x1E86F,
            'ص' => 0x1E87C,
            'ق' => 0x1E870,
            'ر' => 0x1E882,
            'ش' => 0x1E880,
            'ت'..='ث' => 0x1E88B,
            'خ' => 0x1E889,
            'ذ' => 0x1E888,
            'ض' => 0x1E883,
            'ظ' => 0x1E882,
            'غ' => 0x1E881,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_double_struck_italic(c: char) -> char {
        let delta = match c {
            // Letterlike Symbols Block (U+2100..U+214F)
            // Double-struck italic math symbols (U+2145..U+2149)
            'D' => 0x2101,
            'd'..='e' => 0x20E2,
            'i'..='j' => 0x20DF,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    pub fn to_chancery(c: char) -> [char; 2] {
        // Standardized Variation Sequences (uppercase Latin script characters)
        // Variation Sequences (lowercase Latin script characters)
        [to_script(c), VARIATION_SELECTOR_1]
    }

    pub fn to_bold_chancery(c: char) -> [char; 2] {
        // Variation Sequences (Latin script characters)
        [to_bold_script(c), VARIATION_SELECTOR_1]
    }

    pub fn to_roundhand(c: char) -> [char; 2] {
        // Standardized Variation Sequences (uppercase Latin script characters)
        // Variation Sequences (lowercase Latin script characters)
        [to_script(c), VARIATION_SELECTOR_2]
    }

    pub fn to_bold_roundhand(c: char) -> [char; 2] {
        // Variation Sequences (Latin script characters)
        [to_bold_script(c), VARIATION_SELECTOR_2]
    }
}
