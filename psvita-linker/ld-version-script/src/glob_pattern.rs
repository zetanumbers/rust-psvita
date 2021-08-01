use std::{error, fmt};

/// Match `prefix*` patterns
#[derive(Debug)]
pub struct GlobPattern {
    pub prefix: String,
    pub has_suffix: bool,
}

#[derive(Debug)]
pub struct ParseGlobPatternError;

impl error::Error for ParseGlobPatternError {}

impl fmt::Display for ParseGlobPatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("mailformed glob pattern")
    }
}

impl std::str::FromStr for GlobPattern {
    type Err = ParseGlobPatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('*') {
            Some((s, "")) => Ok(GlobPattern {
                prefix: s.to_owned(),
                has_suffix: true,
            }),
            None => Ok(GlobPattern {
                prefix: s.to_owned(),
                has_suffix: false,
            }),
            _ => Err(ParseGlobPatternError),
        }
    }
}
