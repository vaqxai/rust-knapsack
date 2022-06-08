use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;
use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct KnapsackItem {
    id: usize,
    value: i32,
    weight: i32,
}

static ID: AtomicUsize = AtomicUsize::new(0);

impl KnapsackItem {
    pub fn new(value: i32, weight: i32) -> KnapsackItem {

        KnapsackItem {
            id: ID.fetch_add(1, Ordering::SeqCst),
            value,
            weight,
        }
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_weight(&self) -> i32 {
        self.weight
    }
}

impl Debug for KnapsackItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "#{}(V: {} W: {})", self.id, self.value, self.weight)
    }
}

impl Display for KnapsackItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "#{}(V: {} W: {})", self.id, self.value, self.weight)
    }
}

impl FromStr for KnapsackItem {
    type Err = String;

    fn from_str(s: &str) -> Result<KnapsackItem, String> {
        let mut parts = s.split_whitespace();
        let value = parts.next().ok_or("No value found".to_string())?;
        let value = value.parse::<i32>().map_err(|e| e.to_string())?;
        let weight = parts.next().ok_or("No weight found".to_string())?;
        let weight = weight.parse::<i32>().map_err(|e| e.to_string())?;
        Ok(KnapsackItem::new(value, weight))
    }
}

impl PartialEq for KnapsackItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}