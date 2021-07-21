use displaydoc::Display;
use std::{
    collections::{hash_map, HashMap},
    iter,
};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Flags<A> {
    inner: HashMap<&'static str, fn() -> A>,
}

#[derive(Display, Error, Debug)]
/// failed to insert `{flag}` flag, which already exists
pub struct DuplicateFlagError {
    pub flag: &'static str,
}

impl<A> Flags<A> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn try_insert(
        &mut self,
        flag: &'static str,
        producer: fn() -> A,
    ) -> Result<(), DuplicateFlagError> {
        match self.inner.entry(flag) {
            hash_map::Entry::Vacant(e) => Ok(drop(e.insert(producer))),
            hash_map::Entry::Occupied(_) => Err(DuplicateFlagError { flag }),
        }
    }

    pub fn insert(&mut self, flag: &'static str, producer: fn() -> A) {
        self.try_insert(flag, producer)
            .expect("error while inserting flag")
    }

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
