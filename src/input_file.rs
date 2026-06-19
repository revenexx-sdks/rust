use std::path::Path;

/// Represents a file to be uploaded to the API.
#[derive(Debug, Clone)]
pub struct InputFile {
    /// Absolute or relative path to the file on disk.
    pub path: String,
    /// File name reported to the server.
    pub name: String,
}

impl InputFile {
    /// Build an [`InputFile`] from a path, deriving the name from the file name
    /// component of the path.
    pub fn from_path(path: &str) -> Self {
        let name = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();

        InputFile {
            path: path.to_string(),
            name,
        }
    }

    /// Build an [`InputFile`] from a path with an explicit file name.
    pub fn new(path: &str, name: &str) -> Self {
        InputFile {
            path: path.to_string(),
            name: name.to_string(),
        }
    }
}
