mod err;
pub use err::{ParseErr, ReadErr};

use json::JsonValue;
use std::{error::Error, fs};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id:          u32,
    pub description: String,
    pub level:       u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let data = fs::read_to_string(path)
            .map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;

        let parsed: JsonValue = json::parse(&data)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;

        let tasks_json = &parsed["tasks"];
        if tasks_json.len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        let tasks = tasks_json
            .members() // return iterator on a json array
            .map(|t| Task {
                id:          t["id"].as_u32().unwrap(),
                description: t["description"].to_string(),
                level:       t["level"].as_u32().unwrap(),
            })
            .collect();

        Ok(TodoList {
            title: parsed["title"].to_string(),
            tasks,
        })
    }
}
