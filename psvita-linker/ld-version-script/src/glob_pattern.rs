use std::{convert::TryFrom, error, fmt};

/// Match `prefix*` patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobPattern<S: ?Sized> {
    pub has_suffix: bool,
    pub prefix: S,
}

impl<S: ?Sized> GlobPattern<S> {
    pub fn as_ref(&self) -> GlobPattern<&S> {
        GlobPattern {
            has_suffix: self.has_suffix,
            prefix: &self.prefix,
        }
    }
}

impl<S> GlobPattern<S> {
    pub fn map_prefix<F, R>(self, f: F) -> GlobPattern<R>
    where
        F: FnOnce(S) -> R,
    {
        GlobPattern {
            prefix: f(self.prefix),
            has_suffix: self.has_suffix,
        }
    }
}

impl std::str::FromStr for GlobPattern<String> {
    type Err = ParseGlobPatternError;

    fn from_str(s: &str) -> Result<Self, ParseGlobPatternError> {
        Ok(GlobPattern::<&str>::try_from(s)?.map_prefix(String::from))
    }
}

impl<'a> TryFrom<&'a str> for GlobPattern<&'a str> {
    type Error = ParseGlobPatternError;

    fn try_from(s: &'a str) -> Result<GlobPattern<&'a str>, ParseGlobPatternError> {
        match s.split_once('*') {
            Some((s, "")) => Ok(GlobPattern {
                prefix: s,
                has_suffix: true,
            }),
            None => Ok(GlobPattern {
                prefix: s,
                has_suffix: false,
            }),
            _ => Err(ParseGlobPatternError),
        }
    }
}

#[derive(Debug)]
pub struct ParseGlobPatternError;

impl error::Error for ParseGlobPatternError {}

impl fmt::Display for ParseGlobPatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("mailformed glob pattern")
    }
}
