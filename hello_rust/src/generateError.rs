//первым делом в "Cargo.toml" нужно добавить thiserror = "1.0.44"

use thiserror::Error;

#[derive(Debug, Error)]
enum CustomError {
    #[error("Error1 with code {0}")]
    Error1(i32),
    #[error("Error2")]
    Error2,
    #[error("Error3 with code {0}: {1}")]
    Error3(i32, String)
}

fn do_smt() -> Result<(), CustomError> {
    Err(CustomError::Error2)
}


fn main() {
    if let Err(err) = do_smt() {
        println!("{}", err);
    }
}