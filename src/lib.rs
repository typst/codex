//! Human-friendly notation for Unicode symbols.
//!
//! ## Model
//! A [`Symbol`] is a collection of one or more _variants_. Each variant is
//! identified by a set of [_modifiers_](ModifierSet) and has a string as its
//! value. The modifiers themselves can in principle be any non-empty strings
//! that don't contain the character `.`, but codex only defines ones that are
//! entirely made of ASCII alphabetical characters.

pub use self::shared::ModifierSet;

mod shared;

#[cfg(feature = "styling")]
pub mod styling;

/// A module of definitions.
#[derive(Debug, Copy, Clone)]
pub struct Module(&'static [(&'static str, Binding)]);

impl Module {
    /// Try to get a bound definition in the module.
    pub fn get(&self, name: &str) -> Option<Binding> {
        self.0
            .binary_search_by_key(&name, |(k, _)| k)
            .ok()
            .map(|i| self.0[i].1)
    }

    /// Iterate over the module's definition.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, Binding)> {
        self.0.iter().copied()
    }
}

/// A definition bound in a module, with metadata.
#[derive(Debug, Copy, Clone)]
pub struct Binding {
    /// The bound definition.
    pub def: Def,
    /// A deprecation message for the definition, if it is deprecated.
    pub deprecation: Option<&'static str>,
}

impl Binding {
    /// Create a new bound definition.
    pub const fn new(definition: Def) -> Self {
        Self { def: definition, deprecation: None }
    }
}

/// A definition in a module.
#[derive(Debug, Copy, Clone)]
pub enum Def {
    /// A symbol, potentially with modifiers.
    Symbol(Symbol),
    /// A nested module.
    Module(Module),
}

/// A symbol, either a leaf or with modifiers and optional deprecation.
#[derive(Debug, Copy, Clone)]
pub enum Symbol {
    /// A symbol without modifiers.
    Single(&'static str),
    /// A symbol with named modifiers. The symbol defaults to its first variant.
    Multi(&'static [(ModifierSet<&'static str>, &'static str, Option<&'static str>)]),
}

impl Symbol {
    /// Get the symbol's variant for a given set of modifiers, alongside an optional deprecation
    /// message.
    pub fn get(&self, modifs: ModifierSet<&str>) -> Option<(&'static str, Option<&str>)> {
        match self {
            Self::Single(c) => modifs.is_empty().then_some((*c, None)),
            Self::Multi(list) => {
                modifs.best_match_in(list.iter().copied().map(|(m, c, d)| (m, (c, d))))
            }
        }
    }

    /// Iterate over the variants of this symbol.
    ///
    /// Each variant is represented by a tuple `(modifiers, value, deprecation)`.
    pub fn variants(
        &self,
    ) -> impl Iterator<Item = (ModifierSet<&'static str>, &'static str, Option<&'static str>)>
    {
        enum Variants {
            Single(std::iter::Once<&'static str>),
            Multi(
                std::slice::Iter<
                    'static,
                    (ModifierSet<&'static str>, &'static str, Option<&'static str>),
                >,
            ),
        }
        let mut iter = match self {
            Self::Single(c) => Variants::Single(std::iter::once(*c)),
            Self::Multi(sl) => Variants::Multi(sl.iter()),
        };
        std::iter::from_fn(move || match &mut iter {
            Variants::Single(iter) => Some((ModifierSet::default(), iter.next()?, None)),
            Variants::Multi(iter) => iter.next().copied(),
        })
    }

    /// Possible modifiers for this symbol.
    pub fn modifiers(&self) -> impl Iterator<Item = &str> + '_ {
        self.variants()
            .flat_map(|(m, _, _)| m.into_iter())
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
    }
}

/// A module that contains the other top-level modules.
pub const ROOT: Module = Module(&[
    ("emoji", Binding::new(Def::Module(EMOJI))),
    ("sym", Binding::new(Def::Module(SYM))),
]);

include!(concat!(env!("OUT_DIR"), "/out.rs"));

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::BTreeSet;
    #[cfg(feature = "_test-unicode-conformance")]
    use std::collections::HashSet;

    #[test]
    fn all_modules_sorted() {
        fn assert_sorted_recursively(root: Module) {
            assert!(root.0.is_sorted_by_key(|(k, _)| k));

            for (_, entry) in root.iter() {
                if let Def::Module(module) = entry.def {
                    assert_sorted_recursively(module)
                }
            }
        }

        assert_sorted_recursively(ROOT);
    }

    #[test]
    fn unicode_escapes() {
        let Def::Symbol(wj) = SYM.get("wj").unwrap().def else { panic!() };
        assert_eq!(wj.get(ModifierSet::default()).unwrap().0, "\u{2060}");
        let Def::Symbol(space) = SYM.get("space").unwrap().def else { panic!() };
        assert_eq!(space.get(ModifierSet::default()).unwrap().0, " ");
        assert_eq!(
            space.get(ModifierSet::from_raw_dotted("nobreak")).unwrap().0,
            "\u{A0}"
        );
    }

    #[test]
    fn random_sample() {
        for (key, control) in [
            (
                "backslash",
                [("", "\\"), ("not", "⧷"), ("o", "⦸")].as_slice(),
            ),
            ("chi", &[("", "χ")]),
            ("forces", &[("", "⊩"), ("not", "⊮")]),
            ("interleave", &[("", "⫴"), ("big", "⫼"), ("struck", "⫵")]),
            ("uranus", &[("", "⛢"), ("alt", "♅")]),
        ] {
            let Def::Symbol(s) = SYM.get(key).unwrap().def else {
                panic!("{key:?} is not a symbol")
            };
            let variants = s
                .variants()
                .map(|(m, v, _)| (m.into_iter().collect::<BTreeSet<_>>(), v))
                .collect::<BTreeSet<_>>();
            let control = control
                .iter()
                .map(|&(m, v)| {
                    (
                        ModifierSet::from_raw_dotted(m)
                            .into_iter()
                            .collect::<BTreeSet<_>>(),
                        v,
                    )
                })
                .collect::<BTreeSet<_>>();

            assert_eq!(variants, control);
        }
    }

    /// Returns the set of variation sequences defined in a file.
    #[cfg(feature = "_test-unicode-conformance")]
    fn read_sequences(source: &str) -> HashSet<String> {
        source
            .lines()
            .filter_map(|l| {
                let line = l.split('#').next().unwrap_or(l);
                (!line.is_empty()).then_some(line)
            })
            .map(|line| {
                line.split(';')
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|cp| {
                        char::from_u32(u32::from_str_radix(cp, 0x10).unwrap()).unwrap()
                    })
                    .collect()
            })
            .collect()
    }

    /// Returns the set of standardized variation sequences defined by Unicode.
    ///
    /// This does not include emoji variation sequences (also known as
    /// "presentation sequences").
    #[cfg(feature = "_test-unicode-conformance")]
    fn get_valid_standardized_variation_sequences() -> HashSet<String> {
        read_sequences(include_str!(concat!(
            env!("OUT_DIR"),
            "/StandardizedVariants.txt",
        )))
    }

    /// Tests whether a string is a standardized variation sequence.
    ///
    /// This does not include emoji variation sequences (i.e., presentation
    /// sequences). Use [`is_presentation_sequence`] to test whether a string is
    /// a presentation sequence.
    #[cfg(feature = "_test-unicode-conformance")]
    fn is_standardized_variation_sequence(s: &str) -> bool {
        // Non-specific variation selectors from
        // https://unicode.org/charts/PDF/UFE00.pdf.
        (0xFE00..=0xFE0D)
            .map(|cp| char::from_u32(cp).unwrap())
            .any(|vs| s.contains(vs))
    }

    /// Tests that no standardized variation sequence is invalid.
    ///
    /// The validity of emoji variation sequences (i.e., presentation sequences)
    /// is tested by [`no_invalid_presentation_sequence`].
    #[cfg(feature = "_test-unicode-conformance")]
    #[test]
    fn no_invalid_standardized_variation_sequence() {
        let sequences = get_valid_standardized_variation_sequences();
        assert!(
            are_all_variants_valid(ROOT, |c| {
                if is_standardized_variation_sequence(c) {
                    sequences.contains(c)
                } else {
                    true
                }
            }),
            "invalid standardized variation sequence(s) (see list above)",
        )
    }

    /// https://www.unicode.org/reports/tr51/#def_text_presentation_selector.
    const TEXT_PRESENTATION_SELECTOR: char = '\u{FE0E}';
    /// https://www.unicode.org/reports/tr51/#def_emoji_presentation_selector.
    const EMOJI_PRESENTATION_SELECTOR: char = '\u{FE0F}';

    /// Tests whether a string is a text presentation sequence.
    fn is_text_presentation_sequence(s: &str) -> bool {
        s.contains(TEXT_PRESENTATION_SELECTOR)
    }

    /// Tests whether a string is an emoji presentation sequence.
    fn is_emoji_presentation_sequence(s: &str) -> bool {
        s.contains(EMOJI_PRESENTATION_SELECTOR)
    }

    /// Tests whether a string is a presentation sequence.
    #[cfg(feature = "_test-unicode-conformance")]
    fn is_presentation_sequence(s: &str) -> bool {
        is_text_presentation_sequence(s) || is_emoji_presentation_sequence(s)
    }

    #[test]
    fn symbols_are_not_emojis() {
        assert!(
            are_all_variants_valid(SYM, |c| !is_emoji_presentation_sequence(c)),
            "unexpected use of emoji presentation selector in `sym` (see list above)",
        )
    }

    #[test]
    fn emojis_are_not_text() {
        assert!(
            are_all_variants_valid(EMOJI, |c| !is_text_presentation_sequence(c)),
            "unexpected use of text presentation selector in `emoji` (see list above)",
        )
    }

    /// Returns the set of presentation sequences defined by Unicode.
    ///
    /// See: https://www.unicode.org/reports/tr51/#Emoji_Variation_Sequences.
    #[cfg(feature = "_test-unicode-conformance")]
    fn get_valid_presentation_sequences() -> HashSet<String> {
        read_sequences(include_str!(concat!(
            env!("OUT_DIR"),
            "/emoji-variation-sequences.txt",
        )))
    }

    #[cfg(feature = "_test-unicode-conformance")]
    #[test]
    fn no_invalid_presentation_sequence() {
        let sequences = get_valid_presentation_sequences();
        assert!(
            are_all_variants_valid(ROOT, |c| {
                if is_presentation_sequence(c) {
                    sequences.contains(c)
                } else {
                    true
                }
            }),
            "invalid presentation sequence(s) (see list above)",
        )
    }

    #[cfg(feature = "_test-unicode-conformance")]
    #[test]
    fn symbols_have_text_presentation() {
        let require_presentation_selector = get_valid_presentation_sequences()
            .into_iter()
            .map(|s| s.chars().next().unwrap())
            .collect::<HashSet<_>>();
        assert!(
            are_all_variants_valid(SYM, |c| {
                // All emoji variation sequences are exactly 2 codepoints long
                // as of Unicode 17.0, so this doesn't miss anything.
                if require_presentation_selector.contains(&c.chars().next().unwrap()) {
                    is_text_presentation_sequence(c)
                } else {
                    true
                }
            }),
            "missing text presentation selector(s) in `sym` (see list above)",
        )
    }

    #[cfg(feature = "_test-unicode-conformance")]
    #[test]
    fn emojis_have_emoji_presentation() {
        let require_presentation_selector = get_valid_presentation_sequences()
            .into_iter()
            .map(|s| s.chars().next().unwrap())
            .collect::<HashSet<_>>();
        assert!(
            are_all_variants_valid(EMOJI, |c| {
                // All emoji variation sequences are exactly 2 codepoints long
                // as of Unicode 17.0, so this doesn't miss anything.
                if require_presentation_selector.contains(&c.chars().next().unwrap()) {
                    is_emoji_presentation_sequence(c)
                } else {
                    true
                }
            }),
            "missing emoji presentation selector(s) in `emoji` (see list above)",
        )
    }

    /// Returns `false` if, and only if, the predicate returned `false` for at least one variant
    /// within the module.
    ///
    /// Prints all variants for which the predicate returns `false`.
    fn are_all_variants_valid<P: FnMut(&'static str) -> bool>(
        module: Module,
        mut predicate: P,
    ) -> bool {
        let mut all_valid = true;
        fn aux<P: FnMut(&'static str) -> bool>(
            module: Module,
            path: Vec<&'static str>,
            all_valid: &mut bool,
            predicate: &mut P,
        ) {
            for (name, binding) in module.iter() {
                let mut new_path = path.clone();
                new_path.push(name);
                match binding.def {
                    Def::Symbol(s) => {
                        for (modifiers, c, _) in s.variants() {
                            if !predicate(c) {
                                *all_valid = false;
                                eprintln!(
                                    "- {}{}{} {} ({})",
                                    new_path.join("."),
                                    if modifiers.is_empty() { "" } else { "." },
                                    modifiers.as_str(),
                                    c,
                                    c.chars()
                                        .map(|cp| format!("{:04X}", cp as u32))
                                        .collect::<Vec<_>>()
                                        .join(" "),
                                )
                            }
                        }
                    }
                    Def::Module(m) => {
                        aux(m, new_path, all_valid, predicate);
                    }
                }
            }
        }
        aux(module, Vec::new(), &mut all_valid, &mut predicate);
        all_valid
    }
}
