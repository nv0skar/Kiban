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

use derive_more::{Constructor, Display};

#[derive(Clone, PartialEq, Display, Constructor, Debug)]
#[display(fmt = "signed: {} & size: {}", signed, size)]
pub struct NumberDef {
    signed: bool,
    size: Size,
}

#[derive(Clone, PartialEq, Display, Debug)]
pub enum Size {
    _8,
    _16,
    _32,
    _64,
}

impl Default for NumberDef {
    fn default() -> Self {
        Self::new(false, Size::_8)
    }
}
