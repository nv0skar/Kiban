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

use crate::{Input, Lexeme, Token};

use kiban_commons::*;

use compact_str::CompactString;
use derive_more::Display;

#[derive(Clone, PartialEq, Display, Debug)]
#[display(fmt = "\"{}\" ({})", content, typed)]
pub struct Comment {
    pub typed: CommentType,
    pub content: CompactString,
}

#[derive(Clone, PartialEq, Display, Debug)]
pub enum CommentType {
    /// Single lined comments are those that has '//' at the beginning
    #[display(fmt = "single-lined")]
    Single,

    /// Multi lined comments are those that are delimited at the beginning by '/*' and by '*/' at the end
    #[display(fmt = "multi-lined")]
    Multi,
}

impl Lexeme for Comment {
    fn parse(s: &mut Input) -> Option<(Token, Span)> {
        if let Some((content, span)) = s.consume_from("//") {
            Some((
                Token::Comment(Self {
                    typed: CommentType::Single,
                    content: content.get(2..).unwrap().into(),
                }),
                span,
            ))
        } else if let Some((content, span)) = s.consume_delimited("/*", "*/") {
            Some((
                Token::Comment(Self {
                    typed: CommentType::Multi,
                    content: content.get(2..content.len() - 2).unwrap().into(),
                }),
                span,
            ))
        } else {
            None
        }
    }
}
