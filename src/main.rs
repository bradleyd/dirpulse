use clap::Parser;
use std::{cmp::Reverse, os::unix::fs::MetadataExt};
use walkdir::WalkDir;
mod age;
mod display;
mod file_info;
mod utils;

#[derive(Parser)]
struct Cli {
    target_directory: String,
    #[clap(short = 'n', long = "top-size", default_value_t = 10)]
    top_size: u64,
}

fn main() {
    let cli = Cli::parse();
    let td = &cli.target_directory.clone();

    let walker = WalkDir::new(cli.target_directory)
        .follow_links(true)
        .into_iter();

    let mut dstats = file_info::DirStats::new();
    let mut age_stats = age::AgeStats::default();
    for entry in walker.filter_entry(|e| !utils::is_hidden(e)) {
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
                            // get_extension(entry: DirEntry);
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
                                    let stats = file_info::TypeStats {
                                        file_count: 1,
                                        total_size: m.size(),
                                    };
                                    dstats.types.insert(ext.clone().unwrap(), stats);
                                }
                            }

                            // update age stats
                            age::update_age_stats(&m, &mut dstats, &mut age_stats);
                            let finfo = file_info::FileInfo {
                                name: fname,
                                path: direntry.path().to_path_buf(),
                                size: m.size(),
                                extension: ext,
                                mod_time: m.modified().unwrap(),
                            };

                            dstats.total_size += m.size();
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
    // TODO move these helpers off the structs
    let sorted_heap = dstats.top_files.into_sorted_vec();
    let mut top_file_by_size = (&String::new(), 0);
    // need name too
    if let Some(largest) = sorted_heap.first() {
        top_file_by_size = (&largest.0.name, largest.0.size);
    }

    let sum_top_n = sorted_heap.iter().fold(0, |acc, x| acc + x.0.size);
    let top_n_percentage = (sum_top_n as f64 / dstats.total_size as f64) * 100.0;

    display::print_hero(td, &dstats.age, top_file_by_size, top_n_percentage);

    println!();
    display::print_stat_line(dstats.file_count, dstats.dir_count, dstats.total_size);
    println!();
    display::print_top_largest(cli.top_size, &sorted_heap);
    println!();
    display::print_extensions(&dstats.types);
    display::print_file_age(&dstats.age);
}
