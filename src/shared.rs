use std::ops::Deref;

/// A set of modifiers.
///
/// Beware: The [`Eq`] and [`Hash`] implementations are dependent on the ordering
/// of the modifiers, in opposition to what a set would usually constitute.
/// To test for set-wise equality, use [`iter`](Self::iter) and collect into a
/// true set type like [`HashSet`](std::collections::HashSet).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// note: the visibility needs to be `pub(crate)`,
// since build.rs outputs `ModifierSet(...)`
pub struct ModifierSet<S>(pub(crate) S);

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

impl<S: Deref<Target = str>> ModifierSet<S> {
    /// Convert the underlying string to a slice.
    pub fn as_deref(&self) -> ModifierSet<&str> {
        ModifierSet(&self.0)
    }

    /// Get the string of modifiers separated by `.`.
    pub fn as_str(&self) -> &str {
        &self.0
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
        self.into_iter()
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
    ///
    /// If there are multiple best matches, the first of them is returned.
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

impl<'a, S: Deref<Target = str>> IntoIterator for &'a ModifierSet<S> {
    type Item = &'a str;
    type IntoIter = std::str::Split<'a, char>;

    /// Iterate over the list of modifiers in an arbitrary order.
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self.0.split('.');
        if self.0.is_empty() {
            // empty the iterator
            let _ = iter.next();
        }
        iter
    }
}

impl<'a> IntoIterator for ModifierSet<&'a str> {
    type Item = &'a str;
    type IntoIter = std::str::Split<'a, char>;

    /// Iterate over the list of modifiers in an arbitrary order.
    fn into_iter(self) -> Self::IntoIter {
        let mut iter = self.0.split('.');
        if self.0.is_empty() {
            // empty the iterator
            let _ = iter.next();
        }
        iter
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
        assert_eq!(ModifierSet::new_unchecked("a").iter().count(), 1);
        assert_eq!(ModifierSet::new_unchecked("a.b").iter().count(), 2);
        assert_eq!(ModifierSet::new_unchecked("a.b.c").iter().count(), 3);
    }

    #[test]
    fn subset() {
        assert!(
            ModifierSet::new_unchecked("a").is_subset(ModifierSet::new_unchecked("a.b"))
        );
        assert!(
            ModifierSet::new_unchecked("a").is_subset(ModifierSet::new_unchecked("b.a"))
        );
        assert!(ModifierSet::new_unchecked("a.b")
            .is_subset(ModifierSet::new_unchecked("b.c.a")));
    }

    #[test]
    fn best_match() {
        // 1. more modifiers in common with self
        assert_eq!(
            ModifierSet::new_unchecked("a.b").best_match_in([
                (ModifierSet::new_unchecked("a.c"), 1),
                (ModifierSet::new_unchecked("a.b"), 2),
            ].into_iter()),
            Some(2)
        );
        // 2. fewer modifiers in general
        assert_eq!(
            ModifierSet::new_unchecked("a").best_match_in([
                (ModifierSet::new_unchecked("a"), 1),
                (ModifierSet::new_unchecked("a.b"), 2),
            ].into_iter()),
            Some(1)
        );
        // the first rule takes priority over the second
        assert_eq!(
            ModifierSet::new_unchecked("a.b").best_match_in([
                (ModifierSet::new_unchecked("a"), 1),
                (ModifierSet::new_unchecked("a.b"), 2),
            ].into_iter()),
            Some(2)
        );
        // among multiple best matches, the first one is returned
        assert_eq!(
            ModifierSet::default().best_match_in([
                (ModifierSet::new_unchecked("a"), 1),
                (ModifierSet::new_unchecked("b"), 2)
            ].into_iter()),
            Some(1)
        );
    }
}
