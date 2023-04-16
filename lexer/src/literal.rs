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

use crate::*;

use std::mem::discriminant;

/// Tokens that store a literal
#[derive(Copy, Clone, Display, Debug)]
pub enum Literal {
    Bool(bool),
    #[display(fmt = "{} (integer)", _0)]
    Int(usize),
    #[display(fmt = "{} (float)", _0)]
    Float(f32),
    #[display(fmt = "{:?} (char)", _0)]
    Char(char),
    #[display(fmt = "{:?} (str)", _0)]
    Str(ArrayString<1024>),
}

impl Lexeme for Literal {
    fn parse(s: &mut Fragment) -> Option<Token> {
        if let Some(span) = s.consume_pattern("true") {
            Some(Token::new(TokenKind::Literal(Self::Bool(true)), span))
        } else if let Some(span) = s.consume_pattern("false") {
            Some(Token::new(TokenKind::Literal(Self::Bool(false)), span))
        } else if let Some((content, span)) = s.consume_from("\'") {
            Some(Token::new(
                TokenKind::Literal(Self::Char(content.chars().collect::<SVec<_>>()[1])),
                span,
            ))
        } else if let Some((content, span)) = s.consume_from("\"") {
            Some(Token::new(
                TokenKind::Literal(Self::Str(
                    ArrayString::from(&content[1..content.len() - 1]).unwrap(),
                )),
                span,
            ))
        } else if let Some(((is_decimal, number), span)) = s.consume_number() {
            Some(Token::new(
                TokenKind::Literal(if !is_decimal {
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

impl TokenOrigin for Literal {
    fn origin(&self) -> Option<CompactString> {
        Some(match self {
            Literal::Bool(bool) => bool.to_compact_string(),
            Literal::Int(int) => int.to_compact_string(),
            Literal::Float(float) => float.to_compact_string(),
            Literal::Char(ch) => ch.to_compact_string(),
            Literal::Str(str) => str.to_compact_string(),
        })
    }
}
