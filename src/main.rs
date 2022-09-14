mod queue;
use queue::Queue;
mod elem;
use elem::{Elem, ElemIter};
mod boss;
use boss::Boss;
mod error;
use error::Errors;
use std::process;

static CONFIG_DIR: &str = "/var/lib/jarre";
static JARRE_STATE: &str = "/var/lib/jarre/state.jarre";

fn main() { 
    let boss_unit = Boss::init("/home/tim/test"); 
    let mut boss = boss_unit.unwrap(); if let Err(x) = boss.add_queue("TEST_QUEUE", Queue::create(24)) {
        println!("{}", x);
        process::exit(1);
    }

    if let Err(x) = boss.push_to("TEST_QUEUE" , "result.jarre", 5, "/home/tim/test.txt"){
        println!("{}", x);
        process::exit(1);
    }
    if let Err(x) = boss.push_to("TEST_QUEUE" , "result2.jarre", 5, "/home/tim/test2.txt"){
        println!("{}", x);
        process::exit(1);
    }
    if let Err(x) = boss.export_as_file() {
        println!("{}", x);
        process::exit(1);
    }
}


