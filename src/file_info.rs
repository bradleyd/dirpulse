use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    path::PathBuf,
    time::SystemTime,
};

use walkdir::DirEntry;

use crate::age;

#[derive(Debug)]
pub struct DirStats {
    pub total_size: u64,
    pub file_count: u64,
    pub dir_count: u64,
    pub top_files: BinaryHeap<Reverse<FileInfo>>,
    pub types: HashMap<String, TypeStats>,
    pub age: age::AgeStats,
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
    pub fn new() -> DirStats {
        DirStats {
            total_size: 0,
            file_count: 0,
            dir_count: 0,
            top_files: BinaryHeap::new(),
            types: HashMap::new(),
            age: age::AgeStats::default(),
        }
    }

    pub fn process_entry(
        &mut self,
        entry: &DirEntry,
        top_size: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let m = entry.metadata()?;
        if m.is_dir() && entry.depth() > 0 {
            self.dir_count += 1;
            Ok(())
        } else if m.is_file() {
            self.file_count += 1;
            let fname = entry
                .file_name()
                .to_str()
                .map(|s| s.to_string())
                .unwrap_or_default();

            // get_extension(entry: DirEntry);
            let ext = entry
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_string());

            // check is extension already in tstats
            if let Some(extension) = ext.as_ref() {
                if let Some(stats) = self.types.get_mut(extension) {
                    stats.file_count += 1;
                    stats.total_size += m.len();
                } else {
                    // first time seeing this extension name
                    let stats = TypeStats {
                        file_count: 1,
                        total_size: m.len(),
                    };
                    self.types.insert(ext.clone().unwrap(), stats);
                }
            }

            // update age stats
            age::update_age_stats(&m, self);

            let finfo = FileInfo {
                name: fname,
                path: entry.path().to_path_buf(),
                size: m.len(),
                extension: ext,
                mod_time: m.modified().unwrap_or(SystemTime::now()),
            };

            // update total size from this file
            self.total_size += finfo.size;
            if self.top_files.len() < top_size {
                self.top_files.push(Reverse(finfo));
            } else if let Some(Reverse(smallest)) = self.top_files.peek() {
                if finfo.size > smallest.size {
                    self.top_files.pop();
                    self.top_files.push(Reverse(finfo));
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
}

impl Default for DirStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct TypeStats {
    pub file_count: u64,
    pub total_size: u64,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub extension: Option<String>,
    pub mod_time: SystemTime,
}
