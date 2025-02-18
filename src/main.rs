use std::collections::HashMap;
use std::io;

struct Worker {
    name: String,
    department: String,
    position: String,
    salary: String,
}

impl Worker {
    fn print(&self) {
        println!(
            "name: {} department: {}, position: {} salary: {}",
            self.name, self.department, self.position, self.salary
        );
    }
}

fn main() {
    loop {
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
        let employees = employee_list(names);
        for employee in employees {
            employee.1.print()
        }
    }
}

fn get_input() -> Result<String, io::Error> {
    println!("enter names: ");
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
