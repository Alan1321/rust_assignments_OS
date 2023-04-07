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
    sleep_time: u64
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

    let mut consumer1_count = 0;
    let mut consumer2_count = 0;   
    
    let n:u64 = accept_input("Enter N (number of process nodes to generate each time)".to_string());
    let s:u64 = accept_input("Enter S (sleep time in ms between generations)".to_string());
    let m:u64 = accept_input("Enter M (number of times the producer should generate N nodes)".to_string());
    
    println!("\nStarting Simulation\n");

    let buf_heap = Arc::new(Mutex::new(BinaryHeap::new()));

    let producer = buf_heap.clone();
    let producer_thread = thread::spawn(move || {
        for _ in 0..m {
            println!("... producer is starting its work ...");
            let mut i = 0;
            while i < n{
                let priority: u64 = rand::thread_rng().gen_range(0..100);
                let sleep_time: u64 = rand::thread_rng().gen_range(100..2000);
                producer.lock().unwrap().push(__Process{id:i, priority:priority, sleep_time:sleep_time});
                i = i + 1;
            }
            println!("... producer is sleeping ...");
            thread::sleep(Duration::from_millis(s));
        }
        println!("... producer has finished: {} nodes were generated ...", n*m);
    });

    thread::sleep(Duration::from_millis(100));

    let consumer1 = buf_heap.clone();
    let consumer1_thread = thread::spawn(move || {
        loop {
            let heap_data = consumer1.lock().unwrap().pop();
            match heap_data {
                Some(node) =>{
                    println!("Consumer1: executed process {}, pri: {}, for {} ms", node.id, node.priority, node.sleep_time);
                    consumer1_count  = consumer1_count + 1;
                    thread::sleep(Duration::from_millis(node.sleep_time));
                }
                None => {
                    println!("...Consumer1 has completed and executed {} processes", consumer1_count);
                    break;
                }
            }
        }
    });

    let consumer2 = buf_heap.clone();
    let consumer2_thread = thread::spawn(move || {
        loop {
            let heap_data = consumer2.lock().unwrap().pop();
            match heap_data {
                Some(node) =>{
                    println!("Consumer2: executed process {}, pri: {}, for {} ms", node.id, node.priority, node.sleep_time);
                    consumer2_count = consumer2_count + 1;
                    thread::sleep(Duration::from_millis(node.sleep_time));
                }
                None => {
                    println!("...Consumer2 has completed and executed {} processes", consumer2_count);
                    break;
                }
            }
        }
    });


    producer_thread.join().unwrap();
    consumer1_thread.join().unwrap();
    consumer2_thread.join().unwrap();
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