//! Human-friendly notation for Unicode symbols.
//!
//! ## Model
//! A [`Symbol`] is a collection of one or more _variants_. Each variant is
//! identified by a set of [_modifiers_](ModifierSet) and has a single character
//! as its value. The modifiers themselves can in principle be any non-empty
//! strings that don't contain the character `.`, but codex only defines ones
//! that are entirely made of ASCII alphabetical characters.

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
    /// Get the symbol's character for a given set of modifiers, alongside an optional deprecation
    /// message.
    pub fn get(&self, modifs: ModifierSet<&str>) -> Option<(&'static str, Option<&str>)> {
        match self {
            Self::Single(c) => modifs.is_empty().then_some((*c, None)),
            Self::Multi(list) => {
                modifs.best_match_in(list.iter().copied().map(|(m, c, d)| (m, (c, d))))
            }
        }
    }

    /// The characters that are covered by this symbol.
    ///
    /// Each variant is represented by a tuple `(modifiers, character, deprecation)`.
    pub fn variants(
        &self,
    ) -> impl Iterator<Item = (ModifierSet<&str>, &'static str, Option<&str>)> {
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
            .flat_map(|(m, _, _)| m.into_iter().map(|m| m.strip_suffix('?').unwrap_or(m)))
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
                [("", "\\"), ("circle", "⦸"), ("not", "⧷"), ("o", "⦸")].as_slice(),
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
}
