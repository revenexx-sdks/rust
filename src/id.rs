use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the provided id unchanged. Use this to assign a custom id to a
/// resource instead of an auto-generated one.
pub fn custom(id: &str) -> String {
    id.to_string()
}

/// Generates a unique id based on the current timestamp plus a random suffix.
pub fn unique() -> String {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros())
        .unwrap_or(0);

    let choices = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];

    let mut hex_string = format!("{:x}", micros);

    // A small, dependency-free PRNG seeded from the timestamp is enough to
    // de-duplicate ids generated within the same microsecond.
    let mut seed = micros as u64 ^ 0x9E3779B97F4A7C15;
    for _ in 0..7 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        hex_string.push(choices[(seed as usize) % choices.len()]);
    }

    hex_string
}
