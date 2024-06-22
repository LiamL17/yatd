pub enum CLI {
    Add(String),
    List(),
    Remove(i32),
    RemoveAll(),
    Complete(i32),
    View(i32),
}

impl CLI {
    fn add(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        let name = match args.next() {
            None => todo!("We need a name here error message"),
            Some(name) => name,
        };

        Ok(CLI::Add(name))
    }

    fn remove(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        let id_string = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        let id = id_string.parse::<i32>().unwrap();

        Ok(CLI::Remove(id))
    }

    fn view(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        let id_string = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        let id = id_string.parse::<i32>().unwrap();

        Ok(CLI::View(id))
    }

    fn complete(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        let id_string = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        let id = id_string.parse::<i32>().unwrap();

        Ok(CLI::Complete(id))
    }

    fn list() -> Result<CLI, ()> {
        Ok(CLI::List())
    }

    fn remove_all() -> Result<CLI, ()> {
        Ok(CLI::RemoveAll())
    }

    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        args.next();
        // todo!("Validate that we have some args, or show usage if missing");

        let command = match args.next() {
            None => todo!("Again some error message, maybe the usage"),
            Some(arg) => arg,
        };

        match command.to_lowercase().as_str() {
            "add" | "a" => CLI::add(args),
            "list" | "l" => CLI::list(),
            "remove" | "r" => CLI::remove(args),
            "remove-all" | "ra" => CLI::remove_all(),
            "finish" | "f" | "complete" | "c" => CLI::complete(args),
            "view" | "v" => CLI::view(args),
            _ => Err(()),
        }
    }

    // fn usage() {
    //     todo!("Write the usage of the app here.")
    // }
}
