#[macro_use]
extern crate rocket;

mod routes;

use routes::{index, js, favicon, add_task, delete_task, get_tasks, update_task, TaskList};
use std::sync::Mutex;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(TaskList::load())) // Ініціалізація спільного стану
        .mount("/", routes![index, js, favicon, get_tasks, add_task, delete_task, update_task])
}
