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
    container::Scope,
    primitives::{Types, Value},
    Check, Identifier,
};

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
        function: Box<Self>,
        args: Vec<Self>,
    },
    TypeFn {
        function: Box<Self>,
        target: Identifier,
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
    fn check(&self) -> Result<(), String> {
        match self {
            Expression::Scoped(scope) => scope.check(),
            Expression::Unary(value) => value.check(),
            Expression::Binary {
                operator: _,
                lhs,
                rhs,
            } => {
                lhs.check()?;
                rhs.check()?;
                Ok(())
            }
            Expression::CallFn { function: id, args } => {
                id.check()?;
                for arg in args {
                    arg.check()?;
                }
                Ok(())
            }
            Expression::TypeFn {
                function: id,
                target: _,
                args,
            } => {
                id.check()?;
                for arg in args {
                    arg.check()?;
                }
                Ok(())
            }
            Expression::Access { target, to } => {
                target.check()?;
                to.check()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
