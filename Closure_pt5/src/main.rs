use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        Self {
            computation,
            result: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.result {
            Some(cached) => {
                println!("Retrieved from cache instantly!");
                cached.clone()
            }
            None => {
                let result = (self.computation)();
                self.result = Some(result.clone());
                result
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
