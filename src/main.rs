mod todo;
use std::env;
use std::io::Result;
use todo::Todo;

const HELP: &str = "Usage: todo-rs [COMMAND] [ARGUMENTS]
todo is a simple to-do app.
Example: todo (lists all tasks)
Available commands:
- add: adds new task/s
    Example: todo add \"write to-do app\" \"test it\"
- list: lists all tasks
    Example: todo list
- done [index1] [index2]: marks task as done
    Example: todo done 2 3 (marks second and third tasks as completed)
- clear: clears all done tasks
    Example: todo clear
";

fn main() -> Result<()> {
    let todo = Todo::new()?;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = args[1].as_str();
        match command {
            "list" => todo.list()?,
            "add" => todo.add(&args[2..])?,
            "done" => todo.done(&args[2..])?,
            "clear" => todo.clear()?,
            "--help" | "-h" | _ => println!("{}", HELP)
        }
    } else {
        todo.list()?;
    }
    Ok(())
}
