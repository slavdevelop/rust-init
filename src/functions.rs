pub fn run() {
  greeting("Hello", "Slavaaaa");


  let ret = add(1,2);

  println!("{}", ret);

  // Closure
  let add_nums = |n1: i32, n2: i32| n1 + n2;
  println!("C Sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
  println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}