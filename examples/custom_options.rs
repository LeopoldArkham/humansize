extern crate humansize;
use humansize::{format_size, format_size_i, FixedAt, FormatSizeOptions, Kilo, DECIMAL};

fn main() {
    // Declare a fully custom option struct
    let custom_options = FormatSizeOptions {
        kilo: Kilo::Binary,
        units: Kilo::Decimal,
        decimal_places: 3,
        decimal_zeroes: 1,
        fixed_at: FixedAt::No,
        long_units: true,
        space: false,
        suffix: "",
    };

    // Then use it
    println!("{}", format_size(3024usize, custom_options));

    // Or use only some custom parameters and adopt the rest from an existing config
    let semi_custom_options = FormatSizeOptions {
        decimal_zeroes: 3,
        ..DECIMAL
    };

    println!("{}", format_size_i(1000, semi_custom_options));
}
