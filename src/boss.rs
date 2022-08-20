use crate::Queue;
use crate::Elem;
use crate::Errors;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

pub struct Boss {
    pub base_dir: PathBuf,
    queues: HashMap<String, Queue>,
}

impl Boss {
    pub fn init(dir: &str) -> Result<Self, Box<dyn Error>> {
        let mut base_dir = PathBuf::from(dir);

        if base_dir.is_dir() {
            println!("[+] valid dir path");
            base_dir.push("jarre_files");
            // if dir already exist -> update() witch search for the info file to get them
            fs::create_dir(base_dir.as_path())?;
            println!("[+] Base dir created .");
        } else {
            return Err(Errors::InexistantDir)?;
        }

        Ok(Boss {
            base_dir,
            queues: HashMap::new(),
        })
    }

    pub fn add_queue(&mut self, name: &str, queue: Queue) -> Result<(), Box<dyn Error>>{
        if self.queues.contains_key(name) {
            return Err(Errors::QueueAlreadyExist)?;
        }
        println!("[+] queue is not already existing .");

        let mut queue_dir = self.base_dir.clone();
        queue_dir.push(name);
        fs::create_dir(queue_dir)?;
        println!("[+] queue dir created .");
        
        self.queues.insert(name.to_string(), queue);
        Ok(())
    }

    // push & pop
    // push need to take the path of the file then copy or move (function to create)
    // it to the base directory and create elem with name of the file 
    // (pop_left is needed to).
    pub fn push_to(&mut self, q_name: &str, item_name: &str, num_of_pop: u32, pathfile: String) -> Result<(), Box<dyn Error>> {
        // if q exist copy pathfile to basedir/q_name/item_name
        // and add to the queue struct the shit
        if self.queues.contains_key(q_name) {
            let mut dest_path = self.base_dir.clone();
            dest_path.push(q_name);
            dest_path.push(item_name);

            fs::copy(pathfile, dest_path)?;
        } 

        match self.queues.get_mut(q_name) {
            Some(x) => {
                x.push(Elem::new(item_name, num_of_pop, 60*24));
                return Ok(());
            }
            None => return Err(Errors::NoQueueError)?,
        }
    }

    pub fn pop_to(&mut self, name: String) -> Result<Elem, Errors> {
        match self.queues.get_mut(&name) {
            Some(x) => {
                match x.pop() {
                    Some(y) => return Ok(y),
                    None => return Err(Errors::NothingPopError),
                }
            },
            None => return Err(Errors::NoQueueError),
        }
    }

    pub fn pop_index_to(&mut self, name: String, index: u32) -> Result<Elem, Errors> {
        match self.queues.get_mut(&name) {
            Some(x) => {
                match x.pop_index(index) {
                    Some(y) => return Ok(y),
                    None => return Err(Errors::NothingPopError),
                }
            },
            None => return Err(Errors::NoQueueError),
        }
    }
}

