/*!
# **Humansize**

Humansize lets you easily represent file sizes in a human-friendly format.
You can specify your own formatting style, pick among the three defaults provided
by the library:

* Decimal (Multiples of 1000, `KB` units)
* Binary (Multiples of 1024, `KiB` units)
* Conventional (Multiples of 1024, `KB` units)

## How to use it

Simply import the `FileSize` trait and the options module and call the
file_size method on any positive integer, using one of the three standards
provided by the options module.

```rust
extern crate humansize;
use humansize::FileSize;

fn main() {
	let size = 1000;
	println!("Size is {}", size.file_size(humansize::DECIMAL).unwrap());

	println!("Size is {}", size.file_size(humansize::BINARY).unwrap());

	println!("Size is {}", size.file_size(humansize::CONVENTIONAL).unwrap());
}
```

If you wish to customize the way sizes are displayed, you may create your own custom `FormatSizeOptions` struct
and pass that to the method. See the `custom_options.rs` file in the example folder.
*/
pub mod file_size_opts;
mod scales;

fn f64_eq(left: f64, right: f64) -> bool {
    left == right || (left - right).abs() <= std::f64::EPSILON
}

pub use self::file_size_opts::{FormatSizeOptions, BINARY, CONVENTIONAL, DECIMAL};
use self::file_size_opts::{FixedAt, Kilo};

pub trait ToF64 {
    fn to_f64(&self) -> f64;
}

pub trait Unsigned {}

impl Unsigned for usize {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}

impl ToF64 for usize {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for u8 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for u16 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for u32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for u64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for isize {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for i8 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}
impl ToF64 for i16 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for i32 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

impl ToF64 for i64 {
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}

pub fn format_size_i(input: impl ToF64, _options: impl AsRef<FormatSizeOptions>) -> String {
    let opts = _options.as_ref();
    let divider = match opts.divider {
        Kilo::Decimal => 1000.0,
        Kilo::Binary => 1024.0,
    };

    let mut size: f64 = input.to_f64();
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

    format!("{:.*}{}{}{}", places, size, space, scale, opts.suffix)
}

pub fn format_size(input: impl ToF64 + Unsigned, _options: impl AsRef<FormatSizeOptions>) -> String {
    format_size_i(input, _options)
}

pub struct SizeFormatter {
    options: FormatSizeOptions
}

impl SizeFormatter {
    pub fn format(&self, number: impl ToF64) -> String {
        format_size_i(number, &self.options)
    }
}