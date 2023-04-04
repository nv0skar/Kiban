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

use crate::{separated, separator, types::Types, Input};

use kiban_commons::*;
use kiban_lexer::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, terminated},
    IResult,
};
use smol_str::SmolStr;

node_def!( Identifier(pub SmolStr) );

impl Parsable<Input, (Self, Span)> for _Identifier {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            alt((
                tag(Token::Identifier(SmolStr::default())),
                tag(Token::Literal(Literal::Int(isize::default()))),
            )),
            |s: Input| {
                (
                    Self({
                        match Into::<Token>::into(s.clone()) {
                            Token::Identifier(id) => id,
                            Token::Literal(Literal::Int(id)) => {
                                SmolStr::from((id as usize).to_string())
                            }
                            _ => panic!("Expected identifier or literal integer!"),
                        }
                    }),
                    s.span(),
                )
            },
        )(s)
    }
}

node_def! { Definition {
    pub id: Identifier,
    pub kind: Types,
}}

impl Parsable<Input, (Self, Span)> for _Definition {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            pair(
                terminated(
                    Identifier::parse,
                    separated!(both tag(Token::Punctuation(Punctuation::Colon))),
                ),
                Types::parse,
            ),
            |(id, kind)| {
                (
                    Self {
                        id: id.clone(),
                        kind: kind.clone(),
                    },
                    Span::from_spans(id.location, kind.location),
                )
            },
        )(s)
    }
}

node_def!( Namespace(pub SVec<Identifier>) );

impl Parsable<Input, (Self, Span)> for _Namespace {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            separated_list1(
                separated!(both pair(
                    tag(Token::Punctuation(Punctuation::Colon)),
                    tag(Token::Punctuation(Punctuation::Colon)),
                )),
                Identifier::parse,
            ),
            |s| {
                (
                    Self(SVec::from(s.clone())),
                    Span::from_spans(
                        s.first().unwrap().location.clone(),
                        s.last().unwrap().location.clone(),
                    ),
                )
            },
        )(s)
    }
}
