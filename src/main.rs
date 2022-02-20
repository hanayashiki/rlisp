use std::env;
use std::fs;

use ariadne::{sources, Label, Report, ReportKind};
use rlisp::evaluate::Evaluatable;
use rlisp::isolate;
use rlisp::parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let show_ast = args.iter().find(| s | s.as_str() == "--show-ast").is_some();

    let contents = fs::read_to_string(args.get(1).expect("need a path to the source file. ").clone()).expect("Something went wrong reading the file");

    let mut parser = parser::Parser::new(contents.as_str());

    parser.init().unwrap();

    let result = parser.parse();

    match result {
        Ok(ast) => {
            if show_ast {
                println!("{:#?}", ast);
            }

            let mut isolate = isolate::Isolate::new();

            ast.evaluate(&mut isolate).unwrap();
        }
        Err(e) => match e {
            parser::ParserError::SyntaticError { location, message } => {
                let offset = location.offset.try_into().unwrap();

                Report::build(ReportKind::Error, "stdin", offset)
                    .with_message("SyntaticError")
                    .with_label(
                        Label::new(("stdin", offset..offset + 1)).with_message(message.as_str()),
                    )
                    .finish()
                    .print(sources(vec![("stdin", parser.code)]))
                    .unwrap();
            }
            parser::ParserError::LexicalError(lexical_error) => {
                let offset = lexical_error.offset as usize;

                Report::build(ReportKind::Error, "stdin", offset)
                    .with_message("LexicalError")
                    .with_label(
                        Label::new(("stdin", offset..offset + 1))
                            .with_message(lexical_error.message.as_str()),
                    )
                    .finish()
                    .print(sources(vec![("stdin", parser.code)]))
                    .unwrap();
            }
        },
    }
}
