mod colors;
mod log_levels;
use colors::Colors;
use log_levels::LogLevels;
use std::io::{self, Write};

fn define_log_level<S: AsRef<str>>(text: S, level: LogLevels) -> () {
    let colors: Colors = Colors::default();

    let matched_color_with_log_level: String = match level {
        LogLevels::Fatal => colors.red.unwrap(),
        LogLevels::Error => colors.red.unwrap(),
        LogLevels::Warn => colors.yellow.unwrap(),
        LogLevels::Info => colors.cyan.unwrap(),
        LogLevels::Success => colors.green.unwrap(),
        LogLevels::Log => colors.white.unwrap(),
        LogLevels::Debug => colors.gray.unwrap(),
        LogLevels::Trace => colors.blue.unwrap(),
        LogLevels::Verbose => colors.magenta.unwrap(),
    };

    print_with_color::<S>(text, matched_color_with_log_level);
}

fn print_with_color<S: AsRef<str>>(text: S, color: String) {
    let mut handle: io::BufWriter<io::Stdout> = io::BufWriter::new(io::stdout());
    writeln!(handle, "{color} {}\x1b[0m", text.as_ref(), color = color).unwrap();
}

pub fn fatal<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Fatal)
}

pub fn error<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Error)
}

pub fn warn<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Warn)
}

pub fn info<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Info)
}

pub fn success<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Success)
}

pub fn log<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Log)
}

pub fn debug<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Debug)
}

pub fn trace<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Trace)
}

pub fn verbose<S: AsRef<str>>(text: S) {
    define_log_level(text, LogLevels::Verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "Sample text!";

    #[test]
    fn all_log_levels() {
        assert_eq!(fatal(TEXT), ());
        assert_eq!(error(TEXT), ());
        assert_eq!(warn(TEXT), ());
        assert_eq!(info(TEXT), ());
        assert_eq!(success(TEXT), ());
        assert_eq!(log(TEXT), ());
        assert_eq!(debug(TEXT), ());
        assert_eq!(trace(TEXT), ());
        assert_eq!(verbose(TEXT), ());
    }

    #[test]
    fn color_print_with_custom_fg() {
        let magenta_fg = "\x1b[35;1m".into();
        assert_eq!(print_with_color(TEXT, magenta_fg), ());
    }

    #[test]
    fn color_print_with_custom_bg() {
        let blue_bg = "\x1b[44m".into();
        assert_eq!(print_with_color(TEXT, blue_bg), ());
    }

    #[test]
    fn color_print_with_custom_bg_and_fg() {
        let red_fg_with_white_bg = "\x1b[48;5;93;41m\x1b[7m".into();
        assert_eq!(print_with_color(TEXT, red_fg_with_white_bg), ());
    }
}
