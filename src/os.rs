use crate::error::{DepotError, DepotResult};
use std::io::Read;

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
                let mut contents = String::new();
                std::fs::File::open("/etc/os-release")
                    .unwrap()
                    .read_to_string(&mut contents)
                    .unwrap();
                match contents
                    .split('\n')
                    .find(|line| line.starts_with("ID="))
                    .unwrap()
                {
                    "ID=arch" => Ok(OperatingSystem::Arch),
                    "ID=alpine" => Ok(OperatingSystem::Alpine),
                    "ID=debian" => Ok(OperatingSystem::Debian),
                    "ID=ubuntu" => Ok(OperatingSystem::Ubuntu),
                    "ID=fedora" => Ok(OperatingSystem::Fedora),
                    _ => Err(DepotError::UnknownOperatingSystem),
                }
            }
            _ => Err(DepotError::UnknownOperatingSystem),
        }
    }
}
