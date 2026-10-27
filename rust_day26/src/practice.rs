// day 26

use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
// mpsc stands for multiple producer, single consumer. It is a channel that allows multiple threads to send messages to a single receiver thread.

fn main() {
    let (tx, rx) = mpsc::channel();  // creates a channel with a transmitter (tx) and a receiver (rx). 
    let mut vector = Vec::new();
    for i in 0..5 {
        let tx = tx.clone();  // clone the transmitter for each thread. We want a unique transmitter for each thread so that they can all send messages to the same receiver.

        let handle = thread::spawn(move || {
            let message = format!("Hello from thread {}", i); 
            tx.send(message).unwrap();  // send the message to the receiver
        });
        vector.push(handle);  // store the handle of the spawned thread
    }
    
    drop(tx); // The original sender remains in the main thread. If it stays alive, the receiver cannot know that no more messages will arrive merely because all workers have finished sending.
    // It is a signal to the receiver that no more messages will be sent.

    for r in rx {
        println!("Received: {}", received);  // print the received message
    }
    

    for handle in vector {
        handle.join().unwrap(); 
    }
}