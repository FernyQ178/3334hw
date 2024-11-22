use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    computation: T,
    result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        ComputeCache {
            computation,
            result: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        match &self.result {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                thread::sleep(Duration::from_secs(2));
                let v = (self.computation)();
                self.result = Some(v.clone());
                v
            }
        }
    }
    }


fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}