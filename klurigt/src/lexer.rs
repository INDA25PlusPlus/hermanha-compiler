#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Plus,
    Equal,
    LessThan,
    GreaterThan,
    Let,
    Assign,
    Semicolon,
    If,
    Else,
    EndIf,
    While,
    EndWhile,
    Print,
    EndPrint,
    TypeNumber,
    TypeString,
    Identifier(String),
    Number(String),
    StringLiteral(String),
}
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.advance();
        while let Some(c) = self.current_char() {
            if c == '"' {
                self.advance();
                break;
            } else {
                result.push(c);
                self.advance();
            }
        }
        result
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        match self.current_char() {
            None => None,
            Some(ch) => Some(match ch {
                '(' => {
                    self.advance();
                    Token::LParen
                }
                ')' => {
                    self.advance();
                    Token::RParen
                }
                '{' => {
                    self.advance();
                    Token::LBracket
                }
                '}' => {
                    self.advance();
                    Token::RBracket
                }
                ':' => {
                    self.advance();
                    Token::Colon
                }
                '+' => {
                    self.advance();
                    Token::Plus
                }
                '"' => Token::StringLiteral(self.read_string()),
                _ if ch.is_ascii_digit() => Token::Number(self.read_number()),
                _ if ch.is_alphabetic() => {
                    let word = self.read_identifier();
                    match word.as_str() {
                        "notalmostthesamethesame" => Token::Equal,
                        "oppositeoftinyerthan" => Token::GreaterThan,
                        "letsdothisinstead" => Token::Else,
                        "keeponswimming" => Token::While,
                        "STOPSWIMMING" => Token::EndWhile,
                        "tinyerthan" => Token::LessThan,
                        "ENDMAYBE" => Token::EndIf,
                        "number" => Token::TypeNumber,
                        "string" => Token::TypeString,
                        "SCREAM" => Token::Print,
                        "maybe" => Token::If,
                        "QUIET" => Token::EndPrint,
                        "make" => Token::Let,
                        "STOP" => Token::Semicolon,
                        "be" => Token::Assign,
                        _ => Token::Identifier(word),
                    }
                }
                _ => {
                    // skip the character and try again
                    self.advance();
                    return self.next();
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_variable_declaration() {
        let input = "make x: number be 42 STOP".to_string();
        let tokens: Vec<Token> = Lexer::new(input).collect();

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Identifier("x".to_string()),
                Token::Colon,
                Token::TypeNumber,
                Token::Assign,
                Token::Number("42".to_string()),
                Token::Semicolon,
            ]
        );
    }
}
