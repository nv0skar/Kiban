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

use kiban_ast::Tree;
use kiban_error::{Error, Kinds};
use kiban_lexer::TokenStream;

use std::{fs, path::PathBuf};

use derive_more::Display;
use getset::Getters;
use miette::NamedSource;

#[derive(Clone, PartialEq, Getters, Display, Debug)]
#[display(
    fmt = "Module {:?} ({:?}) from {:?}: {} -> {:?}",
    id,
    path,
    source,
    lexis,
    tree
)]
#[get = "pub"]
pub struct Module<'a> {
    pub id: Option<String>,
    path: Option<PathBuf>,
    source: Option<&'a str>,
    lexis: State<TokenStream>,
    tree: State<Tree>,
}

#[derive(Clone, PartialEq, Display, Debug)]
pub enum State<T> {
    Resolved(T),
    Unresolved,
}

impl<'a> Module<'a> {
    pub fn new(id: Option<String>) -> Self {
        Self {
            id,
            path: None,
            source: None,
            lexis: State::Unresolved,
            tree: State::Unresolved,
        }
    }

    pub fn set_source(&mut self, source: &'a str) -> Result<(), Error> {
        let _lexis = TokenStream::parse(source);
        todo!()
    }
}

impl<'a> TryFrom<PathBuf> for Module<'a> {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let _id = String::from(value.file_name().unwrap().to_str().unwrap());
        let source = fs::read_to_string(value.clone()).map_err(|_| Error {
            src: NamedSource::new(value.to_str().expect("undefined path"), String::new()),
            location: None,
            error: Kinds::FSError,
        })?;
        let _lexis = TokenStream::parse(&source);
        todo!()
    }
}
