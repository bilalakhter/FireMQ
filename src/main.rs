mod messages;

use messages::{Messages, MsgBox};
use std::thread;
use std::time::Duration;

fn main() {
    let mut messages_instance = Messages::new();
    let message_to_store = vec![4, 5, 6];
    messages::data(&mut messages_instance, message_to_store);
    thread::sleep(Duration::from_secs(5));
    let retrieved_message = messages_instance.get(0).unwrap_or_else(|| vec![]);
    println!("Recieved msg is -> {:?}", retrieved_message);
}