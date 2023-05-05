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
    case Type<'i> {
        Null,
        Infer,
        Paren(Type<'i>),
        Ref(Option<Lifetime<'i>>, MutTy<'i>),
        Ptr(MutTy<'i>),
        Path(Path<'i>),
        Boolean,
        Integer(Number),
        Float(Number),
        Char,
        Array(Type<'i>, Const<'i>),
        Slice(Type<'i>),
        Tup(TupTy<'i>),
        Struct(StructTy<'i>),
        Enum(EnumTy<'i>),
        LocalSelf,
        FnSig(Signature<'i>),
    }
}

node! {
    #[doc = "Define a type"]
    TypeDef<'i> {
        vis: Visibility,
        name: Ident<'i>,
        ty: Type<'i>,
    }
}

node! {
    #[doc = "Define a list of types"]
    TupTy<'i>(SVec<Type<'i>>)
}

node! {
    #[doc = "Define structs"]
    case StructTy<'i> {
        Tup(TupTy<'i>),
        Field(SVec<FieldTy<'i>>)
    }
}

node! {
    #[doc = "Define the field of structs"]
    FieldTy<'i> {
        name: Ident<'i>,
        ty: Type<'i>
    }
}

node! {
    #[doc = "Defines enums"]
    EnumTy<'i>(SVec<VariantTy<'i>>)
}

node! {
    #[doc = "Define enum variants"]
    VariantTy<'i> {
        name: Ident<'i>,
        inner: Option<StructTy<'i>>
    }
}

node! {
    #[doc = "Defines if the type is mutable"]
    MutTy<'i> {
        mutable: Mutable,
        ty: Type<'i>
    }
}
