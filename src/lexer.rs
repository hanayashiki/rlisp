use std::fmt;
use std::str;

pub struct Lexer<'a> {
    pub code: &'a str,
    cur: Option<char>,
    cur_offset: i32,
    cur_row: i32,
    cur_col: i32,
    char_indices: str::CharIndices<'a>,
}

#[derive(fmt::Debug)]
pub struct LexicalError {
    pub offset: i32,
    pub row: i32,
    pub col: i32,
    pub message: String,
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Lexical error at line {} column {}: {}",
            self.row, self.col, self.message
        )
    }
}

#[derive(Debug, Clone)]
pub enum TokenTag {
    LParen,
    RParen,
    Identifier(String),
    IntegerLiteral(i32),
    StringLiteral(String),
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: TokenTag,
    pub offset: i32,
    pub row: i32,
    pub col: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Lexer<'a> {
        Lexer {
            code,
            cur: None,
            cur_offset: -1,
            cur_row: 1,
            cur_col: 0,
            char_indices: code.char_indices(),
        }
    }

    pub fn init(&mut self) -> Result<Token, LexicalError> {
        self.next_char();
        return self.next();
    }

    fn next_char(&mut self) {
        let cur = self.char_indices.next();

        match cur {
            Some((_, c)) => {
                self.cur = Some(c);
                self.cur_offset += 1;
                self.cur_col += 1;
                if c == '\n' {
                    self.cur_col = 0;
                    self.cur_row += 1;
                }
            }
            None => {
                self.cur = None;
            }
        }
    }

    pub fn next(&mut self) -> Result<Token, LexicalError> {
        loop {
            match self.cur {
                Some(' ' | '\t' | '\r' | '\n') => {
                    self.next_char();
                }
                _ => {
                    break;
                }
            }
        }

        let mut token = Ok(Token {
            tag: TokenTag::EOF,
            offset: self.cur_offset,
            row: self.cur_row,
            col: self.cur_col,
        });

        match self.cur {
            Some('(') => {
                token = Ok(Token {
                    tag: TokenTag::LParen,
                    offset: self.cur_offset,
                    row: self.cur_row,
                    col: self.cur_col,
                });
                self.next_char();
            }
            Some(')') => {
                token = Ok(Token {
                    tag: TokenTag::RParen,
                    offset: self.cur_offset,
                    row: self.cur_row,
                    col: self.cur_col,
                });
                self.next_char();
            }
            Some('A'..='Z' | 'a'..='z') => {
                let mut identifier = String::new();
                let offset = self.cur_offset;
                let col = self.cur_col;
                let row = self.cur_row;
                loop {
                    match self.cur {
                        Some(c @ ('A'..='Z' | 'a'..='z' | '0'..='9' | '-')) => {
                            identifier.push(c);
                            self.next_char();
                        }
                        _ => {
                            token = Ok(Token {
                                tag: TokenTag::Identifier(identifier),
                                offset,
                                row,
                                col,
                            });
                            break;
                        }
                    }
                }
            }
            Some('0'..='9') => {
                let mut number = String::new();
                let offset = self.cur_offset;
                let col = self.cur_col;
                let row = self.cur_row;

                loop {
                    match self.cur {
                        Some(c @ ('0'..='9')) => {
                            number.push(c);
                            self.next_char();
                        }
                        _ => {
                            let value_result = str::parse::<i32>(number.as_str());

                            if let Err(e) = value_result {
                                return Err(LexicalError {
                                    offset,
                                    col,
                                    row,
                                    message: e.to_string(),
                                });
                            } else {
                                token = Ok(Token {
                                    tag: TokenTag::IntegerLiteral(value_result.unwrap()),
                                    offset,
                                    row,
                                    col,
                                });
                            }

                            break;
                        }
                    }
                }
            }
            Some('"') => {
                let mut string = String::new();

                let offset = self.cur_offset;
                let col = self.cur_col;
                let row = self.cur_row;

                self.next_char();

                loop {
                    match self.cur {
                        Some('\\') => {
                            self.next_char();

                            match self.cur {
                                Some('n') => string.push('\n'),
                                Some('r') => string.push('\r'),
                                Some('t') => string.push('\t'),
                                Some('"') => string.push('"'),
                                None => {
                                    return Err(LexicalError {
                                        offset,
                                        col,
                                        row,
                                        message: "unexpected EOF when parsing string".to_string(),
                                    })
                                }
                                _ => {
                                    return Err(LexicalError {
                                        offset,
                                        col,
                                        row,
                                        message: "unknown escaped character".to_string(),
                                    })
                                }
                            }
                            self.next_char()
                        }
                        Some('"') => {
                            self.next_char();

                            return Ok(Token {
                                tag: TokenTag::StringLiteral(string),
                                offset,
                                col,
                                row,
                            })
                        }
                        Some(c) => {
                            string.push(c);
                            self.next_char();
                        }
                        None => {
                            return Err(LexicalError {
                                offset,
                                col,
                                row,
                                message: "unexpected EOF when parsing string".to_string(),
                            })
                        }

                    }
                }
            }
            _ => {}
        }

        return token;
    }
}
