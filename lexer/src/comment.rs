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

#[derive(Copy, Clone, PartialEq, Display, Debug)]
#[display(fmt = "\"{}\" ({})", content, typed)]
pub struct Comment {
    pub typed: CommentType,
    pub content: ArrayString<1024>,
}

#[derive(Copy, Clone, PartialEq, Display, Debug)]
pub enum CommentType {
    /// Lined comments are those that has '//' at the beginning, they finish with a line break
    #[display(fmt = "line")]
    Line,

    /// Block comments are those that are delimited at the beginning by '/*' and by '*/' at the end, so they can be multi-lined
    #[display(fmt = "block")]
    Block,
}

impl Lexeme for Comment {
    fn parse(s: &mut Fragment) -> Option<(Token, Span)> {
        if let Some((content, span)) = s.consume_from("//") {
            Some((
                Token::Comment(Self {
                    typed: CommentType::Line,
                    content: ArrayString::from(content.get(2..).unwrap()).unwrap(),
                }),
                span,
            ))
        } else if let Some((content, span)) = s.consume_from("/*") {
            Some((
                Token::Comment(Self {
                    typed: CommentType::Block,
                    content: ArrayString::from(content.get(2..content.len() - 2).unwrap()).unwrap(),
                }),
                span,
            ))
        } else {
            None
        }
    }
}
