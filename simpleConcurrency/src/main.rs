use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Define a shared counter with initial value 0, wrapped in Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    
    // Vector to store thread handles
    let mut handles = Vec::new();

    // Spawn 5 threads
    for i in 1..=5 {
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // Lock the mutex to safely increment the counter
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("Thread {} incremented counter to {}", i, *num);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final value of the counter
    println!("Final value of counter: {}", *counter.lock().unwrap());
}
