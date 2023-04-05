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

use crate::{
    generic::{Definition, Namespace},
    literal::Int,
    map_token_with_field, separated, Input,
};

use kiban_commons::*;
use kiban_lexer::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

node_variant! { Types {
    Name(Namespace),
    Boolean,
    Integer(NumberDef),
    Float(NumberDef),
    Array(SBox<Types>, Int),
    Tuple(Vec<Types>),
    Function {
        params: Vec<Definition>,
        expect: Option<SBox<Types>>,
    },
}}

impl Parsable<Input, (Self, Span)> for _Types {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map(Namespace::parse, |s| (Self::Name(s.clone()), s.location)),
            map(tag(Token::Type(Type::Bool)), |s: Input| {
                (Self::Boolean, s.span())
            }),
            map_token_with_field!(Token::Type, Type::Int, Self::Integer),
            map_token_with_field!(Token::Type, Type::Float, Self::Float),
            map(
                tuple((
                    separated!(right tag(OP_SQ_BRACKET)),
                    separated!(both pair(
                        terminated(
                            Types::parse,
                            separated!(both tag(SEMICOLON))
                        ),
                        Int::parse
                    )),
                    separated!(left tag(CLS_SQ_BRACKET)),
                )),
                |(first, (types, size), last): (Input, (_, _), Input)| {
                    (
                        Self::Array(SBox::new(types), size),
                        Span::from_combination(first.span(), last.span()),
                    )
                },
            ),
            map(
                tuple((
                    separated!(right tag(OP_PAREN)),
                    separated_list1(separated!(both tag(COMMA)), Types::parse),
                    separated!(left tag(CLS_PAREN)),
                )),
                |(first, types, last): (Input, _, Input)| {
                    (
                        Self::Tuple(types),
                        Span::from_combination(first.span(), last.span()),
                    )
                },
            ),
            map(
                tuple((
                    tag(Token::Keyword(Keyword::Fn)),
                    preceded(
                        separated!(both tag(OP_PAREN)),
                        pair(
                            separated_list0(separated!(both tag(COMMA)), Definition::parse),
                            separated!(left tag(CLS_PAREN)),
                        ),
                    ),
                    opt(preceded(separated!(both tag(RTRN_TY)), Types::parse)),
                )),
                |(first, (params, last_sep), expect): (Input, (_, Input), Option<_>)| {
                    (
                        Self::Function {
                            params,
                            expect: expect.clone().map(|s| SBox::new(s)),
                        },
                        Span::from_combination(first.span(), {
                            if let Some(expect) = expect {
                                expect.location
                            } else {
                                last_sep.span()
                            }
                        }),
                    )
                },
            ),
        ))(s)
    }
}
