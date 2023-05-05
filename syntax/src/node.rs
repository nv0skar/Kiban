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

/// Generic node type
#[derive(Clone, PartialEq, Debug)]
pub struct Node<T>(Result<(Arc<T>, Span), Error>);

impl<T> Node<T> {
    pub fn new(inner: T, span: Span) -> Node<T> {
        Node(Ok((Arc::new(inner), span)))
    }

    pub fn new_err(inner: Error) -> Node<T> {
        Node(Err(inner))
    }
}

#[macro_export]
macro_rules! node {
    ($(#[$meta:meta])* case $name:ident$(<$param:lifetime>)? {$($variants:tt)*} $($parser:block)?) => {
        paste::paste! {
            $(#[$meta])*
            pub type $name$(< $param >)? = $crate::node::Node<[<_ $name>] $(< $param >)? >;
            #[derive(Clone, PartialEq, Debug)]
            pub enum [<_ $name>] $(< $param >)? {
                $($variants)*
            }
            $(
                pub fn [<_ $name:lower>]<'i>() -> impl chumsky::Parser<'i, kiban_lexer::TokenStream<'i>, $name<'i>, chumsky::extra::Err<Node<$name<'i>>>> {
                    $parser
                }
            )?
        }
    };
    ($(#[$meta:meta])* $name:ident$(<$param:lifetime>)? {$($fields:tt)*}$($parser:block)?) => {
        paste::paste! {
            $(#[$meta])*
            pub type $name$(< $param >)? = $crate::node::Node<[<_ $name>] $(< $param >)? >;
            #[derive(Clone, PartialEq, Debug)]
            pub struct [<_ $name>] $(< $param >)?{
                $($fields)*
            }
            $(
                pub fn [<_ $name:lower>]<'i>() -> impl chumsky::Parser<'i, kiban_lexer::TokenStream<'i>, $name<'i>, chumsky::extra::Err<Node<$name<'i>>>> $parser
            )?
        }
    };
    ($(#[$meta:meta])* $name:ident$(<$param:lifetime>)? ($($fields:tt)*) $($parser:block)?) => {
        paste::paste! {
            pub type $name $(< $param >)? = crate::node::Node<[<_ $name>] $(< $param >)? >;
            #[derive(Clone, PartialEq, Debug)]
            pub struct [<_ $name>] $(< $param >)? (
                $($fields)*
            );
            $(
                pub fn [<_ $name:lower>]<'i>() -> impl chumsky::Parser<'i, kiban_lexer::TokenStream<'i>, $name<'i>, chumsky::extra::Err<Node<$name<'i>>>> $parser
            )?
        }
    };
}

impl<'i, T> chumsky::error::Error<'i, TokenStream<'i>> for Node<T> {
    fn expected_found<
        E: IntoIterator<
            Item = Option<MaybeRef<'i, <TokenStream<'i> as chumsky::prelude::Input<'i>>::Token>>,
        >,
    >(
        expected: E,
        found: Option<MaybeRef<'i, <TokenStream<'i> as chumsky::prelude::Input<'i>>::Token>>,
        span: <TokenStream<'i> as chumsky::prelude::Input<'i>>::Span,
    ) -> Self {
        Self(Err(Error::Parser {
            found: found.unwrap().origin().unwrap(),
            help: {
                let mut ret: Option<CompactString> = None;
                expected.into_iter().for_each(|s| {
                    let token = s.unwrap().origin().unwrap();
                    if let Some(ret_content) = &mut ret {
                        ret_content.push_str(token.as_str())
                    } else {
                        ret = Some(token)
                    }
                });
                ret
            },
            span: Some(span),
        }))
    }
}
