/*
 * Pointers
 */

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    print!("Hello there are {} args. [{}]", args.len(), args);
}
