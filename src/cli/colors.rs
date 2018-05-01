use term_painter::Color::*;
use term_painter::Painted;
use term_painter::ToStyle;

pub fn paint_success(s: String) -> Painted<String> {
    BrightGreen.bold().paint(s)
}

pub fn paint_error(s: String) -> Painted<String> {
    BrightRed.bold().paint(s)
}

pub fn paint(s: Result<String, String>) -> Painted<String> {
    match s {
        Ok(s) => paint_success(s),
        Err(err) => paint_error(err)
    }
}