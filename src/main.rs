mod employee_management {
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct Department {
        name: String,
        employees: HashMap<&'static str, Worker>,
    }
    #[derive(Debug)]
    pub struct Worker {
        employee_name: String,
        department: String,
        position: String,
    }
    mod personal_info {}

    pub mod company_info {
        use crate::employee_management::{Department, Worker};
        use std::collections::HashMap;
        use std::io;

        //pub fn list_names_in_department {}

        //pub fn list_all_employees {}

        pub fn get_names(input: &str) -> Vec<&str> {
            input.split(char::is_whitespace).collect::<Vec<&str>>()
        }

        pub fn add_names(input: String) -> HashMap<String, Worker> {
            let employee_names = get_names(&input);
            println!("{:?}", employee_names);
            let mut employee_info: HashMap<String, Worker> = HashMap::new();
            for name in employee_names {
                employee_info.entry(name.to_string()).or_insert(Worker {
                    employee_name: name.to_string(),
                    department: String::from("Na"),
                    position: String::from("Na"),
                });
            }
            employee_info
        }
        fn add_department(map: HashMap<String, Worker>, names: String) -> HashMap<String, Worker> {
            let mut input = String::new();
            let employee_info = map;
            println!("enter worker name and department FORMAT: Add NAME to DEPT");
            if io::stdin().read_line(&mut input).is_ok() {
                let mut input = input
                    .trim()
                    .split(char::is_whitespace)
                    .collect::<Vec<&str>>();
                let name = input.swap_remove(1);
                let department = input.swap_remove(3);
            } else {
                println!("could not read input");
            }
            employee_info
        }
    }
}

use crate::employee_management::{company_info, Department, Worker};
use std::collections::HashMap;
use std::io;
fn main() {
    loop {
        let res = get_input();

        if input == "exit" {
            break;
        }
        let employee_info = company_info::add_names(input);

        //employee_info = add_department(employee_info, input);
        println!("{:?}", employee_info);
    }
}

fn get_input() -> String {
    let mut input = String::new();
    println!("type exit to kill");
    println!("enter worker names");
    println!("FORMAT: Name1 Name2 ... Namen");
    let res = io::stdin().read_line(&mut input);
    input = match res {
        Ok(..) => input.trim().to_string(),
        Err(error) => match error.kind() {
            Ok()
            Err()
        }
    }
}
