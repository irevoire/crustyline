use crate::menu::day::{Day};

#[derive(Debug)]
pub struct Week {
    pub header: String,
    pub food_type: Vec<String>,
    pub days: Vec<Day>,
}

impl Week {
    pub fn new() -> Week {
        Week {
            header: String::new(),
            food_type: Vec::new(),
            days: Vec::new(),
        }
    }
}
