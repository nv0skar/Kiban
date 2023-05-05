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

node! {
    #[doc = "Define kinds of literals"]
    case Literal<'i> {
        Bool(bool),
        Int(usize),
        Float(f32),
        Char(char),
        Str(&'i str)
    } {
        select! {
            TokenKind::Literal(LiteralToken::Bool(bool)) = s => Node::new(_Literal::Bool(bool).into(), s),
            TokenKind::Literal(LiteralToken::Int(int)) = s => Node::new(_Literal::Int(int).into(), s),
            TokenKind::Literal(LiteralToken::Float(float)) = s => Node::new(_Literal::Float(float).into(), s),
            TokenKind::Literal(LiteralToken::Char(ch)) = s => {
                if ch.len() == 1 {
                    Node::new(_Literal::Char(ch.chars().next().unwrap()).into(), s)
                } else {
                    // Node::new_err(Error::Parser { found: ch.to_compact_string(), help: Some(CompactString::new("Strings can only be de")), span: () })
                    todo!()
                }
            },
            TokenKind::Literal(LiteralToken::Str(str)) = s => Node::new(_Literal::Str(str).into(), s),
        }.boxed()
    }
}
