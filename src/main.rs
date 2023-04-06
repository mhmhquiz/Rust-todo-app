use mysql::{Pool, params, prelude::Queryable};
use std::io::{stdin, stdout, Write};

fn create_db_pool() -> Pool {
    let url = "mysql://username:pass@localhost/DBname";
    Pool::new(url).expect("Failed to create database pool")
}
fn add_task(pool: &Pool, tasks: &mut Vec<String>) {
    let mut input = String::new();
    print!("enter task: ");
    stdout().flush().expect("Failed to flush");
    stdin().read_line(&mut input).expect("Failed to read line");
    let task = input.trim().to_string();

    let mut conn = pool.get_conn().expect("Failed to connect to database");
    conn.exec_drop("INSERT INTO tasks (task) VALUES (:task)", params! {"task" => &task}).expect("Failed to add task to database");
    tasks.push(task);
}

fn show_tasks(pool: &Pool, tasks: &mut Vec<String>) {
    let mut conn = pool.get_conn().expect("Failed to connect to database");
    let db_tasks: Vec<String> = conn.query("SELECT task FROM tasks").expect("Failed to fetch tasks from database");

    tasks.clear();
    tasks.extend(db_tasks);

    for (i, task) in tasks.iter().enumerate() {
        println!("[task{}] {}", i + 1, task);
    }
}

fn remove_task(pool: &Pool, tasks: &mut Vec<String>, index: usize) {
    let task = &tasks[index - 1];
    let mut conn = pool.get_conn().expect("Failed to connect to database");
    conn.exec_drop("DELETE FROM tasks WHERE task = :task", params! {"task" => task}).expect("Failed to remove task from database");
    tasks.remove(index - 1);
}

fn main() {
    let pool = create_db_pool();
    let mut tasks = Vec::new();

    loop {
        println!("1. Add task");
        println!("2. Show tasks");
        println!("3. Remove task");
        println!("4. Exit");

        let mut input = String::new();
        print!("> ");
        stdout().flush().expect("Failed to flush");
        stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(1) => add_task(&pool, &mut tasks),
            Ok(2) => show_tasks(&pool, &mut tasks),
            Ok(3) => {
                let mut input = String::new();
                print!("Enter task index to remove: ");
                stdout().flush().expect("Failed to flush");
                stdin().read_line(&mut input).expect("Failed to read line");

                match input.trim().parse::<usize>() {
                    Ok(index) => remove_task(&pool, &mut tasks, index),
                    Err(_) => println!("Invalid input"),
                }
            },
            Ok(4) => break,
            _ => println!("Invalid input"),
        }
    }
}
