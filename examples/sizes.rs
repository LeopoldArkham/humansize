extern crate humanize;
use std::str;
use humanize::file_sizes::{FileSize, file_size_opts};

fn main() {
	println!("{}", (5456).file_size(file_size_opts::BINARY).unwrap());
	println!("{}", 1024.file_size(file_size_opts::BINARY).unwrap());
	println!("{}", 1023_654_123_654u64.file_size(file_size_opts::BINARY).unwrap());
	println!("{}", 1023_654_123_654u64.file_size(file_size_opts::DECIMAL).unwrap());
	println!("{}", 123456789.file_size(file_size_opts::CONVENTIONAL).unwrap());
}