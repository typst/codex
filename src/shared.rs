macro_rules! declare_types {
    ($(<$lt:lifetime>)?
        $(derive($($Der:ident),*),)?
        str = $s:ty,
        List = $L:ident<_>
    ) => {
        /// A module of definitions.
        $(#[derive($($Der),*)])?
        pub struct Module<$($lt)?>($L<($s, Binding<$($lt)?>)>);

        /// A definition bound in a module, with metadata.
        $(#[derive($($Der),*)])?
        pub struct Binding<$($lt)?> {
            /// The bound definition.
            pub def: Def<$($lt)?>,
            /// A deprecation message for the definition, if it is deprecated.
            pub deprecation: Option<$s>,
        }

        impl<$($lt)?> Binding<$($lt)?> {
            /// Create a new bound definition.
            pub const fn new(definition: Def<$($lt)?>) -> Self {
                Self { def: definition, deprecation: None }
            }
        }

        /// A definition in a module.
        $(#[derive($($Der),*)])?
        pub enum Def<$($lt)?> {
            /// A symbol, potentially with modifiers.
            Symbol(Symbol<$($lt)?>),
            /// A nested module.
            Module(Module<$($lt)?>),
        }

        /// A symbol, either a leaf or with modifiers.
        $(#[derive($($Der),*)])?
        pub enum Symbol<$($lt)?> {
            /// A symbol without modifiers.
            Single(char),
            /// A symbol with named modifiers. The symbol defaults to its first variant.
            Multi($L<($s, char)>),
        }
    };
}
