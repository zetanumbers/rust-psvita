use crate::GlobPattern;
use std::borrow::{Borrow, Cow};

/// Used to order `GlobPattern`s by prefix
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PrefixInterface<S: ?Sized>(pub GlobPattern<S>);

impl<T, S> AsRef<[T]> for PrefixInterface<S>
where
    S: AsRef<[T]>,
{
    fn as_ref(&self) -> &[T] {
        AsRef::as_ref(&self.0.prefix)
    }
}

impl<T, S> Borrow<[T]> for PrefixInterface<S>
where
    S: Borrow<[T]>,
{
    fn borrow(&self) -> &[T] {
        Borrow::borrow(&self.0.prefix)
    }
}

impl<'a, S> PrefixInterface<Cow<'a, [S]>>
where
    S: Clone,
    [S]: ToOwned,
{
    pub fn into_owned(self) -> PrefixInterface<<[S] as ToOwned>::Owned> {
        PrefixInterface(GlobPattern {
            has_suffix: self.0.has_suffix,
            prefix: self.0.prefix.into_owned(),
        })
    }
}

impl<S> Ord for PrefixInterface<S>
where
    S: ?Sized + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.prefix.cmp(&other.0.prefix)
    }
}

impl<S> PartialOrd for PrefixInterface<S>
where
    S: ?Sized + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.prefix.partial_cmp(&other.0.prefix)
    }
}

impl<S> Eq for PrefixInterface<S> where S: ?Sized + Eq {}

impl<S> PartialEq for PrefixInterface<S>
where
    S: ?Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.prefix.eq(&other.0.prefix)
    }
}
