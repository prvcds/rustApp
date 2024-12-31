mod tasks;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "To-Do CLI")]
#[command(version = "1.0.0")]
#[command(about = "A simple CLI to-do list app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Description of the task
        #[arg(short, long)]
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as done
    Done {
        /// ID of the task to mark as done
        #[arg(short, long)]
        id: usize,
    },
    /// Remove a task
    Remove {
        /// ID of the task to remove
        #[arg(short, long)]
        id: usize,
    },
    /// Search for tasks by description
    Search {
        /// Query string to search for in task descriptions
        #[arg(short, long)]
        query: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { description } => tasks::add_task(description),
        Commands::List => tasks::list_tasks(),
        Commands::Done { id } => tasks::mark_done(id),
        Commands::Remove { id } => tasks::remove_task(id),
        Commands::Search { query } => tasks::search_tasks(&query),
    }
}
