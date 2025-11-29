use clap::Parser;
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
    for entry in walker.filter_entry(|e| !utils::is_hidden(e)).flatten() {
        if let Err(e) = dstats.process_entry(&entry, cli.top_size as usize) {
            eprintln!("There was an error processing entry: {}", e);
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
