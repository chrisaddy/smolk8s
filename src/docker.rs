use std::process::Command;

pub struct Docker {
    pub installed: bool,
    pub version: Option<String>,
}

impl Docker {
    pub fn new() -> Self {
        let installed = Command::new("docker").arg("--version").output();
        match installed {
            Ok(output) => Self {
                installed: output.status.success(),
                version: match std::str::from_utf8(&output.stdout) {
                    Ok(version) => Some(version.to_string()),
                    Err(_) => None,
                },
            },
            Err(_) => Self {
                installed: false,
                version: None,
            },
        }
    }
}
