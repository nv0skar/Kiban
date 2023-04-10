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

use kiban_commons::*;

use compact_str::{CompactString, ToCompactString};
use derive_more::Constructor;

/// input type for the lexer that keeps track of the string's offset relative to the source
#[derive(Clone, Constructor, Default, Debug)]
pub struct Input(usize, CompactString);

impl Input {
    /// convert input to tokens
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
            } else if let Some((id, span)) = self.consume_id() {
                buffer.push((Token::Identifier(id), span));
            } else {
                let (any_char, span) = self.consume_any();
                buffer.push((Token::Unknown(any_char), span));
                continue;
            }
        }
        buffer
    }

    /// check if there is any input left to consume
    pub fn can_consume(&self) -> bool {
        !self.1.is_empty()
    }

    /// consume input
    fn take(&mut self, n: usize) {
        assert!(n != 0, "Cannot take 0 characters from input!");
        self.0 += n;
        self.1 = self.1.get(n..).unwrap().into();
    }

    /// consume a specific character sequence
    pub fn consume_pattern(&mut self, txt: &str) -> Option<Span> {
        if let (offset, Some(to_cmp)) = (self.0, self.1.get(..txt.len())) {
            if txt == to_cmp {
                self.take(txt.len());
                return Some(Span::new(offset, offset + txt.len() - 1));
            }
        }
        None
    }

    /// consume all characters from a set of characters
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

    /// consume all characters delimited by a set of characters
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

    /// try to consume a number
    pub fn consume_number(&mut self) -> Option<((bool, CompactString), Span)> {
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

    /// try to consume an identifier
    pub fn consume_id(&mut self) -> Option<(CompactString, Span)> {
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

    /// consume any character
    pub fn consume_any(&mut self) -> (char, Span) {
        let (offset, any_char) = (self.0, self.1.chars().next().unwrap());
        self.take(1);
        (any_char, Span::new(offset, offset + 1))
    }

    /// divides the input into an array of inputs and it's offsets
    /// the division happens at each whitespace, with the exception of literals and single line comments, where whitespaces are ignored
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
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        Self(0, value.into())
    }
}
