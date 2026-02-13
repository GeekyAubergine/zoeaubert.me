#[derive(Clone, Debug, Default)]
pub struct AboutText {
    pub short: String,
    pub long: String,
}

impl AboutText {
    pub fn new(short: String, long: String) -> Self {
        Self { short, long }
    }
}
