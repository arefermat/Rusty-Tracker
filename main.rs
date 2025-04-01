use clap::{Parser, Subcommand};


#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd : Commands,
}

#[derive(Subcommand)]
#[derive(Debug)]
enum IsDone {
    Done,
    Incomplete,
}
#[derive(Subcommand)]
enum Commands {
    New {
        assigment_name : String,
        due_date : String,
        time_estimate : i32
    },
    View {assigment_name : String},
    Mark {
        assigment_name : String,
        #[command(subcommand)]
        done : IsDone
    },
    Resources {
        subject : String,
    }
}

//Command Functions Below:

fn new_assigment(name: String, due_date: String, time_estimate: i32) {
    println!("Creating a new assigment with name {}, due date of {} and that'll take about {} minute to complete.", name, due_date, time_estimate);
    
}

fn view_assigment(name: String) {
    println!("Viewing an assigment with name {}", name);
}

fn mark_assigment(name: String, done: IsDone) {
    println!("Marking assigment named {} as {:?}", name, done);
}

fn resources(subject: String) {
    println!("Here are a list resources for the subject of {}: ", subject)
}

fn main() {

    let args: Args = Args::parse();
    
    match args.cmd {
        Commands::New { assigment_name, due_date, time_estimate } => new_assigment(assigment_name, due_date, time_estimate),
        Commands::View { assigment_name } => view_assigment(assigment_name),
        Commands::Mark { assigment_name, done } => match done {
            IsDone::Done => mark_assigment(assigment_name, IsDone::Done),
            IsDone::Incomplete => mark_assigment(assigment_name, IsDone::Incomplete),
        },
        Commands::Resources { subject } => resources(subject),
    }

}
