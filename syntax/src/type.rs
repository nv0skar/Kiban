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
    #[doc = "Define types"]
    case Type {
        Null,
        Infer,
        Paren(Type),
        Ref(Option<Lifetime>, MutTy),
        Ptr(MutTy),
        Path(Path),
        Boolean,
        Integer(Number),
        Float(Number),
        Char,
        Array(Type, Const),
        Slice(Type),
        Tup(TupTy),
        Struct(StructTy),
        Enum(EnumTy),
        LocalSelf,
        FnSig(Signature),
    }
}

node! {
    #[doc = "Define a type"]
    TypeDef {
        vis: Visibility,
        name: Ident,
        ty: Type,
    }
}

node! {
    #[doc = "Define a list of types"]
    TupTy(SVec<Type>)
}

node! {
    #[doc = "Define structs"]
    case StructTy {
        Tup(TupTy),
        Field(SVec<FieldTy>)
    }
}

node! {
    #[doc = "Define the field of structs"]
    FieldTy {
        name: Ident,
        ty: Type
    }
}

node! {
    #[doc = "Defines enums"]
    EnumTy(SVec<VariantTy>)
}

node! {
    #[doc = "Define enum variants"]
    VariantTy {
        name: Ident,
        inner: Option<StructTy>
    }
}

node! {
    #[doc = "Defines if the type is mutable"]
    MutTy {
        mutable: Mutable,
        ty: Type
    }
}
