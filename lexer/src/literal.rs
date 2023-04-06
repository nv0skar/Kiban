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
    character::complete::{char, digit1},
    combinator::map,
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
    #[display(fmt = "{:?} (integer / ident)", _0)]
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
        alt((_boolean, _integer, _float, _character, _string))(s)
    }
}

fn _boolean(s: Input) -> IResult<Input, Literal> {
    map(alt((mapped!("true", true), mapped!("false", false))), |s| {
        Literal::Bool(s)
    })(s)
}

fn _integer(s: Input) -> IResult<Input, Literal> {
    map(digit1, |s: Input| Literal::Int(s.parse().unwrap()))(s)
}

fn _float(s: Input) -> IResult<Input, Literal> {
    map(float_parse, |s| Literal::Float(s))(s)
}

fn _character(s: Input) -> IResult<Input, Literal> {
    map(
        delimited(char('\''), take(1_usize), char('\'')),
        |s: Input| Literal::Char(s.chars().next().unwrap()),
    )(s)
}

fn _string(s: Input) -> IResult<Input, Literal> {
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
