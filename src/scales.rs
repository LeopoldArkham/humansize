pub(crate) const SCALE_LENGTH: usize = 9;

pub(crate) const SCALE_DECIMAL: [&str; SCALE_LENGTH] =
    ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

pub(crate) const SCALE_DECIMAL_LONG: [&str; SCALE_LENGTH] = [
    "Bytes",
    "Kilobytes",
    "Megabytes",
    "Gigabytes",
    "Terabytes",
    "Petabytes",
    "Exabytes",
    "Zettabytes",
    "Yottabytes",
];

pub(crate) const SCALE_BINARY: [&str; SCALE_LENGTH] =
    ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

pub(crate) const SCALE_BINARY_LONG: [&str; SCALE_LENGTH] = [
    "Bytes",
    "Kibibytes",
    "Mebibytes",
    "Gibibytes",
    "Tebibytes",
    "Pebibytes",
    "Exbibytes",
    "Zebibytes",
    "Yobibytes",
];

pub(crate) const SCALE_DECIMAL_BIT: [&str; SCALE_LENGTH] = [
    "bits", "kbit", "Mbit", "Gbit", "Tbit", "Pbit", "Ebit", "Zbit", "Ybit",
];

pub(crate) const SCALE_DECIMAL_BIT_LONG: [&str; SCALE_LENGTH] = [
    "Bits",
    "Kilobits",
    "Megabits",
    "Gigabits",
    "Terabits",
    "Petabits",
    "Exabits",
    "Zettabits",
    "Yottabits",
];

pub(crate) const SCALE_BINARY_BIT: [&str; SCALE_LENGTH] = [
    "bits", "Kibit", "Mibit", "Gibit", "Tibit", "Pibit", "Eibit", "Zibit", "Yibit",
];

pub(crate) const SCALE_BINARY_BIT_LONG: [&str; SCALE_LENGTH] = [
    "bits", "Kibibits", "Mebibits", "Gibibits", "Tebibits", "Pebibits", "Exbibits", "Zebibits",
    "Yobibits",
];
