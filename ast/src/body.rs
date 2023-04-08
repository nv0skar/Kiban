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

use crate::{closure::Closure, generic::Definition, types::Types, Input, Parsable};

use kiban_commons::*;

use nom::{combinator::map, multi::fold_many1, IResult};

node_def! { Body {
    pub params: Option<Parameters>,
    pub closure: Closure,
    pub expect: Option<Types>,
}}

node_def!(Parameters(SVec<Definition>));

impl Parsable<Input, (Self, Span)> for _Parameters {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        map(
            fold_many1(Definition::parse, SVec::new, |mut buff, s| {
                buff.push(s);
                buff
            }),
            |s| {
                (
                    Self(s.clone()),
                    Span::from_combination(
                        s.first().unwrap().clone().location,
                        s.last().unwrap().clone().location,
                    ),
                )
            },
        )(s)
    }
}
