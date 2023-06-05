use unicode_truncate::UnicodeTruncateStr;

pub const TEXT_WIDTH: usize = 60;

pub fn sort<T: Ord>(v: &mut Vec<T>) {
    v.sort();
}

pub fn ellipsis(str: &str, width: usize) -> String {
    let truncated = str.unicode_truncate(width).0;
    if truncated.len() < str.len() {
        format!("{}...", truncated)
    } else {
        str.to_owned()
    }
}

pub fn text_block(str: &str) -> String {
    let lines: Vec<String> = str
        .lines()
        .map(|line| format!("    {}", line))
        .collect();
    format!("\n{}\n", lines.join("\n"))
}
