pub enum CLI {
    // Add(String, Option<String>),
    Add(String),
    List(),
    Remove(String),
    // RemoveAll(),
    Complete(String),
}

impl CLI {
    fn add(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        let name = match args.next() {
            None => todo!("We need a name here error message"),
            Some(name) => name,
        };

        Ok(CLI::Add(name))

        // let details = args.next();
        //
        // Ok(CLI::Add(name, details))
    }

    fn remove(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        let id_or_name = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        Ok(CLI::Remove(id_or_name))
    }

    fn complete(mut args: impl Iterator<Item = String>) -> Result<CLI, ()> {
        let id_or_name = match args.next() {
            None => todo!("We need a name here error message"),
            Some(arg) => arg,
        };

        Ok(CLI::Complete(id_or_name))
    }

    fn list() -> Result<CLI, ()> {
        Ok(CLI::List())
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
            "finish" | "f" | "complete" | "c" => CLI::complete(args),
            _ => Err(()),
        }
    }

    // fn usage() {
    //     todo!("Write the usage of the app here.")
    // }
}
