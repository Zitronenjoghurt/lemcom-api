use rustrict::CensorStr;

pub fn alphanumeric(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

pub fn profanity(input: &str) -> String {
    input.censor()
}

pub fn limit_string(input: &str, size: usize) -> String {
    if input.len() > size && size > 3 {
        format!("{}...", &input[0..size - 3])
    } else {
        input.to_string()
    }
}
