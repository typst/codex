/*!
Style mathematical symbols in Unicode.
*/

/// A styled form for mathematical symbols.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum MathStyle {
    /// Normal style and isolated style (for Arabic). May be serif or
    /// sans-serif depending upon the font.
    ///
    /// This is the default.
    #[default]
    Serif,
    /// Normal bold style. May be serif or sans-serif depending upon the font.
    SerifBold,
    /// Normal italic style. May be serif or sans-serif depending upon the
    /// font.
    SerifItalic,
    /// Normal bold italic style. May be serif or sans-serif depending upon the
    /// font.
    SerifItalicBold,
    /// Sans-serif style.
    SansSerif,
    /// Sans-serif bold style.
    SansSerifBold,
    /// Sans-serif italic style.
    SansSerifItalic,
    /// Sans-serif bold italic style.
    SansSerifItalicBold,
    /// Fraktur style. Also known as black-letter.
    Fraktur,
    /// Bold fraktur style.
    FrakturBold,
    /// Script style.
    Script,
    /// Bold script style.
    ScriptBold,
    /// Chancery variant of script style.
    Chancery,
    /// Chancery variant of bold script style.
    ChanceryBold,
    /// Roundhand variant of script style.
    Roundhand,
    /// Roundhand variant of bold script style.
    RoundhandBold,
    /// Double-struck style. Also known as open-face or blackboard-bold.
    DoubleStruck,
    /// Double-struck italic style.
    DoubleStruckItalic,
    /// Monospace style.
    Monospace,
    /// Initial style (for Arabic).
    Initial,
    /// Tailed style (for Arabic).
    Tailed,
    /// Looped style (for Arabic).
    Looped,
    /// Stretched style (for Arabic).
    Stretched,
}

/// Converts a [`char`] to the styled form specified by `style`.
///
/// Note that some styles will convert a character into multiple characters,
/// hence this function always returns an array of two characters.
///
/// # Examples
///
/// ```
/// use codex::styling::{to_style, MathStyle};
///
/// assert_eq!(['𝑴', '\0'], to_style('M', MathStyle::SerifItalicBold));
/// assert_eq!(['𞸪', '\0'], to_style('ك', MathStyle::Initial));
/// assert_eq!(['ⅈ', '\0'], to_style('i', MathStyle::DoubleStruckItalic));
/// assert_eq!(['𝓴', '\u{fe01}'], to_style('k', MathStyle::RoundhandBold));
/// ```
pub fn to_style(c: char, style: MathStyle) -> [char; 2] {
    use mappings::*;
    use MathStyle::*;
    match style {
        Serif => [to_serif(c), '\0'],
        SerifBold => [to_serif_bold(c), '\0'],
        SerifItalic => [to_serif_italic(c), '\0'],
        SerifItalicBold => [to_serif_italic_bold(c), '\0'],
        SansSerif => [to_sans_serif(c), '\0'],
        SansSerifBold => [to_sans_serif_bold(c), '\0'],
        SansSerifItalic => [to_sans_serif_italic(c), '\0'],
        SansSerifItalicBold => [to_sans_serif_italic_bold(c), '\0'],
        Fraktur => [to_fraktur(c), '\0'],
        FrakturBold => [to_fraktur_bold(c), '\0'],
        Script => [to_script(c), '\0'],
        ScriptBold => [to_script_bold(c), '\0'],
        Chancery => to_chancery(c),
        ChanceryBold => to_chancery_bold(c),
        Roundhand => to_roundhand(c),
        RoundhandBold => to_roundhand_bold(c),
        DoubleStruck => [to_double_struck(c), '\0'],
        DoubleStruckItalic => [to_double_struck_italic(c), '\0'],
        Monospace => [to_monospace(c), '\0'],
        Initial => [to_initial(c), '\0'],
        Tailed => [to_tailed(c), '\0'],
        Looped => [to_looped(c), '\0'],
        Stretched => [to_stretched(c), '\0'],
    }
}

/// A trait for converting each [`char`] in a value to its specified styled
/// form.
pub trait MathStyling {
    /// Converts each [`char`] in the given value to the styled form given by
    /// `style`.
    ///
    /// Note that some styles will convert a character into multiple
    /// characters.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use codex::styling::{MathStyle, MathStyling};
    ///
    /// let s = String::from("mono");
    ///
    /// assert_eq!("𝚖𝚘𝚗𝚘", s.to_style(MathStyle::Monospace));
    /// ```
    ///
    /// One character can become multiple:
    ///
    /// ```
    /// # use codex::styling::{MathStyle, MathStyling};
    /// let s = String::from("foo");
    ///
    /// assert_eq!("𝒻\u{fe00}ℴ\u{fe00}ℴ\u{fe00}", s.to_style(MathStyle::Chancery));
    /// ```
    fn to_style(&self, style: MathStyle) -> Self;
}

impl MathStyling for String {
    fn to_style(&self, style: MathStyle) -> Self {
        let str = self.as_str();
        let mut styled = Self::with_capacity(str.len());
        for c in str.chars() {
            match to_style(c, style) {
                [a, '\0'] => styled.push(a),
                [a, b] => {
                    styled.push(a);
                    styled.push(b);
                }
            }
        }
        styled
    }
}

/// Functions which map a [`char`] to its specified styled form.
///
/// Sourced from:
/// - [Unicode Core Specification]
/// - [Mathematical Alphanumeric Symbols]
/// - [Arabic Mathematical Alphabetic Symbols]
/// - <https://www.w3.org/TR/mathml-core/#new-text-transform-mappings>
///
/// ## Examples
///
/// ```
/// use codex::styling::mappings::*;
///
/// assert_eq!('𝔭', to_fraktur('p'));
/// assert_eq!('𝞌', to_sans_serif_bold('ϰ'));
/// assert_eq!('𞺰', to_double_struck('ف'));
/// assert_eq!(['𝒫', '\u{FE01}'], to_roundhand('P'));
/// ```
///
/// [Unicode Core Specification]: <https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-22/#G16021>
/// [Mathematical Alphanumeric Symbols]: <https://unicode.org/charts/PDF/U1D400.pdf>
/// [Arabic Mathematical Alphabetic Symbols]: <https://unicode.org/charts/PDF/U1EE00.pdf>
pub mod mappings {
    const VARIATION_SELECTOR_1: char = '\u{FE00}';
    const VARIATION_SELECTOR_2: char = '\u{FE01}';

    /// The character given by adding `delta` to the codepoint of `c`.
    #[inline]
    fn apply_delta(c: char, delta: u32) -> char {
        std::char::from_u32((c as u32) + delta).unwrap()
    }

    /// To normal and isolated symbols.
    ///
    /// This is the normal unstyled form, and may be serif or sans-serif
    /// depending upon the font.
    ///
    /// This is also the isolated style, the normal form for Arabic.
    pub fn to_serif(c: char) -> char {
        c
    }

    /// To bold symbols.
    ///
    /// This is the normal bold form, and may be serif or sans-serif depending
    /// upon the font.
    pub fn to_serif_bold(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D3BF,
            'a'..='z' => 0x1D3B9,
            'Α'..='Ρ' => 0x1D317,
            'ϴ' => 0x1D2C5,
            'Σ'..='Ω' => 0x1D317,
            '∇' => 0x1B4BA,
            'α'..='ω' => 0x1D311,
            '∂' => 0x1B4D9,
            'ϵ' => 0x1D2E7,
            'ϑ' => 0x1D30C,
            'ϰ' => 0x1D2EE,
            'ϕ' => 0x1D30A,
            'ϱ' => 0x1D2EF,
            'ϖ' => 0x1D30B,
            'Ϝ' | 'ϝ' => 0x1D3EE,
            '0'..='9' => 0x1D79E,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To italic symbols.
    ///
    /// This is the normal italic form, and may be serif or sans-serif
    /// depending upon the font.
    pub fn to_serif_italic(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D3F3,
            'h' => 0x20A6,
            'a'..='z' => 0x1D3ED,
            'ı' => 0x1D573,
            'ȷ' => 0x1D46E,
            'Α'..='Ρ' => 0x1D351,
            'ϴ' => 0x1D2FF,
            'Σ'..='Ω' => 0x1D351,
            '∇' => 0x1B4F4,
            'α'..='ω' => 0x1D34B,
            '∂' => 0x1B513,
            'ϵ' => 0x1D321,
            'ϑ' => 0x1D346,
            'ϰ' => 0x1D328,
            'ϕ' => 0x1D344,
            'ϱ' => 0x1D329,
            'ϖ' => 0x1D345,
            // Missing from MathML Core.
            'ħ' => 0x1FE8,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To bold italic symbols.
    ///
    /// This is the normal bold italic form, and may be serif or sans-serif
    /// depending upon the font.
    pub fn to_serif_italic_bold(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D427,
            'a'..='z' => 0x1D421,
            'Α'..='Ρ' => 0x1D38B,
            'ϴ' => 0x1D339,
            'Σ'..='Ω' => 0x1D38B,
            '∇' => 0x1B52E,
            'α'..='ω' => 0x1D385,
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

    /// To sans-serif symbols.
    pub fn to_sans_serif(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D55F,
            'a'..='z' => 0x1D559,
            '0'..='9' => 0x1D7B2,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To sans-serif bold symbols.
    pub fn to_sans_serif_bold(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D593,
            'a'..='z' => 0x1D58D,
            'Α'..='Ρ' => 0x1D3C5,
            'ϴ' => 0x1D373,
            'Σ'..='Ω' => 0x1D3C5,
            '∇' => 0x1B568,
            'α'..='ω' => 0x1D3BF,
            '∂' => 0x1B587,
            'ϵ' => 0x1D395,
            'ϑ' => 0x1D3BA,
            'ϰ' => 0x1D39C,
            'ϕ' => 0x1D3B8,
            'ϱ' => 0x1D39D,
            'ϖ' => 0x1D3B9,
            '0'..='9' => 0x1D7BC,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To sans-serif italic symbols.
    pub fn to_sans_serif_italic(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D5C7,
            'a'..='z' => 0x1D5C1,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To sans-serif bold italic symbols.
    pub fn to_sans_serif_italic_bold(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D593,
            'a'..='z' => 0x1D58D,
            'Α'..='Ρ' => 0x1D3C5,
            'ϴ' => 0x1D373,
            'Σ'..='Ω' => 0x1D3C5,
            '∇' => 0x1B568,
            'α'..='ω' => 0x1D3BF,
            '∂' => 0x1B587,
            'ϵ' => 0x1D395,
            'ϑ' => 0x1D3BA,
            'ϰ' => 0x1D39C,
            'ϕ' => 0x1D3B8,
            'ϱ' => 0x1D39D,
            'ϖ' => 0x1D3B9,
            '0'..='9' => 0x1D7BC,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To fraktur symbols.
    ///
    /// This style is sometimes known as black-letter.
    pub fn to_fraktur(c: char) -> char {
        let delta = match c {
            'C' => 0x20EA,
            'H' => 0x20C4,
            'I' => 0x20C8,
            'R' => 0x20CA,
            'Z' => 0x20CE,
            'A'..='Z' => 0x1D4C3,
            'a'..='z' => 0x1D4BD,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To bold fraktur symbols.
    pub fn to_fraktur_bold(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D52B,
            'a'..='z' => 0x1D525,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To script symbols.
    pub fn to_script(c: char) -> char {
        let delta = match c {
            'B' => 0x20EA,
            'E'..='F' => 0x20EB,
            'H' => 0x20C3,
            'I' => 0x20C7,
            'L' => 0x20C6,
            'M' => 0x20E6,
            'R' => 0x20C9,
            'A'..='Z' => 0x1D45B,
            'e' => 0x20CA,
            'g' => 0x20A3,
            'o' => 0x20C5,
            'a'..='z' => 0x1D455,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To bold script symbols.
    pub fn to_script_bold(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D48F,
            'a'..='z' => 0x1D489,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To chancery style of script symbols.
    ///
    /// This function returns a variation sequence using the variation selector
    /// `U+FE00`, as specified in the [`StandardizedVariants.txt`] file from
    /// the Unicode Character Database. Note that only the capital Latin
    /// letters are standardized variation sequences, but this function also
    /// maps the small Latin letters.
    ///
    /// [`StandardizedVariants.txt`]: <https://www.unicode.org/Public/UNIDATA/StandardizedVariants.txt>
    pub fn to_chancery(c: char) -> [char; 2] {
        [to_script(c), VARIATION_SELECTOR_1]
    }

    /// To chancery style of bold script symbols.
    ///
    /// This function returns a variation sequence using the variation selector
    /// `U+FE00`. Note however that the bold script symbols are _not_ specified
    /// as standardized variation sequences by Unicode.
    pub fn to_chancery_bold(c: char) -> [char; 2] {
        [to_script_bold(c), VARIATION_SELECTOR_1]
    }

    /// To roundhand style of script symbols.
    ///
    /// This function returns a variation sequence using the variation selector
    /// `U+FE01`, as specified in the [`StandardizedVariants.txt`] file from
    /// the Unicode Character Database. Note that only the capital Latin
    /// letters are standardized variation sequences, but this function also
    /// maps the small Latin letters.
    ///
    /// [`StandardizedVariants.txt`]: <https://www.unicode.org/Public/UNIDATA/StandardizedVariants.txt>
    pub fn to_roundhand(c: char) -> [char; 2] {
        [to_script(c), VARIATION_SELECTOR_2]
    }

    /// To roundhand style of bold script symbols.
    ///
    /// This function returns a variation sequence using the variation selector
    /// `U+FE01`. Note however that the bold script symbols are _not_ specified
    /// as standardized variation sequences by Unicode.
    pub fn to_roundhand_bold(c: char) -> [char; 2] {
        [to_script_bold(c), VARIATION_SELECTOR_2]
    }

    /// To double-struck symbols.
    ///
    /// This style is sometimes known as open-face or blackboard-bold.
    pub fn to_double_struck(c: char) -> char {
        let delta = match c {
            'C' => 0x20BF,
            'H' => 0x20C5,
            'N' => 0x20C7,
            'P'..='Q' => 0x20C9,
            'R' => 0x20CB,
            'Z' => 0x20CA,
            'A'..='Z' => 0x1D4F7,
            'a'..='z' => 0x1D4F1,
            '0'..='9' => 0x1D7A8,
            'ب' => 0x1E879,
            'ج' | 'ع' => 0x1E876,
            'د' | 'ز' => 0x1E874,
            'و' => 0x1E85D,
            'ح' => 0x1E87A,
            'ط' => 0x1E871,
            'ي' => 0x1E85F,
            'ل'..='ن' => 0x1E867,
            'س' => 0x1E87B,
            'ف' => 0x1E86F,
            'ص' => 0x1E87C,
            'ق' => 0x1E870,
            'ر' | 'ظ' => 0x1E882,
            'ش' => 0x1E880,
            'ت'..='ث' => 0x1E88B,
            'خ' => 0x1E889,
            'ذ' => 0x1E888,
            'ض' => 0x1E883,
            'غ' => 0x1E881,
            // Missing from MathML Core.
            'Γ' => 0x1DAB,
            'Π' => 0x1D9F,
            'γ' => 0x1D8A,
            'π' => 0x1D7C,
            '∑' => return '⅀', // Delta is negative.
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To double-struck italic symbols.
    ///
    /// Note that there does not exist codepoints in Unicode for all Latin
    /// letters. There are only a few double-struck italic symbols, and they
    /// are present solely in the Letterlike Symbols block.
    pub fn to_double_struck_italic(c: char) -> char {
        let delta = match c {
            // Missing from MathML Core.
            'D' => 0x2101,
            'd'..='e' => 0x20E2,
            'i'..='j' => 0x20DF,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To monospace symbols.
    pub fn to_monospace(c: char) -> char {
        let delta = match c {
            'A'..='Z' => 0x1D62F,
            'a'..='z' => 0x1D629,
            '0'..='9' => 0x1D7C6,
            _ => return c,
        };
        apply_delta(c, delta)
    }

    /// To initial symbols.
    pub fn to_initial(c: char) -> char {
        let delta = match c {
            'ب' => 0x1E7F9,
            'ج' | 'ع' => 0x1E7F6,
            'ه' => 0x1E7DD,
            'ح' => 0x1E7FA,
            'ي' => 0x1E7DF,
            'ك'..='ن' => 0x1E7E7,
            'س' => 0x1E7FB,
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

    /// To tailed symbols.
    pub fn to_tailed(c: char) -> char {
        let delta = match c {
            'ج' | 'ع' => 0x1E816,
            'ح' => 0x1E81A,
            'ي' => 0x1E7FF,
            'ل' | 'ن' => 0x1E807,
            'س' => 0x1E81B,
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

    /// To stretched symbols.
    pub fn to_stretched(c: char) -> char {
        let delta = match c {
            'ب' => 0x1E839,
            'ج' | 'ع' => 0x1E836,
            'ه' => 0x1E81D,
            'ح' => 0x1E83A,
            'ط' => 0x1E831,
            'ي' => 0x1E81F,
            'ك' | 'م'..='ن' => 0x1E827,
            'س' => 0x1E83B,
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

    /// To looped symbols.
    pub fn to_looped(c: char) -> char {
        let delta = match c {
            'ا'..='ب' => 0x1E859,
            'ج' | 'ع' => 0x1E856,
            'د' | 'ز' => 0x1E854,
            'ه'..='و' => 0x1E83D,
            'ح' => 0x1E85A,
            'ط' => 0x1E851,
            'ي' => 0x1E83F,
            'ل'..='ن' => 0x1E847,
            'س' => 0x1E85B,
            'ف' => 0x1E84F,
            'ص' => 0x1E85C,
            'ق' => 0x1E850,
            'ر' | 'ظ' => 0x1E862,
            'ش' => 0x1E860,
            'ت'..='ث' => 0x1E86B,
            'خ' => 0x1E869,
            'ذ' => 0x1E868,
            'ض' => 0x1E863,
            'غ' => 0x1E861,
            _ => return c,
        };
        apply_delta(c, delta)
    }
}
