use actix_cors::Cors;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::{params, prelude::Queryable, Pool};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

fn create_db_pool() -> Pool {
    let url = "mysql://root:kouki0911@localhost/todo";
    Pool::new(url).expect("Failed to create database pool")
}

fn add_task(pool: &Pool, tasks: &mut Vec<String>, task_str: &str) {
    let task = task_str.trim().to_string();
    let mut conn = pool.get_conn().expect("Failed to connect to database");
    conn.exec_drop(
        "INSERT INTO tasks (task) VALUES (:task)",
        params! {"task" => &task},
    )
    .expect("Failed to add task to database");
    tasks.push(task);
}

fn show_tasks(pool: &Pool, tasks: &mut Vec<String>) -> Vec<String> {
    let mut conn = pool.get_conn().expect("Failed to connect to database");
    let db_tasks: Vec<String> = conn
        .query("SELECT task FROM tasks")
        .expect("Failed to fetch tasks from database");

    tasks.clear();
    tasks.extend(db_tasks);
    tasks.clone()
}

fn remove_task(pool: &Pool, tasks: &mut Vec<String>, index: usize) {
    let task = &tasks[index - 1];
    let mut conn = pool.get_conn().expect("Failed to connect to database");
    conn.exec_drop(
        "DELETE FROM tasks WHERE task = :task",
        params! {"task" => task},
    )
    .expect("Failed to remove task from database");
    tasks.remove(index - 1);
}

#[derive(Serialize, Deserialize)]
struct Task {
    task: String,
}

struct AppState {
    pool: Mutex<Pool>,
    tasks: Mutex<Vec<String>>,
}

#[post("/add_task")]
async fn add_task_endpoint(app_state: web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    add_task(
        &app_state.pool.lock().unwrap(),
        &mut app_state.tasks.lock().unwrap(),
        &task.task,
    );
    HttpResponse::Ok().body("Task added")
}

#[get("/show_tasks")]
async fn show_tasks_endpoint(app_state: web::Data<AppState>) -> impl Responder {
    let tasks = show_tasks(
        &app_state.pool.lock().unwrap(),
        &mut app_state.tasks.lock().unwrap(),
    );
    HttpResponse::Ok().json(tasks)
}

#[delete("/remove_task/{index}")]
async fn remove_task_endpoint(
    app_state: web::Data<AppState>,
    index: web::Path<usize>,
) -> impl Responder {
    remove_task(
        &app_state.pool.lock().unwrap(),
        &mut app_state.tasks.lock().unwrap(),
        index.into_inner(),
    );
    HttpResponse::Ok().body("Task removed")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool();
    let tasks = Vec::new();

    let app_state = web::Data::new(AppState {
        pool: Mutex::new(pool),
        tasks: Mutex::new(tasks),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
				.app_data(app_state.clone())
.wrap(cors)
.service(add_task_endpoint)
.service(show_tasks_endpoint)
.service(remove_task_endpoint)
})
.bind("127.0.0.1:8080")?
.run()
.await
}
