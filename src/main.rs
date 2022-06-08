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
        #[clap(help = "The capacity of the knapsack")]
        knapsack_size: usize,
        #[clap(help = "The items to put in the knapsack, format: \"value weight\" \"value weight\" \"value weight\" ...")]
        items: Vec<KnapsackItem>
    },
    FromFile {
        #[clap(help = "The capacity of the knapsack")]
        knapsack_size: usize,
        #[clap(help = "The file containing the items to put in the knapsack, format: NOT YET IMPLEMENTED")]
        filename: String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Solve { knapsack_size, items } => {
            for item in items {
                println!("{}", item);
            }
            panic!("Not implemented");
        },
        Commands::FromFile { knapsack_size, filename } => {
            panic!("Not implemented");
        },
    }
}
