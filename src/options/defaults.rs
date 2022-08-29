use super::{FixedAt, FormatSizeOptions, Kilo};

/// Options to display sizes in the SI format.
pub const BINARY: FormatSizeOptions = FormatSizeOptions {
    kilo: Kilo::Binary,
    units: Kilo::Binary,
    decimal_places: 2,
    decimal_zeroes: 0,
    fixed_at: FixedAt::No,
    long_units: false,
    space: true,
    suffix: "",
};

/// Options to display sizes in the SI (decimal) format.
pub const DECIMAL: FormatSizeOptions = FormatSizeOptions {
    kilo: Kilo::Decimal,
    units: Kilo::Decimal,
    decimal_places: 2,
    decimal_zeroes: 0,
    fixed_at: FixedAt::No,
    long_units: false,
    space: true,
    suffix: "",
};

/// Options to display sizes in the "conventional" format.
/// This 1024 as the value of the `Kilo`, but displays decimal-style units (`kB`, not `KiB`).
pub const CONVENTIONAL: FormatSizeOptions = FormatSizeOptions {
    kilo: Kilo::Binary,
    units: Kilo::Decimal,
    decimal_places: 2,
    decimal_zeroes: 0,
    fixed_at: FixedAt::No,
    long_units: false,
    space: true,
    suffix: "",
};
