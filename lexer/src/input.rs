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

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Trait for parsable tokens
pub trait Lexeme {
    fn parse(s: &mut Fragment) -> Option<(Token, Span)>;
}

/// Input type for the lexer that keeps track of the string's offset relative to the source
#[derive(Clone, Default, Debug)]
pub struct Input(SVec<Fragment>);

/// Input type for the lexer that keeps track of the string's offset relative to the source
#[derive(Clone, Constructor, Default, Debug)]
pub struct Fragment {
    offset: usize,
    inner: CompactString,
}

/// Describes the state of the input fragmentation process by keeping track of the kind of character it encounters
#[derive(Clone, Default, Debug)]
struct DefragTracker {
    inner: [char; 2],
    lit_char: bool,
    lit_string: bool,
    single_comment: bool,
    delimited_comment: bool,
}

/// Decisition of the defragment tracker
#[derive(Clone, Debug)]
enum DefragResult {
    Defrag,
    PushAndDefrag,
    Continue,
}

impl Input {
    pub fn new(input: &str) -> Self {
        Input::defragment(input)
    }

    pub fn tokenize(&self) -> TokenStream {
        TokenStream(
            {
                #[cfg(feature = "parallel")]
                {
                    self.0
                        .par_iter()
                        .map(|s| s.clone().digest().to_vec())
                        .flatten()
                        .collect::<Vec<(Token, Span)>>()
                        .into()
                }
                #[cfg(not(feature = "parallel"))]
                {
                    self.0
                        .iter()
                        .map(|s| s.clone().digest())
                        .flatten()
                        .collect::<SVec<(Token, Span)>>()
                }
            },
            Option::default(),
        )
    }

    fn defragment(input: &str) -> Self {
        let (mut buffer, mut ctrl_flags): (SVec<Option<Fragment>>, DefragTracker) =
            (SVec::from_elem(None, 1), DefragTracker::default());
        for (ch_offset, ch) in input.chars().enumerate() {
            let last_fragment = buffer.last_mut().unwrap();
            match ctrl_flags.can_defragment(ch) {
                DefragResult::Defrag => buffer.push(None),
                DefragResult::PushAndDefrag => {
                    if let Some(elem) = last_fragment {
                        elem.inner.push(ch)
                    } else {
                        *last_fragment = Some(Fragment::new(ch_offset, ch.to_compact_string()))
                    }
                    buffer.push(None)
                }
                DefragResult::Continue => {
                    if let Some(elem) = last_fragment {
                        elem.inner.push(ch)
                    } else {
                        *last_fragment = Some(Fragment::new(ch_offset, ch.to_compact_string()))
                    }
                }
            }
        }
        Self(buffer.iter().filter_map(|s| s.clone()).collect())
    }
}

impl DefragTracker {
    pub fn can_defragment(&mut self, ch: char) -> DefragResult {
        self.set_char(ch);
        match &self.inner {
            ['/', '/'] => self.single_comment = true,
            ['/', '*'] => self.delimited_comment = true,
            ['*', '/'] => {
                self.delimited_comment = false;
                return DefragResult::PushAndDefrag;
            }
            [_, '\''] => {
                if self.lit_char {
                    self.lit_char = false;
                    return DefragResult::PushAndDefrag;
                } else {
                    self.lit_char = true;
                }
            }
            [_, '\"'] => {
                if self.lit_string {
                    self.lit_string = false;
                    return DefragResult::PushAndDefrag;
                } else {
                    self.lit_string = true;
                }
            }
            _ if ch == '\n' => {
                self.single_comment = false;
                if !self.lit_char && !self.lit_string && !self.delimited_comment {
                    return DefragResult::Defrag;
                }
            }
            _ if ch.is_whitespace() => {
                if !self.single_comment
                    && !self.delimited_comment
                    && !self.lit_char
                    && !self.lit_string
                {
                    return DefragResult::Defrag;
                }
            }
            _ => (),
        }
        DefragResult::Continue
    }

    pub fn set_char(&mut self, ch: char) {
        self.inner.rotate_left(1);
        self.inner[1] = ch;
    }
}

impl Fragment {
    /// Check if there is any input left to consume
    pub fn can_consume(&self) -> bool {
        !self.inner.is_empty()
    }

    /// Consumes input
    fn take(&mut self, n: usize) {
        assert!(n != 0, "Cannot take 0 characters from input!");
        self.offset += n;
        self.inner = self.inner.get(n..).unwrap().into();
    }

    /// Consumes a specific character sequence
    pub fn consume_pattern(&mut self, txt: &str) -> Option<Span> {
        if let (offset, Some(to_cmp)) = (self.offset, self.inner.get(..txt.len())) {
            if txt == to_cmp {
                self.take(txt.len());
                return Some(Span::new(offset, txt.len()));
            }
        }
        None
    }

    /// Consumes all characters from a set of characters
    pub fn consume_from(&mut self, txt: &str) -> Option<(CompactString, Span)> {
        if let Some(to_cmp) = self.inner.get(..txt.len()) {
            if txt == to_cmp {
                let rest = (self.inner.clone(), Span::new(self.offset, self.inner.len()));
                self.take(self.inner.len());
                return Some(rest);
            }
        }
        None
    }

    /// Try to consume a number
    pub fn consume_number(&mut self) -> Option<((bool, CompactString), Span)> {
        let (mut is_decimal, mut length) = (bool::default(), 0_usize);
        for ch in self.inner.chars() {
            if ch.is_numeric() {
                length += 1;
            } else if ch == '.' {
                if is_decimal {
                    return None;
                }
                is_decimal = true;
                length += 1;
            } else {
                break;
            }
        }
        if length != 0 {
            let number_span = Span::new(self.offset, length);
            let res = Some((
                (
                    is_decimal,
                    self.inner.get(..length).unwrap().to_compact_string(),
                ),
                number_span,
            ));
            self.take(length);
            res
        } else {
            None
        }
    }

    /// Try to consume an identifier
    pub fn consume_id(&mut self) -> Option<(CompactString, Span)> {
        let first_char = self.inner.chars().next().unwrap();
        if !first_char.is_digit(10) && !first_char.is_whitespace() {
            let mut buffer: SVec<char> = SVec::new();
            for ch in self.inner.chars() {
                if ch.is_alphanumeric() || ch == '_' {
                    buffer.push(ch)
                } else {
                    break;
                }
            }
            if !buffer.is_empty() {
                let span_res = Span::new(self.offset, buffer.len());
                self.take(buffer.len());
                return Some((buffer.iter().collect(), span_res));
            }
        }
        None
    }

    /// Consumes any character
    pub fn consume_any(&mut self) -> (char, Span) {
        let (offset, any_char) = (self.offset, self.inner.chars().next().unwrap());
        self.take(1);
        (any_char, Span::new(offset, 1))
    }

    /// Converts fragment to token stream
    pub fn digest(&mut self) -> SVec<LocalisedToken> {
        let mut buffer: SVec<LocalisedToken> = SVec::new();
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
            } else if let Some((id, span)) = self.consume_id() {
                buffer.push((
                    Token::Identifier(ArrayString::from(id.as_str()).unwrap()),
                    span,
                ));
            } else {
                let (any_char, span) = self.consume_any();
                buffer.push((Token::Unknown(any_char), span));
                continue;
            }
        }
        buffer
    }
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Input::defragment(value)
    }
}
