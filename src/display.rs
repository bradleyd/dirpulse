use std::cmp::Reverse;
use std::collections::HashMap;

use crate::age;
use crate::file_info;
use crate::utils;

pub fn print_hero(
    td: &str,
    dstats: &age::AgeStats,
    top_file_by_size: (&String, u64),
    top_n_percentage: f64,
) {
    let width = 90;
    let title = format!("dirpulse · {}", td);
    let stats = format!(
        "🔴 {:<5} Stale, 📦 Largest: {} ({}), 📊 Top 10 = {:.2}%",
        utils::bytes_to_human(dstats.stale.size),
        top_file_by_size.0,
        utils::bytes_to_human(top_file_by_size.1),
        top_n_percentage
    );
    println!("┌{}┐", "─".repeat(width));
    println!("│ {:<w$} │", title, w = width - 2);
    println!("├{}┤", "─".repeat(width));
    println!("│ {:<w$} │", stats, w = width - 5);
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
        "── Top {} Largest ────────────────────────────────────────────────────",
        top_size
    );

    let mut count = 1;
    for entry in sorted_heap {
        println!(
            "{:<10}   {:<7}   {:<40}",
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

pub fn print_extensions(file_types: &HashMap<String, file_info::TypeStats>) {
    println!("── By Extension ──────────────────────────────────────────────────────");
    let mut sorted_exts: Vec<_> = file_types.iter().collect();
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
    println!("── File Age ──────────────────────────────────────────────────────────");
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
