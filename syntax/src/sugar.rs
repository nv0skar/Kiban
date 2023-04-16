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
    #[doc = "Define whether a name is public or private"]
    case Visibility {
        Private,
        Public
    }
}

node! {
    #[doc = "Define whether values in scope should be moved to closure"]
    MoveScope(bool)
}

node! {
    #[doc = "Define whether a value should be deferred when assigning"]
    DerefValue(bool)
}

node! {
    #[doc = "Define whether a definition is mutable"]
    Mutable(bool)
}
