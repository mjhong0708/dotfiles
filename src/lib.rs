pub mod commands;
pub mod shell;

use std::fmt;

#[derive(Debug)]
pub enum DotfilesError {
    Io(std::io::Error),
    Shell(String),
    Tool(String),
}

impl fmt::Display for DotfilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DotfilesError::Io(err) => write!(f, "IO error: {}", err),
            DotfilesError::Shell(msg) => write!(f, "Shell error: {}", msg),
            DotfilesError::Tool(msg) => write!(f, "Tool error: {}", msg),
        }
    }
}

impl std::error::Error for DotfilesError {}

impl From<std::io::Error> for DotfilesError {
    fn from(err: std::io::Error) -> Self {
        DotfilesError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, DotfilesError>;

macro_rules! info {
    ($($arg:tt)*) => {
        println!("\x1b[32m[INFO]\x1b[0m {}", format!($($arg)*));
    };
}

macro_rules! warning {
    ($($arg:tt)*) => {
        println!("\x1b[33m[WARN]\x1b[0m {}", format!($($arg)*));
    };
}

macro_rules! error {
    ($($arg:tt)*) => {
        eprintln!("\x1b[31m[ERROR]\x1b[0m {}", format!($($arg)*));
    };
}

pub(crate) use {info, warning, error};