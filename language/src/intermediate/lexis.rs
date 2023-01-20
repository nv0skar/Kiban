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
    container::Callable,
    primitives::{Types, Value},
};

use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Identifier(pub Vec<u8>);

#[derive(Clone, PartialEq, Debug)]
pub struct Identified<T> {
    pub id: Identifier,
    pub content: T,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Lexis {
    Import(Import),
    Constant(Identified<Constant>),
    Blend(Identified<Blend>),
    Function((bool, Identified<Callable>)),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Import(pub Identifier);

#[derive(Clone, PartialEq, Debug)]
pub struct Constant(pub Value);

#[derive(Clone, PartialEq, Debug)]
pub struct Blend(pub Types);

#[derive(Clone, PartialEq, Debug)]
pub struct Defined {
    pub id: Identifier,
    pub kind: Types,
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        String::from_utf8(self.0).unwrap()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<String>::into(self.clone()))
    }
}

impl Display for Lexis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lexis::Import(import) => write!(f, "{}", import),
            Lexis::Constant(constant) => write!(f, "{}", constant),
            Lexis::Blend(blend) => write!(f, "{}", blend),
            Lexis::Function((is_entry, function)) => write!(
                f,
                "{}{}",
                {
                    if *is_entry {
                        format!("entry ")
                    } else {
                        String::new()
                    }
                },
                function
            ),
        }
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "import {};", self.0)
    }
}

impl Display for Identified<Constant> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "const {} = {};", self.id, self.content.0)
    }
}

impl Display for Identified<Blend> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "type {} = {};", self.id, self.content.0)
    }
}

impl Display for Identified<Callable> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fn {}({}) {{\n{}\n}};",
            self.id,
            self.content
                .args
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join(", "),
            self.content.container
        )
    }
}

impl Display for Defined {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.kind)
    }
}
