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

pub mod binary;
pub mod range;
pub mod unary;

use crate::*;

use binary::*;
use range::*;
use unary::*;

node! {
    #[doc = "Define expressions"]
    case Expr<'i> {
        Path(Path<'i>),
        Underscore,
        Parenthesized(Expr<'i>),
        Refer(Expr<'i>),
        Unary(Unary, Expr<'i>),
        Binary {
            op: Binary,
            lhs: Expr<'i>,
            rhs: Expr<'i>,
        },
        Literal(Literal<'i>),
        Struct(Path<'i>, StructTy<'i>),
        Block(Block<'i>),
        Array(SVec<Expr<'i>>),
        Tup(TupExpr<'i>),
        Closure(MoveScope ,Closure<'i>),
        Range(Range<'i>),
        Assign(DerefValue, Ident<'i>, Option<Binary>, Expr<'i>),
        Field(Expr<'i>, Ident<'i>),
        Call(Expr<'i>, Args<'i>),
        Method {
            def: Expr<'i>,
            method: Path<'i>,
            args: Args<'i>,
        },
        Index(Expr<'i>, Expr<'i>),
        Cast(Expr<'i>, Type<'i>),
        Cond {
            check: Expr<'i>,
            then: Expr<'i>,
            not: Option<Expr<'i>>,
        },
        Loop(Expr<'i>),
        ForLoop {
            item: Ident<'i>,
            iter: Expr<'i>,
            block: Expr<'i>,
        },
        While {
            check: Expr<'i>,
            block: Expr<'i>,
        },
        Continue,
        Break,
    }
}

node! {
    #[doc = "Define a list of expressions"]
    TupExpr<'i>(SVec<Expr<'i>>)
}

node! {
    #[doc = "Define struct constructor"]
    case StructExpr<'i> {
        Tup(TupExpr<'i>),
        Field(SVec<FieldExpr<'i>>)
    }
}

node! {
    #[doc = "Define constructor of fields"]
    FieldExpr<'i> {
        path: Path<'i>,
        expr: Expr<'i>
    }
}

node! {
    #[doc = "Defines if the expression is mutable"]
    MutExpr<'i> {
        mutable: Mutable,
        expr: Expr<'i>
    }
}
