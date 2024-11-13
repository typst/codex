/*!
Human-friendly notation for Unicode symbols.
*/

/// A module of definitions.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Module(&'static [(&'static str, Def)]);

impl Module {
    /// Try to get a definition from the module.
    pub fn get(&self, name: &str) -> Option<Def> {
        self.0
            .binary_search_by_key(&name, |(k, _)| k)
            .ok()
            .map(|i| self.0[i].1)
    }

    /// Iterate over the module's definition.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, Def)> {
        self.0.iter().copied()
    }
}

/// A definition in a module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Def {
    /// A symbol, potentially with modifiers.
    Symbol(Symbol),
    /// A nested module.
    Module(Module),
}

/// A symbol, either a leaf or with modifiers.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Symbol {
    /// A symbol without modifiers.
    Single(char),
    /// A symbol with named modifiers. The symbol defaults to its first variant.
    Multi(&'static [(&'static str, char)]),
}

/// A module that contains the other top-level modules.
pub const ROOT: Module =
    Module(&[("sym", Def::Module(SYM)), ("emoji", Def::Module(EMOJI))]);

include!(concat!(env!("OUT_DIR"), "/out.rs"));
