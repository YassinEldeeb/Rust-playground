pub fn get_suffix(score: u32) -> String {
    return match score {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", score),
    };
}
