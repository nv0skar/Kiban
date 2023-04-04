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

use crate::{Input, Parsable};

use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::is_newline,
    combinator::map,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Clone, PartialEq, Display, Debug)]
#[display(fmt = "\"{}\" ({})", content, typed)]
pub struct Comment {
    pub typed: CommentType,
    pub content: String,
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

impl<'a> Parsable<Input<'a>, Self> for Comment {
    fn parse(s: Input) -> IResult<Input, Self> {
        alt((single_lined, multi_lined))(s)
    }
}

fn single_lined(s: Input) -> IResult<Input, Comment> {
    map(
        preceded(tag("//"), take_till(|c| is_newline(c as u8))),
        |s: Input| Comment {
            typed: CommentType::Single,
            content: s.to_string(),
        },
    )(s)
}

fn multi_lined(s: Input) -> IResult<Input, Comment> {
    map(
        delimited(tag("/*"), take_until("*/"), tag("*/")),
        |s: Input| Comment {
            typed: CommentType::Multi,
            content: s.to_string(),
        },
    )(s)
}
