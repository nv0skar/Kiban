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

use self::check::Check;

pub mod check;
pub mod container;
pub mod expression;
pub mod lexis;
pub mod primitives;
pub mod statement;

use {
    check::Error,
    lexis::{Identifier, Lexis},
};

#[derive(Clone, PartialEq, Debug)]
pub struct Module {
    pub id: Option<Identifier>,
    pub content: Vec<Lexis>,
}

impl Module {
    pub fn find_entry(&self) -> Result<Identifier, String> {
        let mut entry_id: Option<Identifier> = None;
        for lexis in &self.content {
            if let Lexis::Function((is_entry, callable)) = lexis {
                if *is_entry {
                    if entry_id.is_some() {
                        return Err("Cannot exist multiple entry functions!".to_string());
                    }
                    entry_id = Some(callable.id.clone());
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

impl Module {
    pub fn check(&self) -> Vec<Error> {
        let mut error_track: Vec<Error> = vec![];
        for lexis in &self.content {
            match lexis {
                Lexis::Import(import) => {
                    if let Some(module_id) = &self.id {
                        if &import.0 == module_id {
                            error_track.push(Error {
                                explanation: format!(
                                    "Module {} cannot import to itself!",
                                    module_id
                                ),
                                where_is: import.to_string(),
                            });
                        }
                    }
                }
                Lexis::Function((_, callable)) => callable.content.check(error_track.as_mut()),
                _ => (),
            }
        }
        error_track
    }
}
