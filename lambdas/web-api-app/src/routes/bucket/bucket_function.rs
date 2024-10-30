//! This mod has s3-related functions.

use crate::routes::return_types::return_data_types::{
    DaysVideos, MonthsVideos, VideoObjects, YearsVideos,
};

/// Read the years that exist items in the s3 bucket.
pub async fn get_years() -> Result<YearsVideos, String> {
    Ok(YearsVideos { years: vec![] })
}

/// Read the month that existing items are narrowed down by year in the s3 bucket.
pub async fn get_months(years: usize) -> Result<MonthsVideos, String> {
    Ok(MonthsVideos { months: vec![] })
}

/// Read the days that existing items are narrowed down by year and month in the s3 bucket.
pub async fn get_days(year: usize, month: usize) -> Result<DaysVideos, String> {
    Ok(DaysVideos { days: vec![] })
}

/// Read the objects that existing items are narrowed down by year, month and day in the s3 bucket.
pub async fn get_objects(year: usize, month: usize, day: usize) -> Result<VideoObjects, String> {
    Ok(VideoObjects { objects: vec![] })
}
