
#![allow(dead_code)]
// src/parser.rs
// Import lexer functions or structs
use super::lexer::*;  // Adjust based on your actual lexer implementation


// parse programs with multiple functions
// loop over everything, outputting generated code.
pub fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
  while !at_end(tokens, *index) {
    match parse_function(tokens, index) {
    Ok(()) => {}
    Err(e) => { return Err(e); }
    }
  }
  return Ok(());
}
  
pub fn at_end(tokens: &Vec<Token>, index: usize) -> bool {
  match tokens[index] {
    Token::End => { true }
    _ => { false }
  }
}
  
  // parse function such as:
  // func main(int a, int b) {
  //    # ... statements here...
  //    # ...
  // }
  // a loop is done to handle statements.
  
pub fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Func => *index += 1,
    _ => return Err(String::from("Parser: Functions must begin with 'func'")),
  }

  match tokens[*index] {
    Token::Ident(_) => *index += 1,
    _ => return Err(String::from("Parser: Functions must have a function identifier")),
  }

  match tokens[*index] {
    Token::LeftParen => *index += 1,
    _ => return Err(String::from("Parser: Funtion expects '('")),
  }

  // Handling parameter declarations (e.g., func(int example, int a, int b))
  while !matches!(tokens[*index], Token::RightParen) {
    match parse_declaration(tokens, index) {
      Ok(()) => {}
      Err(e) => return Err(e),
    }

    if tokens[*index] == Token::Comma {
      *index += 1;
      match parse_declaration(tokens, index) {
        Ok(()) => {}
        Err(e) => return Err(e),
      }
    }
  }

  match tokens[*index] {
    Token::RightParen => *index += 1,
    _ => return Err(String::from("Parser: Function expects ')'")),
  }

  match tokens[*index] {
    Token::LeftCurly => *index += 1,
    _ => return Err(String::from("Parser: Function expects '{'")),
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index) {
      Ok(()) => {}
      Err(e) => return Err(e),
    }
  }

  match tokens[*index] {
    Token::RightCurly => *index += 1,
    _ => return Err(String::from("Parser: Function expects '}'")),
  }

  Ok(())
}
  
  
  //our declaration for in function declarations
  // Declaration parsing for function parameter declarations
pub fn parse_declaration(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Int => *index += 1,
    _ => return Err(String::from("Parser: Function declaration statements must begin with 'int' keyword")),
  }

  // Handle `int [num] ident` logic
  if tokens[*index] == Token::LeftBracket {
    *index += 1;
    
    match tokens[*index] {
      Token::Num(_) => *index += 1,
      _ => return Err(String::from("Parser: Function declarations of arrays must have Type [Num] Ident form")),
    }

    match tokens[*index] {
      Token::RightBracket => *index += 1,
      _ => return Err(String::from("Parser: Function declarations of arrays in Type [Num] Ident form require a closing bracket")),
    }
  }

  match tokens[*index] {
    Token::Ident(_) => *index += 1,
    _ => return Err(String::from("Parser: Function declarations must have an identifier")),
  }

  Ok(())
}

  
  // parsing a statement such as:
  // int a;
  // a = a + b;
  // a = a % b;
  // print(a)
  // read(a)
  // returns epsilon if '}'

pub fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Int => parse_declaration_statement(tokens, index),
    Token::Ident(_) => parse_assignment_statement(tokens, index),
    Token::Return => parse_return_statement(tokens, index),
    Token::Print => parse_print_statement(tokens, index),
    Token::Read => parse_read_statement(tokens, index),

    // Control flow statements
    Token::If => parse_if_statement(tokens, index),
    Token::While => parse_while_statement(tokens, index),

    // Loop control statements
    Token::Break => parse_break_statement(tokens, index),
    Token::Continue => parse_continue_statement(tokens, index),

    _ => Err(String::from("Parser: Invalid statement")),
  }
}
  
  
pub fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Int => *index += 1,
    _ => return Err(String::from("Parser: Declaration statements must begin with 'int' keyword")),
  }

  // Handle `int [num] ident` logic
  if tokens[*index] == Token::LeftBracket {
    *index += 1;
    
    match tokens[*index] {
      Token::Num(_) => *index += 1,
      _ => return Err(String::from("Parser: Declarations of arrays must have Type [Num] Ident form")),
    }

    match tokens[*index] {
      Token::RightBracket => *index += 1,
      _ => return Err(String::from("Parser: Declarations of Type [Num] Ident form require a closing bracket")),
    }
  }

  match tokens[*index] {
    Token::Ident(_) => *index += 1,
    _ => return Err(String::from("Parser: Declarations must have an identifier")),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Declarations statements must end with a semicolon")),
  }

  Ok(())
}


pub fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Ident(_) => *index += 1,
    _ => return Err(String::from("Parser: Assignment statements must begin with an identifier")),
  }

  // Support for array indexing assignment (e.g., arr[expression] = var)
  if tokens[*index] == Token::LeftBracket {
    *index += 1;

    match parse_expression(tokens, index) {
      Ok(()) => {},
      Err(e) => return Err(e),
    }

    match tokens[*index] {
      Token::RightBracket => *index += 1,
      _ => return Err(String::from("Parser: Array assignments must have a closing bracket ']'")),
    }
  }

  match tokens[*index] {
    Token::Assign => *index += 1,
    _ => return Err(String::from("Parser: Assignment statement is missing the '=' operator")),
  }

  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Assignment statements must end with a semicolon ';'")),
  }

  Ok(())
}

fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
  Token::Return => {*index += 1;}
  _ => {return Err(String::from("Parser: Return statements must begin with a return keyword"));}
  }

  match parse_expression(tokens, index) {
  Ok(()) => {},
  Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Parser: Return statements must end with a semicolon"));}
  }

  return Ok(());
}

  
pub fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Print => *index += 1,
    _ => return Err(String::from("Parser: Print statements must begin with the 'print' keyword")),
  }

  // Parse the expression inside the parentheses
  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  // Expect semicolon
  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Print statements must end with a semicolon ';'")),
  }

  Ok(())
}

  
pub fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Read => *index += 1,
    _ => return Err(String::from("Parser: Read statements must begin with the 'read' keyword")),
  }

  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Read statements must end with a semicolon ';'")),
  }

  Ok(())
}

  
pub fn parse_break_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Break => *index += 1,
    _ => return Err(String::from("Parser: Expected 'break' keyword")),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Break statements must end with a semicolon ';'")),
  }

  Ok(())
}

  
pub fn parse_continue_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::Continue => *index += 1,
    _ => return Err(String::from("Parser: Expected 'continue' keyword")),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Continue statements must end with a semicolon ';'")),
  }

  Ok(())
}

  
pub fn parse_while_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::While => *index += 1,
    _ => return Err(String::from("Parser: Expected 'while' keyword")),
  }

  match parse_bool(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  match tokens[*index] {
    Token::LeftCurly => *index += 1,
    _ => return Err(String::from("Parser: While statement execution code must begin with '{'")),
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index) {
      Ok(()) => {},
      Err(e) => return Err(e),
    }
  }

  match tokens[*index] {
    Token::RightCurly => *index += 1,
    _ => return Err(String::from("Parser: While statement expects '}'")),
  }

  Ok(())
}

  
  
pub fn parse_if_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    Token::If => *index += 1,
    _ => return Err(String::from("Parser: If statement expects 'if' keyword")),
  }

  match parse_bool(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  match tokens[*index] {
    Token::LeftCurly => *index += 1,
    _ => return Err(String::from("Parser: If statement execution code must begin with '{'")),
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index) {
      Ok(()) => {},
      Err(e) => return Err(e),
    }
  }

  match tokens[*index] {
    Token::RightCurly => *index += 1,
    _ => return Err(String::from("Parser: If statement expects '}'")),
  }

  if *index < tokens.len() && matches!(tokens[*index], Token::Else) {
    *index += 1;

    match tokens[*index] {
      Token::LeftCurly => *index += 1,
      _ => return Err(String::from("Parser: Else statement execution code must begin with '{'")),
    }

    while !matches!(tokens[*index], Token::RightCurly) {
      match parse_statement(tokens, index) {
        Ok(()) => {},
        Err(e) => return Err(e),
      }
    }

    match tokens[*index] {
      Token::RightCurly => *index += 1,
      _ => return Err(String::from("Parser: Else statement expects '}' after else block")),
    }
  }

  Ok(())
}

  
pub fn parse_bool(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  match tokens[*index] {
    Token::Less 
    | Token::LessEqual 
    | Token::Greater 
    | Token::GreaterEqual 
    | Token::Equality 
    | Token::NotEqual => {
      *index += 1;
    }
    _ => return Err(String::from("Parser: Boolean expression expects a comparison operator")),
  }

  match parse_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  Ok(())
}

  
// Parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  // First, parse a multiply/divide expression
  match parse_multiply_expression(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  // Then, handle addition and subtraction
  loop {
    match tokens[*index] {
      Token::Plus => {
        *index += 1;
        match parse_multiply_expression(tokens, index) {
          Ok(()) => {},
          Err(e) => return Err(e),
        }
      }

      Token::Subtract => {
        *index += 1;
        match parse_multiply_expression(tokens, index) {
          Ok(()) => {},
          Err(e) => return Err(e),
        }
      }

      _ => break, // Exit loop when no addition or subtraction operator is found
    }
  }

  Ok(())
}

  
pub fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  // First, parse a term (e.g., a number, identifier, or parenthesized expression)
  match parse_term(tokens, index) {
    Ok(()) => {},
    Err(e) => return Err(e),
  }

  // Then, handle multiplication, division, and modulus operations
  loop {
    match tokens[*index] {
      Token::Multiply | Token::Divide | Token::Modulus => {
        // Consume the operator (multiply, divide, or modulus)
        let operator = tokens[*index].clone(); // We may want to use it later (debugging, etc.)
        *index += 1;

        // Parse the next term
        match parse_term(tokens, index) {
          Ok(()) => {},
          Err(e) => return Err(e),
        }
      }

      _ => break, // Exit loop when no operator is found
    }
  }

  Ok(())
}

  
  
pub fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<(), String> {
  match tokens[*index] {
    // Handle identifier (e.g., variable, function call, or array access)
    Token::Ident(_) => {
      *index += 1;

      // Handle function call: ident(...)
      if tokens[*index] == Token::LeftParen {
        *index += 1;

        // Parse the function's arguments
        while !matches!(tokens[*index], Token::RightParen) {
          match parse_expression(tokens, index) {
            Ok(()) => {},
            Err(e) => return Err(e),
          }

          // If there's a comma, continue parsing more arguments
          if tokens[*index] == Token::Comma {
            *index += 1; // Skip the comma
          } else {
            break; // End function argument parsing
          }
        }

        // Check for closing parenthesis
        match tokens[*index] {
          Token::RightParen => *index += 1,
          _ => return Err(String::from("Parser: Function call expects closing parenthesis")),
        }
      }

      // Handle array access: ident[expression]
      else if tokens[*index] == Token::LeftBracket {
        *index += 1;

        match parse_expression(tokens, index) {
          Ok(()) => {},
          Err(e) => return Err(e),
        }

        match tokens[*index] {
          Token::RightBracket => *index += 1,
          _ => return Err(String::from("Parser: Array access expects closing bracket ']'")),
        }
      }

      Ok(())
    }

    // Handle numbers directly
    Token::Num(_) => {
      *index += 1;
      Ok(())
    }

    // Handle parenthesized expressions: (expression)
    Token::LeftParen => {
      *index += 1;

      match parse_expression(tokens, index) {
        Ok(()) => {},
        Err(e) => return Err(e),
      }

      match tokens[*index] {
        Token::RightParen => *index += 1,
        _ => return Err(String::from("Parser: Expression expects closing parenthesis ')'")),
      }

      Ok(())
    }

    // Handle invalid token
    _ => Err(String::from("Parser: Unexpected token, expected a term")),
  }
}

  
  