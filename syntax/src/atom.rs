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

node!(
    #[doc = "Holds an identifier"]
    Ident<'i>(&'i str)
);

node! {
    #[doc = "Define a path which is composed by an identifier, it's generics and an optional subpath"]
    Path<'i> {
        ident: Ident<'i>,
        generics: Generics<'i>,
        subpath: Option<Path<'i>>
    }
}

node! {
    #[doc = "Define generics of a type"]
    Generics<'i>(SVec<GenericTypes<'i>>)
}

node! {
    #[doc = "Define the lifetime of the type"]
    Lifetime<'i>(Ident<'i>)
}

node! {
    #[doc = "Define generics of a type"]
    case GenericTypes<'i> {
        Lifetime(Lifetime<'i>),
        Name(Ident<'i>),
        Type(Type<'i>)
    }
}
