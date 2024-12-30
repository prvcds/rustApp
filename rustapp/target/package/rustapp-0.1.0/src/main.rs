mod tasks;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "To-Do CLI")]
#[command(version = "1.0")]
#[command(about = "A simple CLI to-do list app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        description: String,
    },
    List,
    Done {
        #[arg(short, long)]
        id: usize,
    },
    Remove {
        #[arg(short, long)]
        id: usize,
    },
    Search {
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
