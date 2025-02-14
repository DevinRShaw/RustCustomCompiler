# Teh Terik Compiler

The Teh Terik Compiler is a basic compiler targeting a toy language called Teh Terik. This project serves as an exercise in compiler design and Rust development.

## Features

- Error Handling: Includes error handling for lexing and parsing.
- Phased Architecture: Separate modules for lexing, parsing, and other compilation stages.
- Custom Toy Language: Compiles a simple, toy language with basic functionality.

## Project Structure

- `src/`:
  - `lexer.rs`: Handles the lexing phase, converting input text into tokens.
  - `parser.rs`: Handles the parsing phase via recursive descent, processing tokens into an abstract syntax tree (AST).
  - `phases/`: Contains files that implement various stages of the compiler, corresponding to lexing, parsing, and other necessary phases.
  
- `tests/`: Contains test files that check functionality for different phases of the compiler. Each phase has a suite of tests to ensure correctness.

- `examples/`: Contains example `.tt` files. You can run the compiler on any of these files to see the compiler in action.

## Setup and Installation

1. Clone the repository: git clone <repo_url>

2. Ensure that you have Rust installed in your environment.

3. Navigate to the project directory and run the compiler: cargo run -- <path_to_file.tt>

This will compile the specified `.tt` file.

## Testing

The project includes tests for different phases of the compiler. You can run the tests with the following command: cargo test


The tests are organized by phase (e.g., lexer, parser) in the `tests/` directory.

## Examples

You can run the compiler on any `.tt` file located in the `examples/` directory. To do so, run the following script: ./run_examples.sh


This will execute the compiler on all `.tt` files in the `examples/` folder and display the results.

## Contributions

This project is not open for contributions at the moment. However, feel free to ask questions or provide recommendations.



