use displaydoc::Display;
use std::{
    collections::{hash_map, HashMap},
    convert::TryFrom,
    iter::Peekable,
};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Shorts<A> {
    inner: HashMap<[u8; 2], fn(String) -> A>,
}

#[derive(Display, Error, Debug)]
/// failed to insert `{name:?}` short option, which already exists
pub struct DuplicateShortError {
    pub name: [u8; 2],
}

impl<A> Shorts<A> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn try_insert(
        &mut self,
        name: [u8; 2],
        parser: fn(String) -> A,
    ) -> Result<(), DuplicateShortError> {
        match self.inner.entry(name) {
            hash_map::Entry::Vacant(e) => Ok(drop(e.insert(parser))),
            hash_map::Entry::Occupied(_) => Err(DuplicateShortError { name }),
        }
    }

    pub fn insert(&mut self, name: [u8; 2], parser: fn(String) -> A) {
        self.try_insert(name, parser)
            .expect("error while inserting short option")
    }

    pub fn parse_argument<I>(&self, args: &mut Peekable<I>) -> Option<A>
    where
        I: Iterator<Item = String>,
    {
        let arg = args.peek()?;
        let name = <&[u8; 2]>::try_from(&arg.as_bytes()[..2]).ok()?;
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
