use crate::prelude::Result;

#[derive(Clone, Debug, Default)]
pub struct SillyNames {
    pub silly_names: Vec<String>,
}

impl SillyNames {
    pub fn new(silly_names: Vec<String>) -> Self {
        Self { silly_names }
    }
}
