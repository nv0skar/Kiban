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
pub trait Lexeme<'i> {
    fn parse(s: &mut Fragment) -> Option<Token<'i>>;
}

/// Input type for the lexer that keeps track of the string's offset relative to the source
#[derive(Clone, Default, Debug)]
pub struct Input<'i>(SVec<Fragment<'i>>);

/// Input type for the lexer that keeps track of the string's offset relative to the source
#[derive(Copy, Clone, Constructor, Default, Debug)]
pub struct Fragment<'i> {
    offset: usize,
    ptr: &'i str,
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

impl<'i> Input<'i> {
    pub fn new(input: &'i str) -> Self {
        Input::defragment(input)
    }

    pub fn tokenize(&mut self) -> TokenStream {
        TokenStream(
            {
                #[cfg(feature = "parallel")]
                {
                    self.0
                        .par_iter_mut()
                        .map(|s| s.digest().to_vec())
                        .flatten()
                        .collect::<Vec<Token>>()
                        .into()
                }
                #[cfg(not(feature = "parallel"))]
                {
                    self.0
                        .iter_mut()
                        .map(|s| s.digest())
                        .flatten()
                        .collect::<SVec<Token<'i>>>()
                }
            },
            Option::default(),
        )
    }

    fn defragment(i: &'i str) -> Self {
        let (mut buffer, mut ctrl_flags): (SVec<Option<Fragment>>, DefragTracker) =
            (SVec::from_elem(None, 1), DefragTracker::default());
        for (ch_offset, ch) in i.chars().enumerate() {
            let last_fragment = buffer.last_mut().unwrap();
            match ctrl_flags.can_defragment(ch) {
                DefragResult::Defrag => buffer.push(None),
                DefragResult::PushAndDefrag => {
                    if let Some(elem) = last_fragment {
                        elem.ptr = i
                            .get(elem.offset.clone()..elem.offset + elem.ptr.len() + 1)
                            .unwrap();
                    } else {
                        *last_fragment = Some(Fragment::<'i>::new(
                            ch_offset,
                            i.get(ch_offset..ch_offset + 1).unwrap(),
                        ))
                    }
                    buffer.push(None)
                }
                DefragResult::Continue => {
                    if let Some(elem) = last_fragment {
                        elem.ptr = i
                            .get(elem.offset.clone()..elem.offset + elem.ptr.len() + 1)
                            .unwrap();
                    } else {
                        *last_fragment = Some(Fragment::<'i>::new(
                            ch_offset,
                            i.get(ch_offset..ch_offset + 1).unwrap(),
                        ))
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
                    if !self.lit_string {
                        self.lit_char = true;
                    }
                }
            }
            [_, '\"'] => {
                if self.lit_string {
                    self.lit_string = false;
                    return DefragResult::PushAndDefrag;
                } else {
                    if !self.lit_char {
                        self.lit_string = true;
                    }
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

impl<'i> Fragment<'i> {
    /// Consumes input
    fn take(&mut self, n: usize) {
        assert!(n != 0, "Cannot take 0 characters from input!");
        self.offset += n;
        self.ptr = self.ptr.get(n..).unwrap().into();
    }

    /// Consumes a specific character sequence
    pub fn consume_pattern(&mut self, txt: &str) -> Option<Span> {
        if let (offset, Some(to_cmp)) = (self.offset, self.ptr.get(..txt.len())) {
            if txt == to_cmp {
                self.take(txt.len());
                return Some(Span::new(offset, txt.len()));
            }
        }
        None
    }

    /// Consumes all characters from a set of characters
    pub fn consume_from(&mut self, txt: &str) -> Option<(&'i str, Span)> {
        if let Some(to_cmp) = self.ptr.get(..txt.len()) {
            if txt == to_cmp {
                let rest = (self.ptr.clone(), Span::new(self.offset, self.ptr.len()));
                self.take(self.ptr.len());
                return Some(rest);
            }
        }
        None
    }

    /// Try to consume a number
    pub fn consume_number(&mut self) -> Option<((bool, &'i str), Span)> {
        let (mut is_decimal, mut length) = (bool::default(), 0_usize);
        for ch in self.ptr.chars() {
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
            let res = Some(((is_decimal, self.ptr.get(..length).unwrap()), number_span));
            self.take(length);
            res
        } else {
            None
        }
    }

    /// Try to consume an identifier
    pub fn consume_id(&mut self) -> Option<(&'i str, Span)> {
        let first_char = self.ptr.chars().next().unwrap();
        if !first_char.is_digit(10) && !first_char.is_whitespace() {
            let mut length = 0_usize;
            for ch in self.ptr.chars() {
                if ch.is_alphanumeric() || ch == '_' {
                    length += 1;
                } else {
                    break;
                }
            }
            if length != 0 {
                let res = (
                    self.ptr.get(..length).unwrap(),
                    Span::new(self.offset, length),
                );
                self.take(length);
                return Some(res);
            }
        }
        None
    }

    /// Consumes any character
    pub fn consume_any(&mut self) -> (char, Span) {
        let (offset, any_char) = (self.offset, self.ptr.chars().next().unwrap());
        self.take(1);
        (any_char, Span::new(offset, 1))
    }

    /// Converts fragment to token stream
    pub fn digest(&mut self) -> SVec<Token> {
        let mut buffer: SVec<Token> = SVec::new();
        while !self.ptr.is_empty() {
            if let Some(kw) = Keyword::parse(self) {
                buffer.push(kw);
                continue;
            } else if let Some((content, span)) = self.consume_from("//") {
                buffer.push(Token::new(
                    TokenKind::Comment(CommentKind::Line, &content[2..]),
                    span,
                ));
                continue;
            } else if let Some((content, span)) = self.consume_from("/*") {
                buffer.push(Token::new(
                    TokenKind::Comment(CommentKind::Block, &content[2..content.len() - 2]),
                    span,
                ));
                continue;
            } else if let Some((content, span)) = self.consume_from("\'") {
                buffer.push(Token::new(TokenKind::CharLit(&content[1..2]), span))
            } else if let Some((content, span)) = self.consume_from("\"") {
                buffer.push(Token::new(
                    TokenKind::StrLit(&content[1..content.len() - 1]),
                    span,
                ))
            } else if let Some(punc) = Punctuation::parse(self) {
                buffer.push(punc);
                continue;
            } else if let Some(lit) = ProcLit::parse(self) {
                buffer.push(lit);
                continue;
            } else if let Some((id, span)) = self.consume_id() {
                buffer.push(Token::new(TokenKind::Identifier(id), span));
            } else {
                let (any_char, span) = self.consume_any();
                buffer.push(Token::new(TokenKind::Unknown(any_char), span));
                continue;
            }
        }
        buffer
    }
}

impl<'i> From<&'i str> for Input<'i> {
    fn from(value: &'i str) -> Self {
        Input::defragment(value)
    }
}
