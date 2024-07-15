structstruck::strike! {
    /// List of errors that the program can return.
    #[strikethrough[derive(Debug)]]
    pub enum DepotError {
        UnknownOperatingSystem,
        UnknownPackageManager,
        PackageManagerError(enum PackageManagerError {
            InstallFailed(Vec<String>),
            RemoveFailed(Vec<String>),
            SearchFailed(String),
            UpdateFailed(Option<Vec<String>>),
        }),
    }
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
                        "Unable to determine your current operating system.".to_string(),
                    DepotError::UnknownPackageManager =>
                        "The package manager is unknown or not supported.".to_string(),
                    DepotError::PackageManagerError(pme) => match pme {
                        PackageManagerError::InstallFailed(package) =>
                            format!("Failed to install package: {}", package.join(", ")),
                        PackageManagerError::RemoveFailed(package) =>
                            format!("Failed to remove package: {}", package.join(", ")),
                        PackageManagerError::SearchFailed(package) =>
                            format!("Failed to search for package: {}", package),
                        PackageManagerError::UpdateFailed(package) => match package {
                            Some(package) =>
                                format!("Failed to update package: {}", package.join(", ")),
                            None => "Failed to update all packages.".to_string(),
                        },
                    },
                }
            );
            std::process::exit(1);
        }
    }
}
