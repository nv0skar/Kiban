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

use crate::{map_token, Input};

use kiban_commons::*;
use kiban_lexer::*;

use nom::{branch::alt, IResult};

node_variant! { Unary { Negative, Negation }}

impl Parsable<Input, (Self, Span)> for _Unary {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map_token!(MINUS, Self::Negative),
            map_token!(NEGATION, Self::Negation),
        ))(s)
    }
}
