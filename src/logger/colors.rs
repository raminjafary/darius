#[derive(Debug)]
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
