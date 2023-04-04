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

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error(transparent)]
pub enum Linting {
    #[error("module")]
    Module(#[diagnostic_source] Module),
    #[error("body")]
    Body(#[diagnostic_source] Body),
    #[error("statement")]
    Statement(#[diagnostic_source] Statement),
    #[error("expression")]
    Expression(#[diagnostic_source] Expression),
}

#[derive(Error, Diagnostic, Debug)]
pub enum Module {
    #[error("Module is importing itself!")]
    #[diagnostic(code(kiban_error::checking::Module::CircularImport))]
    CircularImport,
}

#[derive(Error, Diagnostic, Debug)]
pub enum Body {
    #[error("Multiple returns specified!")]
    #[diagnostic(code(kiban_error::checking::Body::MultipleReturns))]
    MultipleReturns,
    #[error("Invalid return type for function!")]
    #[diagnostic(code(kiban_error::checking::Body::InvalidReturnType))]
    InvalidReturnType,
}

#[derive(Error, Diagnostic, Debug)]
pub enum Statement {
    #[error("Neither a type or value wasn't specified!")]
    #[diagnostic(code(kiban_error::checking::Statement::UndefinedDeclaration))]
    UndefinedDeclaration,
    #[error("Declaration type and value doesn't match!")]
    #[diagnostic(code(kiban_error::checking::Statement::InvalidDeclaration))]
    InvalidDeclaration,
}

#[derive(Error, Diagnostic, Debug)]
pub enum Expression {
    #[error("Invalid type for condition!")]
    #[diagnostic(code(kiban_error::checking::Expression::InvalidConditionType))]
    InvalidConditionType,
    #[error("Invalid type for loop!")]
    #[diagnostic(code(kiban_error::checking::Expression::InvalidLoopType))]
    InvalidLoopType,
}
