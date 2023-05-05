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

/// Trait for parsable tokens
pub trait Lexeme<'i> {
    fn parse(s: &mut Fragment) -> Option<Token<'i>>;
}

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
    line_comment: bool,
    block_comment: bool,
}

/// Decisition of the defragment tracker
#[derive(Clone, Debug)]
enum DefragResult {
    Digest,
    Push,
    LineComment,
    BlockComment,
    Ch,
    Str,
}

impl<'i> TokenStream<'i> {
    pub fn new(input: &'i str) -> Self {
        TokenStream::defragment(input)
    }

    fn defragment(i: &'i str) -> Self {
        let (mut pool, token_buffer, mut fragment, mut ctrl_flags): (
            _,
            SegQueue<Token<'i>>,
            Option<Fragment<'i>>,
            DefragTracker,
        ) = (
            Pool::new(std::thread::available_parallelism().unwrap().get() as u32),
            SegQueue::new(),
            None,
            DefragTracker::default(),
        );
        pool.scoped(|scope| {
            let mut chars: std::iter::Enumerate<std::str::Chars> = i.chars().enumerate();
            while chars.clone().count() != 0 || fragment.is_some() {
                let (ch_offset, ch) = {
                    if let Some(chars) = chars.next() {
                        chars
                    } else {
                        if let Some(value) = fragment {
                            let token_buffer = &token_buffer;
                            scope.execute(move || {
                                value.digest().iter().for_each(|s| token_buffer.push(*s));
                            });
                        }
                        break;
                    }
                };
                let _can_defragment = ctrl_flags.can_defragment(ch);
                match _can_defragment {
                    DefragResult::Push => {
                        if let Some(value) = fragment.as_mut() {
                            value.ptr = i
                                .get(value.offset..value.offset + value.ptr.len() + 1)
                                .unwrap();
                        } else {
                            fragment.replace(Fragment::<'i>::new(
                                ch_offset,
                                i.get(ch_offset..ch_offset + 1).unwrap(),
                            ));
                        }
                    }
                    DefragResult::Digest => {
                        if let Some(value) = fragment.take() {
                            let token_buffer = &token_buffer;
                            scope.execute(move || {
                                value.digest().iter().for_each(|s| token_buffer.push(*s));
                            });
                        }
                    }
                    _ => {
                        if let Some(mut value) = fragment.take() {
                            value.ptr = i
                                .get(value.offset.clone()..value.offset + value.ptr.len() + 1)
                                .unwrap();
                            token_buffer.push(Token::new(
                                {
                                    match _can_defragment {
                                        DefragResult::LineComment => {
                                            TokenKind::Comment(Comment::new(
                                                CommentKind::Line,
                                                value.ptr.get(2..).unwrap(),
                                            ))
                                        }
                                        DefragResult::BlockComment => {
                                            TokenKind::Comment(Comment::new(
                                                CommentKind::Block,
                                                value.ptr.get(2..value.ptr.len() - 2).unwrap(),
                                            ))
                                        }
                                        DefragResult::Ch => TokenKind::Literal(Literal::Char(
                                            value.ptr.get(1..2).unwrap(),
                                        )),
                                        DefragResult::Str => TokenKind::Literal(Literal::Str(
                                            value.ptr.get(1..value.ptr.len() - 1).unwrap(),
                                        )),
                                        _ => self::panic!(),
                                    }
                                },
                                Span::new(value.offset, value.ptr.len()),
                            ));
                        } else {
                            self::panic!(
                                "Expected a possible token but there is no fragment in the buffer!"
                            )
                        }
                    }
                }
            }
        });
        Self(
            token_buffer
                .into_iter()
                .sorted_unstable_by(|lhs, rhs| match (lhs.span.offset(), rhs.span.offset()) {
                    (lhs, rhs) if lhs > rhs => std::cmp::Ordering::Greater,
                    (lhs, rhs) if lhs == rhs => std::cmp::Ordering::Equal,
                    (lhs, rhs) if lhs < rhs => std::cmp::Ordering::Less,
                    (_, _) => self::panic!(),
                })
                .collect::<SVec<Token<'i>>>(),
            None,
        )
    }
}

impl DefragTracker {
    pub fn can_defragment(&mut self, ch: char) -> DefragResult {
        self.set_char(ch);
        match &self.inner {
            ['/', '/'] => self.line_comment = true,
            ['/', '*'] => self.block_comment = true,
            ['*', '/'] => {
                self.block_comment = false;
                return DefragResult::BlockComment;
            }
            [_, '\''] => {
                if !self.line_comment {
                    if self.lit_char {
                        self.lit_char = false;
                        return DefragResult::Ch;
                    } else if !self.lit_string {
                        self.lit_char = true;
                    }
                }
            }
            [_, '\"'] => {
                if !self.line_comment {
                    if self.lit_string {
                        self.lit_string = false;
                        return DefragResult::Str;
                    } else if !self.lit_char {
                        self.lit_string = true;
                    }
                }
            }
            _ if ch == '\n' => {
                if !self.lit_char && !self.lit_string && !self.block_comment {
                    if self.line_comment {
                        self.line_comment = false;
                        return DefragResult::LineComment;
                    } else {
                        return DefragResult::Digest;
                    }
                }
            }
            _ if ch.is_whitespace() => {
                if !self.line_comment && !self.block_comment && !self.lit_char && !self.lit_string {
                    return DefragResult::Digest;
                }
            }
            _ => (),
        }
        DefragResult::Push
    }

    pub fn set_char(&mut self, ch: char) {
        self.inner.rotate_left(1);
        self.inner[1] = ch;
    }
}

impl<'i> Fragment<'i> {
    /// Check if can be consumed from the fragment
    pub fn can_consume(&self) -> bool {
        !self.ptr.is_empty()
    }

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

    /// Consumes any character once
    pub fn consume_any_once(&mut self) -> (char, Span) {
        let (offset, any_char) = (self.offset, self.ptr.chars().next().unwrap());
        self.take(1);
        (any_char, Span::new(offset, 1))
    }

    /// Converts fragment to token stream
    pub fn digest(&self) -> SVec<Token<'i>> {
        let mut fragment = self.clone();
        let mut buffer: SVec<Token> = SVec::new();
        while fragment.can_consume() {
            if let Some(kw) = Keyword::parse(&mut fragment) {
                buffer.push(kw);
                continue;
            } else if let Some(punc) = Punctuation::parse(&mut fragment) {
                buffer.push(punc);
                continue;
            } else if let Some(lit) = Literal::parse(&mut fragment) {
                buffer.push(lit);
                continue;
            } else if let Some((id, span)) = fragment.consume_id() {
                buffer.push(Token::new(TokenKind::Identifier(id), span));
            } else {
                let (any_char, span) = fragment.consume_any_once();
                buffer.push(Token::new(TokenKind::Unknown(any_char), span));
                continue;
            }
        }
        buffer
    }
}

impl<'i> From<&'i str> for TokenStream<'i> {
    fn from(value: &'i str) -> Self {
        TokenStream::defragment(value)
    }
}
