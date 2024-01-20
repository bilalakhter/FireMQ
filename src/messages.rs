use std::sync::{Arc, Mutex};

pub trait Storer {
    fn push(&mut self, data: Vec<u8>) -> usize;
    fn get(&self, offset: usize) -> Option<Vec<u8>>;
}
pub struct Messages {
    data: Vec<Vec<u8>>,
    mutex: Mutex<()>,
}

impl Messages {
   pub fn new() -> Self {
        Messages {
            data: Vec::new(),
            mutex: Mutex::new(()),
        }
    }
}

impl Storer for Messages {
    fn push(&mut self, data: Vec<u8>) -> usize {
        let _guard = self.mutex.lock().unwrap();
        let index = self.data.len();
        self.data.push(data);
        index
    }

    fn get(&self, offset: usize) -> Option<Vec<u8>> {
        let _guard = self.mutex.lock().unwrap();
        if offset >= self.data.len() {
            None
        } else {
            Some(self.data[offset].clone())
        }
    }
}

fn data() {
    let store = Arc::new(Mutex::new(Messages::new()));
    let cloned_store = Arc::clone(&store);
    let handle = std::thread::spawn(move || {
        let mut store = cloned_store.lock().unwrap();
        store.push(vec![1, 2, 3]);
        println!("{:?}", store.get(0));
    });
    handle.join().unwrap();
}
