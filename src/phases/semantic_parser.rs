#![allow(dead_code)]
// src/parser.rs
// Import lexer functions or structs
use super::lexer::*;  // Adjust based on your actual lexer implementation


fn peek_error(tokens: &[Token], index: &mut usize) -> Result<Token, String> {
  if *index >= tokens.len() {
      return Err("Unexpected end of input".to_string());
  }
  Ok(tokens[*index].clone()) // Assuming Token is cloneable
}

fn create_temp() -> String {
  static mut TEMP_COUNT: usize = 0; // Unsafe but works for quick testing
  unsafe {
      let temp_name = format!("t{}", TEMP_COUNT);
      TEMP_COUNT += 1;
      temp_name
  }
}

fn create_label() -> String {
  static mut LABEL_COUNT: usize = 0; // Unsafe, consider using thread-safe alternatives
  unsafe {
      let label_name = format!(":{}", LABEL_COUNT);
      LABEL_COUNT += 1;
      label_name
  }
}


//static mut scope_stack: Vec<Vec<(String, SymbolType)>> = vec![vec![]]; // Initialize with global scope

static mut scope_stack: Vec<Vec<(String, SymbolType)>> = Vec::new();

static mut in_loop: bool = false; //for confirming break statements 

static mut CURRENT_END_LABEL: String =  String::new(); //for jumping to end on break 

fn enter_scope() {
    unsafe {
        scope_stack.push(vec![]); // Create a new scope
    }
}

fn exit_scope() {
    unsafe {
        scope_stack.pop(); // Remove the current scope
    }
}

fn find_symbol(symbol: &String) -> bool {
  unsafe {
      if let Some(current_scope) = scope_stack.last() {
          for (symbol_in_table, _) in current_scope {
              if symbol_in_table.eq(symbol) {
                  return true;
              }
          }
      }
  }
  false
}


// Function to add a symbol to the current scope
fn add_symbol(ident: String, symbol_type: SymbolType) {
  unsafe {
      // Ensure we have at least one scope to push to
      if let Some(current_scope) = scope_stack.last_mut() {
          current_scope.push((ident, symbol_type)); // Push the symbol to the current scope
      }
  }
}



fn find_symbol_type(symbol: &String) -> SymbolType {
  unsafe {
      if let Some(current_scope) = scope_stack.last() {
          for (symbol_in_table, typ) in current_scope {
              if symbol_in_table.eq(symbol) {
                  return typ.clone(); // Return the type if found
              }
          }
      }
  }
  SymbolType::Variable // Default type if not found in the current scope
}




#[derive(Clone)]
#[derive(PartialEq)]
enum SymbolType {
  Variable,   // Regular scalar variable
  Function,   // Function declaration
  Array,      // Array variable
}








// parse programs with multiple functions
// loop over everything, outputting generated code.
pub fn parse_program(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {


  let mut ir_code: String = String::new();


  // Enter global scope to track function declarations
  enter_scope();
  



  assert!(tokens.len() >= 1 && matches!(tokens[tokens.len() - 1], Token::End));
  while !at_end(tokens, *index) {
    match parse_function(tokens, index) {

        //append fucntion ir code 
        Ok(function_ir_code) => {

            ir_code += &function_ir_code.unwrap();
            
        }


        Err(e) => { return Err(e); }
    }
  }

  unsafe{

    if !find_symbol( &"main".to_string() ){
      return Err(format!("Semantic Analysis: Did not find main function"));
    }
  }

  // Exit global scope after parsing is complete
  exit_scope();


  return Ok(ir_code);
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
  
pub fn parse_function(tokens: &Vec<Token>, index: &mut usize) -> Result<Option<String>, String> {

    //%func main()
    //%endfunc

    match tokens[*index] {
    Token::Func => *index += 1,
    _ => return Err(String::from("Parser: Functions must begin with 'func'")),
    }


    let mut code: String = String::new();

     

    match &tokens[*index] {
        Token::Ident(identifier_name) => {
          *index += 1;
            //duplicate function 
          unsafe { 
            if find_symbol( identifier_name ){
              return Err(format!("Semantic Analysis: Found duplicate function {identifier_name}"));
            }
            
            //symbol_table.push((identifier_name.clone(), SymbolType::Function));
            add_symbol(identifier_name.clone(), SymbolType::Function);


          }

            //this needs to be edited to handle declarations 
            code += &format!("%func {identifier_name}(");
        },
        _ => return Err(String::from("Parser: Functions must have a function identifier")),
    }



    match tokens[*index] {
    Token::LeftParen => *index += 1,
    _ => return Err(String::from("Parser: Funtion expects '('")),
    }


    // **Enter a new scope for parameters & function body**
    enter_scope();

    // Handling parameter declarations (e.g., func(int example, int a, int b))
    while !matches!(tokens[*index], Token::RightParen) {
        match parse_declaration(tokens, index) {
            Ok(declaration_code) => { code += &declaration_code; }
            Err(e) => return Err(e),
        }

        // If there's a comma, advance and parse the next parameter
        if matches!(tokens[*index], Token::Comma) {
            *index += 1;
            code += ", ";
        } else {
            break;
        }
    }

    match tokens[*index] {
    Token::RightParen => *index += 1,
    _ => return Err(String::from("Parser: Function expects ')'")),
    }

    code += ")\n";

    match tokens[*index] {
      Token::LeftCurly => *index += 1,
        _ => return Err(String::from("Parser: Function expects '{'")),
    }


    while !matches!(tokens[*index], Token::RightCurly) {
      match parse_statement(tokens, index) {
          Ok(statment_code) => {code += &format!("{}",statment_code);}
          Err(e) => return Err(e),
      }
    }

    match tokens[*index] {
    Token::RightCurly => *index += 1,
    _ => return Err(String::from("Parser: Function expects '}'")),
    }

    // Exit local scope after parsing the function body
    exit_scope();

    

    code += "%endfunc\n";

    return Ok(Some(code));
}
  
  
  //our declaration for in function declarations
  // Declaration parsing for function parameter declarations
pub fn parse_declaration(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

    let mut code: String = String::new();

    let mut symbol_type = SymbolType::Variable;


    match tokens[*index] {
        Token::Int => *index += 1,
        _ => return Err(String::from("Parser: Function declaration statements must begin with 'int' keyword")),
    }

    // Handle `int [num] ident` logic
    // We aren't assessed on this so skip, also is unclear for the actual parser why we would declare a function with sized array 
    if tokens[*index] == Token::LeftBracket {
        *index += 1;

        match tokens[*index] {
            Token::Num(num) => {
              *index += 1;
              if num == 0 {
                return Err(format!("Semantic Analysis: Cannot have array sized 0"))
              }
            },
            _ => return Err(String::from("Parser: Function declarations of arrays must have Type [Num] Ident form")),
        }

        match tokens[*index] {
            Token::RightBracket => *index += 1,
            _ => return Err(String::from("Parser: Function declarations of arrays in Type [Num] Ident form require a closing bracket")),
        }

        symbol_type = SymbolType::Array;
    }

    match &tokens[*index] {
    Token::Ident(ident) =>
    {
      //duplicate symbols 
      unsafe { 
        if find_symbol(ident){
          return Err(format!("Semantic Analysis: Found duplicate variable {ident}"));
        }

        add_symbol(ident.clone(), symbol_type);
      }
        *index += 1;
        code += &format!("%int {ident}");

    },
    _ => return Err(String::from("Parser: Function declarations must have an identifier")),
    }

    return Ok(code);
}

  
  // parsing a statement such as:
  // int a;
  // a = a + b;
  // a = a % b;
  // print(a)
  // read(a)
  // returns epsilon if '}'

pub fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
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
  
  
pub fn parse_declaration_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

    let mut code: String = String::new();
    let mut symbol_type = SymbolType::Variable;


    match tokens[*index] {
        Token::Int => *index += 1,
        _ => return Err(String::from("Parser: Declaration statements must begin with 'int' keyword")),
    }

    code += "%int";


    
    let mut arrayNum: String = String::new();
    arrayNum = "".to_string();

    // Handle `int [num] ident` logic
    if tokens[*index] == Token::LeftBracket {
      
      *index += 1;

      code += "[]";

      match tokens[*index] {
          Token::Num(num) => {
              *index += 1;
              if num == 0 {
                return Err(format!("Semantic Analysis: Cannot have array sized 0"))
              }
              arrayNum = format!(", {num}")
          },

          _ => return Err(String::from("Parser: Declarations of arrays must have Type [Num] Ident form")),
      }

      

      match tokens[*index] {
          Token::RightBracket => *index += 1,
          _ => return Err(String::from("Parser: Declarations of Type [Num] Ident form require a closing bracket")),
      }

      symbol_type = SymbolType::Array;
    }

    match &tokens[*index] {
        Token::Ident(ident) => {
          //finding duplicates
          unsafe { 
            if find_symbol(ident){
              return Err(format!("Semantic Analysis: Found duplicate variable {ident}"));
            }
    
            add_symbol(ident.clone(), symbol_type);
          }

          *index += 1;
          code += &format!(" {ident}");
        },
        _ => return Err(String::from("Parser: Declarations must have an identifier")),
    }

    match tokens[*index] {
        Token::Semicolon => *index += 1,
        _ => return Err(String::from("Parser: Declarations statements must end with a semicolon")),
    }

    code += &arrayNum;
    code += "\n";

    return Ok(code);
}


pub fn parse_assignment_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

  let mut code: String = String::new();
  let mut dest: String = String::new();
  let mut src: String = String::new();
  let mut varHold: String = String::new();

  let symbol_type = SymbolType::Variable; 

  match &tokens[*index] {
    Token::Ident(ident) => {
      //finding non declared variables 
      unsafe { 
        if !find_symbol(ident){
          return Err(format!("Semantic Analysis: Variable {ident} not declared before assignment"));
        }
      }
      *index += 1;
      varHold = ident.to_string();
    },
    _ => return Err(String::from("Parser: Assignment statements must begin with an identifier")),
  }




  // Support for array indexing assignment (e.g., arr[expression] = var)
  if tokens[*index] == Token::LeftBracket {
    *index += 1;


    if find_symbol_type(&varHold) != SymbolType::Array {
      return Err(format!("Semantic Analysis: {varHold} is not array"));
    }
    dest += "[";
    dest += &format!("{varHold} + ");

    match parse_expression(tokens, index) {
      Ok(expression) => {
        code += &expression.code;
        dest += &expression.name
      },
      Err(e) => return Err(e),
    }

    match tokens[*index] {
      Token::RightBracket => *index += 1,
      _ => return Err(String::from("Parser: Array assignments must have a closing bracket ']'")),
    }
    dest += "]";
  }


  else {dest += &varHold;}
  
  



  match tokens[*index] {
    Token::Assign => *index += 1,
    _ => return Err(String::from("Parser: Assignment statement is missing the '=' operator")),
  }


  match parse_expression(tokens, index) {
    Ok(expression) => {
      code += &expression.code; 
      src = expression.name;
    },
    Err(e) => return Err(e),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Assignment statements must end with a semicolon ';'")),
  }

  

  code += &format!("%mov {dest}, {src}\n");

  return Ok(code);
}


//%ret value
fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

  let mut code = String::new();
  
  let mut expr = Expression {
    code: String::new(),
    name: String::new(),
  };
  
  match tokens[*index] {
    Token::Return => {*index += 1;}
    _ => {return Err(String::from("Parser: Return statements must begin with a return keyword"));}
  }

  match parse_expression(tokens, index) {
    Ok(expression) => {
      expr = Expression {
        code: expression.code,
        name: expression.name,
      };

      code += &expr.code;
      let statement = expr.name; 
      code += &format!("%ret {statement}\n");

    },
    Err(e) => {return Err(e);}
  }

  match tokens[*index] {
    Token::Semicolon => {*index += 1;}
    _ => {return Err(String::from("Parser: Return statements must end with a semicolon"));}
  }

  return Ok(code);

}


pub fn parse_print_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

  let mut code = String::new();
  
  let mut expr = Expression {
    code: String::new(),
    name: String::new(),
  };


  match tokens[*index] {
    Token::Print => *index += 1,
    _ => return Err(String::from("Parser: Print statements must begin with the 'print' keyword")),
  }

  match parse_expression(tokens, index) {
    Ok(expression) => {
      expr = Expression {
        code: expression.code,
        name: expression.name,
      };

      code += &expr.code;
      let statement = expr.name; 


      code += &format!("%out {statement}\n");

    },
    Err(e) => {return Err(e);}
  }

  // Expect semicolon
  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Print statements must end with a semicolon ';'")),
  }

  return Ok(code);

}



pub fn parse_read_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

  let mut code = String::new();
  
  let mut expr = Expression {
    code: String::new(),
    name: String::new(),
  };


  match tokens[*index] {
    Token::Read => *index += 1,
    _ => return Err(String::from("Parser: Read statements must begin with the 'read' keyword")),
  }

  match parse_expression(tokens, index) {
    Ok(expression) => {
      expr = Expression {
        code: expression.code,
        name: expression.name,
      };

      code += &expr.code;
      let statement = expr.name; 
      code += &format!("%input {statement}\n");

    },
    Err(e) => {return Err(e);}
  }

  // Expect semicolon
  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Print statements must end with a semicolon ';'")),
  }

  return Ok(code);

}


pub fn parse_break_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

   unsafe {
        if !in_loop {
            return Err(String::from("Semantic Analysis: break statement not within a loop"));
        }
    }

  match tokens[*index] {
    Token::Break => *index += 1,
    _ => return Err(String::from("Parser: Expected 'break' keyword")),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Break statements must end with a semicolon ';'")),
  }



  unsafe {
        // Check if CURRENT_END_LABEL is empty
        if CURRENT_END_LABEL.is_empty() {
            return Err(String::from("Error: CURRENT_END_LABEL is empty."));
        }
        return Ok(format!("%jmp {CURRENT_END_LABEL}\n"));
  }
}


pub fn parse_continue_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {
  unsafe {
        if !in_loop {
            return Err(String::from("Error: continue statement not within a loop"));
        }
  }

  match tokens[*index] {
    Token::Continue => *index += 1,
    _ => return Err(String::from("Parser: Expected 'continue' keyword")),
  }

  match tokens[*index] {
    Token::Semicolon => *index += 1,
    _ => return Err(String::from("Parser: Continue statements must end with a semicolon ';'")),
  }

  unsafe {
    // Check if CURRENT_END_LABEL is empty
    if CURRENT_END_LABEL.is_empty() {
        return Err(String::from("Error: CURRENT_END_LABEL is empty."));
    }
    return Ok(format!("%jmp {CURRENT_END_LABEL}\n"));
  }
}


pub fn parse_while_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {

  let mut code = String::new(); 
  let mut condition = String::new();

  let end_label = create_label();
  let start_label = create_label();

  let mut hold_end_label = String::new();


  unsafe {
    in_loop = true; // Set to true when entering a loop
    hold_end_label = CURRENT_END_LABEL.clone();
    CURRENT_END_LABEL = end_label.clone(); 
  }

  match tokens[*index] {
    Token::While => *index += 1,
    _ => return Err(String::from("Parser: Expected 'while' keyword")),
  }

  code += &format!("{start_label}\n");

  match parse_bool(tokens, index) {
    Ok(expression) => {

      code += &expression.code; //creates code to make first temp in the expression
      condition = expression.name; //holds the temp for the boolean TAC 

    },
    Err(e) => return Err(e),
  }

  code += &format!("%branch_ifn {condition}, {end_label}\n");

  match tokens[*index] {
    Token::LeftCurly => *index += 1,
    _ => return Err(String::from("Parser: While statement execution code must begin with '{'")),
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index) {
      Ok(statement) => {code += &statement;},
      Err(e) => return Err(e),
    }
  }
  

  match tokens[*index] {
    Token::RightCurly => *index += 1,
    _ => return Err(String::from("Parser: While statement expects '}'")),
  }

  code += &format!("{end_label}\n");

  unsafe {
    if hold_end_label.is_empty(){
      in_loop = false; // Set to false when exiting the loop
    }
    CURRENT_END_LABEL = hold_end_label; 
  }

  return Ok(code);
}
  

  
  
pub fn parse_if_statement(tokens: &Vec<Token>, index: &mut usize) -> Result<String, String> {


  let mut code = String::new(); 
  let mut condition = String::new();

  let end_label = create_label();
  let false_label = create_label();


  


  match tokens[*index] {
    Token::If => *index += 1,
    _ => return Err(String::from("Parser: If statement expects 'if' keyword")),
  }

  match parse_bool(tokens, index) {
    Ok(expression) => {

      code += &expression.code; //creates code to make first temp in the expression
      condition = expression.name; //holds the temp for the boolean TAC 

    },
    Err(e) => {return Err(e);}
  }

  code += &format!("%branch_ifn {condition}, {false_label}\n");

  

  match tokens[*index] {
    Token::LeftCurly => *index += 1,
    _ => return Err(String::from("Parser: If statement execution code must begin with '{'")),
  }

  while !matches!(tokens[*index], Token::RightCurly) {
    match parse_statement(tokens, index) {
      Ok(statement) => {code += &statement;},
      Err(e) => return Err(e),
    }
  }

  match tokens[*index] {
    Token::RightCurly => *index += 1,
    _ => return Err(String::from("Parser: If statement expects '}'")),
  }


  code += &format!("{false_label}\n");

  if *index < tokens.len() && matches!(tokens[*index], Token::Else) {
    *index += 1;

    match tokens[*index] {
      Token::LeftCurly => *index += 1,
      _ => return Err(String::from("Parser: Else statement execution code must begin with '{'")),
    }

    while !matches!(tokens[*index], Token::RightCurly) {
      match parse_statement(tokens, index) {
        Ok(statement) => {code += &statement;},
        Err(e) => return Err(e),
      }
    }

    match tokens[*index] {
      Token::RightCurly => *index += 1,
      _ => return Err(String::from("Parser: Else statement expects '}' after else block")),
    }
  }

  code += &format!("{end_label}\n");


  return Ok(code);
}

  
pub fn parse_bool(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {

  
  let mut varHold = String::new();
  let mut statementHold = String::new();

  let mut expr = Expression {
    code: String::from(""),
    name: String::from(""),
  };
  
  


  match parse_expression(tokens, index) {
    Ok(expression) => {

      expr.code += &expression.code; //creates code to make first temp in the expression
      varHold = expression.name; //holds the temp for the boolean TAC 

    },
    Err(e) => {return Err(e);}
  }

  let t = create_temp(); //the destination of the generated code for boolean 

  expr.code += &format!("%int {t}\n");

  match tokens[*index] {
    Token::Less => {
        *index += 1;
        expr.code += "%lt ";
    }
    Token::LessEqual => {
        *index += 1;
        expr.code += "%le ";
    }
    Token::Greater => {
        *index += 1;
        expr.code += "%gt ";
    }
    Token::GreaterEqual => {
        *index += 1;
        expr.code += "%ge ";
    }
    Token::Equality => {
        *index += 1;
        expr.code += "%eq ";
    }
    Token::NotEqual => {
        *index += 1;
        expr.code += "%neq ";
    }
    _ => return Err(String::from("Parser: Boolean expression expects a comparison operator")),
  }

  


  statementHold += &format!("{t}, {varHold}, "); //still have to generate the code for the other part before we use this 


  match parse_expression(tokens, index) {
    Ok(expression) => {

      expr.code += &expression.code; //creates code to make first temp in the expression
      varHold = expression.name; //holds the temp for the boolean TAC 

    },
    Err(e) => {return Err(e);}
  }

  statementHold += &varHold; 

  expr.code += &statementHold;
  expr.code += "\n";

  expr.name = t;



  

  return Ok(expr);
}

  


struct Expression{
  code: String, //the code that creates temp/call/index to use in the expression 
  name: String, //the part that is used in TAC instructions 
}

// Parsing complex expressions such as: "a + b - (c * d) / (f + g - 8);
pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {

  let mut expr = parse_multiply_expression(tokens, index)?;

  loop {

    let opcode = match peek_error(tokens, index)?{
      Token::Plus => "%add",
      Token::Subtract => "%sub",
      _ => { break;}
    };

    *index += 1;

    let m_expr = parse_multiply_expression(tokens, index)?;
    let t = create_temp();
    let instr = format!("%int {}\n{opcode} {}, {}, {}\n", t, t, expr.name, m_expr.name);
    expr.code += &m_expr.code;
    expr.code += &instr;
    expr.name = t; 

  }

  return Ok(expr);

}

  
pub fn parse_multiply_expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
  
  let mut expr = parse_term(tokens, index)?;
  loop { 
    let opcode = match peek_error(tokens, index)?{ 
      Token::Multiply => "%mult",
      Token::Divide => "%div",
      Token::Modulus => "%mod",
      _ => {break;}

    };


    *index += 1;
    let node = parse_term(tokens, index)?;
    expr.code += &node.code;
    let t = create_temp();
    let instr = format!("%int {}\n{opcode} {}, {}, {}\n", t, t, expr.name, node.name);
    expr.code += &instr;
    expr.name = t; 
  }

  return Ok(expr);
}

  
  
pub fn parse_term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {

  match &tokens[*index] {
    
    // Handle identifier (e.g., variable, function call, or array access)
    
    
    Token::Ident(ident) => {

      *index += 1;

      unsafe { 
        if !find_symbol(ident){
          return Err(format!("Semantic Analysis: Variable {ident} not declared before use as term"));
        }
      }


      let mut expr = Expression {
        code: String::from(""),
        name: String::from(""),
      };

      // Handle function call: ident(...)
        //create a temp, load the value into the temp

        //c = add(a, b);
          // %call t, add(a,b) 
          // %mov c, t 

      if tokens[*index] == Token::LeftParen {
        *index += 1;
        
        //%call temp, funcname(variables)
          //generate the code for the variables, append the name to the call string 
        
        let t = create_temp();
        let mut callCode: String = String::new();
        callCode += &format!("%call {t}, {ident}");
        callCode += "(";


        // Parse the function's arguments
        while !matches!(tokens[*index], Token::RightParen) {
          

          match parse_expression(tokens, index) {
            Ok(expression) => {
              expr.code += &expression.code;
              callCode += &expression.name;
            },

            Err(e) => return Err(e),
          }

          // If there's a comma, continue parsing more arguments
          if tokens[*index] == Token::Comma {
            *index += 1; // Skip the comma
            callCode += ",";
          } else {
            break; // End function argument parsing
          }
        }

        // Check for closing parenthesis
        match tokens[*index] {
          Token::RightParen => *index += 1,
          _ => return Err(String::from("Parser: Function call expects closing parenthesis")),
        }

        callCode += ")\n";

        expr.code += &format!("%int {}\n", t);
        expr.code += &callCode;
        expr.name += &t;

        return Ok(expr);
        
      }



      // Handle array access: ident[expression]
      else if tokens[*index] == Token::LeftBracket {
        *index += 1;
        expr.name += &format!("[{ident} + ");
        

        match parse_expression(tokens, index) {
          Ok(expression) => {
            expr.code += &expression.code;
            expr.name += &expression.name;
          },
          Err(e) => return Err(e),
        }

        match tokens[*index] {
          Token::RightBracket => *index += 1,
          _ => return Err(String::from("Parser: Array access expects closing bracket ']'")),
        }

        expr.name += "]";

        let statement = expr.name;

        //fix print/return/etc array[0] error 
        let t = create_temp();
        expr.code += &format!("%int {t}\n");
        expr.code += &format!("%mov {t}, {statement}\n");
        expr.name = t; 


        return Ok(expr);
      }



      //if just a variable 
      else{

        if find_symbol_type(ident) != SymbolType::Variable {
          return Err(format!("Semantic Analysis: {ident} is not variable"));
        }

        let expr = Expression {
          code: String::from(""),
          name: ident.clone(),
        };
        return Ok(expr);
      }

    }

      
  
    

    //deprecated version for variable 
    /*
    Token::Ident(ident)=>{
      *index += 1;
      let expr = Expression {
        code: String::from(""),
        name: ident.clone(),
      };
      return Ok(expr);
    }
    */

    // Handle numbers directly
    Token::Num(num) => {
      *index += 1;
      let expr = Expression{
        code : String::from(""),
        name: format!("{num}"),
      };
      return Ok(expr);
    }


    // Handle parenthesized expressions: (expression)
    Token::LeftParen => {
      *index += 1;

      let mut expr = Expression {
        code: String::from(""),
        name: String::from(""),
      };

      match parse_expression(tokens, index) {
        Ok(expression) => {expr = expression;},
        Err(e) => return Err(e),
      }

      match tokens[*index] {
        Token::RightParen => *index += 1,
        _ => return Err(String::from("Parser: Expression expects closing parenthesis ')'")),
      }
      
      return Ok(expr);
    }
    

    // Handle invalid token
    _ => Err(String::from("Parser: Unexpected token, expected a term")),
  }
}


  
  