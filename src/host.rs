use std::env::consts;

#[derive(Debug)]
pub struct Machine {
    os: String,
    family: String,
    arch: String,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            os: consts::OS.to_string(),
            family: consts::FAMILY.to_string(),
            arch: consts::ARCH.to_string(),
        }
    }
}
impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.os, self.family, self.arch)
    }
}
