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

use std::ops::Range;

use derive_more::{Constructor, Display};
use getset::Getters;
use miette::SourceSpan;

#[derive(Clone, PartialEq, Constructor, Getters, Display, Debug)]
#[display(fmt = "{}..+{}", offset, length)]
#[get = "pub"]

pub struct Span {
    offset: usize,
    length: usize,
}

pub trait Spanned {
    fn span(&self) -> Span;
}

impl Span {
    pub fn location(&self) -> Range<usize> {
        self.offset..(self.offset + self.length)
    }

    pub fn from_combination(start: Self, end: Self) -> Self {
        Self::new(start.offset, (end.offset + end.length) - start.offset)
    }
}

impl Into<SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        SourceSpan::new(self.offset.into(), self.length.into())
    }
}
