use std::fs::File;
// use std::io::ErrorKind;

use std::fs;
use std::io;
use std::io::Read;
pub fn base() {
  let f = File::open("hello.txt")?;
    //  let mut s = String::new();

    // fs::read_to_string(&mut s)?;

    // Ok(s)
    // =--------------------
  //  let f = File::open("hllo.txt");

  //   let f = match f {
  //       Ok(file) => file,
  //       Err(error) => match error.kind() {
  //           ErrorKind::NotFound => match File::create("h.txt") {
  //               Ok(fc) => fc,
  //               Err(e) => panic!("Problem creating the file: {:?}", e),
  //           },
  //           other_error => {
  //               panic!("Problem opening the file: {:?}", other_error)
  //           }
  //       },
  //   };
}