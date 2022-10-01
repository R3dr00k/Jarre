use crate::{Errors, CONFIG_DIR, JARRE_CONFIG};
use std::path::Path;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::io::prelude::*;


pub struct Config {
    pub base_dir: String,
}

impl Config {
    pub fn setup() -> Result<Self, Box<dyn Error>> {
        /*IF there is a config dir & a config file -> try to read it 
         * ELSE create them with default value */

        if !Path::new(CONFIG_DIR).is_dir() {
            Config::create_config_file()?;
            print!("Aucun fichier de configuration trouvé . base_dir = /var/lib/jarre");

            if !Path::new("/var/lib/jarre").is_dir() {
                fs::create_dir("/var/lib/jarre")?;
            }
            return Ok(Config {base_dir: String::from("/var/lib/jarre")});

        } else {
            if Path::new(JARRE_CONFIG).is_file() {
                let base_dir = Config::get("base_dir")?;

                if Path::new(&base_dir).is_dir(){
                    return Ok(Config{base_dir});
                } else {
                    return Err(Errors::NoSuchDir)?
                }

            } else {

                print!("Aucun fichier de configuration trouvé . base_dir = /var/lib/jarre");

                if !Path::new("/var/lib/jarre").is_dir() {
                    fs::create_dir("/var/lib/jarre")?;
                }
                return Ok(Config {base_dir: String::from("/var/lib/jarre")});
            }
        }
    }


    fn create_config_file() -> Result<(), Box<dyn Error>> {
        fs::create_dir(CONFIG_DIR)?;
        
        let mut file = File::create(JARRE_CONFIG)?;
        // default content
        file.write(b"#Jarre configuration file \nbase_dir=/var/lib/jarre")?;

        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o644);

        Ok(())
    }

    fn get(key: &str) -> Result<String, Box<dyn Error>> {
        if Path::new(JARRE_CONFIG).is_file() {
            if key.starts_with("#") {
                return Err(Errors::ConfigWrongKey)?
            }

            let mut file = File::open(JARRE_CONFIG)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            for line in content.lines(){
                if line.starts_with(key) {
                    match line.split("=").last() {
                        Some(x) => return Ok(x.to_string()),
                        None => return Err(Errors::ConfigFileWrongFormat)?
                    }
                }
            }
            return Err(Errors::ConfigWrongKey)?;

        } else {
            return Err(Errors::NoConfigFile)?
        }
        
    }
}
