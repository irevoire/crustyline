use crate::menu::food::{Food};

#[derive(Debug)]
pub struct Day {
    pub name: String,
    pub food: Vec<Food>,
}

impl Day {
    pub fn new(name: String) -> Day {
        Day {
            name: name,
            food: Vec::new(),
        }
    }

    pub fn add(&mut self, name: String, price: String) {
        self.food.push(Food::new(name, price));
    }
}
