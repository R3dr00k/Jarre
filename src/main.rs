mod queue;
use queue::Queue;// nothing here
mod elem;
use elem::Elem;
mod boss;
use boss::Boss;
mod error;
use error::Errors;
use std::process;

fn main() {
    let boss_unit = Boss::init("/home/tim/test");
    let mut boss = boss_unit.unwrap();

    if let Err(x) = boss.add_queue("TEST_QUEUE", Queue::create(24)) {
        println!("{}", x);
        process::exit(1);
    }

    if let Err(x) = boss.push_to("TEST_QUEUE" , "result.jarre", 5, "/home/tim/test.txt"){
        println!("{}", x);
        process::exit(1);
    }
}


