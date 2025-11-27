use clap::Parser;
use colored;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    os::unix::fs::MetadataExt,
    path::PathBuf,
    time::Duration,
    time::SystemTime,
};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
struct Cli {
    target_directory: String,
    #[clap(short = 'n', long = "top-size", default_value_t = 10)]
    top_size: u64,
}

#[derive(Default, Clone, Copy, Debug)]
struct BucketStats {
    count: u64,
    size: u64,
}

#[derive(Default, Debug, Clone, Copy)]
struct AgeStats {
    fresh: BucketStats,
    aging: BucketStats,
    stale: BucketStats,
    missing: BucketStats,
}

#[derive(Debug)]
enum AgeBucket {
    Fresh,
    Aging,
    Stale,
    Missing,
}

#[derive(Debug)]
struct DirStats {
    total_size: u64,
    file_count: u64,
    dir_count: u64,
    top_files: BinaryHeap<Reverse<FileInfo>>,
    types: HashMap<String, TypeStats>,
    age: AgeStats,
}

impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for FileInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl DirStats {
    fn new() -> DirStats {
        DirStats {
            total_size: 0,
            file_count: 0,
            dir_count: 0,
            top_files: BinaryHeap::new(),
            types: HashMap::new(),
            age: AgeStats::default(),
        }
    }
}

#[derive(Debug, Default)]
struct TypeStats {
    file_count: u64,
    total_size: u64,
}
#[derive(PartialEq, Eq, Debug)]
struct FileInfo {
    name: String,
    path: PathBuf,
    size: u64,
    extension: Option<String>,
    mod_time: SystemTime,
}

fn classify_age(modified_time: SystemTime) -> AgeBucket {
    let now = SystemTime::now();
    let delta = now
        .duration_since(modified_time)
        .expect("Something went wrong. File time is in the future.");
    let day: u64 = 24 * 60 * 60;
    let thirty_days = Duration::from_secs(30 * day);
    let six_months = Duration::from_secs(30 * 6 * day);

    if delta < thirty_days {
        AgeBucket::Fresh
    } else if delta < six_months && delta >= thirty_days {
        AgeBucket::Aging
    } else if delta >= six_months {
        AgeBucket::Stale
    } else {
        AgeBucket::Missing
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn bytes_to_human(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, UNITS[unit])
}

fn main() {
    let cli = Cli::parse();
    let td = &cli.target_directory.clone();

    let walker = WalkDir::new(cli.target_directory)
        .follow_links(true)
        .into_iter();

    let mut dstats = DirStats::new();
    let mut age_stats = AgeStats::default();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        // is file or directory?
        // increment dir_count or file_count accordingly
        // if file add to size for dir stats
        // get extension and update Types HashMap
        // get mod time, classify_age, then update age stats.
        match entry {
            Ok(direntry) => {
                match direntry.metadata() {
                    Ok(m) => {
                        // skip passed in directory on first walk
                        if m.is_dir() && direntry.depth() > 0 {
                            dstats.dir_count += 1;
                        } else if m.is_file() {
                            dstats.file_count += 1;
                            let mut fname = String::new();
                            if let Some(file_name) = direntry.file_name().to_str() {
                                fname = file_name.to_string();
                            }
                            let ext = direntry
                                .clone()
                                .path()
                                .extension()
                                .and_then(|e| e.to_str())
                                .map(|e| e.to_string());

                            // check is extension already in tstats
                            if let Some(extension) = ext.as_ref() {
                                if let Some(stats) = dstats.types.get_mut(extension) {
                                    stats.file_count += 1;
                                    stats.total_size += m.size();
                                } else {
                                    // first time seeing this extension name
                                    let stats = TypeStats {
                                        file_count: 1,
                                        total_size: m.size(),
                                    };
                                    dstats.types.insert(ext.clone().unwrap(), stats);
                                }
                            }

                            // update age stats
                            if let Ok(mtime) = m.modified() {
                                let classified_time = classify_age(mtime);
                                match classified_time {
                                    AgeBucket::Fresh => {
                                        let mut current = age_stats.fresh;
                                        current.count += 1;
                                        current.size += m.size();
                                        age_stats.fresh = current;
                                        dstats.age = age_stats;
                                    }
                                    AgeBucket::Aging => {
                                        let mut current = age_stats.aging;
                                        current.count += 1;
                                        current.size += m.size();
                                        age_stats.aging = current;
                                        dstats.age = age_stats;
                                    }
                                    AgeBucket::Stale => {
                                        let mut current = age_stats.stale;
                                        current.count += 1;
                                        current.size += m.size();
                                        age_stats.stale = current;
                                        dstats.age = age_stats;
                                    }
                                    AgeBucket::Missing => {
                                        let mut current = age_stats.missing;
                                        current.count += 1;
                                        current.size += m.size();
                                        age_stats.missing = current;
                                        dstats.age = age_stats;
                                    }
                                }
                            } else {
                                println!(
                                    "There was no modified time for {}",
                                    direntry.file_name().to_string_lossy()
                                );
                            }

                            let finfo = FileInfo {
                                name: fname,
                                path: direntry.path().to_path_buf(),
                                size: m.size(),
                                extension: ext,
                                mod_time: m.modified().unwrap(),
                            };

                            dstats.total_size += m.size();
                            // heap logic
                            //if heap.len() < N:
                            //    heap.push(FileInfo)
                            //else if size > heap.peek().size:
                            //    heap.pop()
                            //    heap.push(FileInfo)
                            if dstats.top_files.len() < cli.top_size as usize {
                                dstats.top_files.push(Reverse(finfo));
                            } else if let Some(Reverse(smallest)) = dstats.top_files.peek()
                                && finfo.size > smallest.size
                            {
                                dstats.top_files.pop();
                                dstats.top_files.push(Reverse(finfo));
                            }
                        }
                    }
                    Err(errm) => eprintln!("There was an issue getting file metdaata {}", errm),
                }
            }
            Err(err) => eprintln!("DirEntry error: {}", err),
        }
    }
    let sorted_heap = dstats.top_files.into_sorted_vec();
    let mut top_file_by_size = (&String::new(), 0);
    // need name too
    if let Some(largest) = sorted_heap.first() {
        top_file_by_size = (&largest.0.name, largest.0.size);
    }

    let sum_top_n = sorted_heap.iter().fold(0, |acc, x| acc + x.0.size);
    let top_n_percentage = (sum_top_n as f64 / dstats.total_size as f64) * 100.0;

    let width = 90;
    let title = format!("dirpulse · {}", td);
    let stats = format!(
        "🔴 {:<5} Stale, 📦 Largest: {} ({}), 📊 Top 10 = {:.2}%",
        bytes_to_human(dstats.age.stale.size),
        top_file_by_size.0,
        bytes_to_human(top_file_by_size.1),
        top_n_percentage
    );
    println!("┌{}┐", "─".repeat(width));
    println!("│ {:<w$} │", title, w = width - 2);
    println!("├{}┤", "─".repeat(width));
    println!("│ {:<w$} │", stats, w = width - 5);
    println!("└{}┘", "─".repeat(width));

    println!();
    println!(
        "📁 {} files · {} directories · {} total",
        dstats.file_count,
        dstats.dir_count,
        bytes_to_human(dstats.total_size)
    );
    println!();
    println!(
        "── Top {} Largest ────────────────────────────────────────────────────",
        &cli.top_size
    );

    let mut count = 1;
    for entry in sorted_heap {
        println!(
            "{:<10}   {:<7}   {:<40}",
            count,
            bytes_to_human(entry.0.size),
            entry
                .0
                .path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );
        count += 1;
    }

    println!("── By Extension ──────────────────────────────────────────────────────");
    let mut sorted_exts: Vec<_> = dstats.types.iter().collect();
    sorted_exts.sort_by(|a, b| b.1.file_count.cmp(&a.1.file_count));
    for (key, value) in sorted_exts {
        println!(
            ".{:<10}         {:<10}           {:<10}",
            key,
            value.file_count,
            bytes_to_human(value.total_size)
        );
    }

    println!("── File Age ──────────────────────────────────────────────────────────");
    println!(
        "{:<30} {:>5} files   {:>10} ",
        "🟢 Fresh (< 30 days)",
        dstats.age.fresh.count,
        bytes_to_human(dstats.age.fresh.size)
    );
    println!(
        "{:<30} {:>5} files   {:>10} ",
        "🟡 Aging (30 days - 6 mo)",
        dstats.age.aging.count,
        bytes_to_human(dstats.age.aging.size)
    );
    println!(
        "{:<30} {:>5} files   {:>10} ",
        "🔴 Stale (> 6 months)",
        dstats.age.stale.count,
        bytes_to_human(dstats.age.stale.size)
    );
}
