/// Обрезает текст
pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len])
    } else {
        text.to_string()
    }
}

 
pub fn format_date(date: &str) -> String {
    date.split('T').next().unwrap_or(date).to_string()
}
