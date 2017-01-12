extern crate humansize;
use humansize::{FileSize, file_size_opts as opts};

fn main() {
	// Declare a fully custom option struct
	let custom_options = opts::FileSizeOpts {
		divider: opts::Standard::Binary,
		units: opts::Standard::Decimal,
		decimal_places: 3,
		decimal_zeroes: 1,
		long_suffix: true,
		space: false,
		suffix: ""
	};
	// Then use it
	println!("{}", 3024.file_size(custom_options).unwrap());


	// Or use only some custom parameters and adopt the rest from an existing config
	let semi_custom_options = opts::FileSizeOpts {decimal_zeroes: 3, ..opts::DECIMAL};

	println!("{}", 1000.file_size(semi_custom_options).unwrap());
}
