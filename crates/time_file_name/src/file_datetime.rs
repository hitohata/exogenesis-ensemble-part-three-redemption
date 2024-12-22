//! This module handles conversion from the file path to the date time

use chrono::offset::LocalResult;
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

/// The date time
pub struct PathDateTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub file_name: String,
    pub unix_time: i64,
    pub iso_string: String,
}

impl PathDateTime {
    /// Convert from the file name to the struct
    ///
    /// # Example
    /// ```rust
    /// # use time_file_name::file_datetime::PathDateTime;
    /// # use chrono::{ Utc, TimeZone };
    /// # fn main() {
    /// let file_path = "/1984/4/4/1984-4-4-12-34-50.video";
    /// let date_time = PathDateTime::parse(file_path).unwrap();
    /// assert_eq!(date_time.year, 1984);
    /// assert_eq!(date_time.month, 4);
    /// assert_eq!(date_time.day, 4);
    /// assert_eq!(date_time.hour, 12);
    /// assert_eq!(date_time.minute, 34);
    /// assert_eq!(date_time.second, 50 );
    /// assert_eq!(date_time.file_name, "1984-4-4-12-34-50.video".to_string());
    /// assert_eq!(date_time.unix_time, Utc.with_ymd_and_hms(1984, 4, 4, 12, 34, 50).unwrap().timestamp_millis());
    /// # }
    /// ```
    pub fn parse(file_path: &str) -> Result<Self, String> {
        let date_time = from_file_name_to_date_time(file_path)?;
        let file_name = retrieve_file_name(file_path)?;

        Ok(Self {
            year: date_time.year(),
            month: date_time.month(),
            day: date_time.day(),
            hour: date_time.hour(),
            minute: date_time.minute(),
            second: date_time.second(),
            file_name,
            unix_time: date_time.timestamp_millis(),
            iso_string: date_time.to_rfc3339().to_string(),
        })
    }
}

/// Convert from the file path to the DateTime of Chrono
fn from_file_name_to_date_time(path: &str) -> Result<DateTime<Utc>, String> {
    let file_path = remove_slash(path);

    let vec_path = file_path.split("/").collect::<Vec<&str>>();

    // it will be four elements, year, month, day, file name
    if vec_path.len() != 4 {
        return Err("invalid file path is provided".to_string());
    };

    let vec_file_name = vec_path[3].split("-").collect::<Vec<&str>>();
    // it will be four elements, year, month, day, hour, minute, sec with extension
    if vec_file_name.len() != 6 {
        return Err("invalid file name is provided".to_string());
    }

    let sec_str = vec_file_name[5].split(".").collect::<Vec<&str>>()[0];

    let Ok(year) = vec_path[0].parse::<i32>() else {
        return Err(format!("invalid year in the file name: {}", vec_path[0]));
    };
    let Ok(month) = vec_path[1].parse::<u32>() else {
        return Err(format!("invalid month in the file name: {}", vec_path[1]));
    };
    let Ok(day) = vec_path[2].parse::<u32>() else {
        return Err(format!("invalid day in the file name: {}", vec_path[2]));
    };
    let Ok(hour) = vec_file_name[3].parse::<u32>() else {
        return Err(format!(
            "invalid hour in the file name: {}",
            vec_file_name[3]
        ));
    };
    let Ok(minute) = vec_file_name[4].parse::<u32>() else {
        return Err(format!(
            "invalid minute in the file name: {}",
            vec_file_name[4]
        ));
    };
    let Ok(sec) = sec_str.parse::<u32>() else {
        return Err(format!("invalid sec in the file name: {}", sec_str));
    };

    match Utc.with_ymd_and_hms(year, month, day, hour, minute, sec) {
        LocalResult::Single(datetime) => Ok(datetime),
        LocalResult::Ambiguous(_, _) => Err(format!("Ambiguous path: {}", path)),
        _ => Err("Invalid path is provided, so cannot convert it.".to_string()),
    }
}

/// retrieve a file name form the file path
fn retrieve_file_name(path: &str) -> Result<String, String> {
    let file_path = remove_slash(path);

    let vec_path = file_path.split("/").collect::<Vec<&str>>();

    // it will be four elements, year, month, day, file name
    if vec_path.len() != 4 {
        return Err("invalid file path is provided".to_string());
    };

    Ok(vec_path[3].to_string())
}

/// remove the slash
fn remove_slash(path: &str) -> String {
    match &path[..1] == "/" {
        true => path[1..].to_owned(),
        false => path.to_owned(),
    }
}

#[cfg(test)]
mod test_from_file_name_to_date_time {
    use super::*;

    #[test]
    fn test_normal_path() {
        // Assert
        let path = "/1984/04/04/1984-4-4-12-34-56.video";

        // Act
        let result = from_file_name_to_date_time(path).unwrap();

        // Assert
        assert_eq!(result.year(), 1984);
        assert_eq!(result.month(), 4);
        assert_eq!(result.day(), 4);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 34);
        assert_eq!(result.second(), 56);
    }

    #[test]
    fn test_normal_path_without_slash() {
        // Assert
        let path = "1984/04/04/1984-4-4-12-34-56.video";

        // Act
        let result = from_file_name_to_date_time(path).unwrap();

        // Assert
        assert_eq!(result.year(), 1984);
        assert_eq!(result.month(), 4);
        assert_eq!(result.day(), 4);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 34);
        assert_eq!(result.second(), 56);
    }

    #[test]
    fn test_invalid_year() {
        // Assert
        let path = "/hoge/04/04/1984-4-4-12-34-56.video";

        // Act
        let result = from_file_name_to_date_time(path);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "invalid year in the file name: hoge");
    }

    #[test]
    fn test_invalid_file_length() {
        // Assert
        let path = "/1984/04/1984-4-4-12-34-56.video";

        // Act
        let result = from_file_name_to_date_time(path);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_file_name() {
        // Assert
        let path = "/1984/04/04/1984-4-4-34-56.video";

        // Act
        let result = from_file_name_to_date_time(path);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_retrieve_file_name() {
        let path = "1984/04/04/1984-4-4-12-34-56.video";
        let object_name = retrieve_file_name(path).unwrap();
        assert_eq!(object_name, "1984-4-4-12-34-56.video".to_string());
    }
}
