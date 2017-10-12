use std::collections::HashMap;
use std::str::FromStr;
use std::rc::Rc;

#[derive(Debug)]
struct User {
    name: String,
    rank: u64,
}

fn main() {
    let pyth = Rc::new(User {
        name: String::from_str("Pythagorus").unwrap(),
        rank: 100,
    });
    let kant = Rc::new(User {
        name: String::from_str("EmanuelKant").unwrap(),
        rank: 200,
    });
    let mut users = HashMap::new();
    users.insert(&pyth.name, pyth.clone());
    users.insert(&kant.name, kant.clone());
    println!("Check out these users: {:?}", users);
}
