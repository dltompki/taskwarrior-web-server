#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::process::Command;
use std::str;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use chrono::{DateTime, Utc};

#[get("/")]
fn index() -> &'static str {
    "You have reached the Taskwarrior Web Server!"
}

async fn export() -> String {
    let output = Command::new("task")
        .arg("export")
        .output()
        .expect("failed to execute process");

    str::from_utf8(&output.stdout).unwrap_or("Failed to capture output").to_string()
}

#[get("/tasks")]
async fn tasks() -> Json<String> {
    Json(export().await)
}

#[derive(Debug, Serialize, Deserialize)]
struct WidgetTask {
    description: String,
    #[serde(with = "date_format")]
    due: Option<DateTime<Utc>>
}

mod date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer, de};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_str: Option<String> = Option::deserialize(deserializer)?;
        match date_str {
            Some(s) => match DateTime::parse_from_rfc3339(&s) {
                Ok(dt) => Ok(Some(dt.with_timezone(&Utc))),
                Err(e) => {
                    println!("{}", e);
                    println!("{}", s);
                    Err(de::Error::invalid_length(s.len(), &"a valid RFC 3339 date"))
                },
            },
            None => Ok(None),
        }
    }

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_str(&dt.to_rfc3339()),
            None => serializer.serialize_none(),
        }
    }
}

#[get("/widget")]
async fn widget() -> Json<String> {
    let tasks: Vec<WidgetTask> = from_str(&export().await).expect("JSON was not well formatted");

    Json(to_string(&tasks).expect("Failed to convert to JSON"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,tasks,widget])
}
