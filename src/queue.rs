use std::fmt;   // impl display

#[derive(PartialEq)]
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
    // PUSH 
    pub fn push(self, mut element: Elem) -> Elem {
        element.next = Some(Box::new(self));
        element
    }

    //POP add a feature for passing var as arg and fill it with val
    //                      CHAINE         POPED
    pub fn pop(mut self) -> (Option<Elem> ,Option<Elem>) {

        if self.pop_left > 1 {
            // here pop_left will be decremented and we return the list without modification
            self.pop_left -= 1;
            return (Some(self), None);
        } else {
            // here self will be poped c
            match self.next {
                Some(x) =>{
                    self.next = None;
                    return (Some(*x), Some(self));
                } 
                None => return (None, Some(self)),
            }
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
                self.val = Some(Elem::new(value.name, value.pop_left, value.time));
            },
        } 
    }

    pub fn pop(&mut self) -> Option<Elem> {
        match self.val.take(){
            Some(x) => {
                let ret_elem: Option<Elem>;
                (self.val, ret_elem) = x.pop();
                ret_elem
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

}

