pub mod models;
pub mod constants;
use std::io::{self, Write, BufWriter};
use std::path::{Path};
use std::fs::{self, File, OpenOptions};
pub use constants::PATH;
use models::Todo;
use crate::models::TodoList;
use serde_json::{self, json};
use serde_json::Result;

pub fn init_file() {
    // Check if the file exists
    let exists = Path::new(PATH).exists();

    // Create the file db if it doesn't exists
    if exists == false {
        File::create(PATH).expect("Cannot create this file.");
    }

    // Read the content of the file
    let content = fs::read_to_string(PATH).unwrap();

    // If the content of the file is empty, add the initial information
    if content == "" {
        let todo_lists = TodoList::new(String::from("Default list"));
        write_to_do_list(&todo_lists);
    }
}

pub fn write_to_do_list(todo_lists: &TodoList) {
    // convert the todo lists into a json string
    let json_object = serde_json::to_string_pretty(&todo_lists).unwrap() as String;
    // Write to the file
    let f = OpenOptions::new()
        .append(true)
        .open(PATH)
        .expect("Unable to open file");
    let mut f = BufWriter::new(f);
    f.write_all(json_object.as_bytes()).expect("Unable to write data");
}

pub fn get_to_do_list() -> TodoList {
    let contents = fs::read_to_string(PATH).expect("Cannot read from this file.");
    let list: TodoList = serde_json::from_str(&contents).expect("Cannot deserialize to todo list");
    list
}

 