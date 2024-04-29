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

pub fn limit_strings(input: &Vec<String>, size: usize, filter_profanity: bool) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for str in input {
        if filter_profanity {
            result.push(profanity(&limit_string(str, size)))
        } else {
            result.push(limit_string(str, size))
        }
    }
    result
}
