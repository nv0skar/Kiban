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
    Ident(CompactString)
);

node! {
    #[doc = "Define a path which is composed by an identifier, it's generics and an optional subpath"]
    Path {
        ident: Ident,
        generics: Generics,
        subpath: Option<Path>
    }
}

node! {
    #[doc = "Define generics of a type"]
    Generics(SVec<GenericTypes>)
}

node! {
    #[doc = "Define the lifetime of the type"]
    Lifetime(Ident)
}

node! {
    #[doc = "Define generics of a type"]
    case GenericTypes {
        Lifetime(Lifetime),
        Name(Ident),
        Type(Type)
    }
}
