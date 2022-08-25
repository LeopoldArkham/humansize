extern crate humansize;
//Import the trait and the options module
use humansize::{format_size, BINARY, DECIMAL, CONVENTIONAL};

fn main() {
    // Call the file_size method on any non-negative integer with the option set you require

    println!("{}", format_size(5456usize, BINARY));
    println!("{}", format_size(1024usize, BINARY));
    println!("{}", format_size(1000usize, DECIMAL));
    println!("{}", format_size(1023_654_123_654u64, DECIMAL));
    println!("{}", format_size(123456789usize, CONVENTIONAL));
}
