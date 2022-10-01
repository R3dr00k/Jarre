use crate::{Queue, Elem, Errors};
use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

mod config;
use config::Config;

pub struct Boss {
    pub config: Config,
    queues: HashMap<String, Queue>,
}

impl Boss {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        // if /etc/jarre/jarre.config doesn't exists -> create it
        // else try read config to get base_dir
        // if there is base_dir -> build from file
        // else create new base_dir  by asking user
        
        let config = Config::setup()?;

        Ok(Boss {
            config,
            queues: HashMap::new(),
        })
    }
    pub fn add_queue(&mut self, name: &str, queue: Queue) -> Result<(), Box<dyn Error>>{ 
        println!("Add queue");
        if self.queues.contains_key(name) { 
            return Err(Errors::QueueAlreadyExist)?;
        }
        println!("[+] queue is not already existing .");

        let mut queue_dir = PathBuf::from(&self.config.base_dir);
        queue_dir.push(name);
        fs::create_dir(queue_dir)?;
        // create .state + change .state of base_dir
        Boss::save_changes(PathBuf::from(&self.config.base_dir), &format!("{}:{}\n", name, queue.get_life_time()))?;

        println!("[+] queue dir created .");
        
        self.queues.insert(name.to_string(), queue);

        Ok(())
    }

    pub fn get_queue(&self, name: &str) -> Option<&Queue> {
        if self.queues.contains_key(name) {
            return Some(&self.queues[name]);
        } else {
            return None
        }
    }

    // push & pop
    // push need to take the path of the file then copy or move (function to create)
    // it to the base directory and create elem with name of the file 
    // (pop_left is needed to).
    pub fn push_to(&mut self, q_name: &str, item_name: &str, num_of_pop: u32, pathfile: &str) -> Result<(), Box<dyn Error>> {
        if self.queues.contains_key(q_name) {
            println!("[+] La queue {} existe !", q_name);
            let mut dest_path = PathBuf::from(&self.config.base_dir);
            dest_path.push(q_name);
            dest_path.push(item_name);

            fs::copy(pathfile, dest_path)?;
            println!("[+] file : {} copied sucessfully", pathfile);
        } 


        match self.queues.get_mut(q_name) {
            Some(x) => {
                x.push(Elem::new(item_name, num_of_pop, 60*24));
                println!("[+] elem added to queue");

                // save the changes in .state file
                let mut queue_dir = PathBuf::from(&self.config.base_dir);
                queue_dir.push(q_name);
                Boss::save_changes(queue_dir, &format!("{}:{}:{}", item_name, num_of_pop, pathfile));

                return Ok(());
            }
            None => return Err(Errors::NoQueueError)?,
        }
    }

    pub fn pop_to(&mut self, name: &str) -> Result<Elem, Box<dyn Error>> {
        match self.queues.get_mut(name) {
            Some(x) => {
                match x.pop() {
                    Some(y) => {

                        let mut remove_elem = PathBuf::from(&self.config.base_dir);
                        remove_elem.push(name);
                        remove_elem.push(&y.name);
                        fs::remove_file(remove_elem)?;

                        // removes the changes in the .state file
                        let mut queue_dir = PathBuf::from(&self.config.base_dir);
                        queue_dir.push(name);
                        Boss::remove_changes(queue_dir, &y.name);

                        return Ok(y);
                    }
                    None => return Err(Errors::NothingPopError)?,
                }
            },
            None => return Err(Errors::NoQueueError)?,
        }
    }

    pub fn pop_index_to(&mut self, name: &str, index: u32) -> Result<Elem, Box<dyn Error>> {
        match self.queues.get_mut(name) {
            Some(x) => {
                match x.pop_index(index) {
                    Some(y) => {

                        let mut remove_elem = PathBuf::from(&self.config.base_dir);
                        remove_elem.push(name);
                        remove_elem.push(&y.name);
                        fs::remove_file(remove_elem)?;

                        // removes the changes in the .state file
                        let mut queue_dir = PathBuf::from(&self.config.base_dir);
                        queue_dir.push(name);
                        Boss::remove_changes(queue_dir, &y.name);
                        return Ok(y);
                    }
                    None => return Err(Errors::NothingPopError)?,
                }
            },
            None => return Err(Errors::NoQueueError)?,
        }
    }
    
    fn save_changes(mut state: PathBuf, changes: &str) -> Result<(), Box<dyn Error>> {
        println!("Here");
        if state.is_dir() {
            state.push(".state");

            if !state.is_file() {
                fs::File::create(&state)?;
            } 
            println!(".state file : {}", state.to_str().expect("failed to oepn"));

            let mut file = OpenOptions::new().append(true).open(state).expect("Unable to open file");   
            file.write_all(changes.as_bytes()).expect("write failed");        
            Ok(())

        } else {
            Err(Errors::NoSuchDir)? 
        }
    }

    fn remove_changes(mut state: PathBuf, changes: &str) -> Result<(), Box<dyn Error>> {
        if state.is_dir() {
            state.push(".state");

            if !state.is_file() {
                fs::File::create(state)?;
                return Err(Errors::NoSuchFile)?
            } 

            let content = fs::read_to_string(&state).unwrap();
            let mut new_content = String::new();

            for line in content.lines(){
                if !line.starts_with(changes) {
                    new_content.push_str(line);
                }
            }
            let mut file = fs::File::open(state).expect("Failed to open .state");
            file.write(new_content.as_bytes());
            Ok(())

        } else {
            Err(Errors::NoSuchDir)? 
        }
        
    }
}

