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

use kiban_lexer_derive::TokenParser;

use derive_more::Display;

#[derive(Clone, PartialEq, TokenParser, Display, Debug)]
pub enum Keyword {
    /// Public declaration
    #[token = "pub"]
    Pub,

    /// Imports declaration
    #[token = "use"]
    Use,

    /// Constant declaration
    #[token = "const"]
    Const,

    /// Type declaration
    #[token = "type"]
    Type,

    /// Function declaration
    #[token = "fn"]
    Fn,

    /// Value declaration
    #[token = "let"]
    Let,

    /// Type casting
    #[token = "as"]
    As,

    /// Loops
    #[token = "loop"]
    Loop,

    /// While
    #[token = "while"]
    While,

    /// Conditional
    #[token = "if"]
    If,

    /// Condition not meeted
    #[token = "else"]
    Else,

    /// Element name of a single inner value of iterable
    #[token = "for"]
    For,

    /// Iterable
    #[token = "in"]
    In,

    /// Skip to next element in iterable
    #[token = "continue"]
    Continue,

    /// Interrupt iterable
    #[token = "break"]
    Break,

    /// Bool type
    #[token = "bool"]
    Bool,

    /// Fn type
    #[token = "Fn"]
    FnTy,

    /// Return from a function
    #[token = "return"]
    Return,
}
