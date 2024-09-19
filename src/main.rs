#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::process::Command;
use std::str;

#[get("/")]
fn index() -> &'static str {
    "You have reached the Taskwarrior Web Server!"
}

#[get("/tasks")]
async fn tasks() -> Json<String> {
    let output = Command::new("task")
        .arg("export")
        .output()
        .expect("failed to execute process");

    let output_str = str::from_utf8(&output.stdout).unwrap_or("Failed to capture output");

    Json(output_str.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,tasks])
}
