use std::env::args;
use yatd::Todo;

mod cli;
use cli::Cli;

fn main() {
    let maybe_todos = get_todos_from_csv(String::from("todos.csv"));
    let todos = match maybe_todos {
        Ok(todos) => todos,
        Err(_) => panic!(),
    };
    let command = Cli::parse(args()).expect("Failed to parse arguments!");

    println!();

    let todos = match match command {
        Cli::Add(name) => Todo::add(todos, name),
        Cli::List() => Todo::list(todos),
        Cli::View(id) => Todo::view(todos, id),
        Cli::Remove(id) => Todo::remove(todos, id),
        Cli::Complete(id) => Todo::complete(todos, id),
        Cli::RemoveAll() => Todo::remove_all(todos),
    } {
        Ok(todos) => todos,
        Err(e) => panic!("{:?}", e),
    };

    println!();
    write_todos(String::from("todos.csv"), todos);
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn write_todos(file_name: String, todos: Vec<Todo>) {
    let maybe_wtr = csv::Writer::from_path(file_name);
    let mut wtr = match maybe_wtr {
        Ok(writer) => writer,
        Err(e) => panic!("{:?}", e),
    };

    for todo in todos {
        let _ = wtr.serialize(todo);
    }
    let _ = wtr.flush();
}

fn get_todos_from_csv(file_name: String) -> Result<Vec<Todo>> {
    let maybe_rdr = csv::Reader::from_path(file_name);
    let mut rdr = match maybe_rdr {
        Ok(reader) => reader,
        Err(e) => panic!("{:?}", e),
    };

    let iter = rdr.deserialize();
    let mut todos: Vec<Todo> = Vec::new();

    for iter_next in iter {
        let record: Todo = iter_next?;
        // dbg!("{:?}", record.borrow());
        todos.push(record);
    }

    Ok(todos)
}
