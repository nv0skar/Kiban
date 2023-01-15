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

pub mod container;
pub mod expression;
pub mod primitives;
pub mod statement;

use std::{collections::HashMap, hash::Hash};

use {
    container::Callable,
    primitives::{Blend, Types, Value},
};

pub trait Check {
    fn check(&self) -> Result<(), String>;
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Identifier(pub Vec<u8>);

#[derive(Clone, PartialEq, Debug)]
pub struct Defined {
    pub id: Identifier,
    pub kind: Types,
}

#[derive(Clone, PartialEq, Debug)]

pub enum Lexis {
    Import,
    Constant(Value),
    Blend(Blend),
    Function((bool, Callable)),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Module {
    pub id: Option<Identifier>,
    pub content: HashMap<Identifier, Lexis>,
}

impl Module {
    pub fn find_entry(&self) -> Result<Identifier, String> {
        let mut entry_id: Option<Identifier> = None;
        for (id, lexis) in &self.content {
            if let Lexis::Function((is_entry, _)) = lexis {
                if *is_entry {
                    if entry_id.is_some() {
                        return Err("Cannot exist multiple entry functions!".to_string());
                    }
                    entry_id = Some(id.clone());
                }
            }
        }
        if let Some(id) = entry_id {
            Ok(id)
        } else {
            Err("No entry function found!".to_string())
        }
    }
}

impl Check for Module {
    fn check(&self) -> Result<(), String> {
        for (id, lexis) in &self.content {
            match lexis {
                Lexis::Import => {
                    if let Some(module_id) = &self.id {
                        if id == module_id {
                            return Err(format!("Module {:?} cannot import to itself!", id));
                        }
                    }
                }
                Lexis::Function((_, callable)) => callable.check()?,
                _ => (),
            }
        }
        Ok(())
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value.as_bytes().to_vec())
    }
}
