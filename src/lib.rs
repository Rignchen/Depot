/// List of errors that the program can return.
pub enum DepotError {
    UnknownOperatingSystem,
}

/// List of all supported operating systems.
pub enum OperatingSystem {
    Arch,
    Alpine,
    Debian,
    Ubuntu,
    Fedora,
}
impl OperatingSystem {
    /// Get the currently running operating system.
    pub fn current() -> Result<OperatingSystem, DepotError> {
        match std::env::consts::OS {
            "linux" => Ok(OperatingSystem::Arch),
            _ => Err(DepotError::UnknownOperatingSystem),
        }
    }
}

