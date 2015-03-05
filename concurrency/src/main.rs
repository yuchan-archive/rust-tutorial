use std::thread::Thread;
use std::sync::{Arc,Mutex};

fn main(){
  let numbers = Arc::new(Mutex::new(vec![1,2,3]));
  
  let guards: Vec<_> = (0..3).map(|i|{
    let number = numbers.clone();
    Thread::scoped(move || {
       let mut array = number.lock().unwrap();
       array[i] += i;
       println!("numbers[{}] is {}", i, array[i]);
      })
    }).collect();
}