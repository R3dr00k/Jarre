use crate::Queue;
use crate::Elem;
use crate::Errors;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Boss {
    base_dir: String,
    queues: HashMap<String, Queue>,
}

impl Boss {
    pub fn init(base_dir: String) -> Self {
        Boss {
            base_dir,
            queues: HashMap::new(),
        }
    }

    pub fn add_queue(&mut self, name: String, queue: Queue){
       self.queues.insert(name, queue);
    }

    // push & pop
    // push need to take the path of the file then copy or move (function to create)
    // it to the base directory and create elem with name of the file 
    // (pop_left is needed to).
    pub fn push_to(&mut self, q_name: String, pathfile: String, item_name: String,num_of_pop: u32) -> Result<(), Errors> {
        // copy(pathfile, destdir, q_name, item_name)
        let destpath = self.path_format(&q_name, &item_name)?;

        if let Err(_) = fs::copy(pathfile, destpath) {
            return Err(Errors::CopyFailed);
        }

        match self.queues.get_mut(&q_name) {
            Some(x) => {
                x.push(Elem::new(item_name, num_of_pop, 60*24));
                return Ok(());
            }
            None => return Err(Errors::NoQueueError),
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

    fn path_format(&self, q_name: &String, item_name: &String) -> Result<String, Errors>{
        // change that for concatenate
        let output = format!("{}{}{}", self.base_dir, '/', q_name);
        println!("{}", output);
        if !Path::new(&output).is_dir() {
            return Err(Errors::DirInexistant(output))
        }
        let output = format!("{}{}{}", output, '/', item_name);
        let output = demultiply(output);
        println!("[OUTPUT] = {}", output);
        Ok(output)
        }
}

fn demultiply(chaine: String) -> String {
    let mut splited = String::from("/");
    splited.push_str(&chaine.split('/').collect::<Vec<&str>>().join("/"));
    println!("{}", splited);
    return splited;
}
