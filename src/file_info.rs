use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    path::PathBuf,
    time::SystemTime,
};

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
#[derive(PartialEq, Eq, Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub extension: Option<String>,
    pub mod_time: SystemTime,
}
