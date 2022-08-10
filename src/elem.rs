use std::fmt;   // impl display

pub struct Elem {   // each element file or dir
    pub name: String,
    pub pop_left: u32,
    pub time: u32,      // will be change later
    pub next: Option<Box<Elem>>,    // chain list in here
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
    pub fn pop(mut self) -> (Option<Box<Elem>> ,Option<Elem>) {

        if self.pop_left > 1 {
            // here pop_left will be decremented and we return the list without modification
            self.pop_left -= 1;
            return (Some(Box::new(self)), None);
        } else {
            // here self will be poped c
            match self.next {
                Some(x) =>{
                    self.next = None;
                    return (Some(x), Some(self));
                } 
                None => return (None, Some(self)),
            }
        }

    }

    pub fn pop_index(mut self, index: u32, len: u32, mut count: u32) -> (Option<Box<Elem>>, Option<Elem>) {
        if index == 0 {
            // pop()
            return self.pop();
        } else if index >= len {
            // ret NONE
            return(Some(Box::new(self)), None);
        } else {
            // Normal case here
            if count != (index-1) {
                match self.next.take() {
                    Some(x) => {
                        count += 1;
                        // Probleme here returned self is droped cause we don't recup it
                        let returned_self: Option<Box<Elem>>;
                        let ret: Option<Elem>;
                        (returned_self, ret) = x.pop_index(index ,len, count);
                        self.next = returned_self;
                        return (Some(Box::new(self)), ret);
                        
                    },
                    None => return (Some(Box::new(self)), None),
                }
            } else {
                match self.next.take() {
                    Some(mut x) => {
                        if x.pop_left > 1 {
                            // decremented here !! self.next must be dropped
                            x.pop_left -= 1; 
                            self.next = Some(x);
                            return (Some(Box::new(self)), None);
                        } else {
                            // returned here
                            self.next = x.next;
                            x.next = None;
                            return (Some(Box::new(self)), Some(*x));
                        }
                    },
                    None => return (Some(Box::new(self)), None),
                }
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

    pub fn length(&self, mut count: u32) -> u32 {
        match self.next {
            Some(ref x) => {
                count += 1;
                return x.length(count);
            },
            None => count
        }
    }
}


// DISPLAY FOR ELEM

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.pop_left, self.time)
    }
}
