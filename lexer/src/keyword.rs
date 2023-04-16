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

use crate::*;

#[derive(Copy, Clone, PartialEq, TokenParser, Display, Debug)]
pub enum Keyword {
    /// Set declaration as public
    #[token = "pub"]
    Pub,

    /// Define a module
    #[token = "mod"]
    Mod,

    /// Import other modules
    #[token = "use"]
    Use,

    /// Constant declaration
    #[token = "const"]
    Const,

    /// Type declaration
    #[token = "type"]
    Type,

    /// Implement methods for a type
    #[token = "impl"]
    Impl,

    /// Create a trait
    #[token = "trait"]
    Trait,

    /// Refer to the type
    #[token = "self"]
    SelfTy,

    /// Refer to self parameter
    #[token = "self"]
    SelfParam,

    /// Function declaration
    #[token = "fn"]
    Fn,

    /// Move values into scope
    #[token = "move"]
    Move,

    /// Value declaration
    #[token = "let"]
    Let,

    /// Flag declaration as mutable
    #[token = "mut"]
    Mut,

    /// Match expression
    #[token = "match"]
    Match,

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

    /// Condition not met
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
