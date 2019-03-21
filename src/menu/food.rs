#[derive(Debug)]
pub struct Food {
    name: String,
    price: String, // because sometimes it's unparseable and I'm lazy
}

impl Food {
    pub fn new(name: String, price: String) -> Food {
        Food {
            name: name,
            price: price,
        }
    }
}
