
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

    pub fn add(&mut self, element: Elem) {
        match self.next {
            Some(ref mut x) => x.add(element),
            None =>{
                self.next = Some(Box::new(element));
            }
        }
    }

    pub fn last(&self) -> &Elem {
        match self.next{
            Some(ref x) => x.last(),
            None => self
        }
    }

    pub fn previous(&self, elem: &Elem) -> Option<&Elem> {
        match self.next {
            Some(ref x) => {
                if x.as_ref() == elem {
                    Some(self)
                } else {
                    x.previous(elem);
                }
            },
            None => None, 
        }
    }

    //pub fn pop(&self) -> Option<Elem> {
    //}
}

// ======================== QUEUE =======================

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
