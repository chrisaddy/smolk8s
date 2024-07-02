use std::process::Command;

pub struct Kubectl {
    pub installed: bool,
    pub version: Option<String>,
}
