use std::error::Error;

#[derive(Debug)]
pub struct Todo {
    id: i32,
    name: String,
    details: String,
    complete: bool,
}

impl Todo {
    pub fn add(mut todos: Vec<Todo>, name: String) -> Result<(), Box<dyn Error>> {
        let id = match todos.last() {
            None => 1,
            Some(todo) => todo.id,
        };

        let todo = Todo {
            id,
            name,
            details: String::from(""),
            complete: false,
        };

        todos.push(todo);
        Ok(())
    }

    fn list(todos: Vec<Todo>) -> Result<(), Box<dyn Error>> {
        if todos.is_empty() {
            println!("There are no todos. You can add some now!");
        }

        todos.iter().for_each(|todo| todo.print());

        Ok(())
    }

    fn print(&self) {
        // 1: Take out trash
        print!("{}: {}", self.id, self.name);
    }
}

struct Project {
    name: String,
    todo: Vec<Todo>,
}

impl Project {
    fn global_project() {
        todo!("Start a global project where all")
    }
}
