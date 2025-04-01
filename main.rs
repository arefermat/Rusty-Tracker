
use std::io::{self, Write};


#[derive(Debug)]
enum IsDone {
    Done,
    Incomplete,
}


/// **Function to get user input and return it as a trimmed string**
fn get_user_input() -> String {
    let mut command_input = String::new();
    
    print!("todo//> ");
    io::stdout().flush().unwrap(); // Ensure the prompt appears before input

    io::stdin().read_line(&mut command_input).unwrap();
    command_input.trim().to_string() // Trim whitespace and return as String
}

/// **Function to get the next argument from an iterator, or print an error and restart loop**
fn get_next_arg<'a, I>(iter: &mut I, error_message: &str) -> String
where
    I: Iterator<Item = &'a str>,
{
    match iter.next() {
        Some(arg) => arg.to_string(),
        None => {
            println!("{}", error_message);
            get_user_input() // Restart input collection if missing argument
        }
    }
}

// Placeholder functions
fn new_assignment(name: String, due_date: String, time_estimate: i32) {
    println!("Creating new assignment '{}' due on {} with estimated {} minutes.", name, due_date, time_estimate);
}

fn view_assignment(name: String) {
    println!("Viewing assignment '{}'.", name);
}

fn mark_assignment(name: String, status: IsDone) {
    println!("Marking assignment '{}' as {:?}", name, status);
}

fn resources(subject: String) {
    println!("Providing resources for '{}'.", subject);
}

fn main() {
    loop {
        let input = get_user_input();
        let mut parts = input.split_whitespace();
        
        let command = match parts.next() {
            Some(cmd) => cmd.to_lowercase(),
            None => {
                println!("Invalid input. Please try again.");
                continue;
            }
        };

        match command.as_str() {
            "new" => {
                let name = get_next_arg(&mut parts, "Please provide an assignment name.");
                let due_date = get_next_arg(&mut parts, "Please provide a due date.");
                let time_estimate: i32 = match get_next_arg(&mut parts, "Please provide a time estimate.").parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Time estimate must be a valid number.");
                        continue;
                    }
                };

                new_assignment(name, due_date, time_estimate);
            }
            "view" => {
                let name = get_next_arg(&mut parts, "Please provide an assignment name.");
                view_assignment(name);
            }
            "mark" => {
                let name = get_next_arg(&mut parts, "Please provide an assignment name.");
                let status = match get_next_arg(&mut parts, "Please provide 'done' or 'incomplete'.").to_lowercase().as_str() {
                    "done" => IsDone::Done,
                    "incomplete" => IsDone::Incomplete,
                    _ => {
                        println!("Invalid status. Use 'done' or 'incomplete'.");
                        continue;
                    }
                };

                mark_assignment(name, status);
            }
            "resources" => {
                let subject = get_next_arg(&mut parts, "Please provide a subject.");
                resources(subject);
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid command. Try again."),
        }
    }
}

