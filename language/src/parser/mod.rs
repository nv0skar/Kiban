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

pub mod grammar;

use grammar::parse;

use crate::intermediate::{Identifier, Module};

use peg::{error::ParseError, str::LineCol};

impl Module {
    pub fn parse(_id: Option<Identifier>, input: &String) -> Result<Self, ParseError<LineCol>> {
        parse::module(input)
    }
}
