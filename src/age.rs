use std::{
    fs::Metadata,
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

#[derive(Debug, PartialEq)]
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
        bucket.size += m.len();
    } else {
        println!("There was no modified time for");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn days_ago(days: u64) -> SystemTime {
        SystemTime::now() - Duration::from_secs(days * 24 * 60 * 60)
    }

    #[test]
    fn classify_age_now_is_fresh() {
        assert_eq!(classify_age(SystemTime::now()), AgeBucket::Fresh);
    }

    #[test]
    fn classify_age_one_day_is_fresh() {
        assert_eq!(classify_age(days_ago(1)), AgeBucket::Fresh);
    }

    #[test]
    fn classify_age_29_days_is_fresh() {
        assert_eq!(classify_age(days_ago(29)), AgeBucket::Fresh);
    }

    #[test]
    fn classify_age_30_days_is_aging() {
        assert_eq!(classify_age(days_ago(30)), AgeBucket::Aging);
    }

    #[test]
    fn classify_age_90_days_is_aging() {
        assert_eq!(classify_age(days_ago(90)), AgeBucket::Aging);
    }

    #[test]
    fn classify_age_179_days_is_aging() {
        assert_eq!(classify_age(days_ago(179)), AgeBucket::Aging);
    }

    #[test]
    fn classify_age_180_days_is_stale() {
        assert_eq!(classify_age(days_ago(180)), AgeBucket::Stale);
    }

    #[test]
    fn classify_age_one_year_is_stale() {
        assert_eq!(classify_age(days_ago(365)), AgeBucket::Stale);
    }

    #[test]
    fn classify_age_future_is_fresh() {
        let future = SystemTime::now() + Duration::from_secs(24 * 60 * 60);
        assert_eq!(classify_age(future), AgeBucket::Fresh);
    }
}
