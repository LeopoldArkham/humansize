//! # **Humansize**
//!
//! Humansize lets you easily represent file sizes in a human-friendly format.
//! You can specify your own formatting style, pick among the three defaults provided
//! by the library:
//!
//! * Decimal (Multiples of 1000, `KB` units)
//! * Binary (Multiples of 1024, `KiB` units)
//! * Conventional (Multiples of 1024, `KB` units)
//!
//! ## How to use it
//!
//! Simply import the `FileSize` trait and the options module and call the
//! file_size method on any positive integer, using one of the three standards
//! provided by the options module.
//!
//! ```rust
//! use humansize::{FileSize, file_size_opts as options};
//!
//! fn main() {
//! 	let size = 1000;
//! 	println!("Size is {}", size.file_size(options::DECIMAL).unwrap());
//!
//! 	println!("Size is {}", size.file_size(options::BINARY).unwrap());
//!
//! 	println!("Size is {}", size.file_size(options::CONVENTIONAL).unwrap());
//! }
//! ```
//!
//! If you wish to customize the way sizes are displayed, you may create your own custom `FileSizeOpts` struct
//! and pass that to the method. See the `custom_options.rs` file in the example folder.

mod scales;
pub mod file_size_opts;


/// The trait for the `file_size` method
pub trait FileSize {
    /// Formats self according to the parameters in `opts`. `opts` can either be one of the
    /// three defaults providedby the `file_size_opts` module, or be custom-defined according
    /// to your needs
    ///
    /// # Errors
    /// Will fail by default if called on a negative number. Override this behavior by setting
    /// `allow_negative` to `True` in a custom options struct.
    ///
    /// # Examples
    /// ```rust
    /// use humansize::{FileSize, file_size_opts as options};
    ///
    /// let size = 5128;
    /// println!("Size is {}", size.file_size(options::DECIMAL).unwrap());
    /// ```
    ///
    fn file_size<T: AsRef<FileSizeOpts>>(&self, opts: T) -> Result<String, String>;
}

fn f64_eq(left: f64, right: f64) -> bool {
    left == right || (left - right).abs() <= std::f64::EPSILON
}

use self::file_size_opts::*;


macro_rules! impl_file_size_u {
    (for $($t:ty)*) => ($(
        impl FileSize for $t {
            fn file_size<T: AsRef<FileSizeOpts>>(&self, _opts: T) -> Result<String, String> {
                let opts = _opts.as_ref();
                let divider = match opts.divider {
                    Kilo::Decimal => 1000.0,
                    Kilo::Binary => 1024.0
                };

                let mut size: f64 = *self as f64;
                let mut scale_idx = 0;

                match opts.fixed_at {
                    FixedAt::No => {
                        while size >= divider {
                            size /= divider;
                            scale_idx += 1;
                        }
                    }
                    val => {
                        while scale_idx != val as usize {
                            size /= divider;
                            scale_idx += 1;
                        }
                    }
                }

                let mut scale = match (opts.units, opts.long_units) {
                    (Kilo::Decimal, false) => scales::SCALE_DECIMAL[scale_idx],
                    (Kilo::Decimal, true) => scales::SCALE_DECIMAL_LONG[scale_idx],
                    (Kilo::Binary, false) => scales::SCALE_BINARY[scale_idx],
                    (Kilo::Binary, true) => scales::SCALE_BINARY_LONG[scale_idx]
                };

                // Remove "s" from the scale if the size is 1.x
                if opts.long_units && f64_eq(size.trunc(), 1.0) { scale = &scale[0 .. scale.len()-1]; }

                let places = if f64_eq(size.fract(), 0.0) {
                    opts.decimal_zeroes
                } else {
                    opts.decimal_places
                };

                let space = if opts.space {" "} else {""};

                Ok(format!("{:.*}{}{}{}", places, size, space, scale, opts.suffix))
            }
        }
    )*)
}


macro_rules! impl_file_size_i {
    (for $($t:ty)*) => ($(
        impl FileSize for $t {
            fn file_size<T: AsRef<FileSizeOpts>>(&self, _opts: T) -> Result<String, String> {
                let opts = _opts.as_ref();
                if *self < 0 && !opts.allow_negative {
                    return Err("Tried calling file_size on a negative value".to_owned());
                } else {
                    let sign = if *self < 0 {
                        "-"
                    } else {
                        ""
                    };

                    Ok(format!("{}{}", sign, (self.abs() as u64).file_size(opts)?))
                }

            }
        }
    )*)
}


impl_file_size_u!(for usize u8 u16 u32 u64);
impl_file_size_i!(for isize i8 i16 i32 i64);
