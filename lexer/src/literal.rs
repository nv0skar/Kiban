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
pub enum ProcLit {
    #[display(fmt = "{} (bool)", _0)]
    Bool(bool),
    #[display(fmt = "{} (integer)", _0)]
    Int(usize),
    #[display(fmt = "{} (float)", _0)]
    Float(f32),
}

impl<'i> Lexeme<'i> for ProcLit {
    fn parse(s: &mut Fragment) -> Option<Token<'i>> {
        if let Some(span) = s.consume_pattern("true") {
            Some(Token::new(TokenKind::ProcLit(Self::Bool(true)), span))
        } else if let Some(span) = s.consume_pattern("false") {
            Some(Token::new(TokenKind::ProcLit(Self::Bool(false)), span))
        } else if let Some(((is_decimal, number), span)) = s.consume_number() {
            Some(Token::new(
                TokenKind::ProcLit(if !is_decimal {
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

impl PartialEq for ProcLit {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl TokenOrigin for ProcLit {
    fn origin(&self) -> Option<CompactString> {
        Some(match self {
            ProcLit::Bool(bool) => bool.to_compact_string(),
            ProcLit::Int(int) => int.to_compact_string(),
            ProcLit::Float(float) => float.to_compact_string(),
        })
    }
}
