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
    Input, Parsable,
};

use kiban_commons::*;
use kiban_lexer::*;

use nom::IResult;
use nom_recursive::recursive_parser;

macro_rules! construct_binary {
    ($(($token:tt, $type_of:ident)),*) => {
        paste::paste! {
            $(
                #[recursive_parser]
                pub fn [<$type_of:snake>](s: Input) -> IResult<Input, (_Expression, Span)> {
                    nom::combinator::map(
                        nom::sequence::tuple((
                            Expression::parse,
                            crate::map_token!($token, _Binary::$type_of),
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
    LftShift,
    RghtShift,
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
    (LINE, Substraction),
    (STAR, Multiplication),
    (SLASH, Division),
    ((STAR, STAR), Exponentiation),
    (PERCENT, Remainder),
    ((OP_CHEVRON, OP_CHEVRON), LftShift),
    ((CLS_CHEVRON, CLS_CHEVRON), RghtShift),
    (EQ, Eq),
    ((EXCLMTN, EQ), NotEq),
    ((CLS_CHEVRON, EQ), GreaterEq),
    ((OP_CHEVRON, EQ), LessEq),
    (CLS_CHEVRON, Greater),
    (OP_CHEVRON, Less),
    ((AMPRSND, AMPRSND), And),
    ((VERT_BAR, VERT_BAR), Or),
    (CARET, XOr)
}
