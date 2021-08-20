use self::helpers::PrefixInterface;
use crate::GlobPattern;
use std::{
    borrow::Cow,
    collections::BTreeSet,
    fmt,
    iter::{self, FromIterator},
    mem,
    ops::{Bound, RangeBounds},
};

mod helpers;

#[derive(PartialEq, Eq)]
pub struct GlobSet {
    /// Holds only non-intersecting `GlobPattern`s
    patterns: BTreeSet<PrefixInterface<Vec<u8>>>,
}

impl fmt::Debug for GlobSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DisplayCollection<T: IntoIterator>(T);
        const LIMIT_N: usize = 20;

        impl<T> fmt::Debug for DisplayCollection<T>
        where
            T: IntoIterator + Copy,
            T::IntoIter: ExactSizeIterator,
            T::Item: fmt::Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let mut iter = IntoIterator::into_iter(self.0);

                let mut debug_list = f.debug_list();
                debug_list.entries((&mut iter).take(LIMIT_N));

                let other_entries_count = iter.len();
                if other_entries_count > 0 {
                    debug_list.entry(&format_args!("< + {} more entries >", other_entries_count));
                }
                debug_list.finish()
            }
        }

        f.debug_struct("GlobSet")
            .field("patterns", &DisplayCollection(&self.patterns))
            .finish()
    }
}

impl<'a> FromIterator<GlobPattern<Cow<'a, [u8]>>> for GlobSet {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = GlobPattern<Cow<'a, [u8]>>>,
    {
        let mut out = GlobSet::new();
        for pat in iter {
            out.insert(pat);
        }
        out
    }
}

impl GlobSet {
    pub fn new() -> Self {
        Self {
            patterns: BTreeSet::new(),
        }
    }

    /// Duplicates replace old values
    pub fn insert(&mut self, pattern: GlobPattern<Cow<'_, [u8]>>) {
        let pattern = PrefixInterface(pattern);

        if !pattern.0.has_suffix {
            // `something` branch

            if !self.is_match(pattern.as_ref()) {
                self.patterns.insert(pattern.into_owned());
            }
        } else {
            let next_pattern = {
                PrefixInterface(GlobPattern {
                    has_suffix: false,
                    prefix: {
                        let mut next_prefix = pattern.0.prefix.to_vec();

                        match next_prefix.last_mut() {
                            None => {
                                // `*` pattern branch
                                self.patterns.clear();
                                self.patterns.insert(pattern.into_owned());
                                return;
                            }
                            Some(l) => *l += 1,
                        }

                        next_prefix
                    },
                })
            };

            // `something*` branch

            let range = (
                Bound::Included(pattern.as_ref()),
                Bound::Excluded(next_pattern.as_ref()),
            );
            // fist item in range can be a duplicate
            match self.patterns.range::<[u8], _>(range).next() {
                // no pattern intersection (continue)
                None => (),

                // exact duplicate (do nothing)
                Some(pat)
                    if pat.0.prefix.as_slice() == pattern.as_ref() && pat.0.has_suffix == true =>
                {
                    return
                }

                // pattern intersections (remove old, continue)
                Some(_) => drop(self.take_range(range)),
            }

            self.patterns.insert(pattern.into_owned());
        }
    }

    pub fn is_match(&self, s: &[u8]) -> bool {
        let mut range = self
            .patterns
            .range::<[u8], _>((Bound::Unbounded, Bound::Included(s)));

        match range.next_back() {
            Some(PrefixInterface(GlobPattern {
                has_suffix: true,
                prefix,
            })) if s.starts_with(prefix) => true,
            Some(PrefixInterface(GlobPattern {
                has_suffix: false,
                prefix,
            })) if prefix.as_slice() == s => true,
            _ => false,
        }
    }

    #[must_use]
    fn take_range<R>(&mut self, range: R) -> Self
    where
        R: RangeBounds<[u8]>,
    {
        let mut middle = match range.start_bound() {
            Bound::Included(b) => self.patterns.split_off(b),
            Bound::Excluded(b) => self.patterns.split_off(next_lexicographic(&b).as_slice()),
            Bound::Unbounded => mem::take(&mut self.patterns),
        };

        let mut end = match range.end_bound() {
            Bound::Excluded(b) => middle.split_off(b),
            Bound::Included(b) => match prev_lexicographic(&b) {
                Some(prefix) => middle.split_off(prefix.as_ref()),
                None => mem::take(&mut middle),
            },
            Bound::Unbounded => BTreeSet::new(),
        };

        self.patterns.append(&mut end);

        Self { patterns: middle }
    }
}

fn next_lexicographic(current: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(current.len() + 1);
    result.extend_from_slice(current);
    result.push(0);
    result
}

fn prev_lexicographic(current: &[u8]) -> Option<Cow<'_, [u8]>> {
    match current.split_last() {
        Some((0, head)) => Some(head.into()),
        Some((last, head)) => Some(head.iter().copied().chain(iter::once(*last - 1)).collect()),
        None => None,
    }
}
