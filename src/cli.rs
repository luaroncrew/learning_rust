use std::env;
use std::os::macos::raw::stat;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Kirill";
    let status = "happy";
    println!("Args: {:?}", command);

    if command == "hello" {
        println!("hi, {}", name)
    } else if command == "status"{
        println!("status is {}", status)
    } else {
        println!("Not valid command")
    }
}