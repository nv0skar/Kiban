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

use crate::*;

use derive_more::Constructor;
use rclite::Arc;

/// Generic node type
#[derive(Clone, PartialEq, Constructor, Debug)]
pub struct Node<T> {
    pub id: u32,
    pub inner: Result<Arc<T>, ()>,
    pub location: Span,
}

#[macro_export]
macro_rules! node {
    ($(#[$meta:meta])* case $name:ident {$($variants:tt)*}) => {
        paste::paste! {
            $(#[$meta])*
            pub type $name = $crate::node::Node<[<_ $name>]>;
            #[derive(Clone, PartialEq, Debug)]
            pub enum [<_ $name>] {
                $($variants)*
            }
        }
    };
    ($(#[$meta:meta])* $name:ident {$($fields:tt)*}) => {
        paste::paste! {
            $(#[$meta])*
            pub type $name = $crate::node::Node<[<_ $name>]>;
            #[derive(Clone, PartialEq, Debug)]
            pub struct [<_ $name>] {
                $($fields)*
            }
        }
    };
    ($(#[$meta:meta])* $name:ident ($($fields:tt)*)) => {
        paste::paste! {
            pub type $name = crate::node::Node<[<_ $name>]>;
            #[derive(Clone, PartialEq, Debug)]
            pub struct [<_ $name>](
                $($fields)*
            );
        }
    };
}
