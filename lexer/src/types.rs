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

use kiban_commons::types::{NumberDef, Size};

use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    sequence::{pair, preceded},
    IResult,
};

/// Tokens that refer to a type
#[derive(Clone, Display, Debug)]
pub enum Type {
    Bool,
    #[display(fmt = "Integer ({})", _0)]
    Int(NumberDef),
    #[display(fmt = "Float ({})", _0)]
    Float(NumberDef),
    Fn,
}

impl<'a> Parsable<Input<'a>, Self> for Type {
    fn parse(s: Input) -> IResult<Input, Self> {
        alt((
            mapped!("Bool", Self::Bool),
            integer,
            float,
            map(tag("Fn"), |_| Type::Fn),
        ))(s)
    }
}

fn integer(s: Input) -> IResult<Input, Type> {
    map(
        pair(signed, preceded(tag("Int"), size)),
        |(signed, size)| Type::Int(NumberDef::new(signed, size)),
    )(s)
}

fn float(s: Input) -> IResult<Input, Type> {
    map(
        pair(signed, preceded(tag("Float"), size)),
        |(signed, size)| Type::Float(NumberDef::new(signed, size)),
    )(s)
}

fn signed(s: Input) -> IResult<Input, bool> {
    map(opt(char('U')), |s| s.map_or(true, |_| false))(s)
}

fn size(s: Input) -> IResult<Input, Size> {
    alt((
        mapped!("8", Size::_8),
        mapped!("16", Size::_16),
        mapped!("32", Size::_32),
        mapped!("64", Size::_64),
    ))(s)
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
