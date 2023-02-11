pub mod cmds {
    use std::fs::File;
    use std::io::{ BufReader, prelude::* };
    use std::os::unix::fs::FileExt;

    //########   Remove   #########
    pub fn remove() {
        let file = File::options()
            .read(true)
            .append(true)
            .create(true)
            .open("todos.txt")
            .expect("Failed to remove todos that are completed.");
        let todos = BufReader::new(&file)
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| &l[..3] == "...")
            .collect::<Vec<String>>();

        let result = File::create("todos.txt")
            .expect("Failed to remove completed todos.")
            .write(
                format!(
                    "{}\n",
                    todos.join("\n")
                ).as_bytes()
            );

        match result.is_ok() {
            true => println!("Removed all completed todos."),
            false => println!("Failed to remove completed todos.")
        }
    }
    
    //########    List    #########
    pub fn list(lim : Option<String>) {
        let lim : usize = lim
            .unwrap_or( "10".to_string() )
            .parse::<usize>()
            .unwrap();

        let file = File::options()
            .read(true)
            .append(true)
            .create(true)
            .open("todos.txt")
            .expect("Failed to load todos.");
        let lines = BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .take_while(|l| l.len() != 0);

        println!("Oldest {} todos are:", lim);
        println!("-----------------------");
    
        for (i, line) in lines.enumerate() {
            if i == lim {
                break;
            }

            let mut data = line.split(" ");

            let stat = data.next().unwrap();
            let cnt = data
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{}   {}. {}", stat, i+1, cnt);
        }
    }

    //########    Help    #########
    pub fn help(cmd : Option<String>) {
        if cmd.is_some() {
            match cmd.unwrap().as_str() {
                "add" => println!("Adds a new Note. Syntax: 'add \"new todo within quotes\""),
                "list" => println!("Lists the todos. Syntax: 'list [Optional limit as number]'"),
                "help" => println!("Lists all commands available. Syntax: 'help [Optional command name]'"),
                "mark" => println!("Marks the mentioned todo as completed. Syntax: 'mark [todo index]' where index is the number when you list todos."),
                "remove" => println!("Removes all completed todos. Syntax: 'remove'"),
                name => println!("No command named '{}' found.", name)
            }
            return ();
        }
        
        println!("{}",
            [
                "Note Maker in CLI - Help command",
                "__________________",
                "  List of Commands:",
                "    list         List all todos",
                "    add          Add a new todo",
                "    mark         Mark or unmark a todo done",
                "    remove       Remove a todo",
                "    help         Show help about a command",
                "__________________",
                " Run 'help [command_name]' for syntax and how to use commands."
            ].join("\n")
        );
    }

    //########    Add    #########
    pub fn add(cnt : Option<String>) {
        let cnt = cnt.unwrap();

        let mut file = File::options()
            .read(true)
            .append(true)
            .create(true)
            .open("todos.txt")
            .unwrap();

        let result = writeln!(
            file,
            "... {cnt}"
        );

        match result.is_ok() {
            true => println!("Note added."),
            false => println!("Failed to add a todo.")
        };
    }

    //########    Mark    #########
    pub fn mark(id : Option<String>) {
        let id : usize = id.unwrap().parse().unwrap();

        let file = File::options()
            .read(true)
            .write(true)
            .open("todos.txt")
            .unwrap();
        let lines = BufReader::new(&file)
            .lines()
            .map(|l| l.unwrap())
            .take_while(|l| l.len() != 0);

        let mut start_idx : usize = 0;

        for (i, line) in lines.enumerate() {
            if i+1 == id {
                let to = match &line[..3] {
                    "..." => {
                        println!("Marking todo as done: {}", line);
                        "xxx"
                    },
                    "xxx" => {
                        println!("Marking todo as undone: {}", line);
                        "..."
                    },
                    _ => panic!("Failed to parse todo as done or undone.")
                };

                let result = file.write_at(
                    to.as_bytes(),
                    (start_idx+i) as u64
                );
                match result.is_ok() {
                    true => println!("Marked todo as done!"),
                    false => println!("Failed to mark todo as done.")
                };

                break;
            } else {
                start_idx += line.len();
            }
        }
    }
}
