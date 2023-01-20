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
    expression::Expression,
    lexis::Defined,
    primitives::Types,
    Identifier,
};

use std::fmt::Display;

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
    fn check(&self, error_track: &mut Vec<Error>) {
        match self {
            Self::Expression(expression) => expression.check(error_track),
            Self::Declare {
                declaration,
                value: Some(value),
            } => {
                value.check(error_track);
                if declaration.kind != value.infer_type()
                    && declaration.kind != Types::Unknown
                    && (std::mem::discriminant(&declaration.kind)
                        != std::mem::discriminant(&Types::Reference(Identifier(vec![]))))
                {
                    error_track.push(Error {
                        explanation: format!("Expected {} but found {}!", declaration.kind, *value),
                        where_is: self.to_string(),
                    })
                }
            }
            Self::Condition {
                check,
                then,
                if_not,
            } => {
                then.check(error_track);
                if let Some(if_not) = if_not {
                    if_not.check(error_track);
                }
                let check_type = check.infer_type();
                if !(Types::Boolean == check_type || Types::Unknown == check_type) {
                    error_track.push(Error {
                        explanation: format!("Invalid type {} for condition!", *check),
                        where_is: self.to_string(),
                    })
                }
            }
            Self::Loop {
                check: Expression::Unary(value),
                repeat,
            } => {
                repeat.check(error_track);
                if !(Types::Boolean == value.get_type()) {
                    error_track.push(Error {
                        explanation: format!("Invalid type {} for loop!", *value),
                        where_is: self.to_string(),
                    })
                }
            }
            Self::For {
                item: _,
                iterable,
                then,
            } => {
                iterable.check(error_track);
                then.check(error_track);
            }
            Self::Return(Some(expression)) => expression.check(error_track),
            _ => (),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expression(expression) => write!(f, "{}", expression),
            Statement::Declare { declaration, value } => write!(f, "let {}{};", declaration, {
                if let Some(value) = value {
                    format!(" = {}", value)
                } else {
                    String::new()
                }
            }),
            Statement::Assign { reference, value } => write!(f, "{} = {};", reference, value),
            Statement::Condition {
                check,
                then,
                if_not,
            } => write!(f, "if {} {}{}", check, then, {
                if let Some(if_not) = if_not {
                    format!(" {}", if_not)
                } else {
                    String::new()
                }
            }),
            Statement::Loop { check, repeat } => write!(f, "loop {} {}", check, repeat),
            Statement::For {
                item,
                iterable,
                then,
            } => write!(f, "for {} in {} {}", item, iterable, then),
            Statement::Continue => write!(f, "continue;"),
            Statement::Break => write!(f, "break;"),
            Statement::Return(expression) => write!(f, "return{};", {
                if let Some(expression) = expression {
                    format!(" {}", expression)
                } else {
                    String::new()
                }
            }),
        }
    }
}
