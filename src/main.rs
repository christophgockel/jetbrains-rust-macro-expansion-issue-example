fn main() {
    match a_function() {
        Ok(_) => {}
        Err(e) => panic!("{}", format!("... {}", e)),
    };
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnError {
    #[error("Some description")]
    SomeError,
}

fn a_function() -> Result<(), AnError> {
    Ok(())
}
