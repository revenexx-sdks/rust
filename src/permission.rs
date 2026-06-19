/// Grants read access to the given role.
pub fn read(role: &str) -> String {
    format!("read(\"{}\")", role)
}

/// Grants write access (create, update, delete) to the given role.
pub fn write(role: &str) -> String {
    format!("write(\"{}\")", role)
}

/// Grants create access to the given role.
pub fn create(role: &str) -> String {
    format!("create(\"{}\")", role)
}

/// Grants update access to the given role.
pub fn update(role: &str) -> String {
    format!("update(\"{}\")", role)
}

/// Grants delete access to the given role.
pub fn delete(role: &str) -> String {
    format!("delete(\"{}\")", role)
}
