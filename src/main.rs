use std::fs;
use std::path::Path;
use std::io::Error;
use std::env;
extern crate getopts;
use getopts::Options;

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  let matches = match opts.parse(&args[1..]) {
      Ok(m) => { m }
      Err(f) => { panic!("{}", f.to_string()) }
  };

  // TODO: Remove the following debug loop
  for (i, arg) in args.iter().enumerate() {
    println!("i: {}, arg: {}", i, arg);
  }
  println!("Hello, world!");
}

/// Reads a file from a given path into a buffer
/// Returns the buffer
fn read_file(f_path: &Path) -> Result<Vec<u8>, Error> {
  let f_buf = fs::read(f_path);
  f_buf
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs::File;
  use std::fs::remove_file;
  use std::io::prelude::*;

  #[test]
  fn read_file_simple() {
    // Creating a simple file
    let f_path = Path::new("./foo.txt");
    let mut f = File::create(f_path).unwrap();

    // Writing to file
    f.write_all(b"Hello, world!").unwrap();

    // Reading from file
    let f_buf = read_file(f_path);

    // Removing file
    remove_file(f_path).unwrap();
    let expected_vec = String::from("Hello, world!").into_bytes();
    assert_eq!(f_buf.unwrap(), expected_vec);
  }

  #[test]
  fn read_file_empty() {
    // Creating a simple file
    let f_path = Path::new("./foo2.txt");
    let _f = File::create(f_path).unwrap();

    // Reading from file
    let f_buf = read_file(f_path);

    // Removing file
    remove_file(f_path).unwrap();
    assert_eq!(f_buf.unwrap(), Vec::new());
  }

  #[test]
  fn read_file_nonexistent() {
    // Creating a simple file
    let f_path = Path::new("./foo3.txt");

    // Reading from file
    let f_buf = read_file(f_path);
    assert!(f_buf.is_err());
  }
}
