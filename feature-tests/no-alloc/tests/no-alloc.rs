// This sub-crate exists to make sure that everything works well with the `no_alloc` flag enabled
use std::io::Write;

use humansize::{Formatter, DECIMAL};

#[test]
fn test() {
  let mut result: [u8; 4] = [0, 0, 0, 0];
  write!(&mut result[..], "{}", Formatter::new(1000usize, DECIMAL)).unwrap();
  assert_eq!(core::str::from_utf8(&result).unwrap(), "1 kB");
}