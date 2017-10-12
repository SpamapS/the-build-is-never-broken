use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello there are {} args. [{:?}]", args.len(), args);
}
