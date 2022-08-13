mod queue;
use queue::Queue;// nothing here
mod elem;
use elem::Elem;
mod boss;
use boss::Boss;
mod error;
use error::Errors;

fn main() {
    let mut boss_unit = Boss::init("/home/tim/test".to_string());
    boss_unit.add_queue("queue".to_string(), Queue::create(60*24));

    if let Err(error) = boss_unit.push_to("queue".to_string(), "/home/tim/test.txt".to_string(), "item.txt".to_string(), 3) {
        println!("{}", error);
    }
}

