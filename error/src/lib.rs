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

use kiban_commons::*;

use compact_str::CompactString;
use miette::Diagnostic;

#[derive(Clone, thiserror::Error, Diagnostic, PartialEq, Debug)]
pub enum Error {
    #[error("Unexpected token {}", .found)]
    #[diagnostic(code(kiban::parser))]
    Parser {
        found: CompactString,
        #[help]
        help: Option<CompactString>,
        #[label = "unexpected"]
        span: Option<Span>,
    },
}
