use std::io::Read;

/// List of errors that the program can return.
#[derive(Debug)]
pub enum DepotError {
    UnknownOperatingSystem,
}

/// Result type wich wither take a type T or a DepotError.
pub type DepotResult<T> = std::result::Result<T, DepotError>;
pub fn unwrap_depot_error<T>(result: DepotResult<T>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            eprintln!(
                "{}",
                match error {
                    DepotError::UnknownOperatingSystem =>
                        "Unable to determine your current operating system.",
                }
            );
            std::process::exit(1);
        }
    }
}

/// List of all supported operating systems.
#[derive(Debug)]
pub enum OperatingSystem {
    Arch,
    Alpine,
    Debian,
    Ubuntu,
    Fedora,
}
impl OperatingSystem {
    /// Get the currently running operating system.
    pub fn current() -> DepotResult<OperatingSystem> {
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
                match os {
                    "NAME=\"Arch Linux\"" => Ok(OperatingSystem::Arch),
                    "NAME=\"Alpine Linux\"" => Ok(OperatingSystem::Alpine),
                    "NAME=\"Debian GNU/Linux\"" => Ok(OperatingSystem::Debian),
                    "NAME=\"Ubuntu\"" => Ok(OperatingSystem::Ubuntu),
                    "NAME=\"Fedora Linux\"" => Ok(OperatingSystem::Fedora),
                    _ => Err(DepotError::UnknownOperatingSystem),
                }
            }
            _ => Err(DepotError::UnknownOperatingSystem),
        }
    }
}
