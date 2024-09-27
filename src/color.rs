use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Rgb(u8, u8, u8),
}

fn is_bright(color: Color) -> bool {
    color == Color::BrightBlack
        || color == Color::BrightRed
        || color == Color::BrightGreen
        || color == Color::BrightYellow
        || color == Color::BrightBlue
        || color == Color::BrightMagenta
        || color == Color::BrightCyan
        || color == Color::BrightWhite
}

fn prefix() -> String {
    String::from("\x1b[")
}

fn format_text(sgr: &str) -> String {
    format!("{}{}m", prefix(), sgr)
}

fn reset_sgr_attributes() -> String {
    format_text("0")
}

fn color_str(color: Color) -> String {
    match color {
        Color::Black => String::from("0"),
        Color::Red => String::from("1"),
        Color::Green => String::from("2"),
        Color::Yellow => String::from("3"),
        Color::Blue => String::from("4"),
        Color::Magenta => String::from("5"),
        Color::Cyan => String::from("6"),
        Color::White => String::from("7"),
        Color::BrightBlack => String::from("0"),
        Color::BrightRed => String::from("1"),
        Color::BrightGreen => String::from("2"),
        Color::BrightYellow => String::from("3"),
        Color::BrightBlue => String::from("4"),
        Color::BrightMagenta => String::from("5"),
        Color::BrightCyan => String::from("6"),
        Color::BrightWhite => String::from("7"),
        Color::Rgb(r, g, b) => format!("8;2;{};{};{}", r, g, b),
    }
}

fn set_nonbright_foreground_color(color: Color) -> String {
    let sgr = format!("3{}", color_str(color));
    format_text(&sgr)
}

fn set_bright_foreground_color(color: Color) -> String {
    let sgr = format!("9{}", color_str(color));
    format_text(&sgr)
}

fn set_foreground_color(color: Color) -> String {
    if is_bright(color) {
        return set_bright_foreground_color(color);
    }

    set_nonbright_foreground_color(color)
}

pub fn color_text<T: Into<String> + fmt::Display>(text: T, color: Color) -> String {
    let sgr_color = set_foreground_color(color);
    let sgr_reset = reset_sgr_attributes();

    format!("{}{}{}", sgr_color, text, sgr_reset)
}
