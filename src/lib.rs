pub mod types;
pub mod environment;

pub mod parser;
pub mod tokenizer;
pub mod interpreter;

pub mod wasm_lib;

use environment::Environment;

pub fn execute_line(env: &mut Environment, line: String)
{
    let tokens = match tokenizer::tokenize(line)
    {
        Ok(tokens) => tokens,
        Err(error) => {
            env.error = error.to_string();
            env.next_line += 1;
            return;
        }
    };

    let mut window = types::VecWindow::from(tokens);
    let line = match parser::parse_line(&mut window)
    {
        Ok(line) => line,
        Err(error) => {
            env.error = error.to_string();
            env.next_line += 1;
            return;
        }
    };

    if let Err(error) = interpreter::evaluate_line(env, &line)
    {
        env.error = error.to_string();
        return;
    }
}