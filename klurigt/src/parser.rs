use crate::lexer::Token;
use crate::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    
    fn current_token(&self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }
    
    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
    
    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.current_token() {
            Some(token) if token == &expected => {
                self.advance();
                Ok(())
            }
            Some(token) => Err(format!("Expected {:?}, but found {:?}", expected, token)),
            None => Err(format!("Expected {:?}, but there is no token", expected)),
        }
    }
    
    fn parse_term(&mut self) -> Result<Expression, String> {
        match self.current_token() {
            Some(Token::Number(n)) => {
                let num = n.parse::<i64>().unwrap();
                self.advance();
                Ok(Expression::Number(num))
            }
            Some(Token::Identifier(id)) => {
                let identifier = id.clone();
                self.advance();
                Ok(Expression::Identifier(identifier))
            }
            Some(Token::StringLiteral(s)) => {
                let string = s.clone();
                self.advance();
                Ok(Expression::StringLiteral(string))
            }
            Some(_) => Err("".to_string()),
            None => Err("".to_string()),
        }
    }
    
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let left = self.parse_term()?;
        
        if let Some(Token::Plus) = self.current_token() {
            self.advance();
            let right = self.parse_expression()?;
            Ok(Expression::BinaryOp {
                left: Box::new(left),
                op: BinaryOp::Add,
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }
    
    fn parse_condition(&mut self) -> Result<Condition, String> {
        let left = self.parse_expression()?;
        
        let op = match self.current_token() {
            Some(Token::Equal) => CompOp::Equal,
            Some(Token::LessThan) => CompOp::LessThan,
            Some(Token::GreaterThan) => CompOp::GreaterThan,
            Some(_) => return Err("".to_string()),
            None => return Err("".to_string()),
        };
        self.advance();
        
        let right = self.parse_expression()?;
        
        Ok(Condition { left, op, right })
    }
    
    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(Token::LBracket)?;
        
        let mut statements = Vec::new();
        
        while let Some(token) = self.current_token() {
            if token == &Token::RBracket {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        
        self.expect(Token::RBracket)?;
        
        Ok(Block { statements })
    }
    
    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.current_token() {
            Some(Token::Let) => self.parse_var_declaration(),
            Some(Token::Identifier(_)) => self.parse_assignment(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::Print) => self.parse_print_statement(),
            Some(_) => Err("".to_string()),
            None => Err("".to_string()),
        }
    }
    
    fn parse_var_declaration(&mut self) -> Result<Statement, String> {
        self.advance();
        
        let name = match self.current_token() {
            Some(Token::Identifier(id)) => id.clone(),
            Some(_) => return Err("".to_string()),
            None => return Err("".to_string()),
        };
        self.advance();
        
        self.expect(Token::Colon)?;
        
        let var_type = match self.current_token() {
            Some(Token::TypeNumber) => Type::Number,
            Some(Token::TypeString) => Type::String,
            Some(_) => return Err("".to_string()),
            None => return Err("".to_string()),
        };
        self.advance();
        
        self.expect(Token::Assign)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::VarDeclaration { name, var_type, value })
    }
    
    fn parse_assignment(&mut self) -> Result<Statement, String> {
        let name = match self.current_token() {
            Some(Token::Identifier(id)) => id.clone(),
            _ => return Err("".to_string()),
        };
        self.advance();
        
        self.expect(Token::Assign)?;
        let value = self.parse_expression()?;
        self.expect(Token::Semicolon)?;
        
        Ok(Statement::Assignment { name, value })
    }
    
    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        self.advance();
        
        self.expect(Token::LParen)?;
        let condition = self.parse_condition()?;
        self.expect(Token::RParen)?;
        
        let then_block = self.parse_block()?;
        
        let else_block = if let Some(Token::Else) = self.current_token() {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        
        self.expect(Token::EndIf)?;
        
        Ok(Statement::If { condition, then_block, else_block })
    }
    
    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        self.advance();
        
        self.expect(Token::LParen)?;
        let condition = self.parse_condition()?;
        self.expect(Token::RParen)?;
        
        let body = self.parse_block()?;
        
        self.expect(Token::EndWhile)?;
        
        Ok(Statement::While { condition, body })
    }
    
    fn parse_print_statement(&mut self) -> Result<Statement, String> {
        self.advance();
        
        self.expect(Token::LParen)?;
        let value = self.parse_expression()?;
        self.expect(Token::RParen)?;
        
        self.expect(Token::EndPrint)?;
        
        Ok(Statement::Print { value })
    }
    
    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while self.position < self.tokens.len() {
            statements.push(self.parse_statement()?);
        }
        
        if statements.is_empty() {
            return Err("".to_string());
        }
        
        Ok(Program { statements })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_valid_code() {
        let input = "make a: number be 5 STOP";
        let lexer = Lexer::new(input.to_string());
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(tokens);

        let program = parser.parse_program();
        assert!(program.is_ok());
    }

    #[test]
    fn test_illegal_code() {
        let input = "make a: 5 be 5 STOP";
        let lexer = Lexer::new(input.to_string());
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(tokens);

        let program = parser.parse_program();
        assert!(program.is_err());
    }
}

