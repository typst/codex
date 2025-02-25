macro_rules! declare_types {
    ($(<$lt:lifetime>)?
        $(derive($($Der:ident),*),)?
        str = $s:ty,
        List = $List:ident<_>
    ) => {
        /// A module of definitions.
        $(#[derive($($Der),*)])?
        pub struct Module<$($lt)?>($List<($s, Binding<$($lt)?>)>);

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
            /// A symbol with named modifiers.
            /// The symbol defaults to its first variant.
            Multi($List<(ModifierSet<$s>, char)>),
        }
    };
}

/// A set of modifiers.
#[derive(Debug, Copy, Clone)]
pub struct ModifierSet<S>(S);

impl<S: Default> Default for ModifierSet<S> {
    /// Construct the default modifier set.
    ///
    /// This is typically the empty set,
    /// though the remark from [`Self::new_unchecked`] applies
    /// since `S::default()` could technically be anything.
    fn default() -> Self {
        Self(S::default())
    }
}

impl<S: std::ops::Deref<Target = str>> ModifierSet<S> {
    /// Convert the underlying string to a slice.
    pub fn as_deref(&self) -> ModifierSet<&str> {
        ModifierSet(&self.0)
    }

    /// Construct a modifier set from a string,
    /// where modifiers are separated by the character `.`.
    ///
    /// It is not unsafe to use this function wrongly, but it can produce
    /// unexpected results down the line. Correct usage should ensure that
    /// `s` does not contain any empty modifiers (i.e. the sequence `..`)
    /// and that no modifier occurs twice.
    pub fn new_unchecked(s: S) -> Self {
        Self(s)
    }

    /// Whether `self` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add a modifier to the set, without checking that it is a valid modifier.
    ///
    /// It is not unsafe to use this method wrongly, but that can produce
    /// unexpected results down the line. Correct usage should ensure that
    /// `modifier` is not empty and doesn't contain the character `.`.
    pub fn add_unchecked(&mut self, m: &str)
    where
        S: for<'a> std::ops::AddAssign<&'a str>,
    {
        if !self.0.is_empty() {
            self.0 += ".";
        }
        self.0 += m;
    }

    /// Iterate over the list of modifiers in an arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.split('.').filter(|s| !s.is_empty())
    }

    /// Whether the set contains the modifier `m`.
    pub fn contains(&self, m: &str) -> bool {
        self.iter().any(|lhs| lhs == m)
    }

    /// Whether all modifiers in `self` are also present in `other`.
    pub fn is_subset(&self, other: ModifierSet<&str>) -> bool {
        self.iter().all(|m| other.contains(m))
    }

    /// Find the best match from the list.
    ///
    /// To be considered a match, the modifier set must be a superset of
    /// (or equal to) `self`. Among different matches, the best one is selected
    /// by the following two criteria (in order):
    /// 1. Number of modifiers in common with `self` (more is better).
    /// 2. Total number of modifiers (fewer is better).
    pub fn best_match_in<'a, T>(
        &self,
        variants: impl Iterator<Item = (ModifierSet<&'a str>, T)>,
    ) -> Option<T> {
        let mut best = None;
        let mut best_score = None;

        // Find the best table entry with this name.
        for candidate in variants.filter(|(set, _)| self.is_subset(*set)) {
            let mut matching = 0;
            let mut total = 0;
            for modifier in candidate.0.iter() {
                if self.contains(modifier) {
                    matching += 1;
                }
                total += 1;
            }

            let score = (matching, std::cmp::Reverse(total));
            if best_score.is_none_or(|b| score > b) {
                best = Some(candidate.1);
                best_score = Some(score);
            }
        }

        best
    }
}

impl<'a> ModifierSet<&'a str> {
    /// Iterate over the list of modifiers with the original lifetime.
    pub fn to_iter(self) -> impl Iterator<Item = &'a str> {
        self.0.split('.').filter(|s| !s.is_empty())
    }
}
