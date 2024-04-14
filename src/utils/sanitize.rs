use regex::Regex;

pub fn alphanumeric(input: String) -> String {
    let re = Regex::new(r"[^a-zA-Z0-9]").unwrap();
    re.replace_all(&input, "").to_string()
}