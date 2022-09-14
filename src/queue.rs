use crate::{Elem, ElemIter};
// ======================== QUEUE =======================

pub struct Queue {
    pub val: Option<Elem>,
    life_time: u32,     // time in sec
}

impl Queue{
    pub fn create(life_time: u32) -> Self {
        Queue {
            val: None,
            life_time,
        }
    }

    pub fn push(&mut self, value: Elem){
        match self.val.take() {
            Some(x) => {
                self.val =  Some(x.push(value));
            },
            None => {
                self.val = Some(value);
            },
        } 
    }

    pub fn pop(&mut self) -> Option<Elem> {
        match self.val.take(){
            Some(x) => {
                let ret_elem: Option<Elem>;
                let ret_self: Option<Box<Elem>>;

                (ret_self, ret_elem) = x.pop();

                self.val = match ret_self {
                    Some(x) => Some(*x),
                    None => None,
                };

                ret_elem
            },
            None => None,
        }
    }

    pub fn pop_index(&mut self, index: u32) -> Option<Elem> {
        let len: u32 = self.length();
        match self.val.take() {
            Some(x) => {
                let ret: Option<Elem>;
                let ret_self: Option<Box<Elem>>;
                (ret_self, ret) = x.pop_index(index, len, 0);

                self.val = match ret_self {
                    Some(x) => Some(*x),
                    None => None,
                };

                ret
            },
            None => None,
        }
    }

    pub fn render(&self) {
        match self.val {
            Some(ref x) => {
                x.chainprint();
            },
            None => {
                println!("Nothing.")
            }
        }
    }
    
    pub fn length(&self) -> u32 {
        match self.val {
            Some(ref x) => return x.length(1),
            None => return 0,
        }
    }

    // ITERATOR FUNC
    pub fn to_iter(&self) -> ElemIter {
        ElemIter {
            prochain: match self.val {
                Some(ref x) => Some(x),
                None => None,
            }
        } 
    }
}

