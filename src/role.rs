/// Grants access to anyone.
pub fn any() -> String {
    "any".to_string()
}

/// Grants access to a specific user by id, optionally scoped to a status.
pub fn user(id: &str, status: &str) -> String {
    if !status.is_empty() {
        format!("user:{}/{}", id, status)
    } else {
        format!("user:{}", id)
    }
}

/// Grants access to all users, optionally scoped to a status.
pub fn users(status: &str) -> String {
    if !status.is_empty() {
        format!("users/{}", status)
    } else {
        "users".to_string()
    }
}

/// Grants access to all guest (unauthenticated) users.
pub fn guests() -> String {
    "guests".to_string()
}

/// Grants access to a team, optionally scoped to a role within the team.
pub fn team(id: &str, role: &str) -> String {
    if !role.is_empty() {
        format!("team:{}/{}", id, role)
    } else {
        format!("team:{}", id)
    }
}

/// Grants access to a specific team member by id.
pub fn member(id: &str) -> String {
    format!("member:{}", id)
}

/// Grants access to any user with the given label.
pub fn label(id: &str) -> String {
    format!("label:{}", id)
}
