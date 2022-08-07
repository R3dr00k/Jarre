use std::fmt;   // impl display

#[derive(PartialEq, Clone)]
pub struct Elem {   // each element file or dir
    name: String,
    pop_left: u32,
    time: u32,      // will be change later
    next: Option<Box<Elem>>,    // chain list in here
} 

impl Elem {
    pub fn new(name: String, pop_left: u32, time: u32) -> Self {
        Elem {
            name,
            pop_left,
            time,
            next: None,     // first element nothing after it
        }
    }

    pub fn push(&mut self, element: Elem) {
        match self.next {
            Some(ref mut x) => x.push(element),
            None =>{
                self.next = Some(Box::new(element));
            }
        }
    }

    pub fn lastone(&self) -> Option<&Elem>{

        match self.next {
            Some(ref x) => match x.next {
                Some(_) => return x.lastone(),
                None => return Some(self),
            }
            None => {
                return None
            },
        }
    }


    // maybe take the owner ship instead of getting 1 ref & 1 ref mut
    //
    // pop the last elem in the list and replace the pointer of the one before it to none
    pub fn pop(&mut self) -> Option<Elem> {
        match self.lastone() {
            // move x.next -> last et x.next=None puis renvoyer last
            Some(x) => {
                println!("The lastone: {}", x);
                let last = x.next.clone();
                x.next = None;
                match last {
                    Some(y) => return Some(*y),
                    None => return None,
                }
            },
            // case where there is only one element
            None => None, 
        }
    }

    pub fn chainprint(&self) {
        match &self.next {
            Some(x) => {
                print!("{} -> ", self);
                x.chainprint();
            }
            None => {
                println!("{}", self);
            }
        } 
    }
}

// DISPLAY FOR ELEM

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.pop_left, self.time)
    }
}

// ======================== QUEUE =======================

/*
pub struct Queue {
    val: Option<Elem>,
    life_time: u32,     // time in sec
}

impl Queue{
    pub fn create(life_time: u32) -> Self {
        Queue {
            val: None,
            life_time,
        }
    }
}
*/
