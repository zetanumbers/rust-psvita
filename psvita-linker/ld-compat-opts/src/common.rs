use displaydoc::Display;
use thiserror::Error;

pub mod flag;
pub mod long;
pub mod short;

#[derive(Debug)]
pub struct Args<A> {
    pub flags: flag::Flags<A>,
    pub short: short::Shorts<A>,
    pub long: long::Longs<A>,
    pub plain_argument_handler: fn(String) -> A,
}

#[derive(Display, Error, Debug)]
pub enum DuplicateOptionError {
    /// duplicate flag: {0}
    Flag(#[from] flag::DuplicateFlagError),
    /// duplicate short option: {0}
    Short(#[from] short::DuplicateShortError),
    /// duplicate long option: {0}
    Long(#[from] long::DuplicateLongError),
}

impl<A> Args<A> {
    pub fn new(plain_argument_handler: fn(String) -> A) -> Self {
        Self {
            flags: flag::Flags::new(),
            short: short::Shorts::new(),
            long: long::Longs::new(),
            plain_argument_handler,
        }
    }

    pub fn map_iter<'a, I>(&'a self, iter: I) -> impl Iterator<Item = A> + 'a
    where
        I: Iterator<Item = String> + 'a,
    {
        ArgsIter {
            args: &self,
            inner: iter.peekable(),
        }
    }
}

struct ArgsIter<'a, A, I>
where
    I: Iterator<Item = String>,
{
    inner: std::iter::Peekable<I>,
    args: &'a Args<A>,
}

impl<'a, A, I> Iterator for ArgsIter<'a, A, I>
where
    I: Iterator<Item = String>,
{
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! short_circuit {
            ($e:expr) => {
                if let Some(v) = $e {
                    return v;
                }
            };
        }

        let _ = self.inner.peek()?;
        short_circuit!(self.args.flags.parse_argument(&mut self.inner).map(Some));
        short_circuit!(self.args.short.parse_argument(&mut self.inner).map(Some));
        short_circuit!(self.args.long.parse_argument(&mut self.inner).map(Some));

        let other = self.inner.next().unwrap();
        if other.starts_with('-') {
            panic!("unhandled option: {}", &other);
        }
        Some((self.args.plain_argument_handler)(other))
    }
}
