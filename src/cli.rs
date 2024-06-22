pub enum Cli {
    Add(String),
    List(),
    Remove(i32),
    RemoveAll(),
    Complete(i32),
    View(i32),
}

impl Cli {
    fn add(mut args: impl Iterator<Item = String>) -> Result<Cli, ()> {
        let name = match args.next() {
            None => todo!("We need a name here error message"),
            Some(name) => name,
        };

        Ok(Cli::Add(name))
    }

    fn remove(mut args: impl Iterator<Item = String>) -> Result<Cli, ()> {
        let id_string = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        let id = id_string.parse::<i32>().unwrap();

        Ok(Cli::Remove(id))
    }

    fn view(mut args: impl Iterator<Item = String>) -> Result<Cli, ()> {
        let id_string = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        let id = id_string.parse::<i32>().unwrap();

        Ok(Cli::View(id))
    }

    fn complete(mut args: impl Iterator<Item = String>) -> Result<Cli, ()> {
        let id_string = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        let id = id_string.parse::<i32>().unwrap();

        Ok(Cli::Complete(id))
    }

    fn list() -> Result<Cli, ()> {
        Ok(Cli::List())
    }

    fn remove_all() -> Result<Cli, ()> {
        Ok(Cli::RemoveAll())
    }

    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Cli, ()> {
        args.next();
        // todo!("Validate that we have some args, or show usage if missing");

        let command = match args.next() {
            None => todo!("Again some error message, maybe the usage"),
            Some(arg) => arg,
        };

        match command.to_lowercase().as_str() {
            "add" | "a" => Cli::add(args),
            "list" | "l" => Cli::list(),
            "remove" | "r" => Cli::remove(args),
            "remove-all" | "ra" => Cli::remove_all(),
            "finish" | "f" | "complete" | "c" => Cli::complete(args),
            "view" | "v" => Cli::view(args),
            _ => Err(()),
        }
    }

    // fn usage() {
    //     todo!("Write the usage of the app here.")
    // }
}
