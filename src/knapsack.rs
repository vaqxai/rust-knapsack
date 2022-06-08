
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::knapsack_item::KnapsackItem;
pub struct Knapsack {
    id: usize,
    items: Vec<KnapsackItem>,
    capacity: i32,
}

static ID: AtomicUsize = AtomicUsize::new(0);

impl Knapsack {

    pub fn new(capacity: i32, items: Vec<KnapsackItem>) -> Knapsack {

        Knapsack {
            id: ID.fetch_add(1, Ordering::SeqCst),
            capacity,
            items,
        }
    }

    pub fn get_items(&self) -> &Vec<KnapsackItem> {
        &self.items
    }

    pub fn get_capacity(&self) -> i32 {
        self.capacity
    }

    pub fn get_total_value(&self) -> i32 {
        self.items.iter().map(|item| item.get_value()).sum()
    }

    pub fn get_total_weight(&self) -> i32 {
        self.items.iter().map(|item| item.get_weight()).sum()
    }


}

impl PartialEq for Knapsack {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}