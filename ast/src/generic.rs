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

use crate::{types::Types, Input, Parsable};

use kiban_commons::*;
use kiban_lexer::*;

use compact_str::{CompactString, ToCompactString};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{pair, preceded},
    IResult,
};

node_def!( Identifier(pub CompactString) );

impl Parsable<Input, (Self, Span)> for _Identifier {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            alt((
                tag(Token::Identifier(CompactString::default())),
                tag(Token::Literal(Literal::Int(usize::default()))),
            )),
            |s: Input| {
                (
                    Self({
                        match Into::<Token>::into(s.clone()) {
                            Token::Identifier(id) => id,
                            Token::Literal(Literal::Int(id)) => (id as usize).to_compact_string(),
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
    pub kind: Option<Types>,
}}

impl Parsable<Input, (Self, Span)> for _Definition {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            pair(Identifier::parse, opt(preceded(tag(COLON), Types::parse))),
            |(id, kind)| {
                (
                    Self {
                        id: id.clone(),
                        kind: kind.clone(),
                    },
                    Span::from_combination(id.location.clone(), {
                        match kind {
                            Some(ty) => ty.location,
                            None => id.location,
                        }
                    }),
                )
            },
        )(s)
    }
}

node_def!( Namespace(pub SVec<Identifier>) );

impl Parsable<Input, (Self, Span)> for _Namespace {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            separated_list1(pair(tag(COLON), tag(COLON)), Identifier::parse),
            |s| {
                (
                    Self(SVec::from(s.clone())),
                    Span::from_combination(
                        s.first().unwrap().location.clone(),
                        s.last().unwrap().location.clone(),
                    ),
                )
            },
        )(s)
    }
}
