use std::fs;
use std::path::Path;
use std::io::{self, Error, Write};
use clap::Parser;

#[allow(unused)]

#[derive(Parser)]
struct Args {
  /// Path to file to read
  path: std::path::PathBuf,
}

fn main() {
  let args = Args::parse();

  let f_path = args.path.as_path();
  let f_buf = read_file(f_path).expect("Could not read file.");
  let num_bytes = write_stdout(f_buf).expected("Error writing to file.";
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
