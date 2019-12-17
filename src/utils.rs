use serde::Serialize;
use serde_json::Error as JSONError;
use std::fs::File;
use std::io::{Error as IOError, ErrorKind};

#[inline]
pub fn panic_error(text: &str) -> ! {
    error!("{}", text);
    panic!("{}", text);
}

#[inline]
pub fn load_dotenv() {
    #[cfg(debug_assertions)]
    dotenv::from_filename(".debug.env").ok();

    #[cfg(not(debug_assertions))]
    dotenv::from_filename(".release.env").ok();
}

#[inline]
pub fn init_file_if_not_exists<T: Default + Serialize>(
    filename: &str,
) -> Result<bool, FileInitError> {
    match File::open(filename) {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                info!(
                    r#"File "{}" not found, creating it now."#,
                    filename
                );

                let file = File::create(filename)?;

                serde_json::to_writer(&file, &T::default())?;

                Ok(true)
            }
            _ => Err(FileInitError::from(e)),
        },
        Ok(_) => Ok(false),
    }
}

#[derive(Debug)]
pub enum FileInitError {
    IO(IOError),
    JSON(JSONError),
}

impl From<IOError> for FileInitError {
    fn from(e: IOError) -> Self {
        Self::IO(e)
    }
}

impl From<JSONError> for FileInitError {
    fn from(e: JSONError) -> Self {
        Self::JSON(e)
    }
}
