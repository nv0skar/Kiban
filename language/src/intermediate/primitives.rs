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
    container::Callable,
    expression::Expression,
    lexis::{Defined, Identifier},
};

use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub enum Types {
    Unknown,
    Reference(Identifier),
    Boolean,
    Integer,
    Float,
    String,
    Vector(Box<Self>),
    Combination(Vec<Self>),
    Fn {
        args: Vec<Defined>,
        expect: Box<Self>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
    Vector(Vec<Expression>),
    Combination(Vec<Expression>),
    Fn(Callable),
}

impl Value {
    pub fn get_type(&self) -> Types {
        match self {
            Self::Boolean(_) => Types::Boolean,
            Self::Integer(_) => Types::Integer,
            Self::Float(_) => Types::Float,
            Self::String(_) => Types::String,
            Self::Vector(values) => Types::Vector({
                if let Some(first_value) = values.first() {
                    Box::new(first_value.infer_type())
                } else {
                    Box::new(Types::Unknown)
                }
            }),
            Self::Combination(values) => {
                let mut types: Vec<Types> = vec![];
                for value in values {
                    types.push(value.infer_type())
                }
                Types::Combination(types)
            }
            Self::Fn(callable) => Types::Fn {
                args: callable.args.clone(),
                expect: Box::new(callable.container.expect.clone()),
            },
        }
    }
}

impl Check for Value {
    fn check(&self, error_track: &mut Vec<Error>) {
        match self {
            Value::Fn(callable) => callable.check(error_track),
            _ => (),
        }
    }
}

impl PartialEq<Value> for Types {
    fn eq(&self, other: &Value) -> bool {
        self == &other.get_type()
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Unknown => write!(f, "unknown"),
            Types::Reference(id) => write!(f, "{}", id),
            Types::Boolean => write!(f, "bool"),
            Types::Integer => write!(f, "int"),
            Types::Float => write!(f, "float"),
            Types::String => write!(f, "string"),
            Types::Vector(of_type) => write!(f, "vec<{}>", of_type),
            Types::Combination(of_types) => write!(
                f,
                "({})",
                of_types
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Types::Fn { args, expect } => write!(
                f,
                "fn({}) -> {}",
                args.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                expect
            ),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Integer(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Vector(value) => write!(f, "{}", {
                format!(
                    "[{}]",
                    value
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }),
            Value::Combination(value) => write!(f, "{}", {
                format!(
                    "({})",
                    value
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }),
            Value::Fn(value) => write!(
                f,
                "fn({}) {{\n\t{}\n}}",
                value
                    .args
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", "),
                value.container
            ),
        }
    }
}
