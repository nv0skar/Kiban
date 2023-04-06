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

use crate::{expression::Expression, generic::Definition, separated, Input};

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
    Return(Option<Expression>),
}}

impl Parsable<Input, (Self, Span)> for _Statement {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map(
                pair(
                    alt((_declaration, _return)),
                    separated!(left tag(SEMICOLON)),
                ),
                |((statement, statement_location), last)| {
                    (
                        statement,
                        Span::from_combination(statement_location, last.span()),
                    )
                },
            ),
            map(
                pair(Expression::parse, opt(separated!(left tag(SEMICOLON)))),
                |(expr, semicolon)| {
                    if let Some(last) = semicolon {
                        (
                            _Statement::Expression(expr.clone()),
                            Span::from_combination(expr.location, last.span()),
                        )
                    } else {
                        (_Statement::Return(Some(expr.clone())), expr.location)
                    }
                },
            ),
        ))(s)
    }
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

fn _return(s: Input) -> IResult<Input, (_Statement, Span)> {
    map(
        pair(tag(RETURN), separated!(left opt(Expression::parse))),
        |(first, rtrn_value)| {
            (
                _Statement::Return(rtrn_value.clone().map(|s| s)),
                Span::from_combination(first.span(), {
                    match rtrn_value {
                        Some(value) => value.location,
                        None => first.span(),
                    }
                }),
            )
        },
    )(s)
}
