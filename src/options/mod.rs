//! Describes the struct that holds the options needed by the `file_size` method.
//! The three most common formats are provided as constants to be used easily

mod defaults;
pub use self::defaults::*;

#[derive(Debug, PartialEq, Copy, Clone)]
/// Holds the standard to use when displaying the size.
pub enum Kilo {
    /// The decimal scale and units. SI standard.
    Decimal,
    /// The binary scale and units
    Binary,
}

impl Kilo {
    pub(crate) fn value(&self) -> f64 {
        match self {
            Kilo::Decimal =>  1000.0,
            Kilo::Binary =>  1024.0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Forces a certain representation of the resulting file size.
pub enum FixedAt {
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
    No,
}

/// Holds the options for the `file_size` method.
#[derive(Debug, Clone, Copy)]
pub struct FormatSizeOptions {
    /// The scale (binary/decimal) to divide against.
    pub kilo: Kilo,

    /// The unit set to display.
    pub units: Kilo,

    /// The amount of decimal places to display if the decimal part is non-zero.
    pub decimal_places: usize,

    /// The amount of zeroes to display if the decimal part is zero.
    pub decimal_zeroes: usize,

    /// Whether to force a certain representation and if so, which one.
    pub fixed_at: FixedAt,

    /// Whether to use the full unit (e.g. `Kilobyte`) or its abbreviation (`kB`).
    pub long_units: bool,

    /// Whether to place a space between value and units.
    pub space: bool,

    /// An optional suffix which will be appended after the unit. Useful to represent speeds (e.g. `1 kB/s)
    pub suffix: &'static str,
}

impl AsRef<FormatSizeOptions> for FormatSizeOptions {
    fn as_ref(&self) -> &FormatSizeOptions {
        self
    }
}
