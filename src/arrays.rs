// Arrays - Fixed list where elements are the same data types

pub fn run() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];

  println!("{:?}", numbers);

  // get single val
  println!("{}", numbers[0]);

  // Arrays are stack allocated
  println!("{}", std::mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers;
  println!("Slice: {:?}", slice);
}