use std::sync::{Arc, Mutex};
use std::thread;


let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
let len = data.lock().unwrap().len();
let mid = len / 2;


let left_data = Arc::clone(&data);
let left_handle = thread::spawn(move || {
    let mut left = left_data.lock().unwrap();
    for x in &mut left[..mid] {
        *x *= 2;
    }
});

let right_data = Arc::clone(&data);
let right_handle = thread::spawn(move || {
    let mut right = right_data.lock().unwrap();
    for x in &mut right[mid..] {
        *x *= 2;
    }
});

left_handle.join().unwrap();
right_handle.join().unwrap();

println!("{:?}", *data.lock().unwrap()); // [2, 4, 6, 8, 10]


// the main problem, I want you to notice, even though it seems like
// a fine data paralelism, it has a huge problem.
// because you are locking data vector at the moment of updating the value
// it basically means, only one thread at a time can update value
// even though logically it seems correct it's a overhead
fn data_paralelism_rayon() {

use rayon::prelude::*;

let mut data = vec![1, 2, 3, 4, 5];

data.par_iter_mut().for_each(|x| {
    *x *= 2;
});

// from code perspective it seems trivial, but I want you to realize how much heavy lifting happens behind the hood:
//The Rayon library uses work stealing to dynamically balance the workload among threads, 
//providing better performance compared to a static division of work among threads.

// on top it creates a separate scope to escape need to lock data



// Concept of work stealing and separate scope.



println!("{:?}", data); // [2, 4, 6, 8, 10]
}
fn main(){
    data_paralelism_rayon();
    
}