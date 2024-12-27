#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::response::content::RawHtml;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Serialize)]
struct Operation {
    operand1: f64,
    operand2: f64,
    operator: String,
}

#[derive(Debug, Serialize)]
struct Result {
    result: Option<String>,
    error: Option<String>,
}

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../static/index.html"))
}

#[post("/calculate", format = "json", data = "<operation>")]
fn calculate(operation: Json<Operation>, memory: &rocket::State<Mutex<f64>>) -> Json<Result> {
    let operand1 = operation.operand1;
    let operand2 = operation.operand2;
    let operator = &operation.operator;

    let result = match operator.as_str() {
        "+" => Some(operand1 + operand2),
        "-" => Some(operand1 - operand2),
        "*" => Some(operand1 * operand2),
        "/" => {
            if operand2 != 0.0 {
                Some(operand1 / operand2)
            } else {
                return Json(Result {
                    result: None,
                    error: Some("Cannot divide by zero".to_string()),
                });
            }
        }
        _ => None,
    };

    match result {
        Some(res) => {
            let mut mem = memory.lock().unwrap();
            *mem = res; // Store the result in memory
            Json(Result { result: Some(format!("{:.3}", res)), error: None })  // Format result to 3 decimal places
        }
        None => Json(Result { result: None, error: Some("Invalid operation".to_string()) }),
    }
}

#[get("/memory")]
fn get_memory(memory: &rocket::State<Mutex<f64>>) -> Json<Result> {
    let mem = memory.lock().unwrap();
    Json(Result { result: Some(format!("{:.3}", *mem)), error: None })  // Format memory value to 3 decimal places
}

#[post("/use_memory", format = "json")]
fn use_memory(memory: &rocket::State<Mutex<f64>>) -> Json<Operation> {
    let mem = memory.lock().unwrap();
    Json(Operation {
        operand1: *mem,
        operand2: 0.0,  // Default value for operand2
        operator: "+".to_string(),  // Default to addition with memory value
    })
}

#[post("/clear_memory")]
fn clear_memory(memory: &rocket::State<Mutex<f64>>) -> Json<Result> {
    let mut mem = memory.lock().unwrap();
    *mem = 0.0; // Clear memory
    Json(Result { result: Some(format!("{:.3}", *mem)), error: None })  // Format cleared memory value to 3 decimal places
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, calculate, get_memory, use_memory, clear_memory])
        .manage(Mutex::new(0.0))
}

