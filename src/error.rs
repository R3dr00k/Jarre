use std::fmt;

pub enum Errors {
    NoQueueError,
    NothingPopError,
    DirInexistant(String),
    CopyFailed,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::NoQueueError => {
                write!(f, "[Error]:[NoQueueError] Invalid queue .")
            },
            Errors::NothingPopError => {
                write!(f, "[Error]:[NothingPopError] There is nothing to pop")
            },
            Errors::DirInexistant(dir_name) => {
                write!(f, "[Error]:[DirInexistant] Invalid directory path : {}.", dir_name)
            },
            Errors::CopyFailed => {
                write!(f, "[Error]:[CopyFailed] copy failed")
            },
            _ => {
                write!(f, "[Error]:[NoFoundError] This error is not implemented yet!")
            }
        }
    }
}

