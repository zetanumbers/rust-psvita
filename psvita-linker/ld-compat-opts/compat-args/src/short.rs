//! Short options parsing in constant time.
//!
//! Name of a short option is strictly 2 bytes wide.

use std::{
    collections::{hash_map, HashMap},
    convert::TryInto,
    fmt,
    iter::Peekable,
};
use thiserror::Error;

/// Short option descriptions container.
#[derive(Default)]
pub struct Shorts<A> {
    inner: HashMap<[u8; 2], Box<dyn Fn(String) -> A>>,
}

impl<A> fmt::Debug for Shorts<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_set = f.debug_set();
        for option_name in self.inner.keys() {
            match std::str::from_utf8(option_name) {
                Ok(name) => debug_set.entry(&name),
                Err(_) => debug_set.entry(option_name),
            };
        }

        debug_set.finish()
    }
}

/// An error which can be returned when trying to insert a duplicate short option.
#[derive(Error, Debug)]
pub struct DuplicateShortError {
    pub name: [u8; 2],
}

impl fmt::Display for DuplicateShortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(name) = std::str::from_utf8(&self.name) {
            write!(
                f,
                "failed to insert {:?} short option, which already exists",
                name
            )
        } else {
            write!(
                f,
                "failed to insert `{:?}` short option, which already exists",
                self.name
            )
        }
    }
}

impl<A> Shorts<A> {
    /// Create empty short options container.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Try to insert a short option, fails on a duplicate.
    pub fn try_insert<F, S>(&mut self, name: [u8; 2], parser: F) -> Result<(), DuplicateShortError>
    where
        F: Fn(S) -> A + 'static,
        S: From<String>,
    {
        let parser = Box::new(move |s: String| parser(s.into()));
        match self.inner.entry(name) {
            hash_map::Entry::Vacant(e) => Ok(drop(e.insert(parser))),
            hash_map::Entry::Occupied(_) => Err(DuplicateShortError { name }),
        }
    }

    /// Same as [`Shorts::try_insert`] but panics instead.
    pub fn insert<F, S>(&mut self, name: [u8; 2], parser: F)
    where
        F: Fn(S) -> A + 'static,
        S: From<String>,
    {
        self.try_insert(name, parser)
            .expect("error while inserting short option")
    }

    /// Match the argument as one of the short options, consume arguments on success.
    pub fn parse_argument<I>(&self, args: &mut Peekable<I>) -> Option<A>
    where
        I: Iterator<Item = String>,
    {
        let arg = args.peek()?;
        let name: &[u8; 2] = arg.as_bytes().get(..2)?.try_into().ok()?;
        let parser = self.inner.get(name)?;
        Some(parser(if arg.len() <= 2 {
            let _ = args.next()?;
            let v = args.next()?;
            v
        } else {
            let mut v = args.next()?;
            v.replace_range(..2, "");
            v
        }))
    }
}
