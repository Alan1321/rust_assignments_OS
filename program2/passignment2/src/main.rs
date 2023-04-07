/*
Alan Subedi
CS-490 Operating System
Program Overview: This program asks user for total nodes to be inserted into queue/heap, inserts them there and then prints them.
Used: VS Code on Ubuntu/WSL.
*/

use std::io;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use rand::Rng;

//the process structure for our nodes
struct __Process {
    id: i32,
    priority: u32,
    sleep_time: u32,
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

    //accept input -- also check input errors
    let mut _x: i32 = 0;
    loop {
        //These 3 lines accepts user input as String
        println!("Enter the number of process nodes to generate: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        //Here we convert String to int and also do error handling for strings
        let _num: i32 = match input.trim().parse() {
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

    println!("Now creating and adding {} process nodes to a Queue and to a binary minheap", _x);
    //init vecdeque and heap
    let mut buf_queue = VecDeque::new();
    let mut buf_heap: BinaryHeap<__Process> = BinaryHeap::new();
    
    //Make Process Nodes and Add to queue and heap
    let mut i = 0;
    while i < _x{

        let priority: u32 = rand::thread_rng().gen_range(0..100);
        let sleep_time: u32 = rand::thread_rng().gen_range(100..2000);

        buf_queue.push_back(__Process{id:i, priority:priority, sleep_time:sleep_time, description:format!("{}{}", "Process Node: ", i+1)});
        buf_heap.push(__Process{id:i, priority:priority, sleep_time:sleep_time, description:format!("{}{}", "Process Node: ", i+1)});

        i = i + 1;
    }

    //Queue Size
    let queue_size = buf_queue.len();
    //Heap Size
    let heap_size = buf_heap.len();

    //Verify Size of Queue and Heap
    println!("Verifying.  The queue contains {} elements", queue_size);
    println!("Verifying.  The heap contains {} elements\n", heap_size);

    //Pop from Queue and Print
    println!("Now, draining the Queue, one process at a time ... ");
    let mut j = 0;
    while j < queue_size {
        let node = buf_queue.pop_front().unwrap();
        println!("Pid: {}, pri: {}, sleep: {}, desc: {}", node.id, node.priority, node.sleep_time, node.description);
        j = j+1;
    }

    println!("");

    //Pop from heap and print
    println!("Now, draining the MinHeap, one process at a time ...");
    let mut k = 0;
    while k < heap_size {
        let node = buf_heap.pop().unwrap();
        println!("Pid: {}, pri: {}, sleep: {}, desc: {}", node.id, node.priority, node.sleep_time, node.description);
        k = k+1;
    }  

    println!("\nGoodbye.");
}