use humansize::{
    format_size, format_size_i, BaseUnit, FixedAt, FormatSizeOptions, BINARY, DECIMAL, WINDOWS,
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
    assert_eq!(format_size(1024u32, WINDOWS), "1 kB");

    assert_eq!(format_size_i(1000f32, DECIMAL), "1 kB");
    assert_eq!(format_size_i(1000f64, DECIMAL), "1 kB");


    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(DECIMAL).space_after_value(false);
        assert_eq!(format_size(1000u32, CUSTOM_OPTIONS), "1kB");
    }

    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(BINARY).suffix("/s");
        assert_eq!(format_size(999u32, CUSTOM_OPTIONS), "999 B/s");
    }

    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(DECIMAL)
            .suffix("/day")
            .space_after_value(false);
        assert_eq!(format_size(1000u32, CUSTOM_OPTIONS), "1kB/day");
    }

    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(BINARY).fixed_at(Some(FixedAt::Base));
        assert_eq!(format_size(2048u32, CUSTOM_OPTIONS), "2048 B");
    }

    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(BINARY)
            .fixed_at(Some(FixedAt::Base))
            .long_units(true);
        assert_eq!(format_size(2048u32, CUSTOM_OPTIONS), "2048 Bytes");
    }

    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(BINARY).fixed_at(Some(FixedAt::Kilo));
        assert_eq!(format_size(16584975u32, CUSTOM_OPTIONS), "16196.26 KiB");
        assert_eq!(format_size_i(-16584975, CUSTOM_OPTIONS), "-16196.26 KiB");
    }

    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(BINARY)
            .fixed_at(Some(FixedAt::Tera))
            .decimal_places(10);
        assert_eq!(format_size(15284975u32, CUSTOM_OPTIONS), "0.0000139016 TiB");

        assert_eq!((format_size_i(-5500, DECIMAL)), "-5.50 kB");
        assert_eq!((format_size(5500u32, DECIMAL)), "5.50 kB");
    }

    {
        const CUSTOM_OPTIONS: FormatSizeOptions = FormatSizeOptions::from(DECIMAL).base_unit(BaseUnit::Bit);
        assert_eq!((format_size(1usize, CUSTOM_OPTIONS)), "1 bit");
        assert_eq!((format_size(150usize, CUSTOM_OPTIONS)), "150 bits");
        assert_eq!((format_size(1000usize, CUSTOM_OPTIONS)), "1 kbit");
    }
}

#[test]
fn use_custom_option_struct_twice() {
    const OPTIONS: FormatSizeOptions = FormatSizeOptions::from(DECIMAL).long_units(true);

    assert_eq!(format_size(1500u32, &OPTIONS), "1.50 Kilobyte",);
    assert_eq!(format_size(2500u32, &OPTIONS), "2.50 Kilobytes",);
    assert_eq!(format_size_i(-2500000, &OPTIONS), "-2.50 Megabytes",);
}

#[test]
fn pluralization_works() {
    const OPTIONS: FormatSizeOptions = FormatSizeOptions::from(DECIMAL)
        .long_units(true)
        .decimal_zeroes(2);

    assert_eq!(format_size(1u32, &OPTIONS), "1.00 Byte",);

    assert_eq!(format_size(1000u32, &OPTIONS), "1.00 Kilobyte",);

    assert_eq!(format_size(1000000u32, &OPTIONS), "1.00 Megabyte",);

    assert_eq!(format_size(1000000000u32, &OPTIONS), "1.00 Gigabyte",);

    assert_eq!(format_size_i(1000000000000_i64, &OPTIONS), "1.00 Terabyte",);

    assert_eq!(
        format_size_i(1000000000000000_i64, &OPTIONS),
        "1.00 Petabyte",
    );

    assert_eq!(
        format_size_i(1000000000000000000_i64, &OPTIONS),
        "1.00 Exabyte",
    );
}

#[test]
fn max_value_decimal() {
    const OPTIONS: FormatSizeOptions = FormatSizeOptions::from(DECIMAL)
        .decimal_places(7)
        .long_units(true);
    assert_eq!(format_size(core::u64::MAX, &OPTIONS), "18.4467441 Exabytes",);
}

#[test]
fn max_value_binary() {
    const OPTIONS: FormatSizeOptions = FormatSizeOptions::from(BINARY)
        .decimal_places(7)
        .long_units(true);

    assert_eq!(format_size(core::u64::MAX, &OPTIONS), "16 Exbibytes",);
}
