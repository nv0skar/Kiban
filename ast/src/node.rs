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

use crate::Input;

use kiban_commons::{parser::Parsable, span::Span};

use derive_more::Constructor;
use nom::{combinator::map, IResult};

#[macro_export]
macro_rules! node_def {
    ($name:ident {$($fields:tt)*}) => {
        paste::paste! {
            pub type $name = crate::node::Node<[<_ $name>]>;
            #[derive(Clone, PartialEq, Debug)]
            pub struct [<_ $name>] {
                $($fields)*
            }
        }
    };
    ($name:ident ($($fields:tt)*)) => {
        paste::paste! {
            pub type $name = crate::node::Node<[<_ $name>]>;
            #[derive(Clone, PartialEq, Debug)]
            pub struct [<_ $name>](
                $($fields)*
            );
        }
    };
}

#[macro_export]
macro_rules! node_variant {
    ($name:ident {$($variants:tt)*}) => {
        paste::paste! {
            pub type $name = crate::node::Node<[<_ $name>]>;
            #[derive(Clone, PartialEq, Debug)]
            pub enum [<_ $name>] {
                $($variants)*
            }
        }
    };
}

#[derive(Clone, PartialEq, Constructor, Debug)]
pub struct Node<T> {
    pub inner: T,
    pub location: Span,
}

impl<T: Parsable<Input, (T, Span)>> Parsable<Input, Self> for Node<T> {
    fn parse(s: Input) -> IResult<Input, Self> {
        map(T::parse, |(s, span)| Node::new(s, span))(s)
    }
}
