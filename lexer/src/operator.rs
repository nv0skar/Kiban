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
pub enum Operator {
    /// more or equal than
    #[token = ">="]
    MoreEqThan,

    /// less or equal than
    #[token = "<="]
    LessEqThan,

    /// equality
    #[token = "=="]
    Eq,

    /// assignments
    #[token = "="]
    Assign,

    /// inequality
    #[token = "!="]
    NEq,

    /// less than
    #[token = "< "]
    LessThan,

    /// more than
    #[token = " >"]
    MoreThan,

    /// or
    #[token = "||"]
    Or,

    /// xor
    #[token = "^"]
    XOr,

    /// and
    #[token = "&&"]
    And,

    /// addition
    #[token = "+"]
    Plus,

    /// indicates return type
    #[token = "->"]
    RtrnTy,

    /// substraction
    #[token = "-"]
    Minus,

    /// exponentiation
    #[token = "**"]
    Pow,

    /// multiplication or dereference
    #[token = "*"]
    MulDeref,

    /// division
    #[token = "/"]
    Div,

    /// modulo
    #[token = "%"]
    Mod,

    /// negation
    #[token = "!"]
    Negation,

    /// reference
    #[token = "&"]
    Ref,
}
