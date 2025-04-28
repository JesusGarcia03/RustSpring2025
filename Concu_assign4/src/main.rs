use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce in total
    const ITEM_COUNT: usize = 20;
    
    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // Create 2 producer threads
    let mut producers = Vec::new();
    let producer_count = 2;
    let items_per_producer = ITEM_COUNT / producer_count;

    for id in 0..producer_count {
        let tx_clone = tx.clone();
        producers.push(thread::spawn(move || {
            producer(id, tx_clone, items_per_producer);
        }));
    }

    // Create 3 consumer threads
    let mut consumers = Vec::new();
    let consumer_count = 3;

    for id in 0..consumer_count {
        let rx_clone = Arc::clone(&rx);
        consumers.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Wait for all producers to finish
    for producer in producers {
        producer.join().unwrap();
    }

    println!("All producers have finished. Sending termination signals...");

    // Send a termination signal for each consumer
    for _ in 0..consumer_count {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumers to finish
    for consumer in consumers {
        consumer.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..100);
        println!("Producer {id} produced {num}");
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
    println!("Producer {id} finished producing.");
}

// Consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let received = {
            let lock = rx.lock().unwrap();
            lock.recv()
        };

        match received {
            Ok(num) => {
                if num == TERMINATION_SIGNAL {
                    println!("Consumer {id} received termination signal. Exiting.");
                    break;
                } else {
                    println!("Consumer {id} processing number {num}");
                    thread::sleep(Duration::from_millis(200));
                }
            }
            Err(_) => {
                println!("Consumer {id} encountered an error. Exiting.");
                break;
            }
        }
    }
}
