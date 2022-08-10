mod queue;
use queue::Queue;// nothing here
mod elem;
use elem::Elem;
mod boss;

fn main() {
    let mut temp = Queue::create(24*60);
    temp.push(Elem::new(String::from("File1"), 2, 23*60));
    temp.push(Elem::new(String::from("File2"), 2, 11*60));
    temp.push(Elem::new(String::from("File3"), 2, 2*60));
    temp.push(Elem::new(String::from("File4"), 2, 2*60));
    temp.render();
    println!("The removed value : {}", temp.pop_index(1).unwrap_or(Elem::new(String::from("NOTHING"), 0, 0)));
    temp.render();

    println!("the actual length is : {}", temp.length());
}

