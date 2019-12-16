pub fn run() {
  // Primitive arr
  let arr1 = vec![1,2,3];
  let arr2 = &arr1;

  println!("Values: {:?}", (arr1, arr2));
}