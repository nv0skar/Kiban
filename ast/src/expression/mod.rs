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

pub mod operator;
pub mod range;

use crate::{
    body::Body,
    generic::{Identifier, Namespace},
    separated, separator,
    statement::Statement,
    types::Types,
    Input,
};

use operator::Operator;
use range::Range;

use kiban_commons::*;
use kiban_lexer::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use nom_recursive::recursive_parser;

node_variant! { Expression {
    Name(Namespace),
    Refer(SBox<Expression>),
    Derefer(SBox<Expression>),
    Closure(Vec<Statement>),
    Op(Operator),
    Array(Vec<Expression>),
    Tuple(Vec<Expression>),
    Func(Body),
    Range(Range),
    Assign {
        name: Identifier,
        value: SBox<Expression>,
    },
    Field {
        from: SBox<Expression>,
        to: Identifier,
    },
    Call {
        to: SBox<Expression>,
        args: Vec<Expression>,
    },
    Index {
        from: SBox<Expression>,
        to: SBox<Expression>,
    },
    Cast {
        target: SBox<Expression>,
        to: Types,
    },
    Cond {
        check: SBox<Expression>,
        then: SBox<Expression>,
        if_not: Option<SBox<Expression>>,
    },
    Loop {
        repeat: SBox<Expression>,
    },
    While {
        check: SBox<Expression>,
        repeat: SBox<Expression>,
    },
    For {
        item: Identifier,
        iterable: SBox<Expression>,
        then: SBox<Expression>,
    },
    Continue,
    Break,
    Return(Option<SBox<Expression>>),
}}

impl Parsable<Input, (Self, Span)> for _Expression {
    #[recursive_parser]
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map(
                pair(separated!(right tag(REF)), Expression::parse),
                |(ref_token, s)| {
                    (
                        Self::Refer(SBox::new(s.clone())),
                        Span::from_spans(ref_token.span(), s.location),
                    )
                },
            ),
            map(
                pair(separated!(right tag(MUL_DEREF)), Expression::parse),
                |(deref_token, s)| {
                    (
                        Self::Derefer(SBox::new(s.clone())),
                        Span::from_spans(deref_token.span(), s.location),
                    )
                },
            ),
            map(
                tuple((
                    separated!(right tag(OP_SQ_BRACKET)),
                    separated_list1(separated!(both tag(COMMA)), Expression::parse),
                    separated!(left tag(CLS_SQ_BRACKET)),
                )),
                |(start, s, last)| (Self::Array(s), Span::from_spans(start.span(), last.span())),
            ),
            map(
                tuple((
                    separated!(right tag(OP_PAREN)),
                    separated_list1(separated!(both tag(COMMA)), Expression::parse),
                    separated!(left tag(CLS_PAREN)),
                )),
                |(start, s, last)| (Self::Tuple(s), Span::from_spans(start.span(), last.span())),
            ),
            map(
                tuple((
                    terminated(Identifier::parse, separated!(both tag(ASSIGN))),
                    Expression::parse,
                )),
                |(name, value)| {
                    (
                        Self::Assign {
                            name: name.clone(),
                            value: SBox::new(value.clone()),
                        },
                        Span::from_spans(name.location, value.location),
                    )
                },
            ),
            map(
                tuple((terminated(Expression::parse, tag(DOT)), Identifier::parse)),
                |(from, to)| {
                    (
                        Self::Field {
                            from: SBox::new(from.clone()),
                            to: to.clone(),
                        },
                        Span::from_spans(from.location, to.location),
                    )
                },
            ),
            map(Namespace::parse, |s| (Self::Name(s.clone()), s.location)),
        ))(s)
    }
}
