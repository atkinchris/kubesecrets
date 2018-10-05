use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

pub fn write_file(file_path: &str, contents: &str) -> io::Result<()> {
  let path = Path::new(&file_path);
  let display = path.display();
  let mut file = match File::create(path) {
    Err(why) => panic!("couldn't create {}: {}", display, why.description()),
    Ok(file) => file,
  };

  match file.write_all(contents.as_bytes()) {
    Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
    Ok(_) => (),
  }

  return Ok(());
}

pub fn read_file(file_path: &str) -> io::Result<String> {
  let path = Path::new(&file_path);
  let display = path.display();
  let mut file = match File::open(path) {
    Err(why) => {
      return Err(io::Error::new(
        ErrorKind::NotFound,
        format!("couldn't open {}: {}", display, why.description()),
      ))
    }
    Ok(file) => file,
  };

  let mut contents = String::new();

  match file.read_to_string(&mut contents) {
    Err(why) => panic!("couldn't read from {}: {}", display, why.description()),
    Ok(_) => Ok(contents),
  }
}
