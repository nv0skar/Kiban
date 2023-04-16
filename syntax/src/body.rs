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
    Block(SVec<Stmt>)
}

node! {
    #[doc = "Define all the posible closures a function may have"]
    Variants(SVec<Closure>)
}

node! {
    #[doc = "Define a closure which has a signature and an (optional in traits) block"]
    Closure {
        sig: Signature,
        block: Option<Block>,
    }
}

node! {
    #[doc = "Define a closure signature which has generics, parameters and a return type"]
    Signature {
        generics: Generics,
        params: Parameters,
        expect: Type
    }
}

node! {
    #[doc = "Define a named function"]
    FuncDef {
        visible: Visibility,
        name: Ident,
        variants: Variants
    }
}

node! {
    #[doc = "Define parameters which is a list of types"]
    Parameters(SVec<Type>)
}

node! {
    #[doc = "Define arguments which is a list of expressions"]
    Args(SVec<Expr>)
}
