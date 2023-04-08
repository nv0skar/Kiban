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

#[macro_use]
pub mod node;

pub mod body;
pub mod closure;
pub mod expression;
pub mod generic;
pub mod item;
pub mod literal;
pub mod statement;
pub mod types;

pub use node::Parsable;

use item::Item;

use kiban_commons::SVec;
use kiban_lexer::TokenStream;

#[macro_export]
macro_rules! map_token {
    ($token:path, $type_of:path) => {
        nom::combinator::map(nom::bytes::complete::tag($token), |s: Input| {
            ($type_of, s.span())
        })
    };
    (($first_token:path, $second_token:path), $type_of:path) => {
        nom::combinator::map(
            nom::sequence::pair(
                nom::bytes::complete::tag($first_token),
                nom::bytes::complete::tag($second_token),
            ),
            |(start, end): (Input, Input)| {
                (
                    $type_of,
                    kiban_commons::Span::from_combination(start.span(), end.span()),
                )
            },
        )
    };
}

#[macro_export]
macro_rules! map_token_with_field {
    ($token:path, $variant:path, $type_of:path) => {
        nom::combinator::map(
            nom::bytes::complete::tag($token($variant(Default::default()))),
            |s: Input| {
                (
                    $type_of({
                        if let $token($variant(val)) = s.clone().into() {
                            val
                        } else {
                            panic!("Unexpected token!")
                        }
                    }),
                    s.span(),
                )
            },
        )
    };
}

type Input = TokenStream;

#[derive(Clone, PartialEq, Debug)]
pub struct Tree(SVec<Item>);
