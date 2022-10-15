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
    let boss = boss_unit.unwrap();

    boss.print_queues();
}


