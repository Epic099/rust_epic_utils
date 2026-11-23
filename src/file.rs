    use std::{fs, path, io};
    use serde_json;
    use crate::err::{self, FileError};

    pub struct File {
        pub path: path::PathBuf
    }
    impl File {
        pub fn new(filepath: &str, create: bool) -> io::Result<Self> {
            let path = path::absolute(&filepath)?;
            if !path.try_exists()? {
                if create == true {
                    fs::write(&path, "")?;
                }else{
                    return Err(io::Error::new(io::ErrorKind::NotFound, "file does not exist at given path"))
                }
            }
            Ok(Self {path})
        }
        pub fn exists(&self) -> bool {
            self.path.exists()  
        }
        pub fn read(&self) -> io::Result<String> {
            fs::read_to_string(&self.path)
        }
        pub fn read_json(&self) -> Result<serde_json::Value, err::FileError> {
            let content = self.read()?;
            let result: serde_json::Value = serde_json::from_str(content.as_str())?;
            Ok(result)
        }
        pub fn write(&self, data: &str) -> Result<(), io::Error> {
            fs::write(&self.path, data)
        }
        pub fn write_json(&self, value: &serde_json::Value) -> Result<(), FileError> {
            let text = serde_json::to_string_pretty(value)?;
            Ok(self.write(&text)?)
        }
    }