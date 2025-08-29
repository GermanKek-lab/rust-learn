use std::{error::Error, fmt::{Formatter, Display}};

#[derive(Debug)]
enum CustomError {
    Error1(String),
    Error2(i32),
    Error3(i32, String),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Error1(msg) => write!(f, "Error1 {}", msg),
            Self::Error2(code) => write!(f, "Error2 {}", code),
            Self::Error3(code, msg) => write!(f, "Error3 {} - {}", msg, code),
        }
    }
}

impl Error for CustomError {}


fn do_smt() -> Result<(), CustomError> {
    Err(CustomError::Error1("Oh".to_owned()))
}


fn main() {
    match do_smt() {
        Ok(n) => println!("{:?}", n),
        Err(err) => println!("{}", err)
    }
}