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

use crate::{generic::Identifier, separated, Input};

use kiban_commons::*;
use kiban_lexer::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{pair, separated_pair},
    IResult,
};

node_variant! { Range {
    Bounded(Identifier, Identifier),
    From(Identifier),
    To(Identifier),
    Inclusive(Identifier, Identifier),
    ToInclusive(Identifier),
    Full,
}}

impl Parsable<Input, (Self, Span)> for _Range {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((_to, _to_inclusive, _full, _inclusive, _bounded, _from))(s)
    }
}

fn _bounded(s: Input) -> IResult<Input, (_Range, Span)> {
    map(
        separated_pair(
            Identifier::parse,
            separated!(both tag(RANGE)),
            Identifier::parse,
        ),
        |(from, to)| {
            (
                _Range::Bounded(from.clone(), to.clone()),
                Span::from_combination(from.location, to.location),
            )
        },
    )(s)
}

fn _from(s: Input) -> IResult<Input, (_Range, Span)> {
    map(
        pair(Identifier::parse, separated!(left tag(RANGE))),
        |(from, last)| {
            (
                _Range::From(from.clone()),
                Span::from_combination(from.location, last.span()),
            )
        },
    )(s)
}

fn _to(s: Input) -> IResult<Input, (_Range, Span)> {
    map(
        pair(separated!(right tag(RANGE)), Identifier::parse),
        |(start, to)| {
            (
                _Range::To(to.clone()),
                Span::from_combination(start.span(), to.location),
            )
        },
    )(s)
}

fn _inclusive(s: Input) -> IResult<Input, (_Range, Span)> {
    map(
        separated_pair(
            Identifier::parse,
            separated!(both tag(RANGE_INCLUSIVE)),
            Identifier::parse,
        ),
        |(from, to)| {
            (
                _Range::Inclusive(from.clone(), to.clone()),
                Span::from_combination(from.location, to.location),
            )
        },
    )(s)
}

fn _to_inclusive(s: Input) -> IResult<Input, (_Range, Span)> {
    map(
        pair(separated!(right tag(RANGE_INCLUSIVE)), Identifier::parse),
        |(start, to)| {
            (
                _Range::ToInclusive(to.clone()),
                Span::from_combination(start.span(), to.location),
            )
        },
    )(s)
}

fn _full(s: Input) -> IResult<Input, (_Range, Span)> {
    map(tag(RANGE), |s: Input| {
        (
            _Range::Full,
            Span::from_combination(s.clone().span(), s.span()),
        )
    })(s)
}
