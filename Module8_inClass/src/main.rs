// // assign 1

// use std::thread;
// use std::time::Duration;

// fn main() {
//     println!("Main thread starting");

//     // vector to store thread handles
//     let mut handles = vec![];

//     // spawn 3 threads
//     for i in 1..=3 {
//         let handle = thread::spawn(move || {
//             println!("Thread {} starting", i);
//             thread::sleep(Duration::from_millis(500));
//             println!("Thread {} finished", i);
//         });

//         handles.push(handle);
//     }

//     // wait for all threads to complete
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("All threads completed.");
// }

// // assign 2
// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     // shared counter
//     let counter = Arc::new(Mutex::new(0));

//     // vector to store thread handles
//     let mut handles = vec![];

//     // spawn 5 threads
//     for i in 1..=5 {
//         let counter_clone = Arc::clone(&counter);

//         let handle = thread::spawn(move || {
//             for _ in 0..10 {
//                 let mut num = counter_clone.lock().unwrap();
//                 *num += 1;
//             }
//             println!("Thread {} finished incrementing", i);
//         });

//         handles.push(handle);
//     }

//     // wait for all threads to complete
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // print final counter value
//     println!("Final counter value: {}", *counter.lock().unwrap());
// }

// // assign 3
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

// // Message to be sent to workers
// enum Message {
//     NewJob(Job),
//     Terminate,
// }

// // boxed closure job
// type Job = Box<dyn FnOnce() + Send + 'static>;

// // ThreadPool struct
// struct ThreadPool {
//     workers: Vec<Worker>,
//     sender: mpsc::Sender<Message>,
// }

// impl ThreadPool {
//     // create a new ThreadPool
//     fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);

//         // create channel
//         let (sender, receiver) = mpsc::channel();

//         // wrap receiver so workers can share it
//         let receiver = Arc::new(Mutex::new(receiver));

//         // create workers
//         let mut workers = Vec::with_capacity(size);
//         for id in 0..size {
//             workers.push(Worker::new(id, Arc::clone(&receiver)));
//         }

//         // return thread pool
//         ThreadPool { workers, sender }
//     }

//     // execute a job
//     fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//         let job = Box::new(f);
//         self.sender.send(Message::NewJob(job)).unwrap();
//     }
// }

// // clean up when ThreadPool is dropped
// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         // send terminate message to all workers
//         for _ in &self.workers {
//             self.sender.send(Message::Terminate).unwrap();
//         }

//         // join all worker threads
//         for worker in &mut self.workers {
//             if let Some(thread) = worker.thread.take() {
//                 thread.join().unwrap();
//             }
//         }
//     }
// }

// // Worker struct
// struct Worker {
//     id: usize,
//     thread: Option<thread::JoinHandle<()>>,
// }

// impl Worker {
//     fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
//         let thread = thread::spawn(move || {
//             loop {
//                 let message = receiver.lock().unwrap().recv().unwrap();

//                 match message {
//                     Message::NewJob(job) => {
//                         println!("Worker {} is processing a job", id);
//                         job();
//                     }
//                     Message::Terminate => {
//                         println!("Worker {} is terminating", id);
//                         break;
//                     }
//                 }
//             }
//         });

//         Worker {
//             id,
//             thread: Some(thread),
//         }
//     }
// }

// fn main() {
//     // create thread pool with 4 workers
//     let pool = ThreadPool::new(4);

//     // submit 10 tasks
//     for i in 1..=10 {
//         pool.execute(move || {
//             println!("Processing task {}", i);
//             thread::sleep(std::time::Duration::from_millis(500));
//             println!("Completed task {}", i);
//         });
//     }

//     println!("Main thread waiting for tasks to complete...");
//     // when pool goes out of scope, Drop runs automatically
// }

// assign 4
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// special value for termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // total items to produce
    const ITEM_COUNT: usize = 20;

    let num_producers = 2;
    let num_consumers = 3;

    // create channel
    let (tx, rx) = mpsc::channel();

    // shared receiver for consumers
    let shared_rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];

    // create 2 producer threads
    for id in 1..=num_producers {
        let tx_clone = tx.clone();

        // split work between producers
        let items_for_this_producer = ITEM_COUNT / num_producers;

        let handle = thread::spawn(move || {
            producer(id, tx_clone, items_for_this_producer);
        });

        producer_handles.push(handle);
    }

    // create 3 consumer threads
    for id in 1..=num_consumers {
        let rx_clone = Arc::clone(&shared_rx);

        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });

        consumer_handles.push(handle);
    }

    // wait for producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // send termination signal once for each consumer
    for _ in 0..num_consumers {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // wait for consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {} finished", id);
}

// consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = {
            let receiver = rx.lock().unwrap();
            receiver.recv().unwrap()
        };

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal and is exiting", id);
            break;
        }

        println!("Consumer {} processed {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
}