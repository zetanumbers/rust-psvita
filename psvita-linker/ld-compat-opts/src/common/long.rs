use displaydoc::Display;
use std::{
    collections::{hash_map, HashMap},
    iter::Peekable,
};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Longs<A> {
    inner: HashMap<&'static str, fn(String) -> A>,
}

#[derive(Display, Error, Debug)]
/// failed to insert `{name}` option, which already exists
pub struct DuplicateLongError {
    pub name: &'static str,
}

impl<A> Longs<A> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn try_insert(
        &mut self,
        name: &'static str,
        parser: fn(String) -> A,
    ) -> Result<(), DuplicateLongError> {
        match self.inner.entry(name) {
            hash_map::Entry::Vacant(e) => Ok(drop(e.insert(parser))),
            hash_map::Entry::Occupied(_) => Err(DuplicateLongError { name }),
        }
    }

    pub fn insert(&mut self, name: &'static str, parser: fn(String) -> A) {
        self.try_insert(name, parser)
            .expect("error while inserting long option")
    }

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
