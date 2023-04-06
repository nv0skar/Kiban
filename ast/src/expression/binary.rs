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
    map_token, separated, Input,
};

use kiban_commons::*;
use kiban_lexer::*;

use nom::IResult;
use nom_recursive::recursive_parser;

macro_rules! construct_binary {
    ($(($token:path, $type_of:ident)),*) => {
        paste::paste! {
            $(
                #[recursive_parser]
                pub fn [<$type_of:snake>](s: Input) -> IResult<Input, (_Expression, Span)> {
                    nom::combinator::map(
                        nom::sequence::tuple((
                            Expression::parse,
                            separated!(both map_token!($token, _Binary::$type_of)),
                            Expression::parse,
                        )),
                        |(lhs, op, rhs)| {
                            (
                                _Expression::Binary {
                                    operator: op.into(),
                                    lhs: lhs.clone(),
                                    rhs: rhs.clone(),
                                },
                                Span::from_combination(lhs.location, rhs.location),
                            )
                        },
                    )(s)
                }
            )*
        }
    }
}

node_variant! { Binary {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Exponentiation,
    Remainder,
    Eq,
    NotEq,
    Greater,
    Less,
    GreaterEq,
    LessEq,
    And,
    Or,
    XOr,
}}

construct_binary! {
    (PLUS, Addition),
    (MINUS, Substraction),
    (MUL_DEREF, Multiplication),
    (DIV, Division),
    (POW, Exponentiation),
    (MOD, Remainder),
    (EQ, Eq),
    (N_EQ, NotEq),
    (MORE_THAN, Greater),
    (LESS_THAN, Less),
    (MORE_EQ_THAN, GreaterEq),
    (LESS_EQ_THAN, LessEq),
    (AND, And),
    (OR, Or),
    (X_OR, XOr)
}
