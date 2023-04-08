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

use crate::{generic::Namespace, literal::Int, Input, Parsable};

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
    Char,
    Array(SBox<Types>, Int),
    Tuple(SVec<Types>),
    Function {
        params: SVec<Types>,
        expect: Option<SBox<Types>>,
    },
}}

impl Parsable<Input, (Self, Span)> for _Types {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map(Namespace::parse, |s| (Self::Name(s.clone()), s.location)),
            map(tag(BOOL), |s: Input| (Self::Boolean, s.span())),
            map(
                tuple((
                    tag(OP_SQ_BRACKET),
                    pair(terminated(Types::parse, tag(SEMICOLON)), Int::parse),
                    tag(CLS_SQ_BRACKET),
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
                    tag(OP_PAREN),
                    separated_list1(tag(COMMA), Types::parse),
                    tag(CLS_PAREN),
                )),
                |(first, types, last): (Input, _, Input)| {
                    (
                        Self::Tuple(types.into()),
                        Span::from_combination(first.span(), last.span()),
                    )
                },
            ),
            map(
                tuple((
                    tag(FN_TY),
                    preceded(
                        tag(OP_PAREN),
                        pair(separated_list0(tag(COMMA), Types::parse), tag(CLS_PAREN)),
                    ),
                    opt(preceded(pair(tag(LINE), tag(CLS_CHEVRON)), Types::parse)),
                )),
                |(first, (params, last_sep), expect): (Input, (_, Input), Option<_>)| {
                    (
                        Self::Function {
                            params: params.into(),
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
