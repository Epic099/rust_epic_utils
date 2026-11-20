use core::panic;
use std::{fs, path, any::Any, collections::HashMap};
use serde_json;

pub struct File {
    pub path: String
}
impl File {
    pub fn new(filepath: &str, create: bool) -> Self {
        let pathbuf = path::absolute(&filepath).unwrap_or_default();
        let abs_path = pathbuf.to_str().unwrap();
        if !fs::exists(&abs_path).expect("Checking whether file exists failed.") {
            if create == true {
                fs::write(abs_path, "");
            }else{
                panic!("File does not exist")
            }
        }
        Self {
            path: abs_path.to_string()
        }
    }
    pub fn exists(&self) -> bool {
        return fs::exists(&self.path).expect("Checking whether file exists failed.");
    }
    pub fn read(&self) -> String {
        if self.exists() == false { return String::new()}
        return fs::read_to_string(&self.path).expect("Failed to read file");
    }
    pub fn read_json(&self) -> serde_json::Result<serde_json::Value> {
        let content = self.read();
        let result: serde_json::Value = serde_json::from_str(content.as_str()).expect("Invalid json");
        Ok(result)
    }
}