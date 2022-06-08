use clap::{Parser, Subcommand};

mod knapsack_item;
use knapsack_item::KnapsackItem;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Solve {
        knapsack_size: usize,
        items: Vec<KnapsackItem>
    },
    FromFile {
        knapsack_size: usize,
        filename: String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Solve { knapsack_size, items } => {
            panic!("Not implemented");
        },
        Commands::FromFile { knapsack_size, filename } => {
            panic!("Not implemented");
        },
    }
}
