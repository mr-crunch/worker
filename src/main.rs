use std::io;
fn main() {
    loop {
        let mut input = String::new();
        let result = get_input();
        match result {
            Ok(string) => input = string,
            Err(error) => eprintln!("error: {:?}", error),
        }
        if input == "exit" {
            break;
        } else {
            let names = input.split_whitespace().collect::<Vec<&str>>();
            println!("input: {:?}", names);
        }
    }
}

fn get_input() -> Result<String, io::Error> {
    println!("enter input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
