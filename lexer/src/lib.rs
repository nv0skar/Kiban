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
pub mod input;
pub mod keyword;
pub mod literal;
pub mod punctuation;

pub use comment::*;
pub use input::*;
pub use keyword::*;
pub use literal::*;
pub use punctuation::*;

use kiban_commons::*;
use kiban_lexer_derive::TokenParser;

use core::panic;
use std::{
    fmt::Display,
    mem::discriminant,
    ops::{Range, RangeFrom},
};

use chumsky::input::{Input as ParserInput, SliceInput, ValueInput};
use compact_str::{CompactString, ToCompactString};
use crossbeam::queue::SegQueue;
use derive_more::{Constructor, Display};
use itertools::Itertools;
use scoped_threadpool::Pool;
use smallvec::SmallVec;

pub trait TokenOrigin {
    fn origin(&self) -> Option<CompactString>;
}

/// Token stream with recursive info
#[derive(Clone, Default, Debug)]
pub struct TokenStream<'i>(SVec<Token<'i>>, Option<usize>);

/// Localised token
#[derive(Copy, Clone, Constructor, PartialEq, Debug)]
pub struct Token<'i> {
    kind: TokenKind<'i>,
    span: Span,
}

/// Token kinds
#[derive(Copy, Clone, PartialEq, Display, Debug)]
#[display(fmt = "{}")]
pub enum TokenKind<'i> {
    #[display(fmt = "{} (id)", _0)]
    Identifier(&'i str),
    #[display(fmt = "{} (kw)", _0)]
    Keyword(Keyword),
    #[display(fmt = "{} (punct)", _0)]
    Punctuation(Punctuation),
    #[display(fmt = "{} (lit)", _0)]
    Literal(Literal<'i>),
    #[display(fmt = "{}", _0)]
    Comment(Comment<'i>),
    #[display(fmt = "{} (unknown)", _0)]
    Unknown(char),
}

impl Spanned for TokenStream<'_> {
    fn span(&self) -> Span {
        if let (Some(start), Some(end)) = (self.0.first(), self.0.last()) {
            Span::new(
                *start.span.offset(),
                (end.span.offset() + end.span.length()) - start.span.offset(),
            )
        } else {
            panic!("There is token stream to calculate span!")
        }
    }
}

impl PartialEq for TokenStream<'_> {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(info_self), Some(info_other)) = (self.1.clone(), other.1.clone()) {
            info_self == info_other && self.0 == other.0
        } else {
            self.0 == other.0
        }
    }
}

impl<'i> PartialEq<Token<'i>> for TokenStream<'i> {
    fn eq(&self, t: &Token) -> bool {
        if let Some(Token { kind: token, .. }) = self.0.first() {
            return ((discriminant(token) == discriminant(&t.kind)) && {
                match token {
                    TokenKind::Identifier(_) => true,
                    _ => false,
                }
            }) || *token == t.kind;
        } else {
            return false;
        }
    }
}

impl<'i> Iterator for TokenStream<'i> {
    type Item = Token<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.first().cloned();
        if value.is_some() {
            self.0.remove(0);
        };
        value
    }
}

impl<'i> ParserInput<'i> for TokenStream<'i> {
    type Offset = usize;
    type Token = TokenKind<'i>;
    type Span = Span;

    fn start(&self) -> Self::Offset {
        0
    }

    type TokenMaybe = TokenKind<'i>;

    unsafe fn next_maybe(&self, offset: Self::Offset) -> (Self::Offset, Option<Self::TokenMaybe>) {
        self.next(offset)
    }

    unsafe fn span(&self, rng: Range<Self::Offset>) -> Self::Span {
        let _start_token = self.0.get(rng.start).unwrap();
        Span::from_combination(
            _start_token.span,
            self.0.get(rng.end).unwrap_or(_start_token).span,
        )
    }

    fn prev(offset: Self::Offset) -> Self::Offset {
        offset.saturating_sub(1)
    }
}

impl<'i> ValueInput<'i> for TokenStream<'i> {
    unsafe fn next(&self, offset: Self::Offset) -> (Self::Offset, Option<Self::Token>) {
        if let Some(Token { kind, .. }) = self.0.get(offset) {
            (offset + 1, Some(*kind))
        } else {
            (offset, None)
        }
    }
}

impl<'i> SliceInput<'i> for TokenStream<'i> {
    type Slice = TokenStream<'i>;

    fn slice(&self, rng: Range<Self::Offset>) -> Self::Slice {
        TokenStream(
            self.0.get(rng.clone()).unwrap().into(),
            self.1.map(|offset| offset + rng.start),
        )
    }

    fn slice_from(&self, from: RangeFrom<Self::Offset>) -> Self::Slice {
        TokenStream(
            self.0.get(from.clone()).unwrap().into(),
            self.1.map(|offset| offset + from.start),
        )
    }
}

impl<'i> Into<TokenStream<'i>> for Token<'i> {
    fn into(self) -> TokenStream<'i> {
        TokenStream(SmallVec::from(vec![self]), None)
    }
}

impl<'i> From<TokenStream<'i>> for Token<'i> {
    fn from(value: TokenStream<'i>) -> Self {
        if value.0.len() == 1 {
            value.0.first().unwrap().clone()
        } else {
            panic!("Token streams with no or more than one token cannot be converted into tokens!")
        }
    }
}

impl<'i> TokenOrigin for TokenKind<'i> {
    fn origin(&self) -> Option<CompactString> {
        match self {
            Self::Identifier(ident) => Some(ident.to_compact_string()),
            Self::Keyword(kw) => kw.origin(),
            Self::Punctuation(punc) => punc.origin(),
            Self::Literal(lit) => lit.origin(),
            Self::Comment(..) => None,
            Self::Unknown(unknown) => Some(unknown.to_compact_string()),
        }
    }
}

impl<'i> Display for TokenStream<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|s| format!("{} #{}", s.kind, s.span))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
