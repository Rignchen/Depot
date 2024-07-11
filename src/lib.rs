use std::io::Read;
use std::env;

/// List of errors that the program can return.
#[derive(Debug)]
pub enum DepotError {
    UnknownOperatingSystem,
    UnknownPackageManager,
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
                    DepotError::UnknownPackageManager =>
                        "The package manager is unknown or not supported.",
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

/// List of all supported operating systems.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum PackageManager {
    Pacman,
    Yay,
    Apk,
    AptGet,
    Apt,
    Api,
    Pkg,
    Dnf,
}
impl From<&OperatingSystem> for PackageManager {
    /// Get the default package manager for the operating system.
    fn from(os: &OperatingSystem) -> PackageManager {
        match os {
            OperatingSystem::Arch => PackageManager::Pacman,
            OperatingSystem::Alpine => PackageManager::Apk,
            OperatingSystem::Debian => PackageManager::AptGet,
            OperatingSystem::Ubuntu => PackageManager::AptGet,
            OperatingSystem::Fedora => PackageManager::Dnf,
        }
    }
}

pub fn get_package_manager(expected: Option<PackageManager>) -> DepotResult<PackageManager> {
    match expected {
        Some(manager) => Ok(manager),
        None => {
            match env::var("DEPOT_PACKAGE_MANAGER") {
                Ok(manager) => match manager.as_str() {
                    "pacman" => Ok(PackageManager::Pacman),
                    "yay" => Ok(PackageManager::Yay),
                    "apk" => Ok(PackageManager::Apk),
                    "apt-get" => Ok(PackageManager::AptGet),
                    "apt" => Ok(PackageManager::Apt),
                    "api" => Ok(PackageManager::Api),
                    "pkg" => Ok(PackageManager::Pkg),
                    "dnf" => Ok(PackageManager::Dnf),
                    _ => Err(DepotError::UnknownPackageManager),
                },
                Err(_) => {
                    let os = OperatingSystem::current()?;
                    Ok(PackageManager::from(&os))
                }
            }
        }
    }
}
