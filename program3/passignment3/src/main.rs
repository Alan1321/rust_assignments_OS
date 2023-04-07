/*
Alan Subedi
CS-490 Operating System
Program Overview: This program asks user for total nodes to be inserted into queue/heap, inserts them there and then prints them.
Used: VS Code on Ubuntu/WSL.
*/

use std::io;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

//the process structure for our nodes
#[derive(Clone)]
struct __Process {
    id: u64,
    priority: u64,
    sleep_time: u64,
    description: String
}

//implementing ord for process
impl Ord for __Process {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

//implementing partialord for process
impl PartialOrd for __Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//implementing partialeq for process
impl PartialEq for __Process {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

//adding eq as well, without this it won't work
impl Eq for __Process {}

fn main() {
    // Prompt the user for input
    let n = accept_input("Enter number of processes to generate each time: ".to_string());
    let s = accept_input("Enter sleep time in ms between generations: ".to_string());
    let m = accept_input("Enter number of times the producer should generate processes: ".to_string());

    // Create a shared binary heap that will serve as the priority queue
    let heap = Arc::new(Mutex::new(BinaryHeap::new()));

    // Spawn the producer thread
    let producer_heap = Arc::clone(&heap);
    let producer_thread = thread::spawn(move || {
        for _ in 0..m {
            // Generate n processes with random priorities and add them to the heap
            for i in 0..n {
                let priority: u64 = rand::thread_rng().gen_range(0..100);
                let sleep_time: u64 = rand::thread_rng().gen_range(100..2000);
                producer_heap.lock().unwrap().push(__Process{id:i, priority:priority, sleep_time:sleep_time, description:format!("{}{}", "Process Node: ", i+1)});
            }
            // Sleep for s milliseconds
            thread::sleep(Duration::from_millis(s));
        }
    });

    // Wait for a short delay to ensure there are nodes in the heap before consumers start
    thread::sleep(Duration::from_millis(100));

    // Spawn the consumer threads
    let consumer_heap = Arc::clone(&heap);
    let consumer_thread1 = thread::spawn(move || {
        loop {
            let process_opt = consumer_heap.lock().unwrap().pop();
            match process_opt {
                Some(process) => {
                    println!(
                        "Consumer 1: Executing process {} with priority {} for 100ms",
                        process.id, process.priority
                    );
                    thread::sleep(Duration::from_millis(1000));
                }
                None => {
                    println!("Consumer 1: No more processes to execute");
                    break;
                }
            }
        }
    });

    let consumer_heap = Arc::clone(&heap);
    let consumer_thread2 = thread::spawn(move || {
        loop {
            let process_opt = consumer_heap.lock().unwrap().pop();
            match process_opt {
                Some(process) => {
                    println!(
                        "Consumer 2: Executing process {} with priority {} for 100ms",
                        process.id, process.priority
                    );
                    thread::sleep(Duration::from_millis(1000));
                }
                None => {
                    println!("Consumer 2: No more processes to execute");
                    break;
                }
            }
        }
    });

    // Wait for all threads to complete
    producer_thread.join().unwrap();
    consumer_thread1.join().unwrap();
    consumer_thread2.join().unwrap();
}


fn accept_input(msg: String) -> u64{
    //accept input -- also check input errors
    let mut _x: u64 = 0;
    loop {
        //These 3 lines accepts user input as String
        println!("{}", msg);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        //Here we convert String to int and also do error handling for strings
        let _num: u64 = match input.trim().parse() {
            Ok(_num) => {
                if _num < 0{
                    println!("Invalid Input :( .Needs to be greater than 0.");
                    continue;
                }
                _x = _num;
                break;
            },
            Err(_) => {
                println!("Invalid Input :(\n");
                continue;
            }
        };
    }
    return _x
}