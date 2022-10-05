// This sub-crate exists to make sure that everything works well with the `no_alloc` flag enabled
use core::fmt::Write;

use humansize::{SizeFormatter, DECIMAL};


struct Buffer<const N: usize>([u8; N], usize);

impl<const N: usize> Write for Buffer<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let space_left = self.0.len() - self.1;
        if space_left >= s.len() {
            self.0[self.1..][..s.len()].copy_from_slice(s.as_bytes());
            self.1 += s.len();
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

#[test]
fn test() {
  let mut result = Buffer([0u8; 4], 0);
  write!(&mut result, "{}", SizeFormatter::new(1000usize, DECIMAL)).unwrap();
  assert_eq!(core::str::from_utf8(&result.0).unwrap(), "1 kB");
}