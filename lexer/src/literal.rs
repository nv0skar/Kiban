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

use crate::{Input, Lexeme, Token};

use kiban_commons::*;

use std::mem::discriminant;

use compact_str::CompactString;
use derive_more::Display;

/// Tokens that store a literal
#[derive(Clone, Display, Debug)]
pub enum Literal {
    Bool(bool),
    /// can be parsed into a literal or an identifier
    #[display(fmt = "{} (integer / ident)", _0)]
    Int(usize),
    #[display(fmt = "{} (float)", _0)]
    Float(f32),
    #[display(fmt = "{:?} (char)", _0)]
    Char(char),
    #[display(fmt = "{:?} (string)", _0)]
    String(CompactString),
}

impl Lexeme for Literal {
    fn parse(s: &mut Input) -> Option<(Token, Span)> {
        if let Some(span) = s.consume_specific("true") {
            Some((Token::Literal(Self::Bool(true)), span))
        } else if let Some(span) = s.consume_specific("false") {
            Some((Token::Literal(Self::Bool(false)), span))
        } else if let Some((content, span)) = s.consume_delimited("\'", "\'") {
            Some((
                Token::Literal(Self::Char(content.chars().next().unwrap())),
                span,
            ))
        } else if let Some((content, span)) = s.consume_delimited("\"", "\"") {
            Some((Token::Literal(Self::String(content)), span))
        } else if let Some(((is_decimal, number), span)) = s.consume_num() {
            Some((
                Token::Literal(if !is_decimal {
                    Self::Int(number.parse().unwrap())
                } else {
                    Self::Float(number.parse().unwrap())
                }),
                span,
            ))
        } else {
            None
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
