use std::{
    fs::Metadata,
    os::unix::fs::MetadataExt,
    time::{Duration, SystemTime},
};

use crate::file_info;

#[derive(Default, Clone, Copy, Debug)]
pub struct BucketStats {
    pub count: u64,
    pub size: u64,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AgeStats {
    pub fresh: BucketStats,
    pub aging: BucketStats,
    pub stale: BucketStats,
}

#[derive(Debug)]
pub enum AgeBucket {
    Fresh,
    Aging,
    Stale,
}

pub fn classify_age(modified_time: SystemTime) -> AgeBucket {
    let now = SystemTime::now();
    let delta = match now.duration_since(modified_time) {
        Ok(diff) => diff,
        Err(_) => return AgeBucket::Fresh,
    };
    let day: u64 = 24 * 60 * 60;
    let thirty_days = Duration::from_secs(30 * day);
    let six_months = Duration::from_secs(30 * 6 * day);

    if delta < thirty_days {
        AgeBucket::Fresh
    } else if delta >= thirty_days && delta < six_months {
        AgeBucket::Aging
    } else {
        AgeBucket::Stale
    }
}

pub fn update_age_stats(m: &Metadata, dstats: &mut file_info::DirStats) {
    if let Ok(mtime) = m.modified() {
        let classified_time = classify_age(mtime);
        let bucket = match classified_time {
            AgeBucket::Fresh => &mut dstats.age.fresh,
            AgeBucket::Aging => &mut dstats.age.aging,
            AgeBucket::Stale => &mut dstats.age.stale,
        };
        bucket.count += 1;
        bucket.size += m.size();
    } else {
        println!("There was no modified time for");
    }
}
