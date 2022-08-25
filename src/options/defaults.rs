use super::{FormatSizeOptions, FixedAt, Kilo};

/// Options to display sizes in the binary format.
pub const BINARY: FormatSizeOptions = FormatSizeOptions {
    divider: Kilo::Binary,
    units: Kilo::Binary,
    decimal_places: 2,
    decimal_zeroes: 0,
    fixed_at: FixedAt::No,
    long_units: false,
    space: true,
    suffix: "",
};

/// Options to display sizes in the decimal format.
pub const DECIMAL: FormatSizeOptions = FormatSizeOptions {
    divider: Kilo::Decimal,
    units: Kilo::Decimal,
    decimal_places: 2,
    decimal_zeroes: 0,
    fixed_at: FixedAt::No,
    long_units: false,
    space: true,
    suffix: "",
};

/// Options to display sizes in the "conventional" format.
/// This 1024 as the value of the `Kilo`, but displays decimal-style units (`KB`, not `KiB`).
pub const CONVENTIONAL: FormatSizeOptions = FormatSizeOptions {
    divider: Kilo::Binary,
    units: Kilo::Decimal,
    decimal_places: 2,
    decimal_zeroes: 0,
    fixed_at: FixedAt::No,
    long_units: false,
    space: true,
    suffix: "",
};
