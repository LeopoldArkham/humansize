static SCALE_DECIMAL: [&'static str; 9] = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
static SCALE_DECIMAL_LONG: [&'static str; 9] = ["Byte", "Kilobyte", "Megabyte", "Gigabyte", "Terabyte", "Petabyte", "Exabyte", "Zettabyte", "Yottabyte"];

static SCALE_BINARY: [&'static str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
static SCALE_BINARY_LONG: [&'static str; 9] = ["Byte", "Kibibyte", "Mebibyte", "Gibibyte", "Tebibyte", "Pebibyte", "Exbibyte", "Zebibyte", "Yobibyte"];


pub mod file_size_opts {

    #[derive(PartialEq)]
    pub enum Standard {
        Decimal,
        Binary,
    }

    pub struct FileSizeOpts {
        pub divider: Standard,
        pub units: Standard,
        pub decimal_places: usize,
        pub decimal_zeroes: usize,
        pub long_scale: bool
    }

    pub const BINARY: FileSizeOpts = FileSizeOpts {
        divider: Standard::Binary,
        units: Standard::Binary,
        decimal_places: 2,
        decimal_zeroes: 0,
        long_scale: false
    };

    pub const DECIMAL: FileSizeOpts = FileSizeOpts {
        divider: Standard::Decimal,
        units: Standard::Decimal,
        decimal_places: 2,
        decimal_zeroes: 0,
        long_scale: false
    };

    pub const CONVENTIONAL: FileSizeOpts = FileSizeOpts {
        divider: Standard::Binary,
        units: Standard::Decimal,
        decimal_places: 2,
        decimal_zeroes: 0,
        long_scale: false
    };
}

pub trait FileSize {
	fn file_size(&self, opts: FileSizeOpts) -> Result<String, String>;
}

use self::file_size_opts::*;

macro_rules! impl_file_size_u {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
        	fn file_size(&self, opts: FileSizeOpts) -> Result<String, String> {
        		let divider = match opts.divider {
        			Standard::Decimal => 1000.0,
        			Standard::Binary => 1024.0
    			};
			
    			let mut size: f64 = *self as f64;
    			let mut scale_idx = 0;
			
    			while size >= divider {
    			    size /= divider;
    				scale_idx += 1;
    			}
			
    			let scale = match (opts.units, opts.long_scale) {
    				(Standard::Decimal, false) => SCALE_DECIMAL[scale_idx],
    				(Standard::Decimal, true) => SCALE_DECIMAL_LONG[scale_idx],
    				(Standard::Binary, false) => SCALE_BINARY[scale_idx],
    				(Standard::Binary, true) => SCALE_BINARY_LONG[scale_idx]
    			};
			
    			let places = match size.fract() {
    				0.0 => opts.decimal_zeroes,
    				_ => opts.decimal_places
    			};
			
    			Ok(format!("{:.*} {}", places, size, scale))
    		}
	    }
    )*)
}

macro_rules! impl_file_size_i {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {
        	fn file_size(&self, opts: FileSizeOpts) -> Result<String, String> {
        		if *self < 0 {return Err("Tried calling file_size on a negative value".to_owned());}
        		(*self as u64).file_size(opts)

    		}
	    }
    )*)
}

impl_file_size_u!(FileSize for usize u8 u16 u32 u64);
impl_file_size_i!(FileSize for isize i8 i16 i32 i64);


#[test]
fn test_sizes() {
	assert_eq!(0.file_size(BINARY).unwrap(), "0 B");
	assert_eq!(999.file_size(BINARY).unwrap(), "999 B");
	assert_eq!(1000.file_size(BINARY).unwrap(), "1000 B");
	assert_eq!(1000.file_size(DECIMAL).unwrap(), "1 KB");
	assert_eq!(1023.file_size(BINARY).unwrap(), "1023 B");
	assert_eq!(1023.file_size(DECIMAL).unwrap(), "1.02 KB");
	assert_eq!(1024.file_size(BINARY).unwrap(), "1 KiB");
	assert_eq!(1024.file_size(CONVENTIONAL).unwrap(), "1 KB");
}