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
