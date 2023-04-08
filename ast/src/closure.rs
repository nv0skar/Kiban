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

use crate::{statement::Statement, Input, Parsable};

use kiban_commons::*;
use kiban_lexer::*;

use nom::{
    branch::alt, bytes::complete::tag, combinator::map, multi::fold_many0, sequence::tuple, IResult,
};

node_def!( Closure(pub SVec<Statement>) );

impl Parsable<Input, (Self, Span)> for _Closure {
    fn parse(s: Input) -> IResult<Input, (Self, Span)> {
        alt((
            map(
                tuple((
                    tag(OP_BRACE),
                    fold_many0(Statement::parse, SVec::new, |mut buff, s| {
                        buff.push(s);
                        buff
                    }),
                    tag(CLS_BRACE),
                )),
                |(start, statements, last)| {
                    (
                        Self(statements),
                        Span::from_combination(start.span(), last.span()),
                    )
                },
            ),
            map(Statement::parse, |s| {
                (Self(vec![s.clone()].into()), s.location)
            }),
        ))(s)
    }
}
