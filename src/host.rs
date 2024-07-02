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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "windows")]
    fn test_windows() {
        let machine = Machine::new();
        assert_eq!(machine.os, "windows");
        assert_eq!(machine.family, "windows");
        assert_eq!(machine.arch, "x86_64");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux() {
        let machine = Machine::new();
        assert_eq!(machine.os, "linux");
        assert_eq!(machine.family, "unix");
        assert_eq!(machine.arch, "x86_64");
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos() {
        let machine = Machine::new();
        assert_eq!(machine.os, "macos");
        assert_eq!(machine.family, "unix");
        assert_eq!(machine.arch, "aarch64");
    }
}
