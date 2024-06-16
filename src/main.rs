use std::env::args;
use yatd::Todo;

mod cli;
use cli::CLI;

fn main() {
    let todos: Vec<Todo> = Vec::new();
    let maybe_command = CLI::parse(args());

    let command = match maybe_command {
        Ok(result) => result,
        _ => panic!(""),
    };

    match command {
        CLI::Add(name) => Todo::add(todos, name),
        _ => panic!(""),
    };
}
