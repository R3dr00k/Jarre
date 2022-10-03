use crate::{Elem, ElemIter};
use std::cell::RefCell;
// ======================== QUEUE =======================

pub struct Queue {
    pub val: RefCell<Option<Elem>>,
    life_time: u32,     // time in sec
}

impl Queue{
    pub fn create(life_time: u32) -> Self {
        Queue {
            val: RefCell::new(None),
            life_time,
        }
    }

    pub fn push(&self, value: Elem){
        match self.val.take() {
            Some(x) => {
                *self.val.borrow_mut() =  Some(x.push(value));
            },
            None => {
                *self.val.borrow_mut() = Some(value);
            },
        } 
    }

    /*pub fn proto_push(self, value: Elem) {
        match self.val.take() {
            Some(x) => x.push_proto(value),
            None => *self.val.borrow_mut() = Some(value)
        }
        return self.val;
    }*/

    pub fn pop(&self) -> Option<Elem> {
        match self.val.take(){
            Some(x) => {
                let ret_elem: Option<Elem>;
                let ret_self: Option<Box<Elem>>;

                (ret_self, ret_elem) = x.pop(); *self.val.borrow_mut() = match ret_self {
                    Some(x) => Some(*x),
                    None => None,
                };

                ret_elem
            },
            None => None,
        }
    }

    pub fn get_life_time(&self) -> u32 {
        self.life_time
    }

    pub fn pop_index(&self, index: u32) -> Option<Elem> {
        let len: u32 = self.length();
        match self.val.take() {
            Some(x) => {
                let ret: Option<Elem>;
                let ret_self: Option<Box<Elem>>;
                (ret_self, ret) = x.pop_index(index, len, 0);

                *self.val.borrow_mut() = match ret_self {
                    Some(x) => Some(*x),
                    None => None,
                };

                ret
            },
            None => None,
        }
    }

    pub fn render(&self) {
        match self.val.take() {
            Some(ref x) => {
                x.chainprint();
            },
            None => {
                println!("Nothing.")
            }
        }
    }
    
    pub fn length(&self) -> u32 {
        match self.val.take() {
            Some(ref x) => return x.length(1),
            None => return 0,
        }
    }

/*
 * ITERATOR // here no need for now
 * pub fn to_iter(&self) -> ElemIter {
        let prochain = self.val;
        ElemIter {
            prochain,
        }
    }
*/
}
