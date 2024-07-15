/// List of errors that the program can return.
#[derive(Debug)]
pub enum DepotError {
    UnknownOperatingSystem,
    UnknownPackageManager,
    PackageManagerError,
}

/// Result type which wither take a type T or a DepotError.
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
                    DepotError::PackageManagerError =>
                        "An error occurred while running the package manager.",
                }
            );
            std::process::exit(1);
        }
    }
}
