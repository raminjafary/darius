#[derive(PartialEq, Debug)]
pub(crate) struct Colors {
    pub cyan: Option<String>,
    pub red: Option<String>,
    pub yellow: Option<String>,
    pub green: Option<String>,
    pub white: Option<String>,
    pub magenta: Option<String>,
    pub blue: Option<String>,
    pub gray: Option<String>,
}

impl Colors {
    fn new<S>(cyan: S, red: S, yellow: S, green: S, white: S, megnta: S, blue: S, gray: S) -> Colors
    where
        S: Into<String>,
    {
        Colors {
            cyan: Some(cyan.into()),
            red: Some(red.into()),
            yellow: Some(yellow.into()),
            green: Some(green.into()),
            white: Some(white.into()),
            magenta: Some(megnta.into()),
            blue: Some(blue.into()),
            gray: Some(gray.into()),
        }
    }
}

impl Default for Colors {
    fn default() -> Colors {
        Colors {
            cyan: Some("\x1b[36;1m".into()),
            red: Some("\x1b[31;1m".into()),
            yellow: Some("\x1b[33;1m".into()),
            green: Some("\x1b[32;1m".into()),
            white: Some("\x1b[37;1m".into()),
            magenta: Some("\x1b[35;1m".into()),
            blue: Some("\x1b[34;1m".into()),
            gray: Some("\x1b[90;1m".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn colors_with_defaults() {
        let default_colors: Colors = Colors::default();
        let new_colors_with_the_same_as_default: Colors = Colors {
            ..Default::default()
        };

        assert_eq!(default_colors, new_colors_with_the_same_as_default);
    }
    #[test]
    fn overwrite_default_colors() {
        let default_colors: Colors = Colors::default();
        let new_colors: Colors = Colors::new(
            "\x1b[36m", "\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[37m", "\x1b[35m", "\x1b[34m",
            "\x1b[90m",
        );

        assert_ne!(default_colors, new_colors);
    }
}
