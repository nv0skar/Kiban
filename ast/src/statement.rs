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
    expression::{Expression, _Expression},
    generic::Definition,
    node::Node,
    separated, Input,
};

use kiban_commons::*;
use kiban_lexer::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{pair, preceded, tuple},
    IResult,
};

node_variant! { Statement {
    Expression(Expression),
    Declare {
        declaration: Definition,
        value: Option<Expression>,
    },
}}

impl Parsable<Input, (Self, Span)> for _Statement {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map(
                pair(_declaration, separated!(both tag(SEMICOLON))),
                |((declaration, dec_span), last)| {
                    (declaration, Span::from_combination(dec_span, last.span()))
                },
            ),
            alt((
                map(
                    pair(_expression, separated!(both tag(SEMICOLON))),
                    |((expression, exp_span), last)| {
                        (expression, Span::from_combination(exp_span, last.span()))
                    },
                ),
                map(_expression, |(s, location)| {
                    if let _Statement::Expression(expr) = s {
                        (
                            _Statement::Expression(Node {
                                inner: _Expression::Return(Some(SBox::new(expr))),
                                location: location.clone(),
                            }),
                            location,
                        )
                    } else {
                        panic!("Expected an expression for return inferring!")
                    }
                }),
            )),
        ))(s)
    }
}

fn _expression(s: Input) -> IResult<Input, (_Statement, Span)> {
    map(Expression::parse, |s| {
        (_Statement::Expression(s.clone()), s.location)
    })(s)
}

fn _declaration(s: Input) -> IResult<Input, (_Statement, Span)> {
    map(
        tuple((
            separated!(right tag(LET)),
            Definition::parse,
            opt(preceded(separated!(both tag(ASSIGN)), Expression::parse)),
        )),
        |(first, definition, expression)| {
            (
                _Statement::Declare {
                    declaration: definition.clone(),
                    value: expression.clone(),
                },
                Span::from_combination(
                    first.span(),
                    Span::from_combination(first.span(), {
                        if let Some(expression) = expression {
                            expression.location
                        } else {
                            definition.location
                        }
                    }),
                ),
            )
        },
    )(s)
}
