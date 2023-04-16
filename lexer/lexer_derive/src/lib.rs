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

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Expr};

#[proc_macro_derive(TokenParser, attributes(token))]
pub fn derive_token_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let mut refs = quote! {};
    let mut repr = quote! {};
    let mut alts = quote! {};
    if let Data::Enum(DataEnum { variants, .. }) = data {
        for variant in variants {
            let mut attrs = variant.attrs.iter();
            let (field, token) = (
                variant.ident.to_token_stream(),
                loop {
                    let (path, value) = match &attrs.next().unwrap().meta {
                        syn::Meta::List(list) => (&list.path, list.parse_args::<Expr>().unwrap()),
                        syn::Meta::NameValue(named) => (&named.path, named.value.clone()),
                        _ => panic!("Unexpected attribute format!"),
                    };
                    if path.is_ident("token") {
                        break match value {
                            syn::Expr::Lit(literal) => literal.lit.to_token_stream(),
                            _ => panic!("Token is not a literal string!"),
                        };
                    }
                },
            );
            refs.extend(
                TokenStream::from(quote! {
                    paste::paste! {
                        pub const [<#field:snake:upper>]: crate::TokenKind = crate::TokenKind::#ident(#ident::#field);
                    }
                })
                .into_iter(),
            );
            repr.extend(
                TokenStream::from(quote! {
                    Self::#field => #token,
                })
                .into_iter(),
            );
            alts.extend(
                TokenStream::from(quote! {
                    if let Some(span) = s.consume_pattern(#token) {
                        return Some(crate::Token::new(crate::TokenKind::#ident(Self::#field), span));
                    }
                })
                .into_iter(),
            )
        }
    } else {
        panic!("Token parser can only be derived on enums!")
    }
    let output = quote! {
        #refs
        impl crate::Lexeme for #ident {
            fn parse(s: &mut crate::Fragment) -> Option<crate::Token> {
                #alts
                None
            }
        }
        impl crate::TokenOrigin for #ident {
            fn origin(&self) -> Option<compact_str::CompactString> {
                Some(
                    compact_str::CompactString::from(
                        match self {
                            #repr
                        }
                    )
                )
            }
        }
    };
    output.into()
}
