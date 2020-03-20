use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
struct Error {
    msg: String,
}

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error {
        Error {
            msg: err.to_string(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error {
            msg: format!("{:?}", err),
        }
    }
}

fn main() -> Result<(), Error> {
    let dir = PathBuf::from(env::var("OUT_DIR")?);
    let path = dir.join("string.data");
    let mut file = File::create(path)?;
    let bytes = core::get_bytes();
    file.write_all(&bytes)?;
    Ok(())
}
