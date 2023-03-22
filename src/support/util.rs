pub struct Util {}

impl Util {
    pub fn substring_between(line: &String, start: &str, end: &str) -> Option<String> {
        let start_bytes = line.find(start);
        let end_bytes = line.find(end);
        if start_bytes.is_none() || end_bytes.is_none() {
            return None;
        }
        let result = &line[start_bytes.unwrap()..end_bytes.unwrap()];
        Some(String::from(result))
    }

    pub fn substring_after(line: &String, start: &str) -> Option<String> {
        let start_bytes = line.find(start);
        if start_bytes.is_none() {
            return None;
        }
        let result = &line[start_bytes.unwrap() + 1..line.len()];
        Some(String::from(result))
    }

    pub fn substring_before(line: &String, end: &str) -> Option<String> {
        let end_bytes = line.find(end);
        if end_bytes.is_none() {
            return None;
        }
        let result = &line[0..end_bytes.unwrap()];
        Some(String::from(result))
    }
}