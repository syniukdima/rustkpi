use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::fs::NamedFile;
use rocket::State;
use std::path::Path;
use std::fs;
use std::io::Write;
use std::sync::Mutex;


#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn load() -> TaskList {
        match fs::read_to_string("data/tasks.json") {
            Ok(data) => serde_json::from_str(&data).unwrap_or(TaskList { tasks: vec![] }),
            Err(_) => TaskList { tasks: vec![] },
        }
    }

    pub fn save(&self) {
        let json_data = serde_json::to_string(self).unwrap();
        let mut file = fs::File::create("data/tasks.json").unwrap();
        file.write_all(json_data.as_bytes()).unwrap();
    }
}

type SharedTaskList = Mutex<TaskList>;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

#[get("/app.js")]
pub async fn js() -> Option<NamedFile> {
    NamedFile::open("static/app.js").await.ok()
}

#[get("/favicon.ico")]
pub async fn favicon() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/favicon.ico")).await.ok()
}

#[get("/tasks")]
pub fn get_tasks(task_list: &State<SharedTaskList>) -> Json<TaskList> {
    let tasks = task_list.lock().unwrap();
    Json(tasks.clone())
}

#[post("/add", format = "application/json", data = "<task>")]
pub fn add_task(task_list: &State<SharedTaskList>, task: Json<Task>) -> Json<TaskList> {
    let mut tasks = task_list.lock().unwrap();
    tasks.tasks.push(task.into_inner());
    tasks.save();
    Json(tasks.clone())
}

#[post("/delete/<id>")]
pub fn delete_task(task_list: &State<SharedTaskList>, id: u64) -> Json<TaskList> {
    let mut tasks = task_list.lock().unwrap();
    tasks.tasks.retain(|task| task.id != id);
    tasks.save();
    Json(tasks.clone())
}

#[post("/update/<id>", data = "<updated_task>")]
pub fn update_task(
    task_list: &State<SharedTaskList>,
    id: u64,
    updated_task: Json<Task>,
) -> Json<TaskList> {
    let mut tasks = task_list.lock().unwrap();
    if let Some(task) = tasks.tasks.iter_mut().find(|task| task.id == id) {
        *task = updated_task.into_inner();
    }
    tasks.save();
    Json(tasks.clone())
}
