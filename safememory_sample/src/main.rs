fn main() {
  let mut v = vec![];
  
  v.push("hello");
  
  // clone means that this create a copy of the element, leaving the original untouched.
  let x = v[0].clone();
  
  v.push("world");
  
  println!("{}", x);
}

/*
The point is, the Rust compiler and its notion of ownership has saved us from a bug that would crash the program. 
We've achieved safety, at compile time, without needing to rely on a garbage collector to handle our memory.
*/