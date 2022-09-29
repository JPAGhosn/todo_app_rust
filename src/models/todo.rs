use rand::{thread_rng, Rng};
use serde_json::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    title: String
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self { 
            id: thread_rng().gen_range(0..=10000),
            title 
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn modify_title(&mut self, title: String) {
        self.title = title;
    }
}