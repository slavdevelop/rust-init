pub fn run() {

  let mut numbers: Vec<i32> = vec![1,2,3,4,5];


  // alocated bytes
  println!("{:?}", std::mem::size_of_val(&numbers));

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }


  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("NUmbers Vec: {:?}", numbers);
}