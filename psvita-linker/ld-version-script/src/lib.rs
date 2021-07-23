use globset::Glob;
use logos::{Lexer, Logos, Span};
use std::{error, fmt};
use ParseTrivialVersionScriptErrorKind::*;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum Token {
    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("global")]
    Global,

    #[token("local")]
    Local,

    #[token(":")]
    Colon,

    #[regex(r"[a-zA-Z0-9_$\.\*]+")]
    GlobPattern,

    #[token(";")]
    SemiColon,

    #[error]
    #[regex(r"[ \t\n]+", logos::skip)]
    Error,
}

#[derive(Default)]
pub struct TrivialVersionScript {
    pub global: Vec<globset::Glob>,
    pub local: Vec<globset::Glob>,
}

impl fmt::Debug for TrivialVersionScript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DisplayVec<'a>(&'a Vec<globset::Glob>);

        impl fmt::Debug for DisplayVec<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                const LIMIT_N: usize = 50;
                let n = self.0.len();

                let mut debug_list = f.debug_list();
                debug_list.entries(self.0.iter().map(|g| g.glob()).take(LIMIT_N));
                if n > LIMIT_N {
                    debug_list.entry(&format_args!("< + {} more entries >", n - LIMIT_N));
                }
                debug_list.finish()
            }
        }

        f.debug_struct("TrivialVersionScript")
            .field("global", &DisplayVec(&self.global))
            .field("local", &DisplayVec(&self.local))
            .finish()
    }
}

impl std::str::FromStr for TrivialVersionScript {
    type Err = ParseTrivialVersionScriptError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lex: Lexer<'_, Token> = Token::lexer(s);
        let mut vs = TrivialVersionScript::default();

        parse_token(&mut lex, &[Token::BraceOpen])?;
        parse_token(&mut lex, &[Token::Global])?;
        parse_token(&mut lex, &[Token::Colon])?;

        loop {
            let tok = parse_token(&mut lex, &[Token::GlobPattern, Token::Local])?;
            if tok == Token::Local {
                break;
            }
            vs.global
                .push(parse_glob_pattern(tok, lex.span(), lex.slice())?);
            parse_token(&mut lex, &[Token::SemiColon])?;
        }
        parse_token(&mut lex, &[Token::Colon])?;

        loop {
            let tok = parse_token(&mut lex, &[Token::GlobPattern, Token::BraceClose])?;
            if tok == Token::BraceClose {
                break;
            }
            vs.local
                .push(parse_glob_pattern(tok, lex.span(), lex.slice())?);
            parse_token(&mut lex, &[Token::SemiColon])?;
        }
        parse_token(&mut lex, &[Token::SemiColon])?;
        parse_end(&mut lex)?;

        Ok(vs)
    }
}

fn parse_glob_pattern(
    token: Token,
    span: Span,
    slice: &str,
) -> Result<Glob, ParseTrivialVersionScriptError> {
    if token != Token::GlobPattern {
        return Err(ParseTrivialVersionScriptError {
            got: Some((token, span)),
            expected: Some(&[Token::GlobPattern]),
            kind: WrongToken,
        });
    }
    match slice.parse() {
        Ok(g) => Ok(g),
        Err(e) => Err(ParseTrivialVersionScriptError {
            got: Some((token, span)),
            expected: None,
            kind: GlobError(e),
        }),
    }
}

fn parse_token(
    lexer: &mut Lexer<Token>,
    expected: &'static [Token],
) -> Result<Token, ParseTrivialVersionScriptError> {
    let got = lexer.next().ok_or(ParseTrivialVersionScriptError {
        expected: Some(expected),
        got: None,
        kind: UnexpectedEnd,
    })?;
    if expected.iter().any(|&t| t == got) {
        Ok(got)
    } else {
        Err(ParseTrivialVersionScriptError {
            expected: Some(expected),
            got: Some((got, lexer.span())),
            kind: WrongToken,
        })
    }
}

fn parse_end(lexer: &mut Lexer<Token>) -> Result<(), ParseTrivialVersionScriptError> {
    if let Some(got) = lexer.next() {
        Err(ParseTrivialVersionScriptError {
            got: Some((got, lexer.span())),
            expected: None,
            kind: ExpectedEnd,
        })
    } else {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseTrivialVersionScriptError {
    pub got: Option<(Token, Span)>,
    pub expected: Option<&'static [Token]>,
    pub kind: ParseTrivialVersionScriptErrorKind,
}

impl fmt::Display for ParseTrivialVersionScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match &self.kind {
            ErrorToken => "lexing error",
            WrongToken => "wrong token",
            UnexpectedEnd => "unexpected end",
            ExpectedEnd => "expected end",
            GlobError(_) => "",
        })?;
        if let GlobError(e) = &self.kind {
            write!(f, "glob pattern parsing error: {} ", e)?;
        }

        if let Some((tok, span)) = &self.got {
            write!(f, "; got `{:?}` at {:?}", tok, span)?;
        }
        if let Some(expected) = self.expected {
            write!(f, "; expected one of `{:?}`", expected)?;
        }
        Ok(())
    }
}

impl error::Error for ParseTrivialVersionScriptError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            GlobError(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ParseTrivialVersionScriptErrorKind {
    ErrorToken,
    WrongToken,
    UnexpectedEnd,
    ExpectedEnd,
    GlobError(globset::Error),
}
