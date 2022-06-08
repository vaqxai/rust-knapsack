use clap::{Parser, Subcommand};

mod knapsack_item;
use itertools::{Itertools, Powerset};
use std::slice::Iter;
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

fn get_permutation_iter(items:&Vec<KnapsackItem>) -> (Powerset<Iter<KnapsackItem>>, usize) {
    let iter : Iter<KnapsackItem> = items.into_iter();
    (iter.powerset(), items.into_iter().powerset().count())
}

fn reject_above_weight_limit<'a>(items_iter: Powerset<Iter<'a, KnapsackItem>>, knapsack_size: &usize) -> Vec<Vec<&'a KnapsackItem>> {
    items_iter.filter(|items| {
        let total_weight: i32 = items.iter().map(|item| item.get_weight()).sum();
        total_weight <= (*knapsack_size as i32)
    }).collect()
}

fn sum_values(items: &Vec<&KnapsackItem>) -> i32 {
    items.iter().map(|item| item.get_value()).sum()
}

fn sum_weights(items: &Vec<&KnapsackItem>) -> i32 {
    items.iter().map(|item| item.get_weight()).sum()
}

fn get_sorted_by_value<'a>(items: Vec<Vec<&'a KnapsackItem>>) -> Vec<Vec<&'a KnapsackItem>> {
    items.into_iter().sorted_by(|a, b| sum_values(&a).cmp(&sum_values(&b))).collect()
}

fn get_knapsack_signature(all_items: &Vec<KnapsackItem>, chosen_items: &Vec<&KnapsackItem>) -> String {
    let mut result = String::new();
    for item in all_items {
        if chosen_items.contains(&item) {
            result.push_str("1");
        } else {
            result.push_str("0");
        }
    }
    result
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Solve { knapsack_size, items } => {

            let time_begun = std::time::Instant::now();

            for item in items {
                println!("{}", item);
            }
            
            let (items_iter, items_count) = get_permutation_iter(items);
            let filtered_items = reject_above_weight_limit(items_iter, knapsack_size);
            let sorted_items = get_sorted_by_value(filtered_items);

            let time_ended = time_begun.elapsed();

            for chosen_items in sorted_items {
                let signature = get_knapsack_signature(&items, &chosen_items);
                println!("-----------------");
                println!("({}) \t {:?}", signature, chosen_items);
                let total_weight = sum_weights(&chosen_items);
                println!("Total value: {} \t Total weight: {} \t Remaining space: {}", sum_values(&chosen_items), total_weight, *knapsack_size as i32 - total_weight);
            }

            println!("Total number of unique combinations: {}", items_count);
            println!("Time taken: {:?}, with printing: {:?}", time_ended, time_begun.elapsed());

        },
        Commands::FromFile { knapsack_size: _, filename: _ } => {
            panic!("Not implemented");
        },
    }
}
