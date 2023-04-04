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

pub mod linting;

use linting::Linting;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("{} error(s)", .0.len())]
#[diagnostic(code(kiban_error::Error))]
pub struct Many(#[related] pub Vec<Error>);

#[derive(Error, Diagnostic, Debug)]
#[error("error")]
pub struct Error {
    #[source_code]
    pub src: NamedSource,
    #[label("unexpected!")]
    pub location: Option<SourceSpan>,
    #[diagnostic_source]
    pub error: Kinds,
}

#[derive(Error, Diagnostic, Debug)]
#[error(transparent)]
pub enum Kinds {
    #[error("error while opening the file!")]
    FSError,
    #[error("lexer error!")]
    Lexer(#[help] Option<String>),
    #[error("syntax parsing error!")]
    SyntaxParsing(#[help] Option<String>),
    #[error("linting error!")]
    Linting(#[diagnostic_source] Linting),
}

impl From<Vec<Error>> for Many {
    fn from(value: Vec<Error>) -> Self {
        Self(value)
    }
}

impl Error {
    pub fn from_raw_source(src: String, error: Kinds) -> Self {
        Self {
            src: NamedSource::new("", src.clone()),
            location: Some(SourceSpan::new(0.into(), src.len().into())),
            error,
        }
    }
}
