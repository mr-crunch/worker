use std::collections::HashMap;
use std::io;

struct Department {
    department_list: HashMap<Worker, String>,
}

#[derive(Debug)]
struct Worker {
    name: String,
    department: String,
    position: String,
    salary: String,
}

impl Worker {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    loop {
        println!("enter names:");
        let mut input = String::new();
        let result = get_input();
        match result {
            Ok(string) => input = string,
            Err(error) => eprintln!("error: {:?}", error),
        }
        let names = input.split_whitespace().collect::<Vec<&str>>();
        if names[0] == "exit" {
            break;
        }
        let mut employees = employee_list(names);
        add_department(&mut employees);
        for employee in &employees {
            employee.1.print();
        }
    }
}

fn get_input() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn employee_list(input: Vec<&str>) -> HashMap<String, Worker> {
    let mut employees = HashMap::new();
    for name in input {
        employees.entry(name.to_string()).or_insert(Worker {
            name: name.to_string(),
            department: String::from("na"),
            position: String::from("na"),
            salary: String::from("na"),
        });
    }
    employees
}

fn add_department(map: &mut HashMap<String, Worker>) {
    println!("enter name and department");
    println!("FORMAT: Add NAME to DEPARTMENT");
    let input = get_input();
    let mut input_list = String::new();
    match input {
        Ok(string) => input_list = string,
        Err(error) => eprintln!("error: {:?}", error),
    }
    let input_list = input_list.split(char::is_whitespace).collect::<Vec<_>>();
    let name = input_list[1].to_string();
    let department = input_list[3];
    match map.get_mut(&name) {
        Some(worker) => worker.department = department.to_string(),
        None => new_department(department),
    }
    //map.entry(name)
    //.and_modify(|worker| worker.department = department);
}

fn new_department(department: &str) {}
