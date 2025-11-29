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
    pub missing: BucketStats,
}

#[derive(Debug)]
pub enum AgeBucket {
    Fresh,
    Aging,
    Stale,
    Missing,
}

pub fn classify_age(modified_time: SystemTime) -> AgeBucket {
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

pub fn update_age_stats(m: &Metadata, dstats: &mut file_info::DirStats, age_stats: &mut AgeStats) {
    if let Ok(mtime) = m.modified() {
        let classified_time = classify_age(mtime);
        match classified_time {
            AgeBucket::Fresh => {
                let mut current = age_stats.fresh;
                current.count += 1;
                current.size += m.size();
                age_stats.fresh = current;
                dstats.age = age_stats.clone();
            }
            AgeBucket::Aging => {
                let mut current = age_stats.aging;
                current.count += 1;
                current.size += m.size();
                age_stats.aging = current;
                dstats.age = age_stats.clone();
            }
            AgeBucket::Stale => {
                let mut current = age_stats.stale;
                current.count += 1;
                current.size += m.size();
                age_stats.stale = current;
                dstats.age = age_stats.clone();
            }
            AgeBucket::Missing => {
                let mut current = age_stats.missing;
                current.count += 1;
                current.size += m.size();
                age_stats.missing = current;
                dstats.age = age_stats.clone();
            }
        }
    } else {
        println!("There was no modified time for");
    }
}
