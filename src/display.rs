use std::cmp::Reverse;
use std::collections::HashMap;

use crate::age;
use crate::file_info;
use crate::utils;

fn truncate_to_width(s: &str, max_width: usize) -> String {
    if s.chars().count() <= max_width {
        s.to_string()
    } else if max_width <= 3 {
        ".".repeat(max_width)
    } else {
        let truncated: String = s.chars().take(max_width - 3).collect();
        format!("{}...", truncated)
    }
}

// Calculate display width accounting for emojis (4 bytes but 2 display chars)
fn display_width(s: &str) -> usize {
    let mut width = 0;
    for c in s.chars() {
        if c.len_utf8() == 4 {
            width += 2; // emoji - 2 display chars
        } else {
            width += 1;
        }
    }
    width
}

fn pad_to_width(s: &str, target_width: usize) -> String {
    let current_width = display_width(s);
    if current_width >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - current_width))
    }
}

pub fn print_hero(
    td: &str,
    dstats: &age::AgeStats,
    top_file_by_size: (&String, u64),
    top_n_percentage: f64,
) {
    let width = 90;
    let inner_width = width - 2; // space for "│ " and " │"

    // Truncate title if needed
    let title = truncate_to_width(&format!("dirpulse · {}", td), inner_width);

    // Calculate available space for filename in stats line
    // Format: "🔴 {stale} Stale, 📦 Largest: {name} ({size}), 📊 Top 10 = {pct}%"
    let stale_str = utils::bytes_to_human(dstats.stale.size);
    let size_str = utils::bytes_to_human(top_file_by_size.1);
    let pct_str = format!("{:.2}%", top_n_percentage);

    // Calculate fixed parts display width
    // "🔴 " (4) + stale + " Stale, 📦 Largest: " (20) + name + " (" (2) + size + "), 📊 Top 10 = " (16) + pct
    // Each emoji is 2 display chars, so: 🔴=2, 📦=2, 📊=2
    let fixed_display_width = 4 + stale_str.len() + 20 + 2 + size_str.len() + 16 + pct_str.len();
    let max_name_len = inner_width.saturating_sub(fixed_display_width);

    let truncated_name = truncate_to_width(top_file_by_size.0, max_name_len);

    let stats = format!(
        "🔴 {} Stale, 📦 Largest: {} ({}), 📊 Top 10 = {}",
        stale_str, truncated_name, size_str, pct_str
    );

    println!("┌{}┐", "─".repeat(width));
    println!("│ {} │", pad_to_width(&title, inner_width));
    println!("├{}┤", "─".repeat(width));
    println!("│ {} │", pad_to_width(&stats, inner_width));
    println!("└{}┘", "─".repeat(width));
}

pub fn print_stat_line(file_count: u64, dir_count: u64, total_size: u64) {
    println!(
        "📁 {} files · {} directories · {} total",
        file_count,
        dir_count,
        utils::bytes_to_human(total_size)
    );
}

pub fn print_top_largest(top_size: u64, sorted_heap: &Vec<Reverse<file_info::FileInfo>>) {
    println!(
        "── Top {} Largest ───────────────────────────────────────────────────────",
        top_size
    );

    let mut count = 1;
    for entry in sorted_heap {
        println!(
            "{:<10}   {:<10}   {:<40}",
            count,
            utils::bytes_to_human(entry.0.size),
            entry
                .0
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );
        count += 1;
    }
}

pub fn print_extensions(file_types: &HashMap<String, file_info::TypeStats>, top_extensions: u64) {
    println!(
        "── Top {} Extension ─────────────────────────────────────────────────────",
        top_extensions
    );
    let mut sorted_exts: Vec<_> = file_types.iter().take(top_extensions as usize).collect();
    sorted_exts.sort_by(|a, b| b.1.file_count.cmp(&a.1.file_count));
    for (key, value) in sorted_exts {
        println!(
            ".{:<25}         {:<10}           {:<10}",
            key,
            value.file_count,
            utils::bytes_to_human(value.total_size)
        );
    }
}

pub fn print_file_age(file_age: &age::AgeStats) {
    println!("── File Age ──────────────────────────────────────────────────────────────");
    println!(
        "{:<30} {:>5} files   {:>10} ",
        "🟢 Fresh (< 30 days)",
        file_age.fresh.count,
        utils::bytes_to_human(file_age.fresh.size)
    );
    println!(
        "{:<30} {:>5} files   {:>10} ",
        "🟡 Aging (30 days - 6 mo)",
        file_age.aging.count,
        utils::bytes_to_human(file_age.aging.size)
    );
    println!(
        "{:<30} {:>5} files   {:>10} ",
        "🔴 Stale (> 6 months)",
        file_age.stale.count,
        utils::bytes_to_human(file_age.stale.size)
    );
}
