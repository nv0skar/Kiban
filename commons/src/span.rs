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

use std::ops::Range;

use chumsky::span::Span as ParserSpan;
use getset::{Getters, MutGetters};
use miette::SourceSpan;

#[derive(Copy, Clone, PartialEq, Constructor, Getters, MutGetters, Display, Default, Debug)]
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

impl ParserSpan for Span {
    type Context = ();

    type Offset = usize;

    fn new(_: Self::Context, range: Range<Self::Offset>) -> Self {
        Self::new(range.start, range.end - range.start)
    }

    fn context(&self) -> Self::Context {
        ()
    }

    fn start(&self) -> Self::Offset {
        self.offset
    }

    fn end(&self) -> Self::Offset {
        self.offset + self.length
    }
}

impl From<Range<usize>> for Span {
    fn from(rng: Range<usize>) -> Self {
        Self::new(rng.start, rng.end - rng.start)
    }
}

impl Into<Range<usize>> for Span {
    fn into(self) -> Range<usize> {
        self.offset..self.offset + self.length
    }
}

impl Into<SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        SourceSpan::new(self.offset.into(), self.length.into())
    }
}
