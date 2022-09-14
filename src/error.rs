use std::fmt;

#[derive(Debug)]
pub enum Errors {
    NoQueueError,
    NothingPopError,
    InexistantDir,
    CopyFailed,
    QueueAlreadyExist,
    QueueDoesntExist,
    StateFileDoesntExist,
}

impl std::error::Error for Errors {}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Errors::NoQueueError => {
                write!(f, "[Error]:[NoQueueError] Invalid queue .")
            },
            Errors::NothingPopError => {
                write!(f, "[Error]:[NothingPopError] There is nothing to pop")
            },
            Errors::InexistantDir => {
                write!(f, "[Error]:[DirInexistant] Invalid directory path .")
            },
            Errors::CopyFailed => {
                write!(f, "[Error]:[CopyFailed] copy failed")
            },
            Errors::QueueAlreadyExist => {
                write!(f, "[Error]:[QueueAlreadyExist] the queue already exist !")
            },
            Errors::StateFileDoesntExist => {
                write!(f, "[Error]:[StateFileDoesntExist] state don't exsit we try to create one now !")
            },
            _ => {
                write!(f, "[Error]:[NoFoundError] This error is not implemented yet!")
            }
        }
    }
}

