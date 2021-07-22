use std::fmt;

pub mod flag;
pub mod long;
pub mod short;

/// Option handlers container.
pub struct Args<A> {
    pub flags: flag::Flags<A>,
    pub shorts: short::Shorts<A>,
    pub longs: long::Longs<A>,
    pub plain_handler: Option<Box<dyn Fn(String) -> A>>,
}

impl<A> fmt::Debug for Args<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Args")
            .field("flags", &self.flags)
            .field("shorts", &self.shorts)
            .field("longs", &self.longs)
            .field(
                "plain_handler",
                &self.plain_handler.as_ref().map(|_| format_args!("...")),
            )
            .finish()
    }
}

impl<A> Args<A> {
    /// Create empty options container.
    pub fn new() -> Self {
        Self {
            flags: flag::Flags::new(),
            shorts: short::Shorts::new(),
            longs: long::Longs::new(),
            plain_handler: None,
        }
    }

    /// Transform string iterator into arguments iterator
    pub fn map_iter<'a, I>(&'a self, iter: I) -> impl Iterator<Item = A> + 'a
    where
        I: Iterator<Item = String> + 'a,
    {
        ArgsIter {
            args: self,
            inner: iter.peekable(),
        }
    }
}

#[doc(hidden)]
pub const BUILD_SHORT_OPTION_ERROR_MSG: &str = "short option name should be 2 bytes long";

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
        /// Return `Some` early
        macro_rules! short_circuit {
            ($e:expr) => {
                if let Some(v) = $e {
                    return v;
                }
            };
        }

        // return if end of iterator
        let _ = self.inner.peek()?;

        // try to parse arguments in this concrete order to satisfy parsing assumptions
        short_circuit!(self.args.flags.parse_argument(&mut self.inner).map(Some));
        short_circuit!(self.args.shorts.parse_argument(&mut self.inner).map(Some));
        short_circuit!(self.args.longs.parse_argument(&mut self.inner).map(Some));

        let other = self.inner.next().unwrap();
        if other.starts_with('-') {
            panic!("unhandled option: {}", &other);
        }
        Some((self
            .args
            .plain_handler
            .as_ref()
            .expect("did not found plain arguments handler"))(
            other
        ))
    }
}
