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

use crate::intermediate::{
    container::{Callable, Container, Scope},
    expression::{Expression, Operator},
    lexis::{Blend, Constant, Defined, Identified, Identifier, Import, Lexis},
    primitives::{Types, Value},
    statement::Statement,
    Module,
};

use peg;

peg::parser! {
    pub grammar parse() for str {
        rule _() = quiet!{[' ' | '\t' | '\r' | '\n']*}
        rule __() = quiet!{[','] _?}
        rule ___() = quiet!{[';']}

        rule comment() = "//" _ ([_])*

        rule sign() -> bool = sign:$(quiet!{"-"} / expected!("sign"))? {
            match sign {
                Some("-") => false,
                _ => true
            }
        }
        rule number() -> u32 = quiet!{number:$(['0'..='9']*<1,>) { number.parse::<u32>().unwrap() }} / expected!("number")
        rule string() -> String = quiet!{result:((("\"") s:$(!("\"") [_])* ("\"") { s }) / (("'") s:$(!("'") [_])* ("'") { s })) { result.concat() }} / expected!("string")

        rule statements() -> Vec<Statement> = (statement() ** _)
        rule container() -> Container = "{" _ statements:statements() _ "}" { TryFrom::try_from(statements).unwrap() }

        rule scope() -> Scope = container:container() { Scope(container) }

        rule identifier() -> Identifier = quiet!{id:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) { Identifier(id.as_bytes().to_vec()) }} / expected!("identifier")

        rule defined() -> Defined =  quiet!{ref_id:identifier() ":" _ ref_type:types() { Defined { id: ref_id, kind: ref_type } }} / expected!("defined")

        rule type_reference() -> Types = id:identifier() { Types::Reference(id) }
        rule type_bool() -> Types = "bool" { Types::Boolean }
        rule type_integer() -> Types = "int" { Types::Integer }
        rule type_float() -> Types = "float" { Types::Float }
        rule type_string() -> Types = "string" { Types::String }
        rule type_vector() -> Types = "vec<" _ vector_type:types() _ ">" { Types::Vector(Box::new(vector_type)) }
        rule type_combination() -> Types = "(" _ combination_types:(types() ** __ ) _ ")" { Types::Combination(combination_types) }
        rule type_function() -> Types = "fn(" _ args_id_types:(defined()** __ ) _ ") ->" _ return_type:types() { Types::Fn { args: args_id_types, expect: Box::new(return_type) } }

        rule types() -> Types = precedence!{
            "bool" { Types::Boolean }
            --
            "int" { Types::Integer }
            --
            "float" { Types::Float }
            --
            "string" { Types::String }
            --
            "fn(" _? args_id_types:(defined()** __ ) _? ") ->" _ return_type:types() { Types::Fn { args: args_id_types, expect: Box::new(return_type) } }
            --
            "vec<" _? vector_type:types() _? ">" { Types::Vector(Box::new(vector_type)) }
            --
            "(" _? combination_types:(types() ** __ ) _? ")" { Types::Combination(combination_types) }
            --
            id:identifier() { Types::Reference(id) }
        }

        rule value() -> Value = precedence!{
            string:string() { Value::String(string) }
            --
            sign:sign() integer:number() "." float:number() {
                Value::Float(format!("{}.{}", integer, float).parse::<f32>().unwrap() * {
                    match sign {
                        true => 1.0,
                        false => -1.0
                    }
                })
            }
            --
            sign:sign() number:number() {
                Value::Integer(number as i32 * {
                    match sign {
                        true => 1,
                        false => -1
                    }
                })
            }
            --
            value:$(quiet!{"true" / "false"} / expected!("bool")) {
                Value::Boolean(match value {
                    "true" => true,
                    "false" => false,
                    _ => panic!("Unexpected token while parsing!")
                })
            }
            --
            "|" _? function_args:(defined() ** __ ) _? "| ->" _ return_type:types() _ container:container() {
                Value::Fn(
                    Callable {
                        args: function_args,
                        container: Container {
                            statements: container.statements,
                            expect: return_type
                        }
                    }
                )
            }
            --
            "[" _? vector_expressions:(expression() ** __ ) _? "]" { Value::Vector(vector_expressions) }
            --
            "(" _? combination_expressions:(expression() ** __ ) _? ")" { Value::Combination(combination_expressions) }
        }

        rule expression() -> Expression = precedence!{
            container:scope() { Expression::Scoped(container) }
            --
            lhs:(@) _ "^" _ rhs:@ { Expression::Binary { operator: Operator::Exponentiation, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "%" _ rhs:@ { Expression::Binary { operator: Operator::Mod, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            --
            lhs:(@) _ "*" _ rhs:@ { Expression::Binary { operator: Operator::Multiplication, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "/" _ rhs:@ { Expression::Binary { operator: Operator::Division, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            --
            lhs:(@) _ "+" _ rhs:@ { Expression::Binary { operator: Operator::Addition, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "-" _ rhs:@ { Expression::Binary { operator: Operator::Substraction, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            --
            lhs:(@) _ "==" _ rhs:@ { Expression::Binary { operator: Operator::Equal, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "!=" _ rhs:@ { Expression::Binary { operator: Operator::NotEqual, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "&&" _ rhs:@ { Expression::Binary { operator: Operator::And, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "||" _ rhs:@ { Expression::Binary { operator: Operator::Or, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "<" _ rhs:@ { Expression::Binary { operator: Operator::Less, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ "<=" _ rhs:@ { Expression::Binary { operator: Operator::LessOrEqual, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ ">" _ rhs:@ { Expression::Binary { operator: Operator::More, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            lhs:(@) _ ">=" _ rhs:@ { Expression::Binary { operator: Operator::MoreOrEqual, lhs: Box::new(lhs), rhs: Box::new(rhs) } }
            --
            value:value() { Expression::Unary(value) }
            --
            target:(@) "(" _ args:(expression() ** __ ) _ ")" {
                Expression::CallFn { target: Box::new(target), args }
            }
            --
            target:(@) "." function:identifier() "(" _ args:(expression() ** __ ) _ ")" {
                Expression::TypeFn { target: Box::new(target), function, args }
            }
            --
            vector:(@) "[" _ to:expression() _ "]" {
                Expression::Access { target: Box::new(vector), to: Box::new(to) }
            }
            --
            combination:(@) "." to:expression() {
                Expression::Access { target: Box::new(combination), to: Box::new(to) }
            }
            --
            id:identifier() { Expression::Reference(id) }
            --
            "(" _? expression:expression() _? ")" { expression }
        }

        rule statement_expression() -> Statement = expression:expression() {
            Statement::Expression(expression)
        }
        rule statement_typed_declaration() -> Statement = "let " _ defined:defined() expression:(_ "=" _ expression:expression() { expression })? {
            Statement::Declare { declaration: defined, value: expression }
        }
        rule statement_inferred_declaration() -> Statement = "let " _ id:identifier() _ "=" _ expression:expression() {
            Statement::Declare { declaration: Defined { id: id, kind: expression.infer_type() }, value: Some(expression) }
        }
        rule statement_assign() -> Statement = id:identifier() _ "=" _ expression:expression() {
            Statement::Assign { reference: Expression::Reference(id), value: expression }
        }
        rule statement_condition() -> Statement = "if" _ condition:expression() _ then:statement() _ if_not:("else" _ if_not:statement() { Box::new(if_not) })? {
            Statement::Condition { check: condition, then: Box::new(then), if_not }
        }
        rule statement_loop_while() -> Statement = "loop" _ condition:expression() _ repeat:statement() {
            Statement::Loop { check: condition, repeat: Box::new(repeat) }
        }
        rule statement_loop_element() -> Statement = "for" _ item:identifier() _ "in" _ iterable:expression() _ then:statement() {
            Statement::For { item, iterable, then: Box::new(then) }
        }
        rule statement_continue_loop() -> Statement = "continue" {
            Statement::Continue
        }
        rule statement_break_loop() -> Statement = "break" {
            Statement::Break
        }
        rule statement_return() -> Statement = "return" _ expression:expression()? {
            Statement::Return(expression)
        }

        rule statement() -> Statement = statement:(statement_typed_declaration() / statement_inferred_declaration() / statement_assign() / statement_continue_loop() / statement_break_loop() / statement_return()) ___ { statement } / statement:(statement_condition() / statement_loop_while() / statement_loop_element() / statement_expression()) ___? { statement }

        rule imports() -> Lexis = _ "import" _ id:identifier() _ ___ _ { Lexis::Import(Import(id)) }
        rule constants() -> Lexis = _ "const" _ id:identifier() _ "=" _ value:value() _ ___ _ { Lexis::Constant(Identified { id, content: Constant(value) }) }
        rule blend_types() -> Lexis = _ "type" _ id:identifier() _ "=" _ types:types() _ ___ _ { Lexis::Blend(Identified { id, content: Blend(types) }) }
        rule functions() -> Lexis = _ is_entry:"entry"? _ "fn" _ id:identifier() "(" _ args:(defined() ** ___ ) _ ")" _ return_type:("->" _ return_type:types() { return_type })? _ container:container() _ {
            Lexis::Function((is_entry.is_some(), Identified {id, content: Callable {
                args: args,
                container: Container {
                    statements: container.statements, expect: return_type.unwrap_or(Types::Unknown)
                }
            } }))
        }

        pub rule module() -> Module = global_scope:(lexis:(imports() / constants() / blend_types() / functions()) ** _ { lexis }) {
            Module { id: None, content: global_scope }
        }
    }
}
