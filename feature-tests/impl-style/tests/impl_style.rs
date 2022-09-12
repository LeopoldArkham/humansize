// This sub-crate exists to make sure that everything works well with the `impl-style` flag enabled

use humansize::{FormatSize, FormatSizeI, DECIMAL};

#[test]
fn test_impl_style() {
  assert_eq!(1000u64.format_size(DECIMAL), "1 kB");
  assert_eq!((-1000).format_size_i(DECIMAL), "-1 kB");
}