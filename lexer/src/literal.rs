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

use crate::{mapped, Input, Parsable};

use std::mem::discriminant;

use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{take, take_while},
    character::complete::char,
    combinator::{map, map_res},
    number::complete::float as float_parse,
    sequence::delimited,
    IResult,
};
use smol_str::SmolStr;

/// Tokens that store a literal
#[derive(Clone, Display, Debug)]
pub enum Literal {
    Bool(bool),
    /// can be parsed into a literal or an identifier
    #[display(fmt = "{:?} (integer / field)", _0)]
    Int(isize),
    #[display(fmt = "{:?} (float)", _0)]
    Float(f32),
    #[display(fmt = "{:?} (char)", _0)]
    Char(char),
    #[display(fmt = "{:?} (string)", _0)]
    String(SmolStr),
}

impl<'a> Parsable<Input<'a>, Self> for Literal {
    fn parse(s: Input) -> IResult<Input, Self> {
        alt((boolean, integer, float, character, string))(s)
    }
}

fn boolean(s: Input) -> IResult<Input, Literal> {
    map(alt((mapped!("true", true), mapped!("false", false))), |s| {
        Literal::Bool(s)
    })(s)
}

fn integer(s: Input) -> IResult<Input, Literal> {
    map_res(float_parse, |s| {
        if s.round() != s {
            Err(())
        } else {
            Ok(Literal::Int(s.round() as isize))
        }
    })(s)
}

fn float(s: Input) -> IResult<Input, Literal> {
    map(float_parse, |s| Literal::Float(s))(s)
}

fn character(s: Input) -> IResult<Input, Literal> {
    map(
        delimited(char('\''), take(1_usize), char('\'')),
        |s: Input| Literal::Char(s.chars().next().unwrap()),
    )(s)
}

fn string(s: Input) -> IResult<Input, Literal> {
    map(
        delimited(
            char('"'),
            take_while(|_c: char| -> bool { true }),
            char('"'),
        ),
        |s: Input| Literal::String(s.into()),
    )(s)
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
