
use alloc::string::String;
use crate::{IFormatter, Formatter, FormatSizeOptions, ToF64, Unsigned, Signed};

pub trait FormatSize<T> {
    fn format_size(&self, opts: FormatSizeOptions) -> String;
}

pub trait FormatSizeI<T> {
    fn format_size_i(&self, opts: FormatSizeOptions) -> String;
}

impl<T: ToF64 + Unsigned + Copy> FormatSize<T> for T {
    fn format_size(&self, opts: FormatSizeOptions) -> String {
        format!("{}", Formatter::new(*self, opts))
    }
}

impl<T: ToF64 + Signed + Copy> FormatSizeI<T> for T {
    fn format_size_i(&self, opts: FormatSizeOptions) -> String {
        format!("{}", IFormatter::new(*self, opts))
    }
}