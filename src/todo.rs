use std::fs;
use std::fs::File;
use std::io::{Write, Result, BufRead, BufReader, BufWriter};
use std::env;
use std::process;

pub struct Todo {
    todo_path: String
}

impl Todo {
    pub fn new() -> Result<Todo> {
        let home = env::var("HOME").unwrap();
        let todo_path = format!("{}/.todo", &home);
        Ok(Self { todo_path })
    }

    pub fn list(&self) -> Result<()> {
        let file = File::open(self.todo_path.as_str())?;
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            let task = line.unwrap();
            println!("{:>2}. {}", index + 1, task);
        }
        Ok(())
    }

    pub fn add(&self, args: &[String]) -> Result<()> {
        if args.is_empty() {
            eprintln!("add takes at least one argument!");
            process::exit(1);
        }
        let f = File::options().append(true).open(self.todo_path.as_str())?;
        let mut writer = BufWriter::new(f);
        for arg in args {
            let arg = arg.trim();
            if arg.is_empty() {
                continue;
            }
            writer.write_fmt(format_args!("[ ] {}\n", arg))?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn done(&self, args: &[String]) -> Result<()> {
        if args.is_empty() {
            eprintln!("done takes at least one argument!");
            process::exit(1);
        }
        let mut done_tasks: Vec<usize> = Vec::new();
        for arg in args {
            let arg: usize = arg.trim().parse().expect("Invalid task index");
            done_tasks.push(arg);
        }
        let file = File::open(self.todo_path.as_str())?;
        let reader = BufReader::new(file);
        let tmp_file_path = format!("{}{}", self.todo_path.as_str(), ".tmp");
        let tmp_file = File::create(&tmp_file_path)?;
        let mut writer = BufWriter::new(tmp_file);
        for (index, line) in reader.lines().enumerate() {
            let task_num = index + 1;
            let task = line.unwrap();
            if done_tasks.contains(&task_num) {
                writer.write_fmt(format_args!("[x] {}\n", &task[4..]))?;
            } else {
                writer.write_fmt(format_args!("{}\n", &task))?;
            }
        }
        writer.flush()?;
        fs::rename(tmp_file_path.as_str(), self.todo_path.as_str())?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        let file = File::open(self.todo_path.as_str())?;
        let reader = BufReader::new(file);
        let tmp_file_path = format!("{}{}", self.todo_path.as_str(), ".tmp");
        let tmp_file = File::create(&tmp_file_path)?;
        let mut writer = BufWriter::new(tmp_file);
        for line in reader.lines() {
            let task = line.unwrap();
            if &task[..3] != "[x]" {
                writer.write_fmt(format_args!("{}\n", &task))?;
            }
        }
        writer.flush()?;
        fs::rename(tmp_file_path.as_str(), self.todo_path.as_str())?;

        Ok(())
    }
}

const TODO_HELP: &str = "Usage: todo-rs [COMMAND] [ARGUMENTS]
Todo is a simple to-do app.
Example: todo (lists all tasks)
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"write to-do app\" \"test it\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - clear
        clears all done tasks
        Example: todo clear
        ";
pub fn help() {
    println!("{}", TODO_HELP);
}
