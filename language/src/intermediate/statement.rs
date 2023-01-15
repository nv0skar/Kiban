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

use super::{expression::Expression, primitives::Types, Check, Defined, Identifier};

#[derive(Clone, PartialEq, Debug)]
pub enum Statement {
    Expression(Expression),
    Declare {
        declaration: Defined,
        value: Option<Expression>,
    },
    Assign {
        reference: Expression,
        value: Expression,
    },
    Condition {
        check: Expression,
        then: Box<Self>,
        if_not: Option<Box<Self>>,
    },
    Loop {
        check: Expression,
        repeat: Box<Self>,
    },
    For {
        item: Identifier,
        iterable: Expression,
        then: Box<Self>,
    },
    Continue,
    Break,
    Return(Option<Expression>),
}

impl Check for Statement {
    fn check(&self) -> Result<(), String> {
        match self {
            Self::Expression(expression) => expression.check(),
            Self::Declare {
                declaration,
                value: Some(value),
            } => {
                value.check()?;
                if !(declaration.kind == value.infer_type()) {
                    Err(format!(
                        "Expected {:?} but found {:?}!",
                        declaration.kind, *value
                    ))
                } else {
                    Ok(())
                }
            }
            Self::Condition {
                check,
                then,
                if_not,
            } => {
                then.check()?;
                if let Some(if_not) = if_not {
                    if_not.check()?;
                }
                let check_type = check.infer_type();
                if !(Types::Boolean == check_type || Types::Unknown == check_type) {
                    Err(format!("Invalid type {:?} for condition!", *check))
                } else {
                    Ok(())
                }
            }
            Self::Loop {
                check: Expression::Unary(value),
                repeat,
            } => {
                repeat.check()?;
                if !(Types::Boolean == value.get_type()) {
                    Err(format!("Invalid type {:?} for loop!", *value))
                } else {
                    Ok(())
                }
            }
            Self::For {
                item: _,
                iterable,
                then,
            } => {
                iterable.check()?;
                then.check()?;
                Ok(())
            }
            Self::Return(Some(expression)) => expression.check(),
            _ => Ok(()),
        }
    }
}
