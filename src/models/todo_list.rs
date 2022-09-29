use crate::models::Todo;
use serde::{ Serialize, Deserialize };


#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    name: String,
    list: Vec<Todo>
}

impl TodoList {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            list: vec!() 
        }
    }

    pub fn add_new_todo_item(&mut self, todo: Todo) {
        self.list.push(todo);
    }

    pub fn remove_todo_item(&mut self, todo_id: u32) {
        self.list.retain(|x| {
            x.id() != todo_id
        });
    }
}

#[test]
fn should_add_item() {
    let mut list = TodoList::new(String::from("test list"));
    list.add_new_todo_item(Todo::new(String::from("Item 1")));
    assert_eq!(list.list.len(), 1);
}

#[test]
fn should_remove_item() {
    let mut list = TodoList::new(String::from("test list"));
    let todo = Todo::new(String::from("Item 1"));
    let todo_id = todo.id();
    list.add_new_todo_item(todo);
    assert_eq!(list.list.len(), 1);
    list.remove_todo_item(todo_id);
    assert_eq!(list.list.len(), 0);
}

#[test]
fn should_not_panic_remove_item() {
    let mut list = TodoList::new(String::from("test list"));
    list.remove_todo_item(123);
}