use std::env;
use std::fs;

use yoloxide::environment::{
    Environment
};

use yoloxide::tokenizer;
use yoloxide::parser;
use yoloxide::interpreter;

use yoloxide::types::VecWindow;

fn main()
{
    let args: Vec<String> = env::args().collect();

    // Take the first argument as a file path and read it for yolol code
    let yolol_code = fs::read_to_string(&args[1]).unwrap();

    println!("Original code:");
    println!("{}", yolol_code);

    let tokens = tokenizer::tokenize(yolol_code).expect("Tokenizer failure!");
    println!("Tokens:");
    println!("{:?}", tokens);

    let mut token_window = VecWindow::from(tokens);
    let program = parser::parse_program(&mut token_window).expect("Parser failure!");

    println!("AST:");
    for line in &program.0
    {
        println!("{:?}", line);
    }

    let mut test_env = Environment::new("Test Env");

    println!("Re-codified AST:");
    for line in &program.0
    {
        println!("{}", line);
        let eval_output = interpreter::evaluate_line(&mut test_env, &line);

        eval_output.unwrap_or_else(|error| {
            println!("{}", error);
        });
    }

    println!("\n{}", test_env);
}
