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

fn main(){

    //accepting inputs
    let mut _n:u64 = accept_input("Enter N (number of process nodes to generate each time)".to_string());
    let mut _s:u64 = accept_input("Enter S (sleep time in ms between generations)".to_string());
    let mut _m:u64 = accept_input("Enter M (number of times the producer should generate N nodes)".to_string());

    println!("Now creating and adding {} process nodes to a Queue and to a binary minheap", _n);
    //init vecdeque and heap
    let buf_heap = Arc::new(Mutex::new(BinaryHeap::new()));

    //Make Process Nodes and Add to queue and heap
    let producer = buf_heap.clone();

    let p_thread = thread::spawn(move || {
        for _k in 0.._m{
            let mut i = 0;
            while i < _n{
                let priority: u64 = rand::thread_rng().gen_range(0..100);
                let sleep_time: u64 = rand::thread_rng().gen_range(100..2000);
                producer.lock().unwrap().push(__Process{id:i, priority:priority, sleep_time:sleep_time, description:format!("{}{}", "Process Node: ", i+1)});
                i = i + 1;
            }
            thread::sleep(Duration::from_millis(_s));
        }
    });

    thread::sleep(Duration::from_millis(1000));

    let consumer1 = buf_heap.clone();
    let consumer1_thread = thread::spawn(move || {
        loop {
            // let mut heap_data = consumer1.lock().unwrap();
            // let heap_size = heap_data.len();
            // let mut k = 0;
            // while k < heap_size {
            //     let node = heap_data.pop().unwrap();
            //     println!("Thread1: Pid: {}, pri: {}, sleep: {}, desc: {}", node.id, node.priority, node.sleep_time, node.description);
            //     k = k+1;
            //     thread::sleep(Duration::from_millis(node.sleep_time));
            // }
            // if heap_size == 0 {
            //     break;
            // }

            let mut heap_data = consumer1.lock().unwrap();
            if let Some(process) = heap_data.pop() {
                println!("Thread1: Pid: {}, pri: {}, sleep: {}, desc: {}", process.id, process.priority, process.sleep_time, process.description);
                //thread::sleep(Duration::from_millis(process.sleep_time))
            } else {
                println!("Thread1: BREAKING");
                break;
            }

        
        }
    });

    let consumer2 = buf_heap.clone();
    let consumer2_thread = thread::spawn(move || {
        loop {
            // let mut heap_data = consumer2.lock().unwrap();
            // let heap_size = heap_data.len();
            // let mut k = 0;
            // while k < heap_size {
            //     let node = heap_data.pop().unwrap();
            //     println!("Thread2: Pid: {}, pri: {}, sleep: {}, desc: {}", node.id, node.priority, node.sleep_time, node.description);
            //     k = k+1;
            //     thread::sleep(Duration::from_millis(node.sleep_time));
            // }
            // if heap_size == 0 {
            //     break;
            // }

            {
                let mut heap_data = consumer2.lock().unwrap();
                if let Some(process) = heap_data.pop() {
                    println!("Thread2: Pid: {}, pri: {}, sleep: {}, desc: {}", process.id, process.priority, process.sleep_time, process.description);
                    //thread::sleep(Duration::from_millis(process.sleep_time));
                } else {
                    println!("THREAD2: BREAKING");
                    break;
                }
            }
        }
    });

    p_thread.join().unwrap();
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