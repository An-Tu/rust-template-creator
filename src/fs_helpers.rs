use std::fs::{File, create_dir};
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

pub fn read_file<P: AsRef<Path>>(path_to_file: P) -> Result<String> {
    if !path_to_file.as_ref().is_file() {
        return Err(Error::new(ErrorKind::NotFound, "file not found"));
    }
    let mut file = File::open(path_to_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

pub fn create_folder(path: &PathBuf) -> () {
    match create_dir(path) {
        Ok(_) => println!("folder created {:?}", path),
        Err(_) => panic!("can't create folder {:?}", path)
    }
}

pub fn create_file(path: &PathBuf, name: &str, extension: &str, data: &str) -> () {
    let mut file_name = path.join(name);
    file_name.set_extension(extension);

    let mut file = match File::create(&file_name) {
        Ok(f) => f,
        Err(_) => panic!("can't create file {:?}", file_name)
    };

    match file.write(data.as_bytes()) {
        Ok(_) => println!("file created {:?}", file_name),
        Err(_) => panic!("can't create file {:?}", file_name)
    }
}
