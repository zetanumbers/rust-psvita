//! Long options parsing in constant time.

use std::{
    collections::{hash_map, HashMap},
    fmt,
    iter::Peekable,
};
use thiserror::Error;

/// Long option descriptions container.
#[derive(Default)]
pub struct Longs<A> {
    inner: HashMap<&'static str, Box<dyn Fn(String) -> A>>,
}

impl<A> fmt::Debug for Longs<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.inner.keys()).finish()
    }
}

/// An error which can be returned when trying to insert a duplicate long option.
#[derive(Error, Debug)]
#[error("failed to insert `{name}` option, which already exists")]
pub struct DuplicateLongError {
    pub name: &'static str,
}

impl<A> Longs<A> {
    /// Create empty long options container.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Try to insert a long option, fails on a duplicate.
    pub fn try_insert<F, S>(
        &mut self,
        name: &'static str,
        parser: F,
    ) -> Result<(), DuplicateLongError>
    where
        F: Fn(S) -> A + 'static,
        S: From<String>,
    {
        let parser = Box::new(move |s: String| parser(s.into()));
        match self.inner.entry(name) {
            hash_map::Entry::Vacant(e) => Ok(drop(e.insert(parser))),
            hash_map::Entry::Occupied(_) => Err(DuplicateLongError { name }),
        }
    }

    /// Same as [`Longs::try_insert`] but panics instead.
    pub fn insert<F, S>(&mut self, name: &'static str, parser: F)
    where
        F: Fn(S) -> A + 'static,
        S: From<String>,
    {
        self.try_insert(name, parser)
            .expect("error while inserting long option")
    }

    /// Match the argument as one of the long options, consume arguments on success.
    pub fn parse_argument<I>(&self, args: &mut Peekable<I>) -> Option<A>
    where
        I: Iterator<Item = String>,
    {
        let arg = args.peek()?;
        let (name, is_inline) = arg
            .split_once('=')
            .map_or_else(|| (arg.as_str(), false), |a| (a.0, true));
        let name_len = name.len();

        let parser = self.inner.get(name)?;
        Some(parser(if is_inline {
            let mut v = args.next()?;
            v.replace_range(..name_len + 1, "");
            v
        } else {
            let _ = args.next()?;
            let v = args.next()?;
            v
        }))
    }
}
