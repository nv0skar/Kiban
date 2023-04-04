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

use crate::{mapped, Input, Parsable};

use kiban_lexer_derive::TokenParser;

use derive_more::Display;

#[derive(Clone, PartialEq, TokenParser, Display, Debug)]
pub enum Keyword {
    /// imports declaration
    #[token = "use"]
    Use,

    /// constant declaration
    #[token = "const"]
    Const,

    /// type declaration
    #[token = "type"]
    Type,

    /// function declaration
    #[token = "fn"]
    Fn,

    /// value declaration
    #[token = "let"]
    Let,

    /// type casting
    #[token = "as"]
    As,

    /// loops
    #[token = "loop"]
    Loop,

    /// while
    #[token = "while"]
    While,

    /// conditional
    #[token = "if"]
    If,

    /// condition not meeted
    #[token = "else"]
    Else,

    /// element name of a single inner value of iterable
    #[token = "for"]
    For,

    /// iterable
    #[token = "in"]
    In,

    /// skip to next element in iterable
    #[token = "continue"]
    Continue,

    /// interrupt iterable
    #[token = "break"]
    Break,

    /// return from a function
    #[token = "return"]
    Return,
}
