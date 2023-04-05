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

#![allow(non_snake_case)]

use kiban_ast::{closure::Closure, expression::Expression, statement::Statement};
use kiban_commons::parser::Parsable;
use kiban_lexer::TokenStream;

use std::{ffi::OsString, fs};

use clap::{Arg, ArgAction, ArgMatches, Command};
use miette::Result;

fn main() -> Result<()> {
    // let lexed = TokenStream::parse("some_value = [banana.juice, potato.slice]", None)?;
    // let lexed = TokenStream::parse("for item in hello::meaning return hello.0", None)?;
    // let lexed = TokenStream::parse("for item in hello::meaning return hello.0", None)?;
    // let lexed = TokenStream::parse("while cow::speaks return madness", None)?;
    // let lexed = TokenStream::parse("if tomorrow_rains return error else return good", None)?;
    // let lexed = TokenStream::parse("some_value.0 as bool", None)?;
    // let lexed = TokenStream::parse("some_array[some_value]", None)?;
    // let lexed = TokenStream::parse("some_func(hello, bye)", None)?;
    // let lexed = TokenStream::parse("let hello: UInt32 = pot;", None)?;
    // let lexed = TokenStream::parse("hello;", None)?;
    let lexed = TokenStream::parse(
        "{
        let hello: Bool = some_constant;
        hello
    }",
        None,
    )?;
    println!("{}\n", lexed);
    let parsed = Closure::parse(lexed.clone());
    println!("{:#?}", parsed);
    Ok(())
}

fn _run(
    implementation: (Option<String>, Option<String>),
    subcommands: Option<Vec<Command>>,
) -> Result<Option<ArgMatches>, String> {
    let command = Command::new(format!("{} - {}", env!("CARGO_PKG_NAME"), {
        match implementation.0 {
            Some(name) => name,
            None => "No implementation".to_string(),
        }
    }))
    .version(format!("{} - {}", env!("CARGO_PKG_VERSION"), {
        match implementation.1 {
            Some(version) => version,
            None => "No implementation's version".to_string(),
        }
    }))
    .subcommand_required(true)
    .args([
        Arg::new("input")
            .help("input to parse")
            .required(true)
            .value_parser(clap::value_parser!(OsString)),
        Arg::new("takeSource")
            .short('s')
            .long("take-source")
            .help("trait input as source code")
            .required(false)
            .action(ArgAction::SetTrue),
        Arg::new("disableCheck")
            .short('d')
            .long("disable-checking")
            .help("disable source checking")
            .required(false)
            .action(ArgAction::SetTrue),
    ])
    .subcommands({
        let mut base_subcommand = vec![Command::new("representation")];
        if let Some(subcommands) = subcommands {
            base_subcommand.append(subcommands.clone().as_mut());
        }
        base_subcommand
    })
    .get_matches();

    let (input, read_as_source) = (
        command
            .get_raw("input")
            .expect("Input is required!")
            .next()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        *command
            .get_one::<bool>("takeSource")
            .unwrap_or_else(|| &false),
    );

    let (_id, _source, _origin) = (
        {
            if !read_as_source {
                Some(input.clone().rsplit('.').next().unwrap().to_string())
            } else {
                None
            }
        },
        {
            if !read_as_source {
                if let Ok(source) = fs::read_to_string(&input) {
                    source
                } else {
                    println!("File {} couldn't be opened!", &input);
                    return Err("Error while trying to open file!".to_string());
                }
            } else {
                input.clone()
            }
        },
        {
            if !read_as_source {
                Some(input.clone())
            } else {
                None
            }
        },
    );

    todo!()

    /* let parsed = {
        match parse(id.clone(), source) {
            Ok(module) => module,
            Err(error) => {
                let reasoning = format!("Expected one of the following set {:#?}", "");
                println!(
                    "{}",
                    DisplayList::from(Snippet {
                        title: Some(Annotation {
                            label: Some("parsing failed"),
                            id: None,
                            annotation_type: AnnotationType::Error,
                        }),
                        footer: vec![],
                        slices: vec![Slice {
                            source: source.as_str(),
                            line_start: 1,
                            origin: {
                                if let Some(origin) = &origin {
                                    Some(str::from_utf8(origin.as_bytes()).unwrap())
                                } else {
                                    None
                                }
                            },
                            fold: true,
                            annotations: vec![SourceAnnotation {
                                label: reasoning.as_str(),
                                annotation_type: AnnotationType::Error,
                                range: (0, 0),
                            }],
                        }],
                        opt: FormatOptions {
                            color: true,
                            ..Default::default()
                        },
                    })
                );
                return Err("Parsing failed!".to_string());
            }
        }
    };

    if !command
        .get_one::<bool>("disableCheck")
        .unwrap_or_else(|| &false)
    {
        let errors = parsed.check();
        if !errors.is_empty() {
            println!(
                "{}",
                DisplayList::from(Snippet {
                    title: Some(Annotation {
                        label: Some("error encountered"),
                        id: None,
                        annotation_type: AnnotationType::Error,
                    }),
                    footer: vec![],
                    slices: errors
                        .iter()
                        .map(|x| Slice {
                            source: x.where_is.as_str(),
                            line_start: 0,
                            origin: None,
                            fold: true,
                            annotations: vec![SourceAnnotation {
                                label: x.explanation.as_str(),
                                annotation_type: AnnotationType::Error,
                                range: (0, x.where_is.len()),
                            }],
                        })
                        .collect::<Vec<Slice>>(),
                    opt: FormatOptions {
                        color: true,
                        ..Default::default()
                    },
                })
            );
            return Err(format!(
                "Encountered error while checking! {}",
                errors
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
    }

    match command.subcommand() {
        Some(("representation", _)) => {
            parsed.content.0.iter().for_each(|x| println!("{}", x));
            Ok(None)
        }
        _ => Ok(Some(command)),
    } */
}
