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

use super::{primitives::Types, statement::Statement, Check, Defined, Identifier};

#[derive(Clone, PartialEq, Debug)]
pub struct Container {
    pub statements: Vec<Statement>,
    pub expect: Types,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Scope(pub Container);

#[derive(Clone, PartialEq, Debug)]
pub struct Callable {
    pub args: Vec<Defined>,
    pub container: Container,
}

impl Container {
    pub fn infer_type_by_context(statements: Vec<Statement>) -> Result<Types, String> {
        let mut last_type: Option<Types> = None;
        for statement in statements {
            if let Statement::Return(Some(expression)) = statement {
                let expression_type = expression.infer_type();
                if let Some(last_unwrapped) = last_type.clone() {
                    if last_unwrapped != Types::Unknown && expression_type != Types::Unknown {
                        if last_unwrapped != expression_type {
                            return Err(format!(
                                "Multiple return types ({:?} & {:?}) in the same scope!",
                                last_type,
                                expression.infer_type()
                            ));
                        } else if last_unwrapped != Types::Unknown
                            && expression_type == Types::Unknown
                        {
                            last_type = Some(Types::Unknown);
                        }
                    }
                } else {
                    last_type = Some(expression.infer_type());
                }
            }
        }
        Ok(last_type.unwrap_or(Types::Unknown))
    }
}

impl TryFrom<Vec<Statement>> for Container {
    type Error = String;

    fn try_from(value: Vec<Statement>) -> Result<Self, Self::Error> {
        let scope = Self {
            statements: value.clone(),
            expect: { Self::infer_type_by_context(value)? },
        };
        Ok(scope)
    }
}

impl Check for Container {
    fn check(&self) -> Result<(), String> {
        for statement in &self.statements {
            statement.check()?;
        }
        let inferred_type = Self::infer_type_by_context(self.statements.clone())?;
        if self.expect != inferred_type
            && inferred_type != Types::Unknown
            && self.expect != Types::Unknown
            && std::mem::discriminant(&self.expect)
                != std::mem::discriminant(&Types::Reference(Identifier(vec![])))
        {
            return Err(format!(
                "Container returns {:?} but must return {:?}!",
                inferred_type, self.expect
            ));
        } else {
            Ok(())
        }
    }
}

impl Check for Scope {
    fn check(&self) -> Result<(), String> {
        self.0.check()
    }
}

impl Check for Callable {
    fn check(&self) -> Result<(), String> {
        self.container.check()
    }
}
