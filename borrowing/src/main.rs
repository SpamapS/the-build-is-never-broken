use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct User {
    name: String,
    rank: u64,
}

fn main() {
    let pyth = User {
        name: String::from_str("Pythagorus").unwrap(),
        rank: 100,
    };
    let kant = User {
        name: String::from_str("EmanuelKant").unwrap(),
        rank: 200,
    };
    let mut users = HashMap::new();
    users.insert(&pyth.name, pyth);
    users.insert(&kant.name, kant);
    println!("Check out these users: {:?}", users);
}
