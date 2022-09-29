#[derive(Debug)]
pub struct Todo {
    title: String
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self { 
            title 
        }
    }
}