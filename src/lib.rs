//! Human-friendly notation for Unicode symbols.
//!
//! ## Model
//! A [`Symbol`] is a collection of one or more _variants_.
//! Each variant is identified by a set of [_modifiers_](ModifierSet)
//! and has a single character as its value.
//! The modifiers themselves can in principle be any non-empty strings
//! that don't contain the character `.`, but codex only defines
//! ones that are entirely made of ASCII alphabetical characters.

include!("shared.rs");

type StaticSlice<T> = &'static [T];
declare_types! {
    derive(Debug, Copy, Clone),
    str = &'static str,
    List = StaticSlice<_>
}

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

impl Symbol {
    /// Get the symbol's character for a given set of modifiers.
    pub fn get(&self, modifs: ModifierSet<&str>) -> Option<char> {
        match self {
            Self::Single(c) => modifs.is_empty().then_some(*c),
            Self::Multi(list) => modifs.best_match_in(list.iter().copied()),
        }
    }

    /// The characters that are covered by this symbol.
    pub fn variants(&self) -> impl Iterator<Item = (ModifierSet<&str>, char)> {
        enum Variants {
            Single(std::iter::Once<char>),
            Multi(std::slice::Iter<'static, (ModifierSet<&'static str>, char)>),
        }
        let mut iter = match self {
            Self::Single(c) => Variants::Single(std::iter::once(*c)),
            Self::Multi(sl) => Variants::Multi(sl.iter()),
        };
        std::iter::from_fn(move || match &mut iter {
            Variants::Single(iter) => Some((ModifierSet::default(), iter.next()?)),
            Variants::Multi(iter) => iter.next().copied(),
        })
    }

    /// Possible modifiers for this symbol.
    pub fn modifiers(&self) -> impl Iterator<Item = &str> + '_ {
        self.variants()
            .flat_map(|(m, _)| m.to_iter())
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
}
