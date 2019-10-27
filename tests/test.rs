use humansize::{file_size_opts::{self, BINARY, DECIMAL, CONVENTIONAL}, FileSize};

#[test]
fn test_sizes() {

    assert_eq!(0.file_size(BINARY).unwrap(), "0 B");
    assert_eq!(999.file_size(BINARY).unwrap(), "999 B");
    assert_eq!(1000.file_size(BINARY).unwrap(), "1000 B");
    assert_eq!(1000.file_size(DECIMAL).unwrap(), "1 KB");
    assert_eq!(1023.file_size(BINARY).unwrap(), "1023 B");
    assert_eq!(1023.file_size(DECIMAL).unwrap(), "1.02 KB");
    assert_eq!(1024.file_size(BINARY).unwrap(), "1 KiB");
    assert_eq!(1024.file_size(CONVENTIONAL).unwrap(), "1 KB");

    let semi_custom_options = file_size_opts::FileSizeOpts {
        space: false,
        ..file_size_opts::DECIMAL
    };
    assert_eq!(1000.file_size(semi_custom_options).unwrap(), "1KB");

    let semi_custom_options2 = file_size_opts::FileSizeOpts {
        suffix: "/s",
        ..file_size_opts::BINARY
    };
    assert_eq!(999.file_size(semi_custom_options2).unwrap(), "999 B/s");

    let semi_custom_options3 = file_size_opts::FileSizeOpts {
        suffix: "/day",
        space: false,
        ..file_size_opts::DECIMAL
    };
    assert_eq!(1000.file_size(semi_custom_options3).unwrap(), "1KB/day");

    let semi_custom_options4 = file_size_opts::FileSizeOpts {
        fixed_at: file_size_opts::FixedAt::Byte,
        ..file_size_opts::BINARY
    };
    assert_eq!(2048.file_size(semi_custom_options4).unwrap(), "2048 B");

    let semi_custom_options5 = file_size_opts::FileSizeOpts {
        fixed_at: file_size_opts::FixedAt::Kilo,
        ..file_size_opts::BINARY
    };
    assert_eq!(
        16584975.file_size(semi_custom_options5).unwrap(),
        "16196.26 KiB"
    );

    let semi_custom_options6 = file_size_opts::FileSizeOpts {
        fixed_at: file_size_opts::FixedAt::Tera,
        decimal_places: 10,
        ..file_size_opts::BINARY
    };
    assert_eq!(
        15284975.file_size(semi_custom_options6).unwrap(),
        "0.0000139016 TiB"
    );

    let semi_custom_options7 = file_size_opts::FileSizeOpts {
        allow_negative: true,
        ..file_size_opts::DECIMAL
    };
    assert_eq!(
        (-5500).file_size(&semi_custom_options7).unwrap(),
        "-5.50 KB"
    );
    assert_eq!((5500).file_size(&semi_custom_options7).unwrap(), "5.50 KB");
}