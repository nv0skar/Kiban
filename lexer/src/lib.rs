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

pub mod comment;
pub mod input;
pub mod keyword;
pub mod literal;
pub mod punctuation;

pub use comment::*;
pub use input::*;
pub use keyword::*;
pub use literal::*;
pub use punctuation::*;

use kiban_commons::*;

use std::{
    fmt::Display,
    iter::Enumerate,
    mem::discriminant,
    ops::{Range, RangeFrom, RangeFull, RangeTo},
};

use compact_str::CompactString;
use derive_more::{Constructor, Display};
use nom::{
    Compare, CompareResult, FindSubstring, InputIter, InputLength, InputTake, Needed, Offset, Slice,
};
use nom_recursive::{HasRecursiveInfo, HasRecursiveType, RecursiveInfo};
use smallvec::SmallVec;

/// Token stream with recursive info
#[derive(Clone, Constructor, Default, Debug)]
pub struct TokenStream(_TokenStream, Option<RecursiveInfo<_TokenStream>>);

/// Token stream
type _TokenStream = SVec<(Token, Span)>;

/// Token variants
#[derive(Clone, PartialEq, Display, Debug)]
#[display(fmt = "{}")]
pub enum Token {
    #[display(fmt = "{} (id)", _0)]
    Identifier(CompactString),
    #[display(fmt = "{} (kw)", _0)]
    Keyword(Keyword),
    #[display(fmt = "{} (punct)", _0)]
    Punctuation(Punctuation),
    #[display(fmt = "{} (lit)", _0)]
    Literal(Literal),
    #[display(fmt = "{} (comment)", _0)]
    Comment(Comment),
    #[display(fmt = "{} (unknown)", _0)]
    Unknown(char),
}

impl HasRecursiveInfo<_TokenStream> for TokenStream {
    fn get_recursive_info(&self) -> RecursiveInfo<_TokenStream> {
        match &self.1 {
            Some(recursive) => recursive.clone(),
            None => RecursiveInfo::new(),
        }
    }

    fn set_recursive_info(mut self, info: RecursiveInfo<_TokenStream>) -> Self {
        self.1 = Some(info);
        self
    }
}

impl HasRecursiveType<_TokenStream> for TokenStream {
    fn get_value(&self) -> _TokenStream {
        self.clone().0
    }
}

impl Spanned for TokenStream {
    fn span(&self) -> Span {
        if let (Some(start), Some(end)) = (self.0.first(), self.0.last()) {
            Span::new(
                *start.1.offset(),
                (end.1.offset() + end.1.length()) - start.1.offset(),
            )
        } else {
            panic!("There is token stream to calculate span!")
        }
    }
}

impl PartialEq for TokenStream {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(info_self), Some(info_other)) = (self.1.clone(), other.1.clone()) {
            info_self == info_other && self.0 == other.0
        } else {
            self.0 == other.0
        }
    }
}

impl Iterator for TokenStream {
    type Item = (Token, Span);

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.0.first().cloned();
        if value.is_some() {
            self.0.remove(0);
        };
        value
    }
}

impl InputLength for TokenStream {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.len()
    }
}

impl Slice<Range<usize>> for TokenStream {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Self(self.0.as_slice().slice(range).into(), self.1.clone())
    }
}

impl Slice<RangeTo<usize>> for TokenStream {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl Slice<RangeFrom<usize>> for TokenStream {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.0.len())
    }
}

impl Slice<RangeFull> for TokenStream {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        self.clone()
    }
}

impl InputTake for TokenStream {
    fn take(&self, count: usize) -> Self {
        self.slice(0..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl Offset for TokenStream {
    fn offset(&self, second: &Self) -> usize {
        if let Some(target) = second.0.first() {
            let mut offset = 0_usize;
            for actual in &self.0 {
                if actual == target {
                    return offset;
                }
                offset += 1;
            }
            panic!("Cannot calculate offset!");
        } else {
            0
        }
    }
}

impl Compare<Token> for TokenStream {
    fn compare(&self, t: Token) -> CompareResult {
        if let Some((token, _)) = self.0.first() {
            match ((discriminant(token) == discriminant(&t)) && {
                match token {
                    Token::Identifier(_) => true,
                    _ => false,
                }
            }) || token == &t
            {
                true => CompareResult::Ok,
                false => CompareResult::Error,
            }
        } else {
            CompareResult::Incomplete
        }
    }

    fn compare_no_case(&self, _t: Token) -> CompareResult {
        panic!(
            "Case insensitive comparisons aren't available as tokens aren't a stringified structure!"
        )
    }
}

impl FindSubstring<Token> for TokenStream {
    fn find_substring(&self, substr: Token) -> Option<usize> {
        for (index, (token, _)) in self.iter_indices() {
            if token == substr {
                return Some(index);
            }
        }
        None
    }
}

impl InputIter for TokenStream {
    type Item = (Token, Span);
    type Iter = Enumerate<TokenStream>;
    type IterElem = TokenStream;

    #[inline]
    fn iter_indices(&self) -> Enumerate<Self> {
        self.clone().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> Self {
        self.clone()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.clone().into_iter().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.0.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl Into<TokenStream> for (Token, Span) {
    fn into(self) -> TokenStream {
        TokenStream(SmallVec::from(vec![self]), None)
    }
}

impl From<TokenStream> for Token {
    fn from(value: TokenStream) -> Self {
        if value.0.len() == 1 {
            value.0.first().unwrap().0.clone()
        } else {
            panic!("Token streams with no or more than one token cannot be converted into tokens!")
        }
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|s| format!("{} #{}", s.0, s.1))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
