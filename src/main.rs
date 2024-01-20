
mod messages; 

use messages::{Messages, Storer};
use std::sync::{Arc, Mutex};

fn main() {
    let store = Arc::new(Mutex::new(Messages::new()));
    let cloned_store = Arc::clone(&store);
    let handle = std::thread::spawn(move || {
        let mut store = cloned_store.lock().unwrap();
        let index = store.push(vec![1, 2, 3]);
        println!("Added data at index: {}", index);
        let data = store.get(index - 1).unwrap_or_else(|| vec![]);
        println!("Retrieved data: {:?}", data);
    });

    handle.join().unwrap();

    let mut store = store.lock().unwrap();
    let index = store.push(vec![4, 5, 6]);
    println!("Added data at index: {}", index);
    let data = store.get(index - 1).unwrap_or_else(|| vec![]);
    println!("Retrieved data: {:?}", data);
}
