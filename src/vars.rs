// Variables hold primitive data or referances to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Slav";
  let mut age = 37;

  age = 38;

  println!("My name is {}", name);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Slavi", 26);
  println!("{} is {}", my_name, my_age);
}