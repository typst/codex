/*!
Human-friendly notation for Unicode symbols.
*/

/// A module of definitions.
#[derive(Debug, Copy, Clone)]
pub struct Module(&'static [(&'static str, Entry)]);

impl Module {
    /// Try to get a definition from the module.
    pub fn get(&self, name: &str) -> Option<Entry> {
        self.0
            .binary_search_by_key(&name, |(k, _)| k)
            .ok()
            .map(|i| self.0[i].1)
    }

    /// Iterate over the module's definition.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, Entry)> {
        self.0.iter().copied()
    }
}

/// An entry in a module.
#[derive(Debug, Copy, Clone)]
pub struct Entry {
    /// The definition associated with this entry.
    pub definition: Def,
    /// Indicates whether the entry is deprecated.
    ///
    /// If `Some(s)`, the entry is deprecated, with the deprecation message `s`.
    /// Otherwise, the entry is not deprecated.
    pub deprecated: Option<&'static str>,
}

impl Entry {
    pub const fn new(definition: Def) -> Self {
        Self { definition, deprecated: None }
    }

    pub const fn new_deprecated(definition: Def, message: &'static str) -> Self {
        Self { definition, deprecated: Some(message) }
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

/// A symbol, either a leaf or with modifiers.
#[derive(Debug, Copy, Clone)]
pub enum Symbol {
    /// A symbol without modifiers.
    Single(char),
    /// A symbol with named modifiers. The symbol defaults to its first variant.
    Multi(&'static [(&'static str, char)]),
}

/// A module that contains the other top-level modules.
pub const ROOT: Module = Module(&[
    ("emoji", Entry::new(Def::Module(EMOJI))),
    ("sym", Entry::new(Def::Module(SYM))),
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
                if let Def::Module(module) = entry.definition {
                    assert_sorted_recursively(module)
                }
            }
        }

        assert_sorted_recursively(ROOT);
    }
}
