use std::collections::HashMap;
struct Department {
    department_list: HashMap<Worker, String>,
}

#[derive(Debug)]
pub struct Worker {
    pub name: String,
    pub department: String,
    pub position: String,
    pub salary: String,
}

impl Worker {
    pub fn print(&self) {
        println!("{:?}", self);
    }
}
