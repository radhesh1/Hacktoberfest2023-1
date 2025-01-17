use std::io;
use std::str::FromStr;

#[derive(Clone)]
struct Process {
    name: String,
    arrival_time: u32,
    burst_time: u32,
}

impl Process {
    fn new(name: &str, arrival_time: u32, burst_time: u32) -> Process {
        Process {
            name: String::from(name),
            arrival_time,
            burst_time,
        }
    }
}

fn get_user_input(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn main() {
    let mut processes = vec![];

    for i in 1..=4 {
        println!("Enter details for Process P{}:", i);

        let name = format!("P{}", i);
        let arrival_time = get_user_input("Arrival Time:");
        let burst_time = get_user_input("Burst Time:");

        processes.push(Process::new(&name, arrival_time, burst_time));
    }

    // Sort processes by arrival time
    let mut sorted_processes = processes.clone();
    sorted_processes.sort_by_key(|p| p.arrival_time);

    let mut current_time = 0;

    for process in sorted_processes.iter() {
        if process.arrival_time > current_time {
            current_time = process.arrival_time;
        }

        println!("Executing {} (arrives at {}, burst time {})", process.name, process.arrival_time, process.burst_time);

        current_time += process.burst_time;
    }
}