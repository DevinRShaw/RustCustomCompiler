

// used to get the commandline arguments from the commandline.
use std::env;

// used to interact with the file system
use std::fs;

use rustcompiler::phases::lexer::*;
use rustcompiler::phases::parser::*;

mod phases;

//The reading of file, command args and lex call are same as example 
fn main() {

    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lex.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) { //this is a rust style code block, keep that in mind 
      Err(error) => {
          println!("**Error. File \"{}\": {}", filename, error);
          return;
      }

      Ok(code) => {
          code
      } 
    };

    let tokens = match lex(&code) {
      Err(error_message) => {
          println!("**Error**");
          println!("----------------------");
          println!("{}", error_message);
          println!("----------------------");
          return;
      }

      Ok(data) => data,
      
      };
      


    // print out the lex tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("File Contents:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
      println!("{:?}", t);
    }

    //parser part added from phase 2 
    let mut index: usize = 0;
    match parse_program(&tokens, &mut index) {

    Ok(()) => {
        println!("Program Parsed Successfully.");
    }

    Err(message) => {
        println!("**Error**");
        println!("----------------------");
        if tokens.len() == 0 {
            println!("No code has been provided.");
        } else {
            println!("Error: {message}");
            println!("----------------------");
        }
    }

  }
}
