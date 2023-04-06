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

use crate::{map_token_with_field, Input};

use kiban_commons::*;
use kiban_lexer::{literal::Literal as Literal_Token, *};

use nom::{branch::alt, combinator::map, IResult};
use smol_str::SmolStr;

macro_rules! literal {
    ($name:ident($type:ty), $token:path) => {
        paste::paste! {
            node_def!($name($type));

            impl [<_ $name>] {
                pub fn literal(s: Input) -> IResult<Input, (_Literal, Span)> {
                    map($name::parse, |s| (_Literal::$name(s.clone()), s.location))(s)
                }
            }

            impl Parsable<Input, (Self, Span)> for [<_ $name>] {
                fn parse(s: Input) -> IResult<Input, (Self, Span)> {
                    map_token_with_field!(Token::Literal, $token, Self)(s)
                }
            }
        }
    };
}

node_variant! { Literal {
    Bool(Bool),
    Int(Int),
    Float(Float),
    Char(Char),
    String(String)
}}

literal!(Bool(bool), Literal_Token::Bool);
literal!(Int(isize), Literal_Token::Int);
literal!(Float(f32), Literal_Token::Float);
literal!(Char(char), Literal_Token::Char);
literal!(String(SmolStr), Literal_Token::String);

impl Parsable<Input, (Self, Span)> for _Literal {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            alt((
                _Bool::literal,
                _Int::literal,
                _Float::literal,
                _Char::literal,
                _String::literal,
            )),
            |s| s.into(),
        )(s)
    }
}
