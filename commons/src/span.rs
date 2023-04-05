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
use getset::Getters;
use miette::SourceSpan;

#[derive(Clone, PartialEq, Constructor, Getters, Display, Debug)]
#[display(fmt = "{}..{}", start, end)]
#[get = "pub"]
pub struct Span {
    start: usize,
    end: usize,
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Span {
    pub fn from_offset(offset: usize, length: usize) -> Self {
        Self {
            start: offset,
            end: offset + length,
        }
    }

    pub fn from_combination(start: Span, end: Span) -> Span {
        Span::new(*start.start(), *end.end())
    }
}

impl Into<SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        SourceSpan::new(self.start.into(), (self.end - self.start).into())
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
        }
    }
}
