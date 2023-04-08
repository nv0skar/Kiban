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

pub mod binary;
pub mod range;
pub mod unary;

use crate::{
    body::{Body, Parameters, _Body},
    closure::{Closure, _Closure},
    generic::{Identifier, Namespace},
    literal::Literal as LiteralTree,
    node::Node,
    statement::Statement,
    types::Types,
    Input, Parsable,
};

use binary::*;
use range::Range;
use unary::Unary;

use kiban_commons::*;
use kiban_lexer::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::{fold_many0, separated_list0, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use nom_recursive::recursive_parser;

node_variant! { Expression {
    Name(Namespace),
    Refer(Expression),
    Derefer(Expression),
    Unary(Unary, Expression),
    Binary {
        operator: Binary,
        lhs: Expression,
        rhs: Expression,
    },
    Literal(LiteralTree),
    Closure(Closure),
    Array(SVec<Expression>),
    Tuple(SVec<Expression>),
    Func(Body),
    Range(Range),
    Assign {
        name: Identifier,
        value: Expression,
    },
    Field {
        from: Expression,
        to: Identifier,
    },
    Call {
        to: Expression,
        args: SVec<Expression>,
    },
    Index {
        from: Expression,
        to: Expression,
    },
    Cast {
        target: Expression,
        to: Types,
    },
    Cond {
        check: Expression,
        then: Expression,
        if_not: Option<Expression>,
    },
    Loop {
        repeat: Expression,
    },
    While {
        check: Expression,
        repeat: Expression,
    },
    For {
        item: Identifier,
        iterable: Expression,
        then: Expression,
    },
    Continue,
    Break,
}}

impl Parsable<Input, (Self, Span)> for _Expression {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            alt((
                map(tag(CONTINUE), |s: Input| (Self::Continue, s.span())),
                map(tag(BREAK), |s: Input| (Self::Continue, s.span())),
            )),
            _closure,
            alt((_ref, _deref)),
            alt((_func, _cond, _loop, _for, _while)),
            _unary,
            _binary,
            _range,
            _assign,
            alt((_array, _tuple)),
            alt((_call, _field, _index, _cast)),
            _literal,
            map(Namespace::parse, |s| (Self::Name(s.clone()), s.location)),
        ))(s)
    }
}

fn _closure(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        map(
            tuple((
                tag(OP_BRACE),
                fold_many0(Statement::parse, SVec::new, |mut buff, s| {
                    buff.push(s);
                    buff
                }),
                tag(CLS_BRACE),
            )),
            |(start, statements, last)| {
                (
                    _Closure(statements),
                    Span::from_combination(start.span(), last.span()),
                )
            },
        ),
        |s| (_Expression::Closure(s.clone().into()), s.1),
    )(s)
}

fn _ref(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(pair(tag(AMPRSND), Expression::parse), |(ref_token, s)| {
        (
            _Expression::Refer(s.clone()),
            Span::from_combination(ref_token.span(), s.location),
        )
    })(s)
}

fn _deref(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(pair(tag(STAR), Expression::parse), |(ref_token, s)| {
        (
            _Expression::Refer(s.clone()),
            Span::from_combination(ref_token.span(), s.location),
        )
    })(s)
}

fn _unary(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(pair(Unary::parse, Expression::parse), |(op, expr)| {
        (
            _Expression::Unary(op.clone(), expr.clone()),
            Span::from_combination(op.location, expr.location),
        )
    })(s)
}

#[recursive_parser]
fn _binary(s: Input) -> IResult<Input, (_Expression, Span)> {
    alt((
        addition,
        substraction,
        multiplication,
        division,
        exponentiation,
        remainder,
        eq,
        not_eq,
        greater,
        less,
        greater_eq,
        less_eq,
        and,
        or,
        x_or,
    ))(s)
}

fn _literal(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(LiteralTree::parse, |s| {
        (_Expression::Literal(s.clone()), s.location)
    })(s)
}

fn _range(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(Range::parse, |s| {
        (_Expression::Range(s.clone()), s.location)
    })(s)
}

#[recursive_parser]
fn _call(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            Expression::parse,
            preceded(
                tag(OP_PAREN),
                separated_list0(tag(COMMA), Expression::parse),
            ),
            tag(CLS_PAREN),
        )),
        |(expr, to, last)| {
            (
                _Expression::Call {
                    to: expr.clone(),
                    args: to.clone().into(),
                },
                Span::from_combination(expr.location.clone(), last.span()),
            )
        },
    )(s)
}

#[recursive_parser]
fn _field(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        pair(Expression::parse, preceded(tag(DOT), Identifier::parse)),
        |(expr, to)| {
            (
                _Expression::Field {
                    from: expr.clone(),
                    to: to.clone(),
                },
                Span::from_combination(expr.location.clone(), to.location),
            )
        },
    )(s)
}

#[recursive_parser]
fn _cast(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        pair(Expression::parse, preceded(tag(AS), Types::parse)),
        |(expr, to)| {
            (
                _Expression::Cast {
                    target: expr.clone(),
                    to: to.clone(),
                },
                Span::from_combination(expr.location.clone(), to.location),
            )
        },
    )(s)
}

#[recursive_parser]
fn _index(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            Expression::parse,
            preceded(tag(OP_SQ_BRACKET), Expression::parse),
            tag(CLS_SQ_BRACKET),
        )),
        |(expr, to, last)| {
            (
                _Expression::Index {
                    from: expr.clone(),
                    to: to.clone(),
                },
                Span::from_combination(expr.location.clone(), last.span()),
            )
        },
    )(s)
}

fn _array(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            tag(OP_SQ_BRACKET),
            separated_list1(tag(COMMA), Expression::parse),
            tag(CLS_SQ_BRACKET),
        )),
        |(start, s, last)| {
            (
                _Expression::Array(s.into()),
                Span::from_combination(start.span(), last.span()),
            )
        },
    )(s)
}

fn _tuple(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            tag(OP_PAREN),
            separated_list1(tag(COMMA), Expression::parse),
            tag(CLS_PAREN),
        )),
        |(start, s, last)| {
            (
                _Expression::Tuple(s.into()),
                Span::from_combination(start.span(), last.span()),
            )
        },
    )(s)
}

fn _assign(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((terminated(Identifier::parse, tag(EQ)), Expression::parse)),
        |(name, value)| {
            (
                _Expression::Assign {
                    name: name.clone(),
                    value: value.clone(),
                },
                Span::from_combination(name.location, value.location),
            )
        },
    )(s)
}

fn _func(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            tag(VERT_BAR),
            alt((
                map(tag(UNDERLINE), |_| None),
                map(Parameters::parse, |s| Some(s)),
            )),
            preceded(tag(VERT_BAR), Closure::parse),
        )),
        |(from, params, clsr)| {
            (
                _Expression::Func(Node {
                    inner: SBox::new(_Body {
                        params,
                        closure: clsr.clone(),
                        expect: None,
                    }),
                    location: Span::from_combination(from.span(), clsr.clone().location),
                }),
                Span::from_combination(from.span(), clsr.location),
            )
        },
    )(s)
}

fn _cond(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            tag(IF),
            Expression::parse,
            Expression::parse,
            opt(preceded(tag(ELSE), Expression::parse)),
        )),
        |(first, check, then, if_not)| {
            (
                _Expression::Cond {
                    check: check,
                    then: then.clone(),
                    if_not: if_not.clone(),
                },
                Span::from_combination(first.span(), {
                    match if_not {
                        Some(value) => value.location,
                        None => then.location,
                    }
                }),
            )
        },
    )(s)
}

fn _loop(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(tuple((tag(LOOP), Expression::parse)), |(first, repeat)| {
        (
            _Expression::Loop {
                repeat: repeat.clone(),
            },
            Span::from_combination(first.span(), repeat.location),
        )
    })(s)
}

fn _for(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((
            tag(FOR),
            Identifier::parse,
            tag(IN),
            Expression::parse,
            Expression::parse,
        )),
        |(first, item, _, iterable, then)| {
            (
                _Expression::For {
                    item,
                    iterable: iterable,
                    then: then.clone(),
                },
                Span::from_combination(first.span(), then.location),
            )
        },
    )(s)
}

fn _while(s: Input) -> IResult<Input, (_Expression, Span)> {
    map(
        tuple((tag(WHILE), Expression::parse, Expression::parse)),
        |(first, check, repeat)| {
            (
                _Expression::While {
                    check: check,
                    repeat: repeat.clone(),
                },
                Span::from_combination(first.span(), repeat.location),
            )
        },
    )(s)
}
