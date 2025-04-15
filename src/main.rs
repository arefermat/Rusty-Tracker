use std::io::{self, Write, stdout};
use std::collections::HashMap;
use crossterm::{execute, terminal::{Clear, ClearType}};
use serde::{Serialize, Deserialize};
use std::fs;



#[derive(Debug)]
enum IsDone {
    Done,
    Incomplete,
    None
}

#[derive(Debug, PartialEq)]
enum Object {
    Assignment,
    Subject,
    None,
}

#[derive(Serialize, Deserialize)]
struct Assignment {
    name : String,
    subject : String,
    due_date : String,
    e_time : i32,
}

#[derive(Serialize, Deserialize)]
struct Subject {
    name : String,
    color : &str,
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
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

fn help() {
    println!("new, edit, view, mark, resource, remove, add. Type cmdhelp command to see how to use that specific command, e.g. cmdhelp new");
}

//Takes input and trims whitespace
fn get_command() -> String {
    let mut command_input = String::new();
    
    print!("todo//> ");
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
            get_command() 
        }
    }
}

// Placeholder functions
fn new_assignment(subject: String, name: String, due_date: String, time_estimate: i32) {
    println!("Creating new assignment {} with subject {} due on {} with estimated {} minutes", name, subject, due_date, time_estimate);
}

fn edit(object: Object, name: String, new_name: String) {
    if object == Object::None {
        println!("Please enter either '-a' or '-s'");
        
    }
    println!("Changing {:?} named {} to {}", object, name, new_name);
}

fn view_assignment(object: Object, name: String) {
    println!("Viewing {:?} {}",object, name);
}

fn mark_assignment(name: String, status: IsDone) {
    println!("Marking assignment {} as {:?}", name, status);
}

fn get_resources(subject: String) {
    println!("Providing resources for {}", subject);
}

fn remove(object: Object, name: String) {
    println!("Removing {:?} named {}", object, name);
}

fn add_subject(subject_name: String, color: &str) {
    println!("Adding subject named {}{}", color, subject_name);
}

fn main() {
    loop {
        let input: String = get_command();
        let mut parts = input.split_whitespace();
        let colors = get_colors();
        
        let command: String = match parts.next() {
            Some(cmd) => cmd.to_lowercase(),
            None => {
                println!("Invalid input. Please try again.");
                continue;
            }
        };
        /* 
           new subject name due_date time_estimate
           edit -(a/s) name new
           view -(a/s) name
           mark name (done/incomplete{%})
           resource subject (add url)
           remove -(a/s) name
           add subject_name color
        */
        match command.as_str() {
            "new" => {
                let subject = get_next_arg(&mut parts, "Please provide a valid subject");
                let name = get_next_arg(&mut parts, "Please provide an assignment name.");
                let due_date = get_next_arg(&mut parts, "Please provide a due date.");
                let time_estimate: i32 = match get_next_arg(&mut parts, "Please provide a time estimate.").parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Time estimate must be a valid number.");
                        continue;
                    }
                };

                new_assignment(subject, name, due_date, time_estimate);
            },
            "edit" => {
                let object = match get_next_arg(&mut parts, "Please enter either '-a' or '-s'").as_str() {
                    "-a" => Object::Assignment,
                    "-s" => Object::Subject,
                    _ => Object::None,
                };
                let name = get_next_arg(&mut parts, "Please enter a name that you want to edit");
                let new_name = get_next_arg(&mut parts, "Please enter a new name");
                
                edit(object, name, new_name)
            },
            "view" => {
                let object = match get_next_arg(&mut parts, "Please enter either '-a' or '-s'").as_str() {
                    "-a" => Object::Assignment,
                    "-s" => Object::Subject,
                    _ => Object::None,
                };
                let name = get_next_arg(&mut parts, "Please enter an assignment name");
                
                view_assignment(object, name);
            }, 
            "mark" => {
                let name = get_next_arg(&mut parts, "Please enter an assignment name");
                let status = match get_next_arg(&mut parts, "Please enter either done or incomplete").as_str() {
                    "done" => IsDone::Done,
                    "incomplete" => IsDone::Incomplete,
                    _ => IsDone::None
                };
                
                mark_assignment(name, status);
            },
            "resource" => {
                let subject = get_next_arg(&mut parts, "Please enter a subject");

                get_resources(subject);
            },
            "remove" => {
                let object: Object = match get_next_arg(&mut parts, "Please enter either '-a' or '-s'").as_str() {
                    "-a" => Object::Assignment,
                    "-s" => Object::Subject,
                    _ => Object::None,
                };
                let name: String = get_next_arg(&mut parts, "Please enter a name");

                remove(object, name);
            }
            "add" => {
                let subject_name: String = get_next_arg(&mut parts, "Please enter a subject name");
                let color = match get_next_arg(&mut parts, "Please enter a color for the subject").as_str() {
                    "red" => colors["red"],
                    "black" => colors["black"],
                    "green" => colors["green"],
                    "yellow" => colors["yellow"],
                    "blue" => colors["blue"],
                    "magenta" => colors["magenta"],
                    "cyan" => colors["cyan"],

                    _ => colors["reset"],
                };

                add_subject(subject_name, color);
            }

            "help" => help(),
            "clear" => clear_terminal(),
            _ => println!("Unknown command")
        };         
    };
}
