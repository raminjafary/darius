mod colors;
mod log_levels;
use colors::Colors;
use log_levels::LogLevels;

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

    println!(
        "{color} {}\x1b[0m",
        text.as_ref(),
        color = matched_color_with_log_level
    );
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
