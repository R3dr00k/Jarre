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
    if let Err(x) = boss.push_to("File1", "fichier1", 5, "/home/tim/test/file1.txt") {
        eprintln!("errors: {}", x);
    }

    if let Err(x) = boss.push_to("File1", "fichier2", 3, "/home/tim/test/file2.txt") {
        eprintln!("errors: {}", x);
    }

    if let Err(x) = boss.pop_to("File1") {
        eprintln!("errors: {}", x);
    }

    boss.print_queues();
}


