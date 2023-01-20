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

use super::{
    check::{Check, Error},
    container::Scope,
    primitives::{Types, Value},
    Identifier,
};

use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub enum Expression {
    Reference(Identifier),
    Scoped(Scope),
    Unary(Value),
    Binary {
        operator: Operator,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
    CallFn {
        target: Box<Self>,
        args: Vec<Self>,
    },
    TypeFn {
        target: Box<Self>,
        function: Identifier,
        args: Vec<Self>,
    },
    Access {
        target: Box<Self>,
        to: Box<Self>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operator {
    Addition,
    Substraction,
    Multiplication,
    Division,
    Exponentiation,
    Mod,
    Equal,
    NotEqual,
    And,
    Or,
    Less,
    LessOrEqual,
    More,
    MoreOrEqual,
}

impl Expression {
    pub fn infer_type(&self) -> Types {
        match self {
            Expression::Scoped(scope) => scope.0.expect.clone(),
            Expression::Unary(value) => value.get_type(),
            _ => Types::Unknown,
        }
    }
}

impl Check for Expression {
    fn check(&self, error_track: &mut Vec<Error>) {
        match self {
            Expression::Scoped(scope) => scope.check(error_track),
            Expression::Unary(value) => value.check(error_track),
            Expression::Binary {
                operator: _,
                lhs,
                rhs,
            } => {
                lhs.check(error_track);
                rhs.check(error_track);
            }
            Expression::CallFn { target: id, args } => {
                id.check(error_track);
                for arg in args {
                    arg.check(error_track);
                }
            }
            Expression::TypeFn {
                target: id,
                function: _,
                args,
            } => {
                id.check(error_track);
                for arg in args {
                    arg.check(error_track);
                }
            }
            Expression::Access { target, to } => {
                target.check(error_track);
                to.check(error_track);
            }
            _ => (),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Reference(id) => write!(f, "{}", id),
            Expression::Scoped(scope) => write!(f, "{}", scope),
            Expression::Unary(value) => write!(f, "{}", value),
            Expression::Binary { operator, lhs, rhs } => write!(f, "{} {} {}", lhs, operator, rhs),
            Expression::CallFn {
                target: function,
                args,
            } => write!(
                f,
                "{}({})",
                function,
                args.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expression::TypeFn {
                target: function,
                function: target,
                args,
            } => write!(
                f,
                "{}.{}({})",
                target,
                function,
                args.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expression::Access { target, to } => write!(f, "{}.{}", target, to),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "+"),
            Operator::Substraction => write!(f, "-"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Division => write!(f, "/"),
            Operator::Exponentiation => write!(f, "^"),
            Operator::Mod => write!(f, "%"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::Less => write!(f, "<"),
            Operator::LessOrEqual => write!(f, "<="),
            Operator::More => write!(f, ">"),
            Operator::MoreOrEqual => write!(f, ">="),
        }
    }
}
