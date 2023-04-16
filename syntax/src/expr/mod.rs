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
use range::Range;
use unary::Unary;

use kiban_commons::*;

node! {
    #[doc = "Define expressions"]
    case Expr {
        Path(Path),
        Underscore,
        Parenthesized(Expr),
        Refer(Expr),
        Unary(Unary, Expr),
        Binary {
            op: Binary,
            lhs: Expr,
            rhs: Expr,
        },
        Literal(Literal),
        Struct(Path, StructTy),
        Block(Block),
        Array(SVec<Expr>),
        Tup(TupExpr),
        Closure(MoveScope ,Closure),
        Range(Range),
        Assign(DerefValue, Ident, Option<Binary>, Expr),
        Field(Expr, Ident),
        Call(Expr, Args),
        Method {
            def: Expr,
            method: Path,
            args: Args,
        },
        Index(Expr, Expr),
        Cast(Expr, Type),
        Cond {
            check: Expr,
            then: Expr,
            not: Option<Expr>,
        },
        Loop(Expr),
        ForLoop {
            item: Ident,
            iter: Expr,
            block: Expr,
        },
        While {
            check: Expr,
            block: Expr,
        },
        Continue,
        Break,
    }
}

node! {
    #[doc = "Define a list of expressions"]
    TupExpr(SVec<Expr>)
}

node! {
    #[doc = "Define struct constructor"]
    case StructExpr {
        Tup(TupExpr),
        Field(SVec<FieldExpr>)
    }
}

node! {
    #[doc = "Define constructor of fields"]
    FieldExpr {
        path: Path,
        expr: Expr
    }
}

node! {
    #[doc = "Defines if the expression is mutable"]
    MutExpr {
        mutable: Mutable,
        expr: Expr
    }
}
