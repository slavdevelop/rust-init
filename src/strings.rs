// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify
// or own string data

pub fn run() {
  let mut hello = String::from("Hello");

  // Get length
  println!("Lenth: {}", hello.len());

  hello.push('W');

  hello.push_str("orld!");
  
  println!("{}", hello.capacity());
  
  println!("{}", hello.is_empty());

  println!("{}", hello.contains("World"));
  
  println!("{}", hello.replace("World", "FU"));

  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());

  println!("{}", s);
}