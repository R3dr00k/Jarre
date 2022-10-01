mod queue;
use queue::Queue;
mod elem;
use elem::{Elem, ElemIter};
mod boss;
use boss::Boss;
mod error;
use error::Errors;


static CONFIG_DIR: &str = "/etc/jarre";
static JARRE_CONFIG: &str = "/etc/jarre/jarre.conf";

fn main() { 
    let boss_unit = Boss::init(); 
    let mut boss = boss_unit.unwrap();

    if let Err(x) = boss.add_queue("File1", Queue::create(24)) {
        eprintln!("errors:  {}", x);
    }

    if let Err(x) = boss.add_queue("File2", Queue::create(24)) {
        eprintln!("errors:  {}", x);
    }

}


