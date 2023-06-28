use std::fs;
use std::path::Path;
use std::io::prelude::*;

fn main() {
  println!("Hello, world!");
}

fn read_file(f_path: &Path) -> String {
  // let f = fs::File::open(f_path).unwrap();
  // let f_buf = String::new();
  let f_buf = fs::read(f_path).unwrap();
  return String::from_utf8(f_buf).unwrap();
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
    f.write_all(b"Hello, world!").unwrap();
    let f_buf = read_file(f_path);
    remove_file(f_path).unwrap();
    assert_eq!(f_buf, String::from("Hello, world!"));
  }
}
