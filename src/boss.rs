use crate::{Queue, Elem, Errors, CONFIG_DIR, JARRE_STATE};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
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

        // if /var/lib/jarre doesn't exist create it
        if !PathBuf::from("/var/lib/jarre").is_dir() {
            fs::create_dir("/var/lib/jarre")?;
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
            let mut dest_path = self.base_dir.clone();
            dest_path.push(q_name);
            dest_path.push(item_name);

            fs::copy(pathfile, dest_path)?;
            println!("[+] file : {} copied sucessfully", pathfile);
        } 

        match self.queues.get_mut(q_name) {
            Some(x) => {
                x.push(Elem::new(item_name, num_of_pop, 60*24));
                println!("[+] elem added to queue");
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

                        let mut remove_elem = self.base_dir.clone();
                        remove_elem.push(name);
                        remove_elem.push(&y.name);
                        fs::remove_file(remove_elem)?;

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

                        let mut remove_elem = self.base_dir.clone();
                        remove_elem.push(name);
                        remove_elem.push(&y.name);
                        fs::remove_file(remove_elem)?;
                        return Ok(y);
                    }
                    None => return Err(Errors::NothingPopError)?,
                }
            },
            None => return Err(Errors::NoQueueError)?,
        }
    }
    
    // write info to /var/lib/jarre/state.jarre
    // if /jarre dir doesn't exist create it , else replace state.jarre
    // file format : 
    /*
     base_dir
     queue_name:life_time:name,pop_left,time->
     queue_name2:life_time:name,pop_left,time->
     */
    pub fn export_as_file(&self) -> Result<(), Box<dyn Error>>{
        let mut export_content = String::from(self.base_dir.to_str().unwrap());
        // pour chaque clé : ajouter clé/sep/
        for name in self.queues.keys() {
            export_content.push('\n');  // big separator
            export_content.push_str(name);
            export_content.push(':'); // moyen sep
            {
                export_content.push_str(&self.queues.get(name).unwrap().get_life_time().to_string())
            }
            export_content.push(':'); // moyen sep

            for i in self.queues[name].to_iter() {
                export_content.push_str(&format!("{},{},{}->", i.name, i.pop_left, i.time));
            }            
        }
        println!("{}", export_content);

        if !PathBuf::from(CONFIG_DIR).is_dir() {
            fs::create_dir(CONFIG_DIR)?;
        }
        
        fs::write(JARRE_STATE , export_content)?;

        Ok(())
    }

    // if jarre state already exist try to read his content 
    pub fn init_from_file() -> Result<Self, Box<dyn Error>> {
        /*Read /var/lib/jarre/state.jarre and init boss with it */
        // NEED TO ADD VERIFICATION DURING INITIALISATION (QUEUE NAME, ITEM NAME, THE REST CANNOT
        // BE CHECKED)

        if !PathBuf::from(JARRE_STATE).is_file() {
            return Err(Errors::StateFileDoesntExist)?;
        } else {
            // read file here and create boss with it 
            let base_dir: PathBuf;
            let mut queues: HashMap<String, Queue> = HashMap::new();
            
            let mut content = String::new();
            let mut state_file = fs::File::open(JARRE_STATE).unwrap(); // more check here
            state_file.read_to_string(&mut content)?;
           
            let mut lines = content.lines();
            let line = lines.next().unwrap();
            base_dir = PathBuf::from(line);

            for l in lines {
                let v: Vec<&str> = l.split(":").collect();
                if v.len() == 3 { // if there is no elem v[2] = "" need to handle that
                    queues.insert(v[0].to_string(), Queue::create(v[1].parse::<u32>().unwrap()));

                    let elems: Vec<&str> = v[2].split_terminator("->").collect();
                    for elem in elems {
                       let fields: Vec<&str> = elem.split(",").collect(); 
                       if fields.len() == 3 {
                           // si il y a les trois champs d'un élem on ajoute a la queue qu'on vient
                           // de creer
                           queues.get_mut(v[0]).unwrap().push(Elem::new(fields[0], fields[1].parse::<u32>().unwrap(), fields[2].parse::<u32>().unwrap()));
                           println!("Init boss from file : elem push {:?} to {}", fields, v[0]);
                       } // else wrong format
                    }
                } // else wrong format
            }
            
            Ok(Self{
                base_dir,
                queues,
            }) 
        }
    }
}

