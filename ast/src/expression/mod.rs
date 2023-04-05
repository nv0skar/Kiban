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
    separated,
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
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
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
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map(
                pair(separated!(right tag(REF)), Expression::parse),
                |(ref_token, s)| {
                    (
                        Self::Refer(SBox::new(s.clone())),
                        Span::from_combination(ref_token.span(), s.location),
                    )
                },
            ),
            map(
                pair(separated!(right tag(MUL_DEREF)), Expression::parse),
                |(deref_token, s)| {
                    (
                        Self::Derefer(SBox::new(s.clone())),
                        Span::from_combination(deref_token.span(), s.location),
                    )
                },
            ),
            _call,
            _cast,
            _index,
            _field,
            map(
                tuple((
                    separated!(right tag(OP_SQ_BRACKET)),
                    separated_list1(separated!(both tag(COMMA)), Expression::parse),
                    separated!(left tag(CLS_SQ_BRACKET)),
                )),
                |(start, s, last)| {
                    (
                        Self::Array(s),
                        Span::from_combination(start.span(), last.span()),
                    )
                },
            ),
            map(
                tuple((
                    separated!(right tag(OP_PAREN)),
                    separated_list1(separated!(both tag(COMMA)), Expression::parse),
                    separated!(left tag(CLS_PAREN)),
                )),
                |(start, s, last)| {
                    (
                        Self::Tuple(s),
                        Span::from_combination(start.span(), last.span()),
                    )
                },
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
                        Span::from_combination(name.location, value.location),
                    )
                },
            ),
            map(
                tuple((
                    tag(IF),
                    separated!(both Expression::parse),
                    Expression::parse,
                    opt(preceded(separated!(both tag(ELSE)), Expression::parse)),
                )),
                |(first, check, then, if_not)| {
                    (
                        Self::Cond {
                            check: SBox::new(check),
                            then: SBox::new(then.clone()),
                            if_not: if_not.clone().map(|s| SBox::new(s)),
                        },
                        Span::from_combination(first.span(), {
                            match if_not {
                                Some(value) => value.location,
                                None => then.location,
                            }
                        }),
                    )
                },
            ),
            map(
                tuple((tag(LOOP), separated!(left Expression::parse))),
                |(first, repeat)| {
                    (
                        Self::Loop {
                            repeat: SBox::new(repeat.clone()),
                        },
                        Span::from_combination(first.span(), repeat.location),
                    )
                },
            ),
            map(
                tuple((
                    tag(FOR),
                    separated!(both Identifier::parse),
                    tag(IN),
                    separated!(both Expression::parse),
                    Expression::parse,
                )),
                |(first, item, _, iterable, then)| {
                    (
                        Self::For {
                            item,
                            iterable: SBox::new(iterable),
                            then: SBox::new(then.clone()),
                        },
                        Span::from_combination(first.span(), then.location),
                    )
                },
            ),
            map(
                tuple((
                    tag(WHILE),
                    separated!(both Expression::parse),
                    Expression::parse,
                )),
                |(first, check, repeat)| {
                    (
                        Self::While {
                            check: SBox::new(check),
                            repeat: SBox::new(repeat.clone()),
                        },
                        Span::from_combination(first.span(), repeat.location),
                    )
                },
            ),
            map(tag(CONTINUE), |s: Input| (Self::Continue, s.span())),
            map(tag(BREAK), |s: Input| (Self::Continue, s.span())),
            map(
                pair(tag(RETURN), separated!(left opt(Expression::parse))),
                |(first, rtrn_value)| {
                    (
                        Self::Return(rtrn_value.clone().map(|s| SBox::new(s))),
                        Span::from_combination(first.span(), {
                            match rtrn_value {
                                Some(value) => value.location,
                                None => first.span(),
                            }
                        }),
                    )
                },
            ),
            map(Namespace::parse, |s| (Self::Name(s.clone()), s.location)),
        ))(s)
    }
}

#[recursive_parser]
fn _cast(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        pair(
            terminated(Expression::parse, separated!(both tag(AS))),
            Types::parse,
        ),
        |(from, to)| {
            (
                _Expression::Cast {
                    target: SBox::new(from.clone()),
                    to: to.clone(),
                },
                Span::from_combination(from.location, to.location),
            )
        },
    )(s)
}

#[recursive_parser]
fn _index(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            terminated(Expression::parse, separated!(both tag(OP_SQ_BRACKET))),
            Expression::parse,
            separated!(left tag(CLS_SQ_BRACKET)),
        )),
        |(from, to, last)| {
            (
                _Expression::Index {
                    from: SBox::new(from.clone()),
                    to: SBox::new(to.clone()),
                },
                Span::from_combination(from.location, last.span()),
            )
        },
    )(s)
}

#[recursive_parser]
fn _field(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        pair(
            terminated(Expression::parse, separated!(both tag(DOT))),
            Identifier::parse,
        ),
        |(from, to)| {
            (
                _Expression::Field {
                    from: SBox::new(from.clone()),
                    to: to.clone(),
                },
                Span::from_combination(from.location, to.location),
            )
        },
    )(s)
}

#[recursive_parser]
fn _call(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        pair(
            Expression::parse,
            tuple((
                separated!(both tag(OP_PAREN)),
                separated_list0(separated!(both tag(COMMA)), Expression::parse),
                separated!(left tag(CLS_PAREN)),
            )),
        ),
        |(from, (_, to, last))| {
            (
                _Expression::Call {
                    to: SBox::new(from.clone()),
                    args: to.clone(),
                },
                Span::from_combination(from.location, last.span()),
            )
        },
    )(s)
}
