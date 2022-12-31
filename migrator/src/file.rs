use std::env;
use std::io::Result;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileReader {}

impl FileReader {
    fn current_dir(self) -> Result<PathBuf> {
        env::current_dir()
    }

}
