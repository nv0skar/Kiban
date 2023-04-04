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
pub mod expression;
pub mod generic;
pub mod item;
pub mod literal;
pub mod statement;
pub mod types;

use item::Item;

use kiban_commons::SVec;
use kiban_lexer::TokenStream;

#[macro_export]
macro_rules! separator {
    () => {
        map(
            many0(alt((
                tag(Token::Punctuation(Punctuation::Space)),
                tag(Token::Punctuation(Punctuation::Newline)),
            ))),
            |_| (),
        )
    };
}

#[macro_export]
macro_rules! separated {
    (left $parser:expr) => {
        preceded(separator!(), $parser)
    };
    (right $parser:expr) => {
        terminated($parser, separator!())
    };
    (both $parser:expr) => {
        delimited(separator!(), $parser, separator!())
    };
}

#[macro_export]
macro_rules! map_token_with_field {
    ($token:path, $variant:path, $type_of:path) => {
        map(tag($token($variant(Default::default()))), |s: Input| {
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
        })
    };
}

type Input = TokenStream;

#[derive(Clone, PartialEq, Debug)]
pub struct Tree(SVec<Item>);
