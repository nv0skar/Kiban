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
    #[doc = "Define items"]
    case Item<'i> {
        Module(ModuleDef<'i>),
        Import(ImportDef<'i>),
        Const(ConstDef<'i>),
        Type(TypeDef<'i>),
        Impl(ImplDef<'i>),
        Trait(TraitDef<'i>),
        Func(FuncDef<'i>),
    }
}

node! {
    #[doc = "Define module"]
    ModuleDef<'i> {
        vis: Visibility,
        names: Ident<'i>
    }
}

node! {
    #[doc = "Define import"]
    ImportDef<'i> {
        vis: Visibility,
        names: SVec<ImportName<'i>>
    }
}

node! {
    #[doc = "Define subimports"]
    ImportName<'i> {
        path: Path<'i>,
        alias: Option<Ident<'i>>
    }
}
