extern crate rand;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use rand::Rng;

fn pirate(earth: Arc<Mutex<Vec<&'static str>>>) {
    let mut rng = rand::thread_rng();
    loop {
        let mut earth = earth.lock().unwrap();
        if rng.gen() { 
            println!("Yarr, my treasure be at {}", earth.len());
            earth.push("Treasure");
        } else {
            earth.push("Clues");
        }
        thread::sleep(time::Duration::from_millis(10));
    } 
}


fn niffler(earth: Arc<Mutex<Vec<&'static str>>>) {
    let mut pos = 0;
    loop {
        let mut earth = earth.lock().unwrap();
        if earth.len() < 1 {
            continue;
        }
        if earth[pos] == "Treasure" {
            println!("Stole some treasure at pos {}", pos);
            earth.remove(pos);
        }
        pos += 1;
        if pos >= earth.len() {
            pos = 0;
        }
        thread::sleep(time::Duration::from_millis(15));
    }
}

fn main() {
    let earth = Arc::new(Mutex::new(Vec::new()));
    let nif_earth = earth.clone();
    let nif = thread::spawn(move || {
        niffler(nif_earth)
    });
    let pirate_earth = earth.clone();
    let rustbeard = thread::spawn(move || {
        pirate(pirate_earth)
    });
    let watcher = thread::spawn(move || {
        let earth = earth.clone();
        loop {
            let earth = earth.lock().unwrap();
            println!("State of the earth: {:?}", *earth);
            thread::sleep(time::Duration::from_secs(2));
        }
    });
    nif.join().unwrap();
    rustbeard.join().unwrap();
    watcher.join().unwrap();
}
