mod todo;
use std::env;
use todo::Todo;

const HELP: &str = "Usage: todo-rs [COMMAND] [ARGUMENTS]
todo is a simple to-do app.
Example: todo list (lists all tasks)
Available commands:
- add: adds new task/s
    Example: todo add \"write to-do app\" \"test it\"
- list: lists all tasks
    Example: todo list
- addpomo [index1] [index2]: adds pomodoros to tasks
    Example: todo addpomo 2 3 (adds pomodoros to second and third tasks)
- done [index1] [index2]: marks task as done
    Example: todo done 2 3 (marks second and third tasks as completed)
- clear: clears all done tasks
    Example: todo clear
";

fn main() {
    let todo = Todo::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = args[1].as_str();
        match command {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "done" => todo.done(&args[2..]),
            "addpomo" => todo.addpomo(&args[2..]),
            "clear" => todo.clear(),
            "--help" | "-h" | _ => println!("{}", HELP)
        }
    } else {
        println!("{}", HELP);
    }
}
