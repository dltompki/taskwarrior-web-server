#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::process::Command;
use std::str;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value, from_value};
use chrono::{DateTime, Utc};
use humantime::format_duration;

#[get("/")]
fn index() -> &'static str {
    "You have reached the Taskwarrior Web Server!"
}

async fn export() -> Vec<Value> {
    let output = Command::new("/run/current-system/sw/bin/task")
        .arg("export")
        .arg("next")
        .output()
        .expect("failed to execute process");

    let json_string = str::from_utf8(&output.stdout).unwrap_or("Failed to capture output").to_string();
    from_str(&json_string).expect("invalid JSON")
}

#[get("/tasks")]
async fn tasks() -> Json<Vec<Value>> {
    Json(export().await)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WidgetTask {
    description: String,
    #[serde(default, with = "date_format")]
    due: Option<DateTime<Utc>>,
    project: Option<String>
}

mod date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer, de};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date_str: Option<String> = Option::deserialize(deserializer)?;
        match date_str {
            Some(s) => match NaiveDateTime::parse_from_str(&s, "%Y%m%dT%H%M%SZ") {
                Ok(dt) => Ok(Some(dt.and_utc())),
                Err(e) => {
                    Err(de::Error::custom(e))
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
async fn widget() -> Json<Vec<String>> {
    let lines: Vec<String> = from_value::<Vec<WidgetTask>>(Value::Array(export().await))
        .expect("Value could not be translated")
        .into_iter()
        .take(8)
        .flat_map(|task| {
            vec![
            {
                let due_date = match task.due {
                    None => "".to_string(),
                    Some(s) => {
                        let now = Utc::now();
                        let duration = s.signed_duration_since(now).to_std();
                        match duration {
                            Ok(d) => "in ".to_owned() + &format_duration(d).to_string().split_whitespace().next().unwrap(),
                            Err(_) => {
                                let reverse_duration = now.signed_duration_since(s).to_std();
                                match reverse_duration {
                                    Ok(d) => format_duration(d).to_string().split_whitespace().next().unwrap().to_owned() + " ago",
                                    Err(e) => e.to_string()
                                }
                            }
                        }
                    }
                };
                let project = &task.project.unwrap_or("".to_string());
                let spaces = 45 - due_date.len() - project.len();
                due_date + &" ".repeat(spaces) + project
            },
                task.description
            ]
        })
    .collect();

    Json(lines)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,tasks,widget])
}
