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
pub mod keyword;
pub mod literal;
pub mod punctuation;

pub use comment::*;
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

use compact_str::{CompactString, ToCompactString};
use derive_more::{Constructor, Display};
use nom::{
    Compare, CompareResult, FindSubstring, InputIter, InputLength, InputTake, Needed, Offset, Slice,
};
use nom_recursive::{HasRecursiveInfo, HasRecursiveType, RecursiveInfo};
use rayon::prelude::*;
use smallvec::SmallVec;

pub trait Lexeme {
    fn parse(s: &mut Input) -> Option<(Token, Span)>;
}

/// input type for the lexer that keeps track of the string's offset relative to the source
#[derive(Clone, Constructor, Default, Debug)]
pub struct Input(usize, CompactString);

/// token stream with recursive info
#[derive(Clone, Constructor, Default, Debug)]
pub struct TokenStream(_TokenStream, Option<RecursiveInfo<_TokenStream>>);

/// token stream
type _TokenStream = SVec<(Token, Span)>;

/// token variants
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

impl Input {
    pub fn digest(self) -> SVec<Self> {
        let input = self.1.escape_default().collect::<SVec<_>>();
        let input_iter = input.iter().enumerate();
        let (mut buffer, mut ctrl_flags): (SVec<Option<Input>>, [bool; 3]) =
            (SVec::from_elem(None, 1), [bool::default(); 3]);
        for (ch_offset, ch) in input_iter {
            if ch.is_whitespace() && !ctrl_flags[0] && !ctrl_flags[1] && !ctrl_flags[2] {
                buffer.push(None)
            } else {
                if ch_offset != 0 {
                    if *ch == '"' && input[ch_offset - 1] != '\\' {
                        ctrl_flags[0] ^= true;
                    }
                    if *ch == '/' && input[ch_offset - 1] == '/' {
                        ctrl_flags[1] = true;
                    }
                    if *ch == 'n' && input[ch_offset - 1] == '\\' {
                        ctrl_flags[1] = false;
                    }
                    if *ch == '*' && input[ch_offset - 1] == '/' {
                        ctrl_flags[2] = true;
                    }
                    if *ch == '/' && input[ch_offset - 1] == '*' {
                        ctrl_flags[2] = false;
                    }
                }
                let last_elem = buffer.last_mut().unwrap();
                if let Some(elem) = last_elem {
                    elem.1.push(*ch)
                } else {
                    *last_elem = Some(Self(ch_offset, ch.to_compact_string()));
                }
            }
        }
        buffer.iter().filter_map(|s| s.clone()).collect()
    }

    pub fn tokenize(&mut self) -> _TokenStream {
        let mut buffer: SVec<(Token, Span)> = SVec::new();
        while self.can_consume() {
            if let Some(kw) = Keyword::parse(self) {
                buffer.push(kw);
                continue;
            } else if let Some(commnt) = Comment::parse(self) {
                buffer.push(commnt);
                continue;
            } else if let Some(punc) = Punctuation::parse(self) {
                buffer.push(punc);
                continue;
            } else if let Some(lit) = Literal::parse(self) {
                buffer.push(lit);
                continue;
            } else if let Some((id, span)) = self.consume_ident() {
                buffer.push((Token::Identifier(id), span));
            } else {
                let (any_char, span) = self.consume_any_char();
                buffer.push((Token::Unknown(any_char), span));
                continue;
            }
        }
        buffer
    }

    fn take(&mut self, n: usize) {
        assert!(n != 0, "Cannot take 0 characters from input!");
        self.0 += n;
        self.1 = self.1.get(n..).unwrap().into();
    }

    fn can_consume(&self) -> bool {
        !self.1.is_empty()
    }

    pub fn consume_specific(&mut self, txt: &str) -> Option<Span> {
        if let (offset, Some(to_cmp)) = (self.0, self.1.get(..txt.len())) {
            if txt == to_cmp {
                self.take(txt.len());
                return Some(Span::new(offset, offset + txt.len() - 1));
            }
        }
        None
    }

    pub fn consume_from(&mut self, txt: &str) -> Option<(CompactString, Span)> {
        if let Some(to_cmp) = self.1.get(..txt.len()) {
            if txt == to_cmp {
                let rest = (self.1.clone(), Span::new(self.0, self.0 + self.1.len() - 1));
                self.take(self.1.len());
                return Some(rest);
            }
        }
        None
    }

    pub fn consume_delimited(&mut self, from: &str, until: &str) -> Option<(CompactString, Span)> {
        if let Some(to_cmp) = self.1.get(..from.len()) {
            if from == to_cmp {
                let input = self.1.chars().collect::<SVec<_>>();
                let input_iter = input.iter().enumerate();
                let mut buffer: SVec<char> = SVec::new();
                for (ch_offset, ch) in input_iter {
                    buffer.push(*ch);
                    if ch_offset >= until.len() {
                        if until
                            == input
                                .get(ch_offset - until.len() + 1..ch_offset + 1)
                                .unwrap()
                                .iter()
                                .collect::<CompactString>()
                        {
                            let delimited = (
                                buffer.iter().collect::<CompactString>(),
                                Span::new(self.0, self.0 + buffer.len() - 1),
                            );
                            self.take(buffer.len());
                            return Some(delimited);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn consume_num(&mut self) -> Option<((bool, CompactString), Span)> {
        let (mut is_decimal, mut buffer) = (bool::default(), CompactString::default());
        for ch in self.1.escape_default() {
            if ch.is_numeric() {
                buffer.push(ch);
            } else if ch == '.' {
                if is_decimal {
                    return None;
                }
                is_decimal = true;
                buffer.push(ch)
            } else {
                break;
            }
        }
        if !buffer.is_empty() {
            let number_span = Span::new(self.0, self.0 + buffer.len() - 1);
            self.take(buffer.len());
            Some(((is_decimal, buffer), number_span))
        } else {
            None
        }
    }

    pub fn consume_ident(&mut self) -> Option<(CompactString, Span)> {
        let first_char = self.1.escape_default().next().unwrap();
        if !first_char.is_digit(10) && !first_char.is_whitespace() {
            let mut buffer: SVec<char> = SVec::new();
            for ch in self.1.escape_default() {
                if ch.is_alphanumeric() || ch == '_' {
                    buffer.push(ch)
                } else {
                    break;
                }
            }
            let span_res = Span::new(self.0, self.0 + buffer.len() - 1);
            self.take(buffer.len());
            Some((buffer.iter().collect(), span_res))
        } else {
            None
        }
    }

    pub fn consume_any_char(&mut self) -> (char, Span) {
        let (offset, any_char) = (self.0, self.1.chars().next().unwrap());
        self.take(1);
        (any_char, Span::new(offset, offset + 1))
    }
}

impl TokenStream {
    pub fn parse(input: &str) -> Self {
        Self(
            Input::from(input)
                .digest()
                .par_iter()
                .map(|s| s.clone().tokenize().to_vec())
                .flatten()
                .collect::<Vec<(Token, Span)>>()
                .into(),
            None,
        )
    }
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
        if let (Some(first), Some(last)) = (self.0.first(), self.0.last()) {
            Span::new(*first.1.start(), *last.1.end())
        } else {
            Span::default()
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

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self(0, value.into())
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

impl From<Token> for TokenStream {
    fn from(value: Token) -> Self {
        Self(SmallVec::from(vec![(value, Span::default())]), None)
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
