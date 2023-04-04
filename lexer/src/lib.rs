// Kiban
// Copyright (C) 2022 Oscar
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod comment;
pub mod keyword;
pub mod literal;
pub mod operator;
pub mod punctuation;
pub mod types;

pub use comment::*;
pub use keyword::*;
pub use literal::*;
pub use operator::*;
pub use punctuation::*;
pub use types::*;

use kiban_commons::*;
use kiban_error::*;

use std::{
    fmt::Display,
    iter::Enumerate,
    mem::discriminant,
    ops::{Range, RangeFrom, RangeFull, RangeTo},
    path::PathBuf,
};

use derive_more::{Constructor, Display};
use miette::NamedSource;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::{complete::anychar, is_alphanumeric},
    combinator::{consumed, map},
    multi::many0,
    Compare, CompareResult, FindSubstring, IResult, InputIter, InputLength, InputTake, Needed,
    Offset, Slice,
};
use nom_locate::LocatedSpan;
use nom_recursive::{HasRecursiveInfo, HasRecursiveType, RecursiveInfo};
use smallvec::SmallVec;
use smol_str::SmolStr;

#[macro_export]
macro_rules! mapped {
    ($name:expr, $type_of:expr) => {
        nom::combinator::map(nom::bytes::complete::tag($name), |_| $type_of)
    };
}

type Input<'a> = LocatedSpan<&'a str, ()>;

/// token stream with recursive info
#[derive(Clone, Constructor, Default, Debug)]
pub struct TokenStream(_TokenStream, Option<RecursiveInfo<_TokenStream>>);

/// token stream
type _TokenStream = SVec<(Token, Span)>;

/// token variants
#[derive(Clone, PartialEq, Display, Debug)]
#[display(fmt = "{}")]
pub enum Token {
    #[display(fmt = "{} (id)", _0)]
    Identifier(SmolStr),
    #[display(fmt = "{} (kw)", _0)]
    Keyword(Keyword),
    #[display(fmt = "{} (op)", _0)]
    Operator(Operator),
    #[display(fmt = "{} (punct)", _0)]
    Punctuation(Punctuation),
    #[display(fmt = "{} (type)", _0)]
    Type(Type),
    #[display(fmt = "{} (lit)", _0)]
    Literal(Literal),
    #[display(fmt = "{} (comment)", _0)]
    Comment(Comment),
    #[display(fmt = "{} (illegal)", _0)]
    Illegal(char),
}

impl TokenStream {
    pub fn parse<'b>(input: &str, path: Option<&PathBuf>) -> Result<Self, Error> {
        let input = LocatedSpan::from(input);
        let name = match path {
            Some(path) => path.to_str().unwrap(),
            None => "undefined path",
        };
        match many0(Token::parse)(input) {
            Ok((remainder, many_syntax)) => {
                if remainder.is_empty() {
                    Ok(Self(many_syntax.into(), None))
                } else {
                    Err(Error {
                        src: NamedSource::new(name, input.to_string()),
                        location: Some(Span::new(0, remainder.chars().count()).into()),
                        error: Kinds::Lexer(Some(String::from(
                            "Some tokens haven't been consumed!",
                        ))),
                    })
                }
            }
            Err(error) => match error {
                nom::Err::Incomplete(needed) => Err(Error {
                    src: NamedSource::new(name, input.to_string()),
                    location: None,
                    error: Kinds::Lexer(Some(format!(
                        "More data is needed to complete parsing! ({:?})",
                        needed
                    ))),
                }),
                nom::Err::Error(error) => Err(Error {
                    src: NamedSource::new(name, error.input.to_string()),
                    location: Some(Span::new(0, error.input.chars().count()).into()),
                    error: Kinds::Lexer(Some(format!("Error! {:?}", error.code))),
                }),
                nom::Err::Failure(failure) => Err(Error {
                    src: NamedSource::new(name, failure.input.to_string()),
                    location: Some(Span::new(0, failure.input.chars().count()).into()),
                    error: Kinds::Lexer(Some(format!("Failure! {:?}", failure.code))),
                }),
            },
        }
    }
}

impl HasRecursiveInfo<_TokenStream> for TokenStream {
    fn get_recursive_info(&self) -> RecursiveInfo<_TokenStream> {
        match &self.1 {
            Some(recursive) => recursive.clone(),
            None => RecursiveInfo::new(),
        }
    }

    fn set_recursive_info(mut self, info: RecursiveInfo<_TokenStream>) -> Self {
        self.1 = Some(info);
        self
    }
}

impl HasRecursiveType<_TokenStream> for TokenStream {
    fn get_value(&self) -> _TokenStream {
        self.clone().0
    }
}

impl Spanned for TokenStream {
    fn span(&self) -> Span {
        if let (Some(first), Some(last)) = (self.0.first(), self.0.last()) {
            Span::new(*first.1.start(), *last.1.end())
        } else {
            Span::default()
        }
    }
}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(info_self), Some(info_other)) = (self.1.clone(), other.1.clone()) {
            info_self == info_other && self.0 == other.0
        } else {
            self.0 == other.0
        }
    }
}

impl<'a> Parsable<Input<'a>, (Token, Span)> for Token {
    fn parse(s: Input) -> IResult<Input, (Token, Span)> {
        map(
            consumed(alt((
                map(Comment::parse, |s| Self::Comment(s)),
                map(Keyword::parse, |s| Self::Keyword(s)),
                map(Operator::parse, |s| Self::Operator(s)),
                map(Punctuation::parse, |s| Self::Punctuation(s)),
                map(Type::parse, |s| Self::Type(s)),
                map(Literal::parse, |s| Self::Literal(s)),
                map(
                    take_while1(|c: char| -> bool { is_alphanumeric(c as u8) || c == '_' }),
                    |s: Input| Token::Identifier(s.into()),
                ),
                map(anychar, |s| Self::Illegal(s)),
            ))),
            |(consumed, token)| {
                (
                    token,
                    Span::from_offset(s.location_offset(), consumed.chars().count()),
                )
            },
        )(s)
    }
}

impl Iterator for TokenStream {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.first().cloned();
        if value.is_some() {
            self.0.remove(0);
        };
        value
    }
}

impl InputLength for TokenStream {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.len()
    }
}

impl Slice<Range<usize>> for TokenStream {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Self(self.0.as_slice().slice(range).into(), self.1.clone())
    }
}

impl Slice<RangeTo<usize>> for TokenStream {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl Slice<RangeFrom<usize>> for TokenStream {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.0.len())
    }
}

impl Slice<RangeFull> for TokenStream {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        self.clone()
    }
}

impl InputTake for TokenStream {
    fn take(&self, count: usize) -> Self {
        self.slice(0..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl Offset for TokenStream {
    fn offset(&self, second: &Self) -> usize {
        if let Some(target) = second.0.first() {
            let mut offset = 0_usize;
            for actual in &self.0 {
                if actual == target {
                    return offset;
                }
                offset += 1;
            }
            panic!("Cannot calculate offset!");
        } else {
            0
        }
    }
}

impl Compare<Token> for TokenStream {
    fn compare(&self, t: Token) -> CompareResult {
        if let Some((token, _)) = self.0.first() {
            match ((discriminant(token) == discriminant(&t)) && {
                match token {
                    Token::Identifier(_) => true,
                    _ => false,
                }
            }) || token == &t
            {
                true => CompareResult::Ok,
                false => CompareResult::Error,
            }
        } else {
            CompareResult::Incomplete
        }
    }

    fn compare_no_case(&self, _t: Token) -> CompareResult {
        panic!(
            "Case insensitive comparisons aren't available as tokens aren't a stringified structure!"
        )
    }
}

impl FindSubstring<Token> for TokenStream {
    fn find_substring(&self, substr: Token) -> Option<usize> {
        for (index, (token, _)) in self.iter_indices() {
            if token == substr {
                return Some(index);
            }
        }
        None
    }
}

impl InputIter for TokenStream {
    type Item = (Token, Span);
    type Iter = Enumerate<TokenStream>;
    type IterElem = TokenStream;

    #[inline]
    fn iter_indices(&self) -> Enumerate<Self> {
        self.clone().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> Self {
        self.clone()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.clone().into_iter().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.0.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl Into<TokenStream> for (Token, Span) {
    fn into(self) -> TokenStream {
        TokenStream(SmallVec::from(vec![self]), None)
    }
}

impl From<TokenStream> for Token {
    fn from(value: TokenStream) -> Self {
        if value.0.len() == 1 {
            value.0.first().unwrap().0.clone()
        } else {
            panic!("Token streams with no or more than one token cannot be converted into tokens!")
        }
    }
}

impl From<Token> for TokenStream {
    fn from(value: Token) -> Self {
        Self(SmallVec::from(vec![(value, Span::default())]), None)
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|s| format!("{} #{}", s.0, s.1))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
