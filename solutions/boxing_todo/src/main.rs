use std::{fs::File, io::Write};

use boxing_todo::TodoList;

fn main() {
    let files = [
        (
            "todo.json",
            r#"{
                "title" : "TODO LIST FOR PISCINE RUST",
                "tasks": [
                    { "id": 0, "description": "do this", "level": 0 },
                    { "id": 1, "description": "do that", "level": 5 }
                ]
            }"#,
        ),
        (
            "todo_empty.json",
            r#"{
                "title" : "TODO LIST FOR PISCINE RUST",
                "tasks": []
            }"#,
        ),
        (
            "malformed_object.json",
            r#"{
                "something": ,
            }"#,
        ),
    ];

    for (name, content) in files {
        File::create(name)
            .unwrap()
            .write(content.as_bytes())
            .unwrap();

        let todos = TodoList::get_todo(name);
        match todos {
            Ok(list) => println!("{:?}", list),
            Err(e) => {
                println!("{}: {:?}", e.to_string(), e.source());
            }
        }
    }
}