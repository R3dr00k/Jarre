use std::collections::HashMap;
use crate::Queue;

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
        
    }


    // FUNCTIONS TO IMPL
    // EXPORT_AS_FILE
    // READ_CONFIG_FILE
    //
}
