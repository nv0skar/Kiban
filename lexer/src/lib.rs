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
use std::{fmt::Display, mem::discriminant};

use arrayvec::ArrayString;
use compact_str::{CompactString, ToCompactString};
use derive_more::{Constructor, Display};
use smallvec::SmallVec;

pub trait TokenOrigin {
    fn origin(&self) -> Option<CompactString>;
}

/// Token stream with recursive info
#[derive(Clone, Constructor, Default, Debug)]
pub struct TokenStream(SVec<Token>, Option<usize>);

/// Localised token
#[derive(Clone, Constructor, PartialEq, Debug)]
pub struct Token {
    kind: TokenKind,
    location: Span,
}

/// Token kinds
#[derive(Clone, PartialEq, Display, Debug)]
#[display(fmt = "{}")]
pub enum TokenKind {
    #[display(fmt = "{} (id)", _0)]
    Identifier(CompactString),
    #[display(fmt = "{} (kw)", _0)]
    Keyword(Keyword),
    #[display(fmt = "{} (punct)", _0)]
    Punctuation(Punctuation),
    #[display(fmt = "{} (lit)", _0)]
    Literal(Literal),
    #[display(fmt = "{} (comment)", _0)]
    Comment(Comment),
    #[display(fmt = "{} (unknown)", _0)]
    Unknown(char),
}

impl Spanned for TokenStream {
    fn span(&self) -> Span {
        if let (Some(start), Some(end)) = (self.0.first(), self.0.last()) {
            Span::new(
                *start.location.offset(),
                (end.location.offset() + end.location.length()) - start.location.offset(),
            )
        } else {
            panic!("There is token stream to calculate span!")
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

impl PartialEq<Token> for TokenStream {
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

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.first().cloned();
        if value.is_some() {
            self.0.remove(0);
        };
        value
    }
}

impl Into<TokenStream> for Token {
    fn into(self) -> TokenStream {
        TokenStream(SmallVec::from(vec![self]), None)
    }
}

impl From<TokenStream> for Token {
    fn from(value: TokenStream) -> Self {
        if value.0.len() == 1 {
            value.0.first().unwrap().clone()
        } else {
            panic!("Token streams with no or more than one token cannot be converted into tokens!")
        }
    }
}

impl TokenOrigin for TokenKind {
    fn origin(&self) -> Option<CompactString> {
        match self {
            Self::Identifier(ident) => Some(ident.to_compact_string()),
            Self::Keyword(kw) => kw.origin(),
            Self::Punctuation(punc) => punc.origin(),
            Self::Literal(lit) => lit.origin(),
            Self::Comment(_) => None,
            Self::Unknown(unknown) => Some(unknown.to_compact_string()),
        }
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|s| format!("{} #{}", s.kind, s.location))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
