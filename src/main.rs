use std::env;
use std::io::Result;
mod todo;
use todo::{Todo, help};

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
            "help" | _ => help()
        }
    } else {
        todo.list()?;
    }
    Ok(())
}
