use crate::ast::{DefineExpr, Expr, IdentifierExpr, Location, Program, IntegerLiteral};
use crate::lexer;
use crate::lexer::{LexicalError, Token, TokenTag};
use std::fmt;

#[derive(fmt::Debug)]
pub enum ParserError {
    SyntaticError { location: Location, message: String },
    LexicalError(LexicalError),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParserError")
    }
}

impl From<LexicalError> for ParserError {
    fn from(error: LexicalError) -> Self {
        ParserError::LexicalError(error)
    }
}

pub struct Parser<'a> {
    pub code: &'a str,
    lexer: lexer::Lexer<'a>,
    cur_token: Option<lexer::Token>,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Parser<'a> {
        Parser {
            code,
            lexer: lexer::Lexer::new(code),
            cur_token: None,
        }
    }

    pub fn init(&mut self) -> Result<(), lexer::LexicalError> {
        let e = self.lexer.init();
        match e {
            Ok(token) => {
                self.cur_token = Some(token);
                return Ok(());
            }
            Err(err) => return Err(err),
        };
    }

    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        self.cur_token = Some(self.lexer.next()?);
        Ok(self.cur_token())
    }

    pub fn cur_token(&self) -> Token {
        self.cur_token.as_ref().unwrap().clone()
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut exprs = Vec::<Box<dyn Expr>>::new();

        loop {
            let token = self.cur_token();

            match token {
                Token {
                    tag: TokenTag::EOF, ..
                } => {
                    return Ok(Program {
                        location: exprs.first().map_or(
                            Location {
                                col: token.col,
                                row: token.row,
                                offset: token.offset,
                            },
                            |expr| (*expr.location()).clone(),
                        ),
                        exprs,
                    })
                }
                _ => exprs.push(self.parse_expr()?),
            }
        }
    }

    pub fn parse_expr(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        let first_token = self.cur_token();

        match first_token {
            Token {
                tag: TokenTag::LParen,
                ..
            } => {
                // '('
                return self.parse_call_like();
            }
            Token {
                tag: TokenTag::Identifier(_),
                ..
            } => {
                // 'identifier'
                return Ok(self.parse_identifier()?);
            }
            Token {
                tag: TokenTag::IntegerLiteral(_),
                ..
            } => {
                // 'integer'
                return Ok(self.parse_integer()?);
            }
            _ => {
                return Err(ParserError::SyntaticError {
                    location: Location {
                        col: first_token.col,
                        row: first_token.row,
                        offset: first_token.offset,
                    },
                    message: String::from("unexpected token when parsing expression. "),
                })
            }
        };
    }

    pub fn parse_identifier(&mut self) -> Result<Box<IdentifierExpr>, ParserError> {
        let first_token = self.cur_token();

        match first_token {
            Token {
                tag: TokenTag::Identifier(identifier),
                ..
            } => {
                // 'identifier'
                self.next_token()?;
                return Ok(Box::new(IdentifierExpr {
                    location: Location {
                        col: first_token.col,
                        row: first_token.row,
                        offset: first_token.offset,
                    },
                    identifer: identifier,
                }));
            }
            _ => {
                return Err(ParserError::SyntaticError {
                    location: Location {
                        col: first_token.col,
                        row: first_token.row,
                        offset: first_token.offset,
                    },
                    message: String::from("unexpected token when parsing identifier. "),
                })
            }
        }
    }

    pub fn parse_integer(&mut self) -> Result<Box<IntegerLiteral>, ParserError> {
        let first_token = self.cur_token();

        match first_token {
            Token {
                tag: TokenTag::IntegerLiteral(value),
                ..
            } => {
                self.next_token()?;
                return Ok(Box::new(IntegerLiteral {
                    location: Location {
                        col: first_token.col,
                        row: first_token.row,
                        offset: first_token.offset,
                    },
                    value,
                }));
            },
            _ => {
                return Err(ParserError::SyntaticError {
                    location: Location {
                        col: first_token.col,
                        row: first_token.row,
                        offset: first_token.offset,
                    },
                    message: String::from("unexpected token when parsing integer. "),
                })
            }
        }
    }
 
    fn parse_call_like(&mut self) -> Result<Box<dyn Expr>, ParserError> {
        // parse '('
        let lparen = self.cur_token();

        // parse 'identifier'
        let first_arg = self.next_token()?;

        match first_arg {

            Token {
                tag: TokenTag::Identifier(identifier),
                ..
            } => match identifier.as_str() {
                "define" => {
                    self.next_token()?;

                    let identifier_expr = self.parse_identifier()?;

                    let value_expr = self.parse_expr()?;

                    let rparen = self.cur_token();

                    match rparen {
                        Token {
                            tag: TokenTag::RParen,
                            ..
                        } => {
                            self.next_token()?;
                        }
                        _ => {
                            return Err(ParserError::SyntaticError {
                                location: Location {
                                    col: first_arg.col,
                                    row: first_arg.row,
                                    offset: first_arg.offset,
                                },
                                message: String::from(
                                    "expecting ')' at the end of define expression ",
                                ),
                            })
                        }
                    }

                    let define_expr = Box::new(DefineExpr {
                        location: Location {
                            col: lparen.col,
                            offset: lparen.offset,
                            row: lparen.row,
                        },
                        identifier: identifier_expr,
                        value: value_expr,
                    });

                    Ok(define_expr)
                }
                _ => {
                    return Err(ParserError::SyntaticError {
                        location: Location {
                            col: first_arg.col,
                            row: first_arg.row,
                            offset: first_arg.offset,
                        },
                        message: String::from(
                            "unexpected token when parsing call-like expression. ",
                        ),
                    })
                }
            },
            _ => {
                return Err(ParserError::SyntaticError {
                    location: Location {
                        col: first_arg.col,
                        row: first_arg.row,
                        offset: first_arg.offset,
                    },
                    message: String::from("unexpected token when parsing call-like expression. "),
                })
            }
        }
    }
}
