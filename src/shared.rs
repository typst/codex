use std::ops::Deref;

/// A set of modifiers, some of which are marked as "optional".
///
/// Beware: The [`Eq`] and [`Hash`] implementations are dependent on the
/// ordering of the modifiers, in opposition to what a set would usually
/// constitute. To test for set-wise equality, use [`iter`](Self::iter) and
/// collect into a true set type like [`HashSet`](std::collections::HashSet).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ModifierSet<S>(
    // Note: the visibility needs to be `pub(crate)`, since build.rs outputs
    // `ModifierSet(...)`.
    pub(crate) S,
);

impl<S: Deref<Target = str>> ModifierSet<S> {
    /// Constructs a modifier set from a string, where modifiers are separated
    /// by the character `.` and optional modifiers end in the character `?`.
    /// (The `?` is not considered part of the modifier.)
    ///
    /// `s` should not contain any empty modifiers (i.e. it shouldn't contain
    /// the sequences `..` or `.?.`) and no modifier should occur twice. Otherwise,
    /// unexpected errors can occur.
    pub fn from_raw_dotted(s: S) -> Self {
        // Checking the other requirement too feels like it would be a bit too
        // expensive, even for debug mode.
        debug_assert!(
            !s.contains("..") && !s.contains(".?."),
            "ModifierSet::from_dotted called with string containing empty modifier"
        );
        Self(s)
    }

    /// Whether `self` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Whether `self` has any optional modifiers.
    pub fn has_optional(&self) -> bool {
        self.0.ends_with('?') || self.0.contains("?.")
    }

    /// Gets the string of modifiers separated by `.`.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts the underlying string to a slice.
    pub fn as_deref(&self) -> ModifierSet<&str> {
        ModifierSet(&self.0)
    }

    /// Inserts a new required or optional modifier into the set.
    ///
    /// `m` should not
    /// - be empty,
    /// - contain the character `.`,
    /// - contain the character `?` other than at the end, or
    /// - already be in the set.
    ///
    /// Otherwise, unexpected errors can occur.
    pub fn insert_raw(&mut self, m: &str)
    where
        S: for<'a> std::ops::AddAssign<&'a str>,
    {
        if !self.0.is_empty() {
            self.0 += ".";
        }
        self.0 += m;
    }

    /// Iterates over the list of modifiers in an arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = Modifier> {
        self.into_iter()
    }

    /// Whether the set contains the modifier `m`.
    ///
    /// Note that the final `?` for optional modifiers is not considered part
    /// of the modifier, so they must be passed without it.
    /// ```rust
    /// # use codex::ModifierSet;
    /// let ms = ModifierSet::from_raw_dotted("a?.b");
    /// assert!(ms.contains("a"));
    /// assert!(ms.contains("b"));
    /// assert!(!ms.contains("c"));
    /// ```
    pub fn contains(&self, m: &str) -> bool {
        self.iter().any(|lhs| lhs.name() == m)
    }

    /// Finds the best match from the list.
    ///
    /// To be considered a match, the modifier set must satisfy
    /// `it.required_is_subset(self) && self.is_subset(it)`.
    /// Among different matches, the best one is selected by the following two
    /// criteria (in order):
    /// 1. Number of modifiers in common with `self` (more is better).
    /// 2. Total number of modifiers (fewer is better).
    ///
    /// If there are multiple best matches, the first of them is returned.
    pub fn best_match_in<'a, T>(
        &self,
        variants: impl Iterator<Item = (ModifierSet<&'a str>, T)>,
    ) -> Option<T> {
        let mut best = None;
        let mut best_score = None;

        // Find the best table entry with this name.
        for candidate in variants.filter(|(set, _)| {
            set.required_is_subset(self.as_deref()) && self.is_subset(*set)
        }) {
            let mut matching = 0;
            let mut total = 0;
            for modifier in candidate.0.iter() {
                if self.contains(modifier.name()) {
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

    /// Whether all modifiers in `self` are also present in `other`.
    /// Ignores whether modifiers are optional or not.
    pub fn is_subset(&self, other: ModifierSet<&str>) -> bool {
        self.iter().all(|m| other.contains(m.name()))
    }

    /// Whether all *non-optional* modifiers in `self` are also present in `other`,
    /// optional or not.
    pub fn required_is_subset(&self, other: ModifierSet<&str>) -> bool {
        self.iter().filter(|m| !m.is_optional()).all(|m| other.contains(m.as_str()))
    }
}

impl<S: Default> Default for ModifierSet<S> {
    /// Constructs the default modifier set.
    ///
    /// This is typically the empty set, though the remark from
    /// [`Self::from_raw_dotted`] applies since `S::default()` could technically
    /// be anything.
    fn default() -> Self {
        Self(S::default())
    }
}

#[derive(Copy, Clone)]
pub struct Modifier<'a>(&'a str);

impl<'a> Modifier<'a> {
    pub fn as_str(self) -> &'a str {
        self.0
    }

    pub fn name_and_is_optional(self) -> (&'a str, bool) {
        match self.0.strip_suffix('?') {
            Some(name) => (name, true),
            None => (self.0, false),
        }
    }

    pub fn name(self) -> &'a str {
        self.0.strip_suffix('?').unwrap_or(self.0)
    }

    pub fn is_optional(self) -> bool {
        self.0.ends_with('?')
    }
}

pub struct ModifierSetIter<'a> {
    inner: std::str::Split<'a, char>,
}

impl<'a> Iterator for ModifierSetIter<'a> {
    type Item = Modifier<'a>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Modifier)
    }
}

impl<'a, S: Deref<Target = str>> IntoIterator for &'a ModifierSet<S> {
    type Item = Modifier<'a>;
    type IntoIter = ModifierSetIter<'a>;

    /// Iterate over the list of modifiers in an arbitrary order.
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self.0.split('.');
        if self.0.is_empty() {
            // Empty the iterator
            let _ = iter.next();
        }
        ModifierSetIter { inner: iter }
    }
}

impl<'a> IntoIterator for ModifierSet<&'a str> {
    type Item = Modifier<'a>;
    type IntoIter = ModifierSetIter<'a>;

    /// Iterate over the list of modifiers in an arbitrary order.
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self.0.split('.');
        if self.0.is_empty() {
            // Empty the iterator
            let _ = iter.next();
        }
        ModifierSetIter { inner: iter }
    }
}

#[cfg(test)]
mod tests {
    type ModifierSet = super::ModifierSet<&'static str>;

    #[test]
    fn default_is_empty() {
        assert!(ModifierSet::default().is_empty());
    }

    #[test]
    fn iter_count() {
        assert_eq!(ModifierSet::default().iter().count(), 0);
        assert_eq!(ModifierSet::from_raw_dotted("a").iter().count(), 1);
        assert_eq!(ModifierSet::from_raw_dotted("a.b").iter().count(), 2);
        assert_eq!(ModifierSet::from_raw_dotted("a.b.c").iter().count(), 3);
    }

    #[test]
    fn subset() {
        assert!(ModifierSet::from_raw_dotted("a")
            .is_subset(ModifierSet::from_raw_dotted("a.b")));
        assert!(ModifierSet::from_raw_dotted("a")
            .is_subset(ModifierSet::from_raw_dotted("b.a")));
        assert!(ModifierSet::from_raw_dotted("a.b")
            .is_subset(ModifierSet::from_raw_dotted("b.c.a")));
        assert!(ModifierSet::from_raw_dotted("a")
            .is_subset(ModifierSet::from_raw_dotted("a?.b")));
        assert!(ModifierSet::from_raw_dotted("a?")
            .is_subset(ModifierSet::from_raw_dotted("a.b")));
        assert!(ModifierSet::from_raw_dotted("a?")
            .is_subset(ModifierSet::from_raw_dotted("a?.b")));
    }

    #[test]
    fn best_match() {
        // 1. more modifiers in common with self
        assert_eq!(
            ModifierSet::from_raw_dotted("a.b").best_match_in(
                [
                    (ModifierSet::from_raw_dotted("a?.c?"), 1),
                    (ModifierSet::from_raw_dotted("a?.b?"), 2),
                ]
                .into_iter()
            ),
            Some(2)
        );
        // 2. fewer modifiers in general
        assert_eq!(
            ModifierSet::from_raw_dotted("a").best_match_in(
                [
                    (ModifierSet::from_raw_dotted("a?"), 1),
                    (ModifierSet::from_raw_dotted("a?.b?"), 2),
                ]
                .into_iter()
            ),
            Some(1)
        );
        // the first rule takes priority over the second
        assert_eq!(
            ModifierSet::from_raw_dotted("a.b").best_match_in(
                [
                    (ModifierSet::from_raw_dotted("a?"), 1),
                    (ModifierSet::from_raw_dotted("a?.b?"), 2),
                ]
                .into_iter()
            ),
            Some(2)
        );
        // among multiple best matches, the first one is returned
        assert_eq!(
            ModifierSet::default().best_match_in(
                [
                    (ModifierSet::from_raw_dotted("a?"), 1),
                    (ModifierSet::from_raw_dotted("b?"), 2)
                ]
                .into_iter()
            ),
            Some(1)
        );
    }
}
