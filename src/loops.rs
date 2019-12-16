// Loops = 

pub fn run() {
  let mut count = 0;

  //Infinite loop
  loop {
    count += 1;
    println!("Number: {}", count);

    if count == 20 {
      break;
    }
  }
}