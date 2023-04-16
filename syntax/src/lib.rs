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

#[macro_use]
pub mod node;

pub mod atom;
pub mod body;
pub mod r#const;
pub mod expr;
pub mod r#impl;
pub mod item;
pub mod literal;
pub mod stmt;
pub mod sugar;
pub mod r#type;

pub use atom::*;
pub use body::*;
pub use expr::*;
pub use item::*;
pub use literal::*;
pub use r#const::*;
pub use r#impl::*;
pub use r#type::*;
pub use stmt::*;
pub use sugar::*;

use kiban_commons::*;
use kiban_lexer::*;

use compact_str::CompactString;

#[derive(Clone, PartialEq, Debug)]
pub struct Syntax(SVec<Item>);
