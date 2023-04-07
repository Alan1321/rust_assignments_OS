use std::io;
use rand::Rng;

struct Process {
    id: u32,
    priority: u32,
    sleep_time: u32,
    description: String
}

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let _x: i32 = input.trim().parse().expect("Input is not an integer");

    let mut i = 0;
    while i < _x{
        println!("Random Number: {}", rand::thread_rng().gen_range(0..100));
        i = i + 1;
    }
}