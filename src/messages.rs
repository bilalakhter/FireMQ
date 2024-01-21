use std::sync:: Mutex;

pub trait MsgBox {
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

impl MsgBox for Messages {
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

pub fn data(messages_instance: &mut Messages, message: Vec<u8>) {
    messages_instance.push(message);
}
