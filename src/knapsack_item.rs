use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;
use std::str::FromStr;

pub struct KnapsackItem {
    value: i32,
    weight: i32,
}

impl KnapsackItem {
    pub fn new(value: i32, weight: i32) -> KnapsackItem {
        KnapsackItem {
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

    pub fn get_ratio(&self) -> f64 {
        (self.value as f64) / (self.weight as f64)
    }

    pub fn get_ratio_as_string(&self) -> String {
        format!("{:.2}", self.get_ratio())
    }
}

impl Debug for KnapsackItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "KnapsackItem {{ value: {}, weight: {} }}", self.value, self.weight)
    }
}

impl Display for KnapsackItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "KnapsackItem {{ value: {}, weight: {} }}", self.value, self.weight)
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