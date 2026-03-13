//! Various ways of displaying non-negative integers.

use std::fmt::{Display, Formatter};

use chinese_number::{ChineseCase, ChineseVariant, from_u64_to_chinese_ten_thousand};

macro_rules! declare_named {
    (
        $( #[$attr:meta] )*
        $vis:vis enum $Ty:ident {
            $(
                $( #[$vattr:meta] )*
                $Value:ident = $name:literal $( ($shorthand:literal) )?,
            )*
        }
    ) => {
        $( #[$attr] )*
        $vis enum $Ty {
            $(
                $( #[$vattr] )*
                $Value,
            )*
        }

        impl $Ty {
            /// Returns the named numeral system associated with a name, if any.
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $( $name => Some(Self::$Value), )*
                    _ => None,
                }
            }

            /// Returns the name of a named numeral system.
            pub const fn name(self) -> &'static str {
                match self {
                    $( Self::$Value => $name, )*
                }
            }

            /// Returns the named numeral system associated with a shorthand.
            ///
            /// A shorthand is a short string that identifies a named numeral
            /// system, such as `"I"` for
            /// [upper Roman numerals](Self::UpperRoman), or `"א"` for
            /// [Hebrew numerals](Self::Hebrew). Usually, this is the number one
            /// represented in the system.
            pub fn from_shorthand(shorthand: &str) -> Option<Self> {
                match shorthand {
                    $( $( $shorthand => Some(Self::$Value), )? )*
                    _ => None,
                }
            }

            /// Returns the optional [shorthand](Self::from_shorthand) for a
            /// named numeral system.
            pub fn shorthand(self) -> Option<&'static str> {
                match self {
                    $( $( Self::$Value => Some($shorthand), )? )*
                    _ => None,
                }
            }

            /// Returns an iterator over the values of this type.
            pub fn iter() -> impl Iterator<Item = Self> {
                [
                    $( Self::$Value, )*
                ].into_iter()
            }
        }
    };
}

declare_named! {
    /// A list of named numeral systems.
    ///
    /// The underlying system of a named numeral system can be obtained with the
    /// [`system`](Self::system) method.
    #[non_exhaustive]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum NamedNumeralSystem {
        /// Decimal positional notation using
        /// [Western Arabic numerals](https://en.wikipedia.org/wiki/Arabic_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11
        Arabic = "arabic" ("1"),
        /// Circled decimal positional notation using
        /// [Western Arabic numerals](https://en.wikipedia.org/wiki/Arabic_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// Non-negative integers up to and including fifty can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > ⓪, ①, ②, ③, ④, ⑤, ⑥, ⑦, ⑧, ⑨, ⑩, ⑪
        CircledArabic = "arabic.o" ("①"),
        /// Double circled decimal positional notation using
        /// [Western Arabic numerals](https://en.wikipedia.org/wiki/Arabic_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// Positive integers up to and including ten can be represented.
        ///
        /// ## Example
        ///
        /// The integers from one to ten are represented as follows:
        ///
        /// > ⓵, ⓶, ⓷, ⓸, ⓹, ⓺, ⓻, ⓼, ⓽, ⓾
        DoubleCircledArabic = "arabic.oo" ("⓵"),
        /// Lowercase
        /// [Latin letters](https://en.wikipedia.org/wiki/Latin_alphabet).
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twenty-eight positive integers are represented as follows:
        ///
        /// > a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v,
        /// > w, x, y, z, aa, ab
        LowerLatin = "latin" ("a"),
        /// Uppercase
        /// [Latin letters](https://en.wikipedia.org/wiki/Latin_alphabet).
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twenty-eight positive integers are represented as follows:
        ///
        /// > A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
        /// > W, X, Y, Z, AA, AB
        UpperLatin = "Latin" ("A"),
        /// Lowercase
        /// [Roman numerals](https://en.wikipedia.org/wiki/Roman_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > n, i, ii, iii, iv, v, vi, vii, viii, ix, x, xi
        LowerRoman = "roman" ("i"),
        /// Uppercase
        /// [Roman numerals](https://en.wikipedia.org/wiki/Roman_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > N, I, II, III, IV, V, VI, VII, VIII, IX, X, XI
        UpperRoman = "Roman" ("I"),
        /// Lowercase
        /// [Greek numerals](https://en.wikipedia.org/wiki/Greek_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first thirteen non-negative integers are represented as follows:
        ///
        /// > 𐆊, α, β, γ, δ, ε, στ, ζ, η, θ, ι, ια, ιβ
        LowerGreek = "greek" ("α"),
        /// Uppercase
        /// [Greek numerals](https://en.wikipedia.org/wiki/Greek_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first thirteen non-negative integers are represented as follows:
        ///
        /// > 𐆊, Α, Β, Γ, Δ, Ε, ΣΤ, Ζ, Η, Θ, Ι, ΙΑ, ΙΒ
        UpperGreek = "Greek" ("Α"),
        /// Lowercase
        /// [Armenian numerals](https://en.wikipedia.org/wiki/Armenian_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > ա, բ, գ, դ, ե, զ, է, ը, թ, ժ, ժա, ժբ
        LowerArmenian = "armenian" ("ա"),
        /// Uppercase
        /// [Armenian numerals](https://en.wikipedia.org/wiki/Armenian_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > Ա, Բ, Գ, Դ, Ե, Զ, Է, Ը, Թ, Ժ, ԺԱ, ԺԲ
        UpperArmenian = "Armenian" ("Ա"),
        /// [Hebrew alphabetic numerals](https://en.wikipedia.org/wiki/Hebrew_numerals)
        /// without a
        /// [gershayim](https://en.wikipedia.org/wiki/Hebrew_numerals#Gershayim)
        /// or geresh.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > <span dir="auto">א</span>, <span dir="auto">ב</span>,
        /// > <span dir="auto">ג</span>, <span dir="auto">ד</span>,
        /// > <span dir="auto">ה</span>, <span dir="auto">ו</span>,
        /// > <span dir="auto">ז</span>, <span dir="auto">ח</span>,
        /// > <span dir="auto">ט</span>, <span dir="auto">י</span>,
        /// > <span dir="auto">יא</span>, <span dir="auto">יב</span>
        Hebrew = "hebrew" ("א"),
        /// Everyday ordinary simplified
        /// [Chinese numerals](https://en.wikipedia.org/wiki/Chinese_numerals#Ordinary_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first thirteen non-negative integers are represented as follows:
        ///
        /// > 零, 一, 二, 三, 四, 五, 六, 七, 八, 九, 十, 十一, 十二
        LowerSimplifiedChinese = "chinese.simple" ("一"),
        /// Financial ("capital") ordinary simplified
        /// [Chinese numerals](https://en.wikipedia.org/wiki/Chinese_numerals#Ordinary_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first thirteen non-negative integers are represented as follows:
        ///
        /// > 零, 壹, 贰, 叁, 肆, 伍, 陆, 柒, 捌, 玖, 拾, 拾壹, 拾贰
        UpperSimplifiedChinese = "Chinese.simple" ("壹"),
        /// Everyday ordinary traditional
        /// [Chinese numerals](https://en.wikipedia.org/wiki/Chinese_numerals#Ordinary_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first thirteen non-negative integers are represented as follows:
        ///
        /// > 零, 一, 二, 三, 四, 五, 六, 七, 八, 九, 十, 十一, 十二
        LowerTraditionalChinese = "chinese.trad",
        /// Financial ("capital") ordinary traditional
        /// [Chinese numerals](https://en.wikipedia.org/wiki/Chinese_numerals#Ordinary_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first thirteen non-negative integers are represented as follows:
        ///
        /// > 零, 壹, 貳, 參, 肆, 伍, 陸, 柒, 捌, 玖, 拾, 拾壹, 拾貳
        UpperTraditionalChinese = "Chinese.trad",
        /// Hiragana in the gojūon order. Includes n but excludes wi and we.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > あ, い, う, え, お, か, き, く, け, こ, さ, し
        HiraganaAiueo = "hiragana.aiueo" ("あ"),
        /// Hiragana in the iroha order. Includes wi and we but excludes n.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > い, ろ, は, に, ほ, へ, と, ち, り, ぬ, る, を
        HiraganaIroha = "hiragana.iroha" ("い"),
        /// Katakana in the gojūon order. Includes n but excludes wi and we.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > ア, イ, ウ, エ, オ, カ, キ, ク, ケ, コ, サ, シ
        KatakanaAiueo = "katakana.aiueo" ("ア"),
        /// Katakana in the iroha order. Includes wi and we but excludes n.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > イ, ロ, ハ, ニ, ホ, ヘ, ト, チ, リ, ヌ, ル, ヲ
        KatakanaIroha = "katakana.iroha" ("イ"),
        // TODO: Improve Korean numeral systems based on https://github.com/typst/typst/issues/7335.
        /// Korean jamo.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > ㄱ, ㄴ, ㄷ, ㄹ, ㅁ, ㅂ, ㅅ, ㅇ, ㅈ, ㅊ, ㅋ, ㅌ
        KoreanJamo = "korean.jamo" ("ㄱ"),
        /// Korean syllables.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > 가, 나, 다, 라, 마, 바, 사, 아, 자, 차, 카, 타
        KoreanSyllable = "korean.syllable" ("가"),
        /// Decimal positional notation using
        /// [Eastern Arabic numerals](https://en.wikipedia.org/wiki/Eastern_Arabic_numerals#Numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > <span dir="auto">٠</span>, <span dir="auto">١</span>,
        /// > <span dir="auto">٢</span>, <span dir="auto">٣</span>,
        /// > <span dir="auto">٤</span>, <span dir="auto">٥</span>,
        /// > <span dir="auto">٦</span>, <span dir="auto">٧</span>,
        /// > <span dir="auto">٨</span>, <span dir="auto">٩</span>,
        /// > <span dir="auto">١٠</span>, <span dir="auto">١١</span>
        EasternArabic = "arabic.eastern" ("١"),
        /// Decimal positional notation using the Persian variant of
        /// [Eastern Arabic numerals](https://en.wikipedia.org/wiki/Eastern_Arabic_numerals#Numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > <span dir="auto">۰</span>, <span dir="auto">۱</span>,
        /// > <span dir="auto">۲</span>, <span dir="auto">۳</span>,
        /// > <span dir="auto">۴</span>, <span dir="auto">۵</span>,
        /// > <span dir="auto">۶</span>, <span dir="auto">۷</span>,
        /// > <span dir="auto">۸</span>, <span dir="auto">۹</span>,
        /// > <span dir="auto">۱۰</span>, <span dir="auto">۱۱</span>
        Persian = "persian" ("۱"),
        /// Decimal positional notation using
        /// [Devanagari numerals](https://en.wikipedia.org/wiki/Devanagari_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > ०, १, २, ३, ४, ५, ६, ७, ८, ९, १०, ११
        Devanagari = "devanagari" ("१"),
        /// Decimal positional notation using
        /// [Bengali numerals](https://en.wikipedia.org/wiki/Bengali_numerals).
        ///
        /// ## Representable Numbers
        ///
        /// All non-negative integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve non-negative integers are represented as follows:
        ///
        /// > ০, ১, ২, ৩, ৪, ৫, ৬, ৭, ৮, ৯, ১০, ১১
        Bengali = "bengali" ("১"),
        /// Bengali letters.
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first twelve positive integers are represented as follows:
        ///
        /// > ক, খ, গ, ঘ, ঙ, চ, ছ, জ, ঝ, ঞ, ট, ঠ
        BengaliLetters = "bengali.letter" ("ক"),
        /// Repeating
        /// [note numbering symbols](https://en.wikipedia.org/wiki/Note_(typography)#Numbering_and_symbols).
        ///
        /// ## Representable Numbers
        ///
        /// All positive integers can be represented.
        ///
        /// ## Example
        ///
        /// The first fourteen positive integers are represented as follows:
        ///
        /// > *, †, ‡, §, ¶, ‖, **, ††, ‡‡, §§, ¶¶, ‖‖, ***, †††
        Symbols = "symbol" ("*"),
    }
}

impl NamedNumeralSystem {
    /// Returns the underlying numeral system.
    pub fn system(self) -> NumeralSystem<'static> {
        match self {
            Self::Arabic => NumeralSystem::Positional(&[
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
            ]),

            Self::CircledArabic => NumeralSystem::Fixed(&[
                '⓪', '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪', '⑫', '⑬',
                '⑭', '⑮', '⑯', '⑰', '⑱', '⑲', '⑳', '㉑', '㉒', '㉓', '㉔', '㉕', '㉖',
                '㉗', '㉘', '㉙', '㉚', '㉛', '㉜', '㉝', '㉞', '㉟', '㊱', '㊲', '㊳',
                '㊴', '㊵', '㊶', '㊷', '㊸', '㊹', '㊺', '㊻', '㊼', '㊽', '㊾', '㊿',
            ]),

            Self::DoubleCircledArabic => NumeralSystem::ZerolessFixed(&[
                '⓵', '⓶', '⓷', '⓸', '⓹', '⓺', '⓻', '⓼', '⓽', '⓾',
            ]),

            Self::LowerLatin => NumeralSystem::Bijective(&[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
                'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ]),

            Self::UpperLatin => NumeralSystem::Bijective(&[
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
                'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ]),

            Self::LowerRoman => NumeralSystem::Additive(&[
                ("m̅", 1000000),
                ("d̅", 500000),
                ("c̅", 100000),
                ("l̅", 50000),
                ("x̅", 10000),
                ("v̅", 5000),
                ("i̅v̅", 4000),
                ("m", 1000),
                ("cm", 900),
                ("d", 500),
                ("cd", 400),
                ("c", 100),
                ("xc", 90),
                ("l", 50),
                ("xl", 40),
                ("x", 10),
                ("ix", 9),
                ("v", 5),
                ("iv", 4),
                ("i", 1),
                ("n", 0),
            ]),

            Self::UpperRoman => NumeralSystem::Additive(&[
                ("M̅", 1000000),
                ("D̅", 500000),
                ("C̅", 100000),
                ("L̅", 50000),
                ("X̅", 10000),
                ("V̅", 5000),
                ("I̅V̅", 4000),
                ("M", 1000),
                ("CM", 900),
                ("D", 500),
                ("CD", 400),
                ("C", 100),
                ("XC", 90),
                ("L", 50),
                ("XL", 40),
                ("X", 10),
                ("IX", 9),
                ("V", 5),
                ("IV", 4),
                ("I", 1),
                ("N", 0),
            ]),

            Self::LowerGreek => NumeralSystem::Additive(&[
                ("͵θ", 9000),
                ("͵η", 8000),
                ("͵ζ", 7000),
                ("͵ϛ", 6000),
                ("͵ε", 5000),
                ("͵δ", 4000),
                ("͵γ", 3000),
                ("͵β", 2000),
                ("͵α", 1000),
                ("ϡ", 900),
                ("ω", 800),
                ("ψ", 700),
                ("χ", 600),
                ("φ", 500),
                ("υ", 400),
                ("τ", 300),
                ("σ", 200),
                ("ρ", 100),
                ("ϟ", 90),
                ("π", 80),
                ("ο", 70),
                ("ξ", 60),
                ("ν", 50),
                ("μ", 40),
                ("λ", 30),
                ("κ", 20),
                ("ι", 10),
                ("θ", 9),
                ("η", 8),
                ("ζ", 7),
                ("στ", 6),
                ("ε", 5),
                ("δ", 4),
                ("γ", 3),
                ("β", 2),
                ("α", 1),
                ("𐆊", 0),
            ]),

            Self::UpperGreek => NumeralSystem::Additive(&[
                ("͵Θ", 9000),
                ("͵Η", 8000),
                ("͵Ζ", 7000),
                ("͵Ϛ", 6000),
                ("͵Ε", 5000),
                ("͵Δ", 4000),
                ("͵Γ", 3000),
                ("͵Β", 2000),
                ("͵Α", 1000),
                ("Ϡ", 900),
                ("Ω", 800),
                ("Ψ", 700),
                ("Χ", 600),
                ("Φ", 500),
                ("Υ", 400),
                ("Τ", 300),
                ("Σ", 200),
                ("Ρ", 100),
                ("Ϟ", 90),
                ("Π", 80),
                ("Ο", 70),
                ("Ξ", 60),
                ("Ν", 50),
                ("Μ", 40),
                ("Λ", 30),
                ("Κ", 20),
                ("Ι", 10),
                ("Θ", 9),
                ("Η", 8),
                ("Ζ", 7),
                ("ΣΤ", 6),
                ("Ε", 5),
                ("Δ", 4),
                ("Γ", 3),
                ("Β", 2),
                ("Α", 1),
                ("𐆊", 0),
            ]),

            Self::LowerArmenian => NumeralSystem::Additive(&[
                ("ք", 9000),
                ("փ", 8000),
                ("ւ", 7000),
                ("ց", 6000),
                ("ր", 5000),
                ("տ", 4000),
                ("վ", 3000),
                ("ս", 2000),
                ("ռ", 1000),
                ("ջ", 900),
                ("պ", 800),
                ("չ", 700),
                ("ո", 600),
                ("շ", 500),
                ("ն", 400),
                ("յ", 300),
                ("մ", 200),
                ("ճ", 100),
                ("ղ", 90),
                ("ձ", 80),
                ("հ", 70),
                ("կ", 60),
                ("ծ", 50),
                ("խ", 40),
                ("լ", 30),
                ("ի", 20),
                ("ժ", 10),
                ("թ", 9),
                ("ը", 8),
                ("է", 7),
                ("զ", 6),
                ("ե", 5),
                ("դ", 4),
                ("գ", 3),
                ("բ", 2),
                ("ա", 1),
            ]),

            Self::UpperArmenian => NumeralSystem::Additive(&[
                ("Ք", 9000),
                ("Փ", 8000),
                ("Ւ", 7000),
                ("Ց", 6000),
                ("Ր", 5000),
                ("Տ", 4000),
                ("Վ", 3000),
                ("Ս", 2000),
                ("Ռ", 1000),
                ("Ջ", 900),
                ("Պ", 800),
                ("Չ", 700),
                ("Ո", 600),
                ("Շ", 500),
                ("Ն", 400),
                ("Յ", 300),
                ("Մ", 200),
                ("Ճ", 100),
                ("Ղ", 90),
                ("Ձ", 80),
                ("Հ", 70),
                ("Կ", 60),
                ("Ծ", 50),
                ("Խ", 40),
                ("Լ", 30),
                ("Ի", 20),
                ("Ժ", 10),
                ("Թ", 9),
                ("Ը", 8),
                ("Է", 7),
                ("Զ", 6),
                ("Ե", 5),
                ("Դ", 4),
                ("Գ", 3),
                ("Բ", 2),
                ("Ա", 1),
            ]),

            Self::Hebrew => NumeralSystem::Additive(&[
                ("ת", 400),
                ("ש", 300),
                ("ר", 200),
                ("ק", 100),
                ("צ", 90),
                ("פ", 80),
                ("ע", 70),
                ("ס", 60),
                ("נ", 50),
                ("מ", 40),
                ("ל", 30),
                ("כ", 20),
                ("יט", 19),
                ("יח", 18),
                ("יז", 17),
                ("טז", 16),
                ("טו", 15),
                ("י", 10),
                ("ט", 9),
                ("ח", 8),
                ("ז", 7),
                ("ו", 6),
                ("ה", 5),
                ("ד", 4),
                ("ג", 3),
                ("ב", 2),
                ("א", 1),
            ]),

            Self::LowerSimplifiedChinese => {
                NumeralSystem::Chinese(ChineseVariant::Simple, ChineseCase::Lower)
            }

            Self::UpperSimplifiedChinese => {
                NumeralSystem::Chinese(ChineseVariant::Simple, ChineseCase::Upper)
            }

            Self::LowerTraditionalChinese => {
                NumeralSystem::Chinese(ChineseVariant::Traditional, ChineseCase::Lower)
            }

            Self::UpperTraditionalChinese => {
                NumeralSystem::Chinese(ChineseVariant::Traditional, ChineseCase::Upper)
            }

            Self::HiraganaAiueo => NumeralSystem::Bijective(&[
                'あ', 'い', 'う', 'え', 'お', 'か', 'き', 'く', 'け', 'こ', 'さ', 'し',
                'す', 'せ', 'そ', 'た', 'ち', 'つ', 'て', 'と', 'な', 'に', 'ぬ', 'ね',
                'の', 'は', 'ひ', 'ふ', 'へ', 'ほ', 'ま', 'み', 'む', 'め', 'も', 'や',
                'ゆ', 'よ', 'ら', 'り', 'る', 'れ', 'ろ', 'わ', 'を', 'ん',
            ]),

            Self::HiraganaIroha => NumeralSystem::Bijective(&[
                'い', 'ろ', 'は', 'に', 'ほ', 'へ', 'と', 'ち', 'り', 'ぬ', 'る', 'を',
                'わ', 'か', 'よ', 'た', 'れ', 'そ', 'つ', 'ね', 'な', 'ら', 'む', 'う',
                'ゐ', 'の', 'お', 'く', 'や', 'ま', 'け', 'ふ', 'こ', 'え', 'て', 'あ',
                'さ', 'き', 'ゆ', 'め', 'み', 'し', 'ゑ', 'ひ', 'も', 'せ', 'す',
            ]),

            Self::KatakanaAiueo => NumeralSystem::Bijective(&[
                'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ',
                'ス', 'セ', 'ソ', 'タ', 'チ', 'ツ', 'テ', 'ト', 'ナ', 'ニ', 'ヌ', 'ネ',
                'ノ', 'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 'マ', 'ミ', 'ム', 'メ', 'モ', 'ヤ',
                'ユ', 'ヨ', 'ラ', 'リ', 'ル', 'レ', 'ロ', 'ワ', 'ヲ', 'ン',
            ]),

            Self::KatakanaIroha => NumeralSystem::Bijective(&[
                'イ', 'ロ', 'ハ', 'ニ', 'ホ', 'ヘ', 'ト', 'チ', 'リ', 'ヌ', 'ル', 'ヲ',
                'ワ', 'カ', 'ヨ', 'タ', 'レ', 'ソ', 'ツ', 'ネ', 'ナ', 'ラ', 'ム', 'ウ',
                'ヰ', 'ノ', 'オ', 'ク', 'ヤ', 'マ', 'ケ', 'フ', 'コ', 'エ', 'テ', 'ア',
                'サ', 'キ', 'ユ', 'メ', 'ミ', 'シ', 'ヱ', 'ヒ', 'モ', 'セ', 'ス',
            ]),

            Self::KoreanJamo => NumeralSystem::Bijective(&[
                'ㄱ', 'ㄴ', 'ㄷ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅅ', 'ㅇ', 'ㅈ', 'ㅊ', 'ㅋ', 'ㅌ',
                'ㅍ', 'ㅎ',
            ]),

            Self::KoreanSyllable => NumeralSystem::Bijective(&[
                '가', '나', '다', '라', '마', '바', '사', '아', '자', '차', '카', '타',
                '파', '하',
            ]),

            Self::EasternArabic => NumeralSystem::Positional(&[
                '٠', '١', '٢', '٣', '٤', '٥', '٦', '٧', '٨', '٩',
            ]),

            Self::Persian => NumeralSystem::Positional(&[
                '۰', '۱', '۲', '۳', '۴', '۵', '۶', '۷', '۸', '۹',
            ]),

            Self::Devanagari => NumeralSystem::Positional(&[
                '०', '१', '२', '३', '४', '५', '६', '७', '८', '९',
            ]),

            Self::Bengali => NumeralSystem::Positional(&[
                '০', '১', '২', '৩', '৪', '৫', '৬', '৭', '৮', '৯',
            ]),

            Self::BengaliLetters => NumeralSystem::Bijective(&[
                'ক', 'খ', 'গ', 'ঘ', 'ঙ', 'চ', 'ছ', 'জ', 'ঝ', 'ঞ', 'ট', 'ঠ', 'ড', 'ঢ',
                'ণ', 'ত', 'থ', 'দ', 'ধ', 'ন', 'প', 'ফ', 'ব', 'ভ', 'ম', 'য', 'র', 'ল',
                'শ', 'ষ', 'স', 'হ',
            ]),

            Self::Symbols => NumeralSystem::Symbolic(&['*', '†', '‡', '§', '¶', '‖']),
        }
    }
}

impl From<NamedNumeralSystem> for NumeralSystem<'static> {
    fn from(value: NamedNumeralSystem) -> Self {
        value.system()
    }
}

/// Represents a numeral system of one of multiple predefined kinds.
///
/// Values of this type can be constructed manually. Alternatively, common
/// numeral systems are listed as the values of [`NamedNumeralSystem`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum NumeralSystem<'a> {
    /// A big-endian
    /// [positional notation](https://en.wikipedia.org/wiki/Positional_notation)
    /// system.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any non-negative integer.
    ///
    /// ## Example
    ///
    /// With the digits `['0', '1', '2']`, we obtain the ternary numeral system:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 0      | 0              |
    /// | 1      | 1              |
    /// | 2      | 2              |
    /// | 3      | 10             |
    /// | 4      | 12             |
    /// | 5      | 12             |
    /// | 6      | 20             |
    Positional(&'a [char]),

    /// A big-endian
    /// [bijective numeration](https://en.wikipedia.org/wiki/Bijective_numeration)
    /// system. This is similar to positional notation, but without a digit for
    /// zero.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any positive integer.
    ///
    /// ## Example
    ///
    /// With the digits `['A', 'B', 'C']`, we obtain a system similar to one
    /// commonly used to number columns in spreadsheet software:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | A              |
    /// | 2      | B              |
    /// | 3      | C              |
    /// | 4      | AA             |
    /// | 5      | AB             |
    /// | 6      | AC             |
    /// | 7      | BA             |
    Bijective(&'a [char]),

    /// An additive
    /// [sign-value notation](https://en.wikipedia.org/wiki/Sign-value_notation)
    /// system.
    ///
    /// The numerals must be specified by decreasing value.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any positive integer. If a
    /// numeral with null value is provided, the system can represent zero as
    /// well.
    ///
    /// ## Examples
    ///
    /// With the numerals `[("V", 5), ("IV", 4), ("I", 1)]`, we obtain the start
    /// of the Roman numeral system:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | I              |
    /// | 2      | II             |
    /// | 3      | III            |
    /// | 4      | IV             |
    /// | 5      | V              |
    /// | 6      | VI             |
    /// | 7      | VII            |
    ///
    /// With the numerals `[("I", 1), ("Z", 0)]`, we obtain a unary system that
    /// can represent zero with a non-empty string:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 0      | Z              |
    /// | 1      | I              |
    /// | 2      | II             |
    /// | 3      | III            |
    /// | 4      | IIII           |
    /// | 5      | IIIII          |
    Additive(&'a [(&'a str, u64)]),

    /// A system that uses repeating symbols.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind can represent any positive integer.
    ///
    /// ## Example
    ///
    /// With the symbols `['A', 'B', 'C']`, we obtain the following
    /// representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | A              |
    /// | 2      | B              |
    /// | 3      | C              |
    /// | 4      | AA             |
    /// | 5      | BB             |
    /// | 6      | CC             |
    /// | 7      | AAA            |
    Symbolic(&'a [char]),

    /// A system that uses a fixed set of symbols to represent the first
    /// non-negative integers.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind with `n` symbols can represent any
    /// non-negative integer up to `n - 1`.
    ///
    /// ## Example
    ///
    /// With the symbols `['A', 'B', 'C']`, we obtain the following
    /// representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 0      | A              |
    /// | 1      | B              |
    /// | 2      | C              |
    Fixed(&'a [char]),

    /// A system that uses a fixed set of symbols to represent the first
    /// positive integers.
    ///
    /// ## Representable Numbers
    ///
    /// A numeral system of this kind with `n` symbols can represent any
    /// positive integer up to `n`.
    ///
    /// ## Example
    ///
    /// With the symbols `['A', 'B', 'C']`, we obtain the following
    /// representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 1      | A              |
    /// | 2      | B              |
    /// | 3      | C              |
    ZerolessFixed(&'a [char]),

    /// A Chinese numeral system.
    ///
    /// ## Representable Numbers
    ///
    /// Chinese numeral systems can represent any non-negative integer.
    ///
    /// ## Example
    ///
    /// With [`ChineseVariant::Simple`] and [`ChineseCase::Lower`], we
    /// obtain the following representations:
    ///
    /// | Number | Representation |
    /// |--------|----------------|
    /// | 0      | 零              |
    /// | 1      | 一              |
    /// | 2      | 二              |
    /// | 3      | 三              |
    /// | 4      | 四              |
    /// | 5      | 五              |
    /// | 6      | 六              |
    Chinese(ChineseVariant, ChineseCase),
}

impl<'a> NumeralSystem<'a> {
    /// Tries to represent a number in this numeral system.
    ///
    /// If `Ok(r)` is returned, `r` is a value of a type that implements
    /// [`Display`] by printing the number as represented in this numeral
    /// system.
    pub const fn represent(
        self,
        number: u64,
    ) -> Result<impl Display, RepresentationError> {
        match self {
            Self::Positional(_) | Self::Chinese(_, _) => {}
            Self::Bijective(_) | Self::Symbolic(_) => {
                if number == 0 {
                    return Err(RepresentationError::Zero);
                }
            }
            Self::Additive(numerals) => {
                if number == 0 && !matches!(numerals.last(), Some((_, 0))) {
                    return Err(RepresentationError::Zero);
                }
            }
            Self::Fixed(symbols) => {
                if number as usize >= symbols.len() {
                    return Err(RepresentationError::TooLarge);
                }
            }
            Self::ZerolessFixed(symbols) => {
                if number == 0 {
                    return Err(RepresentationError::Zero);
                }
                if number as usize > symbols.len() {
                    return Err(RepresentationError::TooLarge);
                }
            }
        }
        Ok(RepresentedNumber { system: self, number })
    }
}

/// A number, together with a numeral system in which it is representable.
///
/// Notably, this type implements [`Display`] and is thus compatible with
/// [`format!()`].
///
/// Values of this type are constructed by [`NumeralSystem::represent`].
#[derive(Debug, Clone, Copy)]
struct RepresentedNumber<'a> {
    /// Invariant: This system must be able to represent the number.
    system: NumeralSystem<'a>,
    number: u64,
}

impl<'a> Display for RepresentedNumber<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.system {
            NumeralSystem::Positional(digits) => {
                let mut n = self.number;

                if n == 0 {
                    return write!(f, "{}", digits[0]);
                }

                let radix = digits.len() as u64;
                let size = n.ilog(radix) + 1;
                // The place value of the most significant digit. For a number
                // of size 1, the MSD's place is the ones place, hence `- 1`.
                let mut msd_place = radix.pow(size - 1);
                for _ in 0..size {
                    let msd = n / msd_place;
                    write!(f, "{}", digits[msd as usize])?;
                    n -= msd * msd_place;
                    msd_place /= radix;
                }
                Ok(())
            }

            NumeralSystem::Bijective(digits) => {
                let mut n = self.number;

                assert_ne!(n, 0);

                let radix = digits.len() as u64;
                // Number of digits when representing `n` in this system.
                // From https://en.wikipedia.org/wiki/Bijective_numeration#Properties_of_bijective_base-k_numerals.
                let size = ((n + 1) * (radix - 1)).ilog(radix);
                // Remove from `n` the number consisting of `size - 1` ones in
                // base-`radix`, and the print the result using the symbols as
                // a positional numeral system.
                n -= (radix.pow(size) - 1) / (radix - 1);
                // The place value of the most significant digit. For a number
                // of size 1, the MSD's place is the ones place, hence `- 1`.
                let mut msd_place = radix.pow(size - 1);
                for _ in 0..size {
                    let msd = n / msd_place;
                    write!(f, "{}", digits[msd as usize])?;
                    n -= msd * msd_place;
                    msd_place /= radix;
                }
                Ok(())
            }

            NumeralSystem::Additive(numerals) => {
                let mut n = self.number;

                if n == 0 {
                    if let Some(&(numeral, 0)) = numerals.last() {
                        return write!(f, "{}", numeral);
                    }
                    unreachable!()
                }

                // Greedily add any symbol that fits.
                for (numeral, weight) in numerals {
                    if *weight == 0 || *weight > n {
                        continue;
                    }
                    let reps = n / weight;
                    for _ in 0..reps {
                        write!(f, "{}", numeral)?
                    }

                    n -= weight * reps;
                }
                Ok(())
            }
            NumeralSystem::Symbolic(symbols) => {
                let n = self.number;
                assert_ne!(n, 0);
                let symbol_count = symbols.len() as u64;
                for _ in 0..n.div_ceil(symbol_count) {
                    write!(f, "{}", symbols[((n - 1) % symbol_count) as usize])?
                }
                Ok(())
            }

            NumeralSystem::Fixed(symbols) => {
                write!(f, "{}", symbols[self.number as usize])
            }

            NumeralSystem::ZerolessFixed(symbols) => {
                write!(f, "{}", symbols[(self.number - 1) as usize])
            }

            NumeralSystem::Chinese(variant, case) => write!(
                f,
                "{}",
                from_u64_to_chinese_ten_thousand(variant, case, self.number),
            ),
        }
    }
}

/// A reason why a number cannot be represented in a numeral system.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum RepresentationError {
    /// Zero cannot be represented in the numeral system.
    Zero,
    /// The number is too large for the numeral system.
    TooLarge,
}

#[cfg(test)]
mod tests {
    use super::{NamedNumeralSystem, NumeralSystem};

    /// Makes sure shorthands correspond to the way the number one is
    /// represented in the corresponding system.
    #[test]
    fn shorthands_are_one() {
        for named in NamedNumeralSystem::iter() {
            if let Some(shorthand) = named.shorthand() {
                assert_eq!(
                    named
                        .system()
                        .represent(1)
                        .unwrap_or_else(|_| panic!(
                            "one should be representable with `{}`",
                            named.name(),
                        ))
                        .to_string(),
                    shorthand,
                )
            }
        }
    }

    /// Makes sure fixed systems are implemented properly.
    #[test]
    fn test_fixed_systems() {
        let symbols = ('a'..='z').collect::<Vec<_>>();
        for n in 0..symbols.len() {
            for i in 0..n {
                assert_eq!(
                    NumeralSystem::Fixed(&symbols[0..n])
                        .represent(i as u64)
                        .unwrap()
                        .to_string(),
                    symbols[i].to_string(),
                );
                assert_eq!(
                    NumeralSystem::ZerolessFixed(&symbols[0..n])
                        .represent(i as u64 + 1)
                        .unwrap()
                        .to_string(),
                    symbols[i].to_string(),
                )
            }
        }
    }

    /// Makes sure [`NamedNumeralSystem::Arabic`] represents numbers properly.
    /// This also tests [`NumeralSystem::Positional`].
    #[test]
    fn test_arabic_numerals() {
        for n in 0..=9999 {
            assert_eq!(
                NamedNumeralSystem::Arabic.system().represent(n).unwrap().to_string(),
                n.to_string(),
            )
        }
    }

    /// Makes sure [`NamedNumeralSystem::LowerLatin`] and
    /// [`NamedNumeralSystem::UpperLatin`] represent numbers properly. This also
    /// tests [`NumeralSystem::Bijective`].
    #[test]
    fn test_latin() {
        let mut n = 1;
        for c1 in 'a'..='z' {
            assert_eq!(
                NamedNumeralSystem::LowerLatin
                    .system()
                    .represent(n)
                    .unwrap()
                    .to_string(),
                format!("{c1}"),
            );
            assert_eq!(
                NamedNumeralSystem::UpperLatin
                    .system()
                    .represent(n)
                    .unwrap()
                    .to_string(),
                format!("{c1}").to_uppercase(),
            );
            n += 1;
        }
        for c2 in 'a'..='z' {
            for c1 in 'a'..='z' {
                assert_eq!(
                    NamedNumeralSystem::LowerLatin
                        .system()
                        .represent(n)
                        .unwrap()
                        .to_string(),
                    format!("{c2}{c1}"),
                );
                assert_eq!(
                    NamedNumeralSystem::UpperLatin
                        .system()
                        .represent(n)
                        .unwrap()
                        .to_string(),
                    format!("{c2}{c1}").to_uppercase(),
                );
                n += 1;
            }
        }
        for c3 in 'a'..='z' {
            for c2 in 'a'..='z' {
                for c1 in 'a'..='z' {
                    assert_eq!(
                        NamedNumeralSystem::LowerLatin
                            .system()
                            .represent(n)
                            .unwrap()
                            .to_string(),
                        format!("{c3}{c2}{c1}"),
                    );
                    assert_eq!(
                        NamedNumeralSystem::UpperLatin
                            .system()
                            .represent(n)
                            .unwrap()
                            .to_string(),
                        format!("{c3}{c2}{c1}").to_uppercase(),
                    );
                    n += 1;
                }
            }
        }
    }

    /// Makes sure Roman numerals work properly. This also tests
    /// [`NumeralSystem::Additive`].
    #[test]
    fn test_roman() {
        for (n, expect) in [
            "n", "i", "ii", "iii", "iv", "v", "vi", "vii", "viii", "ix", "x", "xi",
            "xii", "xiii", "xiv", "xv", "xvi", "xvii", "xviii", "xix", "xx", "xxi",
            "xxii", "xxiii", "xxiv", "xxv", "xxvi", "xxvii", "xxviii", "xxix", "xxx",
            "xxxi", "xxxii", "xxxiii", "xxxiv", "xxxv", "xxxvi", "xxxvii", "xxxviii",
            "xxxix", "xl", "xli", "xlii", "xliii", "xliv", "xlv", "xlvi",
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(
                &NamedNumeralSystem::LowerRoman
                    .system()
                    .represent(n as u64)
                    .unwrap()
                    .to_string(),
                expect,
            );
            assert_eq!(
                NamedNumeralSystem::UpperRoman
                    .system()
                    .represent(n as u64)
                    .unwrap()
                    .to_string(),
                expect.to_uppercase(),
            );
        }
    }

    /// Makes sure [`NumeralSystem::Symbolic`] represents numbers properly.
    #[test]
    fn test_symbolic() {
        let expected = ["a", "b", "aa", "bb", "aaa", "bbb", "aaaa", "bbbb"];
        for (i, r) in expected.iter().enumerate() {
            assert_eq!(
                &NumeralSystem::Symbolic(&['a', 'b'])
                    .represent(i as u64 + 1)
                    .unwrap()
                    .to_string(),
                r,
            )
        }
    }
}
