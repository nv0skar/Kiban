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

use super::{container::Callable, expression::Expression, Check, Defined, Identifier};

#[derive(Clone, PartialEq, Debug)]
pub struct Blend(pub Types);

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
    fn check(&self) -> Result<(), String> {
        match self {
            Value::Fn(callable) => callable.check(),
            _ => Ok(()),
        }
    }
}

impl PartialEq<Value> for Types {
    fn eq(&self, other: &Value) -> bool {
        self == &other.get_type()
    }
}
