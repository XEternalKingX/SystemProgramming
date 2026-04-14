// // Assignment 3
// // starter code:
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

// // Message to be sent to the workers
// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// // Job type is a boxed closure that can be sent across threads
// type Job = Box<dyn FnOnce() + Send + 'static>;

// // ThreadPool struct
// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// impl ThreadPool {
//     // Create a new ThreadPool with the specified size
//     fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);
        
//         // TODO: Create a channel for sending jobs
//         let (sender, receiver) = mpsc::channel();
//         let receiver = Arc:: new(Mutex::new(receiver));
        
//         // TODO: Create and store workers
//         let mut workers = Vec::with_capacity(size);

//         for id in 0..size {
//             // each worker gets its own id and a clone of the shared receiver
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }
        
//         // TODO: Return the ThreadPool
//         ThreadPool { workers, sender }
//     }
    
//     // Execute a job in the thread pool
//     fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         // TODO: Create a job from the closure and send it to a worker
//         let job = Box::new(f);
//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// // Clean up resources when ThreadPool is dropped
// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         // TODO: Send terminate message to all workers
//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }
        
//         // TODO: Wait for all workers to finish
//         for worker in &mut self.workers {
//             println!("Shutting down worker {}", worker.id);

//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// // Worker struct represents a thread that can process jobs
// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     // Create a new worker with the specified ID
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
//         // TODO: Create a thread that loops and receives jobs from the channel
//         let thread = thread::spawn(move || {
//             loop {
//                 // Lock the receiver so this worker can safely receive one message.
//                 // After receive finishes, the lock is released.
//                 let message = receiver.lock().unwrap().recv().unwrap();

//                 match message {
//                     Message::NewJob(job) => {
//                         //printing which worker is handling the job
//                         println!("Worker {} got a job; executing.", id);
//                         job(); // running the closure
//                     }
//                     Message::Terminate => {
//                         // stoping the worker when termination message recieved
//                         println!("Worker {} was told to terminate.", id);
//                         break;
//                     }
//                 }
//             }
//         });
        
//         // TODO: Return the Worker
//         Worker {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

// fn main() {
//     // Create a new thread pool with 4 workers
//     let pool = ThreadPool::new(4);
    
//     // Submit 10 tasks to the pool
//     for i in 1..=10 {
//         pool.execute(move || {
//             println!("Processing task {}", i);
//             thread::sleep(std::time::Duration::from_millis(500));
//             println!("Completed task {}", i);
//         });
//     }
    
//     println!("Main thread waiting for tasks to complete...");
//     // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
// }


// Assignment 4
// Starter Code:
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    // tx for sender side of the channel and rx for the reciver side of the channel
    let (tx, rx) = mpsc::channel();

    let rx = Arc::new(Mutex::new(rx));

    let num_producers = 2;
    let num_consumers = 3;

    let mut producer_handles = Vec::new(); // this stores the JoinHandles for producer threads
    let mut consumer_handles = Vec::new(); // this stores the JoinHandles for consumer threads
    
    // TODO: Create 2 producer threads
    let items_per_producer = ITEM_COUNT / num_producers;

    for id in 0..num_producers {
        let tx_clone = tx.clone();

        let handle = thread::spawn(move || {
            producer(id + 1, tx_clone, items_per_producer);
        });
        producer_handles.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    for id in 0..num_consumers {
        let rx_clone = Arc::clone(&rx);

        let handle = thread::spawn(move || {
            consumer(id + 1, rx_clone);
        });
        consumer_handles.push(handle);
    }

    // Wait for all producers to finish first.
    // This ensures all normal data has been sent before termination signals.
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // After all producers are done, main sends one termination signal
    // for each consumer so every consumer gets a chance to exit.
    for _ in 0..num_consumers {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    drop(tx); // drop the org. sender sfter all mesg. have been sent
    
    // TODO: Wait for all threads to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng(); // creates a random number local to producer thread

    for item_num in 1..=item_count {
        // Generates a random number between 1 and 100.
        let value = rng.gen_range(1..=100);

        // Prints which producer made the value.
        println!(
            "Producer {} generated item {} with value {}",
            id, item_num, value
        );
        tx.send(value).unwrap(); // ssends value into the chsnnel
        thread::sleep(Duration::from_millis(150)); // small delay 
    }
    println!("Producer {} finished producing.", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        // Locks the shared receiver so this consumer can safely receive one value
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal and is exiting.", id);
            break;
        }

        // Simulates processing the value
        println!("Consumer {} processed value {}", id, value);
        thread::sleep(Duration::from_millis(200));
    }
}