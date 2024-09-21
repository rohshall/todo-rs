# todo-rs

A simple to-do list app written in Rust.

## Features

Stores tasks in a text file `~/.todo` which can be easily edited, synced through cloud, etc.

## Usage
```
todo-rs [COMMAND] [ARGUMENTS]
```
The default command is `list`.

Available commands:
- add <task1> <task2>: adds new task(s)
Example:
```todo add \"write to-do app\" \"test it\"```
- list: lists all tasks along with their current indices.
Example:
```todo list```
- done <task-index1> <task-index2>: marks task as done
Example:
```todo done 2 3```
marks second and third tasks as completed
- clear: clears all done tasks
Example:
```todo clear```
