#![no_std]
/*!
# **Humansize**

Humansize lets you easily represent file sizes in a human-friendly format.
You can specify your own formatting style, pick among the three defaults provided
by the library:

* Decimal (kilo = 1000, unit format is `kB`)
* Binary (kilo = 1024, unit format is `KiB`)
* Windows/Conventional (kilo = 1024, unit format is `kB`)

## How to use it

Simply import the `FileSize` trait and the options module and call the
file_size method on any positive integer, using one of the three standards
provided by the options module.

```rust
extern crate humansize;
use humansize::format_size;

fn main() {
    let size = 1000usize;
    println!("Size is {}", format_size(size, humansize::DECIMAL));

    println!("Size is {}", format_size(size, humansize::BINARY));

    println!("Size is {}", format_size(size, humansize::CONVENTIONAL));
}
```

If you wish to customize the way sizes are displayed, you may create your own custom `FormatSizeOptions` struct
and pass that to the method. See the `custom_options.rs` file in the example folder.
*/

#[macro_use]
#[cfg(not(feature = "no_alloc"))]
extern crate alloc;
extern crate libm;

use core::f64;
use libm::{fabs, modf};

mod options;
pub use options::{FixedAt, FormatSizeOptions, Kilo, BINARY, CONVENTIONAL, DECIMAL};

mod scales;
mod traits;
use traits::{ToF64, Unsigned};

#[cfg(not(feature = "no_alloc"))]
mod allocating;
#[cfg(not(feature = "no_alloc"))]
pub use allocating::*;

fn f64_eq(left: f64, right: f64) -> bool {
    left == right || fabs(left - right) <= f64::EPSILON
}

pub struct IFormatter<T: ToF64, O: AsRef<FormatSizeOptions>> {
    pub value: T,
    pub options: O,
}

impl<V: ToF64, O: AsRef<FormatSizeOptions>> IFormatter<V, O> {
    pub fn new(value: V, options: O) -> Self {
        IFormatter { value, options }
    }
}

impl<T: ToF64, O: AsRef<FormatSizeOptions>> core::fmt::Display for IFormatter<T, O> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let opts = self.options.as_ref();
        let divider = opts.kilo.value();

        let mut size: f64 = self.value.to_f64();
        let mut scale_idx = 0;

        match opts.fixed_at {
            FixedAt::No => {
                while fabs(size) >= divider {
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
            (Kilo::Binary, true) => scales::SCALE_BINARY_LONG[scale_idx],
        };

        // Remove "s" from the scale if the size is 1.x
        let (fpart, ipart) = modf(size);
        if opts.long_units && f64_eq(ipart, 1.0) {
            scale = &scale[0..scale.len() - 1];
        }

        let places = if f64_eq(fpart, 0.0) {
            opts.decimal_zeroes
        } else {
            opts.decimal_places
        };

        let space = if opts.space { " " } else { "" };

        write!(f, "{:.*}{}{}{}", places, size, space, scale, opts.suffix)
    }
}

impl<'a, U: ToF64 + Unsigned + Copy, O: AsRef<FormatSizeOptions>> From<&'a Formatter<U, O>>
    for IFormatter<U, &'a O>
{
    fn from(source: &'a Formatter<U, O>) -> Self {
        IFormatter {
            value: source.value,
            options: &source.options,
        }
    }
}

pub struct Formatter<T: ToF64 + Unsigned, O: AsRef<FormatSizeOptions>> {
    value: T,
    options: O,
}

impl<V: ToF64 + Unsigned, O: AsRef<FormatSizeOptions>> Formatter<V, O> {
    pub fn new(value: V, options: O) -> Self {
        Formatter { value, options }
    }
}

impl<T: ToF64 + Unsigned + Copy, O: AsRef<FormatSizeOptions> + Copy> core::fmt::Display
    for Formatter<T, O>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", IFormatter::from(self))
    }
}
