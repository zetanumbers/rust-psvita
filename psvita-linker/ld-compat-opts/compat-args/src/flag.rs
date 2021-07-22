//! Flags parsing in constant time.

use std::{
    collections::{hash_map, HashMap},
    fmt, iter,
};
use thiserror::Error;

/// Flag descriptions container.
#[derive(Default)]
pub struct Flags<A> {
    pub inner: HashMap<&'static str, Box<dyn Fn() -> A>>,
}

impl<A> fmt::Debug for Flags<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.inner.keys()).finish()
    }
}

/// An error which can be returned when trying to insert a duplicate flag.
#[derive(Error, Debug)]
#[error("failed to insert `{flag}` flag, which already exists")]
pub struct DuplicateFlagError {
    pub flag: &'static str,
}

impl<A> Flags<A> {
    /// Create empty flags container.
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Try to insert a flag, fails on a duplicate.
    pub fn try_insert<F>(
        &mut self,
        flag: &'static str,
        producer: F,
    ) -> Result<(), DuplicateFlagError>
    where
        F: Fn() -> A + 'static,
    {
        let producer = Box::new(producer);
        match self.inner.entry(flag) {
            hash_map::Entry::Vacant(e) => Ok(drop(e.insert(producer))),
            hash_map::Entry::Occupied(_) => Err(DuplicateFlagError { flag }),
        }
    }

    /// Same as [`Flags::try_insert`] but panics instead.
    pub fn insert<F>(&mut self, flag: &'static str, producer: F)
    where
        F: Fn() -> A + 'static,
    {
        self.try_insert(flag, producer)
            .expect("error while inserting a flag")
    }

    /// Match the argument as one of the flags, consume one argument on success.
    pub fn parse_argument<I>(&self, args: &mut iter::Peekable<I>) -> Option<A>
    where
        I: Iterator<Item = String>,
    {
        let arg = args.peek().unwrap();
        let producer = self.inner.get(arg.as_str())?;
        let _ = args.next();
        Some(producer())
    }
}
