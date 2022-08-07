mod queue;
use queue::Elem;// nothing here

fn main() {
    let mut liste = Elem::new(String::from("File1"), 3, 22*60);
    liste.push(Elem::new(String::from("File2"), 2, 12*60));
    liste.push(Elem::new(String::from("File3"), 7, 12*60));
    liste.push(Elem::new(String::from("File4"), 1, 12*60));

    liste.chainprint();

    let removed = liste.pop().unwrap();
    println!("Removed={}", removed);

    liste.chainprint();
}
