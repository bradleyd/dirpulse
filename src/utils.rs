use walkdir::DirEntry;

pub fn bytes_to_human(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, UNITS[unit])
}

pub fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_to_human_zero() {
        assert_eq!(bytes_to_human(0), "0.00 B");
    }

    #[test]
    fn bytes_to_human_kilobytes() {
        assert_eq!(bytes_to_human(1024), "1.00 KB");
    }

    #[test]
    fn bytes_to_human_megabytes() {
        assert_eq!(bytes_to_human(1_048_576), "1.00 MB");
    }
}
