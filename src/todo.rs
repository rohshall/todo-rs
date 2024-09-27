extern crate dirs;
use std::fs;
use std::fs::File;
use std::io::{Write, BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use std::process;
use colored::Colorize;

pub struct Todo {
    todo_path: PathBuf
}

impl Todo {
    pub fn new() -> Todo {
        let mut todo_path = dirs::config_dir().unwrap();
        todo_path.push("todo-rs");
        fs::create_dir_all(&todo_path).unwrap();
        todo_path.push("config");
        Self { todo_path }
    }

    pub fn list(&self) {
        match File::open(&self.todo_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    let task = line.unwrap();
                    let pomos: usize = task[&task.len()-2..&task.len()-1].parse().expect("expected pomos to be a number");
                    let pomos = "🍅".repeat(pomos);
                    if &task[..3] == "[x]" {
                        println!("{:>2}. {} {}", index + 1, &task[4..&task.len()-4].strikethrough(), pomos);
                    } else {
                        println!("{:>2}. {} {}", (index + 1).to_string().yellow(), &task[4..&task.len()-4], pomos);
                    }
                }
            },
            Err(_) => {
                eprintln!("First add some tasks!");
            }
        }
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("add takes at least one argument!");
            process::exit(1);
        }
        let f = File::options().create(true).append(true).open(&self.todo_path).unwrap();
        let mut writer = BufWriter::new(f);
        for arg in args {
            let arg = arg.trim();
            if arg.is_empty() {
                continue;
            }
            writer.write_fmt(format_args!("[ ] {} (0)\n", arg)).unwrap();
        }
        writer.flush().unwrap();
    }

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("done takes at least one argument!");
            process::exit(1);
        }
        let mut done_tasks: Vec<usize> = Vec::new();
        for arg in args {
            let arg: usize = arg.trim().parse().expect("Invalid task index");
            done_tasks.push(arg);
        }
        match File::open(&self.todo_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut tmp_file_path = std::env::temp_dir();
                tmp_file_path.push("todo-rs-config");
                let tmp_file = File::create(&tmp_file_path).unwrap();
                let mut writer = BufWriter::new(tmp_file);
                for (index, line) in reader.lines().enumerate() {
                    let task_num = index + 1;
                    let task = line.unwrap();
                    if done_tasks.contains(&task_num) {
                        writer.write_fmt(format_args!("[x] {}\n", &task[4..])).unwrap();
                    } else {
                        writer.write_fmt(format_args!("{}\n", &task)).unwrap();
                    }
                }
                writer.flush().unwrap();
                fs::rename(tmp_file_path, &self.todo_path).unwrap();
            },
            Err(_) => {
                eprintln!("First add some tasks!");
            }
        }
    }

    pub fn addpomo(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("done takes at least one argument!");
            process::exit(1);
        }
        let mut pomos_tasks: Vec<usize> = Vec::new();
        for arg in args {
            let arg: usize = arg.trim().parse().expect("Invalid task index");
            pomos_tasks.push(arg);
        }
        match File::open(&self.todo_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut tmp_file_path = std::env::temp_dir();
                tmp_file_path.push("todo-rs-config");
                let tmp_file = File::create(&tmp_file_path).unwrap();
                let mut writer = BufWriter::new(tmp_file);
                for (index, line) in reader.lines().enumerate() {
                    let task_num = index + 1;
                    let task = line.unwrap();
                    if pomos_tasks.contains(&task_num) {
                        let pomos: usize = task[&task.len()-2..&task.len()-1].parse().expect("expected pomos to be a number");
                        writer.write_fmt(format_args!("{} ({})\n", &task[..&task.len()-4], pomos+1)).unwrap();
                    } else {
                        writer.write_fmt(format_args!("{}\n", &task)).unwrap();
                    }
                }
                writer.flush().unwrap();
                fs::rename(tmp_file_path, &self.todo_path).unwrap();
            },
            Err(_) => {
                eprintln!("First add some tasks!");
            }
        }
    }

    pub fn clear(&self) {
        match File::open(&self.todo_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut tmp_file_path = std::env::temp_dir();
                tmp_file_path.push("todo-rs-config");
                let tmp_file = File::create(&tmp_file_path).unwrap();
                let mut writer = BufWriter::new(tmp_file);
                for line in reader.lines() {
                    let task = line.unwrap();
                    if &task[..3] != "[x]" {
                        writer.write_fmt(format_args!("{}\n", &task)).unwrap();
                    }
                }
                writer.flush().unwrap();
                fs::rename(tmp_file_path, &self.todo_path).unwrap();
            },
            Err(_) => {
                eprintln!("First add some tasks!");
            }
        }
    }
}
