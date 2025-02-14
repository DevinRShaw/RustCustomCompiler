#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Plus,             
    Subtract,         
    Multiply,         
    Divide,           
    Modulus,          
    Assign,           
    Less,             
    LessEqual,        
    Greater,          
    GreaterEqual,     
    Equality,         
    NotEqual,         
    Num(i32),         
    Ident(String),    
    If,               
    While,            
    Read,             
    Func,             
    Return,           
    Int,              
    Print,            
    Else,             
    Break,            
    Continue,         
    LeftParen,        
    RightParen,       
    LeftCurly,        
    RightCurly,       
    LeftBracket,      
    RightBracket,     
    Comma,            
    Semicolon,        
    End,              
}

// Make the function public so it can be used in main.rs
pub fn create_identifier(code: &str) -> Token {
    match code {
        "func" => Token::Func,
        "return" => Token::Return,
        "int" => Token::Int,
        "print" => Token::Print,
        "else" => Token::Else,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "while" => Token::While,
        "if" => Token::If,
        "read" => Token::Read,
        _ => Token::Ident(String::from(code)), 
    }
}

// Make the lex function public
pub fn lex(code: &str) -> Result<Vec<Token>, String> {
    let bytes = code.as_bytes();
    let mut tokens: Vec<Token> = vec![];
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i] as char;

        match c {
            'a'..='z' | 'A'..='Z' => {
                let start = i;
                i += 1;
                while i < bytes.len() {
                    let character = bytes[i] as char;
                    if character.is_alphanumeric() || bytes[i] == b'_' {
                        i += 1;
                    } else {
                        break;
                    }
                }
                let end = i;
                let string_token = &code[start..end];
                tokens.push(create_identifier(string_token));
            }
            '0'..='9' => {
                let start = i;
                i += 1;
                while i < bytes.len() && (bytes[i] as char).is_numeric() {
                    i += 1;
                }
                let end = i;

                if i < bytes.len() && ((bytes[i] as char).is_alphabetic() || bytes[i] == b'_') {
                    return Err(format!(
                        "Lexer: Invalid variable name starting with a number at: {}",
                        &code[start..i + 1]
                    ));
                }

                let string_token = &code[start..end];
                let number_value = string_token.parse::<i32>().unwrap();
                tokens.push(Token::Num(number_value));
            }
            '#' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            '<' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    tokens.push(Token::LessEqual);
                    i += 2;
                    continue;
                }
                tokens.push(Token::Less);
                i += 1;
            }
            '>' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    tokens.push(Token::GreaterEqual);
                    i += 2;
                    continue;
                }
                tokens.push(Token::Greater);
                i += 1;
            }
            '!' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    tokens.push(Token::NotEqual);
                    i += 2;
                    continue;
                }
                return Err(format!("Lexer: Unrecognized symbol '{}'", c));
            }
            '=' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'=' {
                    tokens.push(Token::Equality);
                    i += 2;
                    continue;
                }
                tokens.push(Token::Assign);
                i += 1;
            }
            ' ' | '\n' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Subtract);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Multiply);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Divide);
                i += 1;
            }
            '%' => {
                tokens.push(Token::Modulus);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                i += 1;
            }
            '{' => {
                tokens.push(Token::LeftCurly);
                i += 1;
            }
            '}' => {
                tokens.push(Token::RightCurly);
                i += 1;
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                i += 1;
            }
            ']' => {
                tokens.push(Token::RightBracket);
                i += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                i += 1;
            }
            _ => {
                return Err(format!("Lexer: Unrecognized symbol '{}'", c));
            }
        }
    }

    tokens.push(Token::End);
    Ok(tokens)
}
