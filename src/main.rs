use ariadne::{Label, Report, ReportKind, sources};
use rlisp::parser;

fn main() {
    let mut parser = parser::Parser::new("(define a 12)");

    parser.init().unwrap();

    let result = parser.parse();

    match result {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => match e {
            parser::ParserError::SyntaticError { location, message } => {
                let offset = location.offset.try_into().unwrap();

                Report::build(ReportKind::Error, "stdin", offset)
                    .with_message("SyntaticError")
                    .with_label(
                        Label::new(("stdin", offset..offset + 1))
                            .with_message(message.as_str())
                    )
                    .finish()
                    .print(sources(vec![
                        ("stdin", parser.code),
                    ]))
                    .unwrap();
            }
            parser::ParserError::LexicalError(lexical_error) => {
                let offset = lexical_error.offset as usize;

                Report::build(ReportKind::Error, "stdin", offset)
                    .with_message("LexicalError")
                    .with_label(
                        Label::new(("stdin", offset..offset + 1))
                            .with_message(lexical_error.message.as_str())
                    )
                    .finish()
                    .print(sources(vec![
                        ("stdin", parser.code),
                    ]))
                    .unwrap();
            }
        },
    }
}
