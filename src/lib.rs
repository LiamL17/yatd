use colored::Colorize;
use std::error::Error;
use std::io;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Todo {
    id: i32,
    name: String,
    details: String,
    complete: bool,
}

impl Todo {
    pub fn add(mut todos: Vec<Todo>, name: String) -> Result<Vec<Todo>, Box<dyn Error>> {
        let id = match todos.last() {
            None => 1,
            Some(todo) => todo.id + 1,
        };

        let todo = Todo {
            id,
            name,
            details: String::from(""),
            complete: false,
        };

        print!("{}: ", "Added".blue());
        todo.print();

        todos.push(todo);

        Ok(todos)
    }

    pub fn list(todos: Vec<Todo>) -> Result<Vec<Todo>, Box<dyn Error>> {
        if todos.is_empty() {
            println!("There are no todos. You can add some now!");
            return Ok(todos);
        }

        todos.iter().for_each(|todo| todo.print());

        Ok(todos)
    }

    pub fn remove_all(todos: Vec<Todo>) -> Result<Vec<Todo>, Box<dyn Error>> {
        println!("Are you sure you want to remove all todos? y/N");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let maybe_char = buffer.chars().next();
        let answer = match maybe_char {
            None => panic!("Could not find a character entered!"),
            Some(answer) => answer,
        };

        if answer.to_lowercase().to_string().eq("y") {
            print!("{}!", "Removing all todos".red());
            Ok(Vec::new())
        } else {
            Ok(todos)
        }
    }

    pub fn view(todos: Vec<Todo>, id: i32) -> Result<Vec<Todo>, Box<dyn Error>> {
        let maybe_todo = todos.iter().find(|x| x.id == id);
        match maybe_todo {
            None => panic!("{}: {}", "Could not find todo with id".red(), id),
            Some(todo) => {
                print!("{}: ", "Viewing".blue());
                todo.print()
            }
        }

        Ok(todos)
    }

    pub fn remove(mut todos: Vec<Todo>, id: i32) -> Result<Vec<Todo>, Box<dyn Error>> {
        let maybe_removed = todos.iter().find(|x| x.id == id);
        match maybe_removed {
            None => panic!("{}: {}", "Could not find todo with id".red(), id),
            Some(todo) => {
                print!("{}: ", "Removed".red());
                todo.print()
            }
        }
        todos.retain(|x| x.id != id);

        Ok(todos)
    }

    pub fn complete(mut todos: Vec<Todo>, id: i32) -> Result<Vec<Todo>, Box<dyn Error>> {
        if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
            todo.complete = true;
            print!("{}: ", "Completed".green());
            todo.print()
        }

        Ok(todos)
    }

    pub fn print(&self) {
        // 1: Take out trash
        println!("{:?}", self);
    }
}

struct _Project {
    name: String,
    todo: Vec<Todo>,
}

// impl _Project {
// fn global_project() {
// todo!("Start a global project where all")
// }
// }
