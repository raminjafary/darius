#[derive(Debug)]
pub struct Colors {
    pub cyan: Option<String>,
    pub red: Option<String>,
    pub yellow: Option<String>,
    pub green: Option<String>,
    pub white: Option<String>,
}

impl Colors {
    fn new(cyan: String, red: String, yellow: String, green: String, white: String) -> Colors {
        Colors {
            cyan: Some(cyan),
            red: Some(red),
            yellow: Some(yellow),
            green: Some(green),
            white: Some(white),
        }
    }
}

impl Default for Colors {
    fn default() -> Colors {
        Colors {
            cyan: Some("\x1b[36m".to_string()),
            red: Some("\x1b[31m".to_string()),
            yellow: Some("\x1b[33m".to_string()),
            green: Some("\x1b[32m".to_string()),
            white: Some("\x1b[37m".to_string()),
        }
    }
}
