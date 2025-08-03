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
                modifs.best_match_in(list.iter().copied().map(|(ms, c, d)| (ms, (c, d))))
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
            .flat_map(|(ms, _, _)| ms.into_iter().map(|m| m.name()))
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
                .map(|(ms, v, _)| {
                    (ms.into_iter().map(|m| m.as_str()).collect::<BTreeSet<_>>(), v)
                })
                .collect::<BTreeSet<_>>();
            let control = control
                .iter()
                .map(|&(ms, v)| {
                    (
                        ModifierSet::from_raw_dotted(ms)
                            .into_iter()
                            .map(|m| m.as_str())
                            .collect::<BTreeSet<_>>(),
                        v,
                    )
                })
                .collect::<BTreeSet<_>>();

            assert_eq!(variants, control);
        }
    }

    #[test]
    fn no_overlap() {
        recur("", ROOT);

        /// Iterate over all symbols in a module, recursing into submodules.
        fn recur(prefix: &str, m: Module) {
            for (name, b) in m.iter() {
                match b.def {
                    Def::Module(m) => {
                        let new_prefix = if prefix.is_empty() {
                            name.to_string()
                        } else {
                            prefix.to_string() + "." + name
                        };
                        recur(&new_prefix, m);
                    }
                    Def::Symbol(s) => check_symbol(prefix, name, s),
                }
            }
        }

        /// Check the no overlap rule for a single symbol
        fn check_symbol(prefix: &str, name: &str, sym: Symbol) {
            // maximum number of modifiers per variant (we don't need to check more than this).
            let max_modifs =
                sym.variants().map(|(ms, ..)| ms.iter().count()).max().unwrap();
            let modifs = sym.modifiers().collect::<Vec<_>>();
            let max_index = modifs.len().saturating_sub(1);

            for k in 0..=max_modifs {
                let mut indices = (0..k).collect::<Vec<_>>();
                loop {
                    let mset = indices.iter().map(|i| modifs[*i]).fold(
                        ModifierSet::<String>::default(),
                        |mut res, m| {
                            res.insert_raw(m);
                            res
                        },
                    );

                    if sym.variants().filter(|(ms, ..)| mset.is_candidate(*ms)).count()
                        > 1
                    {
                        panic!(
                            "Overlap in symbol {prefix}.{name} for modifiers {}",
                            mset.as_str()
                        );
                    }

                    if next_subseq(&mut indices, max_index).is_none() {
                        break;
                    }
                }
            }
        }

        /// Produces the (lexicographically) next strictly increasing array of numbers
        /// less than or equal to `max_index`.
        ///
        /// Example:
        /// ```text
        /// [0,1,2], [0,1,3], [0,1,4], [0,2,3], [0,2,4], [0,3,4], [1,2,3], [1,2,4], [1,3,4], [2,3,4]
        /// ```
        ///
        /// Invariants:
        /// - `indices` is strictly increasing
        /// - All elements of `indices` are `<= max_index`
        /// - `indices.len() <= max_index + 1` (this is already implied by the previous two)
        fn next_subseq(indices: &mut [usize], max_index: usize) -> Option<()> {
            match indices {
                [] => None,
                [single] => {
                    if *single < max_index {
                        *single += 1;
                        Some(())
                    } else {
                        None
                    }
                }
                [left @ .., last] => {
                    assert_ne!(max_index, 0);
                    assert_ne!(left.len(), 0);
                    if *last < max_index {
                        *last += 1;
                    } else {
                        next_subseq(left, max_index - 1)?;
                        *last = left.last().unwrap() + 1;
                    }
                    Some(())
                }
            }
        }
    }
}
