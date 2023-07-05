use std::fs;
use std::path::Path;
use std::io::{self, Error, Write};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  if args.len() <= 1 {
    std::process::exit(exitcode::USAGE);
  }

  // let mut opts = Options::new();
  // let matches = match opts.parse(&args[1..]) {
  //     Ok(m) => { m }
  //     Err(f) => { panic!("{}", f.to_string()) }
  // };

  // TODO: Remove the following debug loop
  for (i, arg) in args.iter().enumerate() {
    println!("i: {}, arg: {}", i, arg);
  }
  let f_path = args[1].clone();
  let f_buf = read_file(Path::new(f_path.as_str())).unwrap();
  let num_bytes = write_stdout(f_buf).unwrap();
  println!("num_bytes: {}", num_bytes);
  println!("Rust Cat Run!");
}

/// Reads a file from a given path into a buffer
/// Returns the buffer
fn read_file(f_path: &Path) -> Result<Vec<u8>, Error> {
  let f_buf = fs::read(f_path);
  f_buf
}

/// Writes out a file buffer to stdout
fn write_stdout(f_buf: Vec<u8>) -> Result<usize, io::Error> {
  let mut stdout = io::stdout().lock();
  let num_bytes = stdout.write(f_buf.as_slice());
  num_bytes
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

  #[test]
  fn write_stdout_small_buffer() {
    // Creating small Vec<u8> buffer
    let buf = String::from("Hello, world!").into_bytes();
    let num_bytes = write_stdout(buf);
    assert_eq!(num_bytes.unwrap(), 13);
  }
}
