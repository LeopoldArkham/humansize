use humansize::{
    format_size, format_size_i, FixedAt, FormatSizeOptions, BINARY, CONVENTIONAL, DECIMAL,
};

#[test]
fn test_sizes() {
    assert_eq!(format_size(0u32, BINARY), "0 B");
    assert_eq!(format_size(999u32, BINARY), "999 B");
    assert_eq!(format_size(1000u32, BINARY), "1000 B");
    assert_eq!(format_size(1000u32, DECIMAL), "1 kB");
    assert_eq!(format_size(1023u32, BINARY), "1023 B");
    assert_eq!(format_size(1023u32, DECIMAL), "1.02 kB");
    assert_eq!(format_size(1024u32, BINARY), "1 KiB");
    assert_eq!(format_size(1024u32, CONVENTIONAL), "1 kB");

    let semi_custom_options = FormatSizeOptions {
        space: false,
        ..DECIMAL
    };
    assert_eq!(format_size(1000u32, semi_custom_options), "1kB");

    let semi_custom_options2 = FormatSizeOptions {
        suffix: "/s",
        ..BINARY
    };
    assert_eq!(format_size(999u32, semi_custom_options2), "999 B/s");

    let semi_custom_options3 = FormatSizeOptions {
        suffix: "/day",
        space: false,
        ..DECIMAL
    };
    assert_eq!(format_size(1000u32, semi_custom_options3), "1kB/day");

    let semi_custom_options4 = FormatSizeOptions {
        fixed_at: FixedAt::Byte,
        ..BINARY
    };
    assert_eq!(format_size(2048u32, semi_custom_options4), "2048 B");

    let semi_custom_options5 = FormatSizeOptions {
        fixed_at: FixedAt::Kilo,
        ..BINARY
    };
    assert_eq!(
        format_size(16584975u32, semi_custom_options5),
        "16196.26 KiB"
    );

    assert_eq!(
        format_size_i(-16584975, semi_custom_options5),
        "-16196.26 KiB"
    );

    let semi_custom_options6 = FormatSizeOptions {
        fixed_at: FixedAt::Tera,
        decimal_places: 10,
        ..BINARY
    };
    assert_eq!(
        format_size(15284975u32, semi_custom_options6),
        "0.0000139016 TiB"
    );

    let semi_custom_options7 = FormatSizeOptions { ..DECIMAL };
    assert_eq!((format_size_i(-5500, &semi_custom_options7)), "-5.50 kB");
    assert_eq!((format_size(5500u32, &semi_custom_options7)), "5.50 kB");
}

#[test]
fn use_custom_option_struct_twice() {
    let options = FormatSizeOptions {
        long_units: true,
        ..DECIMAL
    };

    assert_eq!(format_size(1500u32, &options), "1.50 Kilobyte",);

    assert_eq!(format_size(2500u32, &options), "2.50 Kilobytes",);
    assert_eq!(format_size_i(-2500000, &options), "-2.50 Megabytes",);
}

#[test]
fn pluralization_works() {
    let options = FormatSizeOptions {
        long_units: true,
        decimal_zeroes: 2,
        ..DECIMAL
    };

    assert_eq!(format_size(1u32, &options), "1.00 Byte",);

    assert_eq!(format_size(1000u32, &options), "1.00 Kilobyte",);

    assert_eq!(format_size(1000000u32, &options), "1.00 Megabyte",);

    assert_eq!(format_size(1000000000u32, &options), "1.00 Gigabyte",);

    assert_eq!(format_size_i(1000000000000_i64, &options), "1.00 Terabyte",);

    assert_eq!(
        format_size_i(1000000000000000_i64, &options),
        "1.00 Petabyte",
    );

    assert_eq!(
        format_size_i(1000000000000000000_i64, &options),
        "1.00 Exabyte",
    );
}

#[test]
fn max_value_decimal() {
    let options = FormatSizeOptions {
        long_units: true,
        decimal_places: 7,
        ..DECIMAL
    };

    assert_eq!(format_size(std::u64::MAX, &options), "18.4467441 Exabytes",);
}

#[test]
fn max_value_binary() {
    let options = FormatSizeOptions {
        long_units: true,
        decimal_places: 7,
        ..BINARY
    };

    assert_eq!(format_size(std::u64::MAX, &options), "16 Exbibytes",);
}
