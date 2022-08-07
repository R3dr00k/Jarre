mod queue;
use queue::{Elem, Queue};// nothing here

fn main() {
    let mut temp = Queue::create(24*60);
    temp.push(Elem::new(String::from("File1"), 3, 23*60));
    temp.push(Elem::new(String::from("File2"), 9, 11*60));
    temp.push(Elem::new(String::from("File3"), 50, 2*60));
    temp.render();
    println!("The removed value : {}", temp.pop().unwrap_or(Elem::new(String::from("NOTHING"), 0, 0)));
    temp.render();
}

