use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();

  let command = args[1].clone();

  let name = "SLAVI";
  let status = "100%";

  print!("Command: {}", command);

  if command == "hello" {
    println!("Hi, {}, how are you?", name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    print!("That is not a valid command");
  }
}