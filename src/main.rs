use std::io::{self, Write, stdout};
use std::collections::HashMap;
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;


#[derive(Serialize, Deserialize)]
struct Assignment {
    name : String,
    due_date : String,
    status : bool,
}


//Creates a HashMap and inserts values for color codes to control text color
fn get_colors() -> HashMap<&'static str, &'static str> {
    let mut colors = HashMap::new();
    colors.insert("black", "\x1b[30m");
    colors.insert("red", "\x1b[31m");
    colors.insert("green", "\x1b[32m");
    colors.insert("yellow", "\x1b[33m");
    colors.insert("blue", "\x1b[34m");
    colors.insert("magenta", "\x1b[35m");
    colors.insert("cyan", "\x1b[36m");
    colors.insert("white", "\x1b[37m");
    colors.insert("reset", "\x1b[0m"); 
    colors
}

fn clear_terminal() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();
}

fn help() {
    println!("new, edit, view, mark, remove. Type cmdhelp command to see how to use that specific command, e.g. cmdhelp new");
}

fn cmdhelp(command: &str) {
    match command {
        "new" => println!("new ASSIGNMENT_NAME (one word) DUE_DATE (one word) STATUS_OF_ASSIGNMENT ('done' or 'incomplete')"),
        "edit" => println!("edit ASSIGNMENT_NAME (existing assignment) THING_TO_EDIT ('name' or 'due_date') CHANGE_TO (pretty self explanatory)"),
        "view" => println!("view (view all assignments)"),
        "mark" => println!("mark ASSIGNMENT_NAME (existing assignment) STATUS_OF_ASSIGNMENT ('done' or 'incomplete')"),
        "remove" => println!("remove ASSIGNMENT_NAME (existing assignment)"),
        _ => println!("That command does not exist, so I literally cannot help you bud. :|")
    }
}

//Takes input and trims whitespace
fn get_command() -> String {
    let mut command_input = String::new();
    
    print!("<todo> ");
    io::stdout().flush().unwrap(); 

    io::stdin().read_line(&mut command_input).unwrap();
    command_input.trim().to_string() // "         hello world     " -> "hello world"
}

// Takes a iterator and takes the next value of it returning it as a String
fn get_next_arg<'a, I>(iter: &mut I, error_message: &str) -> String 
where I: Iterator<Item = &'a str>,
{
    match iter.next() {
        Some(arg) => arg.to_string(),
        None => {
            println!("{}", error_message);
            String::new()
        }
    }
}

// Save the list of todos to "todos.json"
fn save_assignments(todos: &Vec<Assignment>) {
    let json = serde_json::to_string_pretty(todos).unwrap();
    fs::write("src/assignments.json", json).unwrap();
}


fn load_assignments() -> Vec<Assignment> {
    if Path::new("src/assignments.json").exists() {
        let data = fs::read_to_string("src/assignments.json").unwrap();
        serde_json::from_str(&data).unwrap()
    } else {
        Vec::new() // Return an empty list if the file doesn't exist
    }
}

fn add_assignment(name: String, due_date: String, status: bool) {
    let mut todos = load_assignments();
    println!("{}Created assignment with name \"{}\"{}", get_colors()["green"], name, get_colors()["reset"]);
    todos.push(Assignment { name: name, due_date: due_date, status: status});
    save_assignments(&todos);
}

// Place Holder Functions
fn edit(name: String, change: &str, new_change: String) {
    if change == "changed assignment" {
        println!("Please enter either 'name', or 'due_date'");
    }
    let mut assignments = load_assignments();
    if let Some(assignment) = assignments.iter_mut().find(|t| t.name == name) {
        match change {
            "name" => {
                println!("Changed name from '{}' to '{}'", name, new_change);
                assignment.name = new_change.to_string();
            },
            "due_date" => {
                println!("Changed due date to '{}'", new_change);
                assignment.due_date = new_change.to_string();
            },
            _ => println!("There is no atrribute '{}' with assignments", change),
        };
        save_assignments(&assignments);
    } else {println!("{}Couldn't find an assignment name with \"{}\"{}", get_colors()["red"], name, get_colors()["reset"])}
}

fn view() {
    let assignments = load_assignments();
    for assignment in assignments {
        println!("Name : {}, Due : {}, Status : {}", assignment.name, assignment.due_date, if assignment.status {"x"} else {" "});
    }
}

fn mark_assignment(name: String, status: bool) {
    let mut assignments = load_assignments();
    if let Some(assignment) = assignments.iter_mut().find(|t| t.name == name) {
        assignment.status = status;


        save_assignments(&assignments);
    }
}


fn remove(name: &str) {
    let mut assignments = load_assignments();

    let original_len = assignments.len();

    // Remove all todos where the title matches exactly
    assignments.retain(|todo| todo.name != name);

    if assignments.len() < original_len {
        println!("{}Removed \"{}\"{}", get_colors()["red"], name, get_colors()["reset"]);
        save_assignments(&assignments);
    } else {
        println!("{}No assignment found with the title \"{}\"{}",get_colors()["red"], name, get_colors()["reset"]);
    }
}



fn main() {
    loop {
        let input: String = get_command();
        let mut parts = input.split_whitespace();
        let _colors = get_colors();
        
        let command: String = match parts.next() {
            Some(cmd) => cmd.to_lowercase(),
            None => {
                println!("Invalid input. Please try again.");
                continue;
            }
        };

        match command.as_str() {
            "new" => {
                let name = get_next_arg(&mut parts, "Please provide an assignment name.");
                if name.is_empty() {
                    continue;
                }
                let due_date = get_next_arg(&mut parts, "Please provide a due date.");
                if due_date.is_empty() {
                    continue;
                }
                let status = match get_next_arg(&mut parts, "Please enter the assignment status").as_str() {
                    "done" => true,
                    "incomplete" => false,
                    _ => false,
                };

                add_assignment(name, due_date, status);
            },
            "edit" => {
                let name = get_next_arg(&mut parts, "Please enter a name that you want to edit");
                if name.is_empty() {
                    continue;
                }
                let change = match get_next_arg(&mut parts, "Enter what part of the assignment you want to edit").as_str() {
                    "name" => "name",
                    "due_date" => "due_date",
                    _ => "changed assignment"
                };
                if change.is_empty() {
                    continue;
                }
                let new_change = get_next_arg(&mut parts, "Enter what you would like to change this to");
                if new_change.is_empty() {
                    continue;
                }
                
                edit(name, change, new_change)
            },
            "view" => {
                view();
            }, 
            "mark" => {
                let name = get_next_arg(&mut parts, "Please enter an assignment name");
                if name.is_empty() {
                    continue;
                }
                let status = match get_next_arg(&mut parts, "Please enter either done or incomplete").as_str() {
                    "done" => true,
                    "incomplete" => false,
                    _ => false,
                };
                
                mark_assignment(name, status);
            },
            "remove" => {

                let name: String = get_next_arg(&mut parts, "Please enter a name");
                if name.is_empty() {
                    continue;
                }

                remove(name.as_str());
            }
            "cmdhelp" => {
                let command = get_next_arg(&mut parts, "Please enter a command to learn about");
                if command.is_empty() {
                    continue;
                }

                cmdhelp(command.as_str());
            }

            "help" => help(),
            "clear" => clear_terminal(),
            _ => println!("Unknown command")
        };     
            
    };
}
