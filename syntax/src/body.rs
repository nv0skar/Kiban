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

node! {
    #[doc = "Define a sequence of statements"]
    Block<'i>(SVec<Stmt<'i>>)
}

node! {
    #[doc = "Define all the posible closures a function may have"]
    Variants<'i>(SVec<Closure<'i>>)
}

node! {
    #[doc = "Define a closure which has a signature and an (optional in traits) block"]
    Closure<'i> {
        sig: Signature<'i>,
        block: Option<Block<'i>>,
    }
}

node! {
    #[doc = "Define a closure signature which has generics, parameters and a return type"]
    Signature<'i> {
        generics: Generics<'i>,
        params: Parameters<'i>,
        expect: Type<'i>
    }
}

node! {
    #[doc = "Define a named function"]
    FuncDef<'i> {
        visible: Visibility,
        name: Ident<'i>,
        variants: Variants<'i>
    }
}

node! {
    #[doc = "Define parameters which is a list of types"]
    Parameters<'i>(SVec<Type<'i>>)
}

node! {
    #[doc = "Define arguments which is a list of expressions"]
    Args<'i>(SVec<Expr<'i>>)
}
