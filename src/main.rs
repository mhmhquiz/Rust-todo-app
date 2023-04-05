use std::io::{stdin, stdout, Write};

fn add_task(tasks: &mut Vec<String>) {
    let mut input = String::new();
    print!("enter task: ");
    stdout().flush().expect("Failed to flush");
    stdin().read_line(&mut input).expect("Failed to read line");
    let task = input.trim().to_string();
    tasks.push(task);
}

fn show_tasks(tasks: &Vec<String>) {
    for (i, task) in tasks.iter().enumerate() {
        println!("[task{}] {}", i + 1, task);
    }
}

fn main() {
    let mut tasks = Vec::new();

    loop {
        println!("1. Add task");
        println!("2. Show tasks");
        println!("3. Exit");

        let mut input = String::new();
        print!("> ");
        stdout().flush().expect("Failed to flush");
        stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(1) => add_task(&mut tasks),
            Ok(2) => show_tasks(&tasks),
            Ok(3) => break,
            _ => println!("Invalid input"),
        }
    }
}
