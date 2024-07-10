use std::io::Read;

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
            "linux" => {
                let mut file = std::fs::File::open("/etc/os-release").unwrap();
                let mut contents = String::new();
                // Read the file and look for the NAME field.
                file.read_to_string(&mut contents).unwrap();
                let os = contents
                    .split('\n')
                    .find(|line| line.starts_with("NAME="))
                    .unwrap();
                Ok(OperatingSystem::Arch)
            }
            _ => Err(DepotError::UnknownOperatingSystem),
        }
    }
}

