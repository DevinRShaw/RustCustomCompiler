
// Include both lexer and parser modules
use rustcompiler::phases::lexer::*;
use rustcompiler::phases::parser::*;



#[test]
fn test_declaration_statements() {
    // Basic variable declaration
    let tokens = lex("int x;").unwrap();
    assert!(parse_declaration_statement(&tokens, &mut 0).is_ok());

    // Array declaration
    let tokens = lex("int[10] arr;").unwrap();
    assert!(parse_declaration_statement(&tokens, &mut 0).is_ok());

    // Invalid declarations
    let tokens = lex("int;").unwrap();
    assert!(parse_declaration_statement(&tokens, &mut 0).is_err());

    let tokens = lex("int[10];").unwrap();
    assert!(parse_declaration_statement(&tokens, &mut 0).is_err());

    let tokens = lex("int x").unwrap(); // Missing semicolon
    assert!(parse_declaration_statement(&tokens, &mut 0).is_err());
}

#[test]
fn test_assignment_statements() {
    // Simple assignment
    let tokens = lex("x = 10;").unwrap();
    assert!(parse_assignment_statement(&tokens, &mut 0).is_ok());

    // Array assignment
    let tokens = lex("arr[0] = 10;").unwrap();
    assert!(parse_assignment_statement(&tokens, &mut 0).is_ok());

    // Complex array index
    let tokens = lex("arr[i + 1] = x * 2;").unwrap();
    assert!(parse_assignment_statement(&tokens, &mut 0).is_ok());

    // Invalid assignments
    let tokens = lex("x = ;").unwrap();
    assert!(parse_assignment_statement(&tokens, &mut 0).is_err());

    let tokens = lex("arr[] = 10;").unwrap();
    assert!(parse_assignment_statement(&tokens, &mut 0).is_err());

    let tokens = lex("x = 10").unwrap(); // Missing semicolon
    assert!(parse_assignment_statement(&tokens, &mut 0).is_err());
}

#[test]
fn test_if_statements() {
    // Basic if statement
    let tokens = lex("if x < 10 { x = 20; }").unwrap();
    assert!(parse_if_statement(&tokens, &mut 0).is_ok());

    // If-else statement
    let tokens = lex("if x < 10 { x = 20; } else { x = 30; }").unwrap();
    assert!(parse_if_statement(&tokens, &mut 0).is_ok());

    // Complex condition
    let tokens = lex("if (x + 1) <= (y * 2) { x = 20; }").unwrap();
    assert!(parse_if_statement(&tokens, &mut 0).is_ok());

    // Invalid if statements
    let tokens = lex("if x < 10 x = 20; }").unwrap(); // Missing {
    assert!(parse_if_statement(&tokens, &mut 0).is_err());

    let tokens = lex("if x < 10 { x = 20;").unwrap(); // Missing }
    assert!(parse_if_statement(&tokens, &mut 0).is_err());

    let tokens = lex("if { x = 20; }").unwrap(); // Missing condition
    assert!(parse_if_statement(&tokens, &mut 0).is_err());
}

#[test]
fn test_while_statements() {
    // Basic while loop
    let tokens = lex("while x < 10 { x = x + 1; }").unwrap();
    assert!(parse_while_statement(&tokens, &mut 0).is_ok());

    // Complex condition
    let tokens = lex("while (x + 1) <= (y * 2) { x = x + 1; }").unwrap();
    assert!(parse_while_statement(&tokens, &mut 0).is_ok());

    // With break and continue
    let tokens = lex("while x < 10 { if x == 5 { break; } x = x + 1; }").unwrap();
    assert!(parse_while_statement(&tokens, &mut 0).is_ok());

    // Invalid while statements
    let tokens = lex("while x < 10 x = x + 1; }").unwrap(); // Missing {
    assert!(parse_while_statement(&tokens, &mut 0).is_err());

    let tokens = lex("while x < 10 { x = x + 1;").unwrap(); // Missing }
    assert!(parse_while_statement(&tokens, &mut 0).is_err());

    let tokens = lex("while { x = x + 1; }").unwrap(); // Missing condition
    assert!(parse_while_statement(&tokens, &mut 0).is_err());
}

#[test]
fn test_function_declarations() {
    // Basic function
    let tokens = lex("func main() { return 0; }").unwrap();
    assert!(parse_function(&tokens, &mut 0).is_ok());


    // Function with parameters
    let tokens = lex("func add(int a, int b) { return a + b; }").unwrap();
    println!("Lexed tokens: {:?}", tokens);

    let result = parse_function(&tokens, &mut 0);
    println!("Parsing result: {:?}", result);
    if let Err(e) = &result {
        println!("Parsing failed with error: {:?}", e);
    }

    assert!(result.is_ok());



    // Function with array parameter
    let tokens = lex("func process(int[10] arr) { return arr; }").unwrap();
    assert!(parse_function(&tokens, &mut 0).is_ok());

    // Invalid functions
    let tokens = lex("func { return 0; }").unwrap(); // Missing name
    assert!(parse_function(&tokens, &mut 0).is_err());

    let tokens = lex("func main { return 0; }").unwrap(); // Missing ()
    assert!(parse_function(&tokens, &mut 0).is_err());

    let tokens = lex("func main() { return 0;").unwrap(); // Missing }
    assert!(parse_function(&tokens, &mut 0).is_err());
}

#[test]
fn test_expressions() {
    // Test parse_expression
    let tokens = lex("a + b * c").unwrap();
    assert!(parse_expression(&tokens, &mut 0).is_ok());

    let tokens = lex("(a + b) * (c - d)").unwrap();
    assert!(parse_expression(&tokens, &mut 0).is_ok());

    let tokens = lex("a * b / c % d").unwrap();
    assert!(parse_expression(&tokens, &mut 0).is_ok());

    // Invalid expressions
    let tokens = lex("a +").unwrap(); // Missing right operand
    assert!(parse_expression(&tokens, &mut 0).is_err());

    let tokens = lex("a * (b + c").unwrap(); // Missing )
    assert!(parse_expression(&tokens, &mut 0).is_err());

    let tokens = lex("a[c!=b]").unwrap(); // Missing )
    assert!(parse_expression(&tokens, &mut 0).is_err());
}

#[test]
fn test_boolean_expressions() {
    let test_cases = vec![
        "x < y",
        "x <= y",
        "x > y",
        "x >= y",
        "x == y",
        "x != y",
        "(a + b) < (c * d)",
    ];

    for case in test_cases {
        let tokens = lex(case).unwrap();
        assert!(parse_bool(&tokens, &mut 0).is_ok());
    }

    // Invalid boolean expressions
    let tokens = lex("x <").unwrap(); // Missing right operand
    assert!(parse_bool(&tokens, &mut 0).is_err());

    let tokens = lex("x << y").unwrap(); // Invalid operator
    assert!(parse_bool(&tokens, &mut 0).is_err());
}

#[test]
fn test_io_statements() {
    // Print statements
    let tokens = lex("print x;").unwrap();
    assert!(parse_print_statement(&tokens, &mut 0).is_ok());

    let tokens = lex("print x + y;").unwrap();
    assert!(parse_print_statement(&tokens, &mut 0).is_ok());

    // Read statements
    let tokens = lex("read x;").unwrap();
    assert!(parse_read_statement(&tokens, &mut 0).is_ok());

    
    // Read statement with array indexing
    let tokens = lex("read arr[i];").unwrap();
    println!("Lexed tokens: {:?}", tokens);

    let mut index = 0;
    let result = parse_read_statement(&tokens, &mut index);

    println!("Parsing result: {:?}", result);

    if let Err(e) = &result {
        println!("Parsing failed with error: {:?}", e);
    }

    assert!(result.is_ok());
    


    // Invalid IO statements
    let tokens = lex("print;").unwrap(); // Missing expression
    assert!(parse_print_statement(&tokens, &mut 0).is_err());

    let tokens = lex("read;").unwrap(); // Missing variable
    assert!(parse_read_statement(&tokens, &mut 0).is_err());
}

#[test]
fn test_function_calls() {
    // Valid function calls
    let tokens = lex("foo();").unwrap();
    assert!(parse_expression(&tokens, &mut 0).is_ok());

    let tokens = lex("bar(x, y, z);").unwrap();
    assert!(parse_expression(&tokens, &mut 0).is_ok());

    let tokens = lex("compute(b, c[2], 5);").unwrap();
    assert!(parse_expression(&tokens, &mut 0).is_ok());

    let tokens = lex("compute(a + b, c * d);").unwrap();
    assert!(parse_expression(&tokens, &mut 0).is_ok());

    // Invalid function calls
    let tokens = lex("func;").unwrap(); // Missing parentheses
    assert!(parse_expression(&tokens, &mut 0).is_err());

    let tokens = lex("func(;").unwrap(); // Missing closing parenthesis
    assert!(parse_expression(&tokens, &mut 0).is_err());

    let tokens = lex("func()").unwrap(); // Missing semicolon
    assert!(parse_expression(&tokens, &mut 0).is_err());
}

#[test]
fn test_invalid_if_while_statements() {
    // Invalid if statements ending with };
    let tokens = lex("if (x < 10) { x = 20; }; ").unwrap();
    assert!(parse_if_statement(&tokens, &mut 0).is_err());

    let tokens = lex("if (x == 5) { y = 10; }; ").unwrap();
    assert!(parse_if_statement(&tokens, &mut 0).is_err());

    // Invalid while statements ending with };
    let tokens = lex("while (x < 10) { x = x + 1; }; ").unwrap();
    assert!(parse_while_statement(&tokens, &mut 0).is_err());

    let tokens = lex("while (y > 0) { y = y - 1; }; ").unwrap();
    assert!(parse_while_statement(&tokens, &mut 0).is_err());
}
