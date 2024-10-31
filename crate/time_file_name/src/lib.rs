//! this is a shared crate for this app.
//! The local file name will be /yyyy/MM/dd/yyyy-MM-dd-hh-mm-ss.{extension}

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono::offset::LocalResult;

/// This struct is designed for generating the file path
/// This main function is `generate_file_path`, but this struct must be created beforehand.
/// 
/// # Example
/// When the input is an ISO format string
/// ```rust
/// # use time_file_name::FilePath;
/// # fn main() {
///  let date_time = "1984-04-04T12:34:50Z";
///  let file_path = FilePath::new().generate_file_path(date_time, "video");
///  assert_eq!(file_path.unwrap(), "/1984/4/4/1984-4-4-12-34-50.video");
/// # }
/// ``` 
/// # Example
/// When the input is an epoch timestamp
/// ```rust
/// # use time_file_name::FilePath;
/// # fn main() {
///  let date_time = 449930090000_u128;
///  let file_path = FilePath::new().generate_file_path(date_time, "video");
///  assert_eq!(file_path.unwrap(), "/1984/4/4/1984-4-4-12-34-50.video");
/// # }
/// ```
pub struct FilePath {}

impl FilePath {
    
    pub fn new() -> Self { FilePath{} }
    
    /// Check if the extension is not empty
    fn check_extension(&self, extension: &str) -> Result<(), String> { 
        match extension.len() < 1 {
            true => Err("Invalid extension".to_string()),
            false => Ok(())
        } 
    }

    /// Generate a file path
    /// Acceptable type is u128, epoch time, and &str, the ISO 8061 string.
    
    pub fn generate_file_path<DateTimeType>(&self, date_time: DateTimeType, extension: &str) -> Result<String, String> where Self: GenerateFile<DateTimeType> {
        self.generate_file_path_from_datetime(date_time, extension)
    }
}

pub trait GenerateFile<DateTimeType> {
    /// A path is created by the date time.
    fn generate_file_path_from_datetime(&self, date_time: DateTimeType, extension: &str) -> Result<String, String>;
}

impl GenerateFile<u128> for FilePath {
    /// The date time must be the epoch time.
    fn generate_file_path_from_datetime(&self, date_time: u128, extension: &str) -> Result<String, String> {
        self.check_extension(extension)?;
        let date_time_chrono = epoch_to_datetime(date_time)?;
        convert_file_name(date_time_chrono, extension)
    }
}

impl GenerateFile<&str> for FilePath {
    /// date time must be a ISO 8061 date time string
    fn  generate_file_path_from_datetime(&self, date_time: &str, extension: &str) -> Result<String, String> {
        self.check_extension(extension)?;
        let date_time_chrono = iso_date_time_to_datetime(date_time)?;
        convert_file_name(date_time_chrono, extension)
    }
}

/// create the file path from the date time.
/// It is /yyyy/MM/dd/yyyy-MM-dd-hh-mm-ss.{extension}
fn convert_file_name(
    datetime: DateTime<Utc>,
    extension: &str,
) -> Result<String, String> {
    
    let without_dot_extension = match extension.starts_with(".") {
        true => extension.split_at(1).1,
        false => extension,
    };

    Ok(format!(
        "/{}/{}/{}/{}-{}-{}-{}-{}-{}.{}",
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        without_dot_extension,
    ))
} 

/// take an epoch time as an argument, then returns the datetime struct
fn epoch_to_datetime(epoch_time: u128) -> Result<DateTime<Utc>, String> {
    match DateTime::from_timestamp_millis(epoch_time as i64) {
        Some(datetime) => Ok(datetime),
        None => Err("Cannot convert the provided epoch time".to_string()),
    }
}

/// take the ISO 8061 ,
fn iso_date_time_to_datetime(date_time: &str) -> Result<DateTime<Utc>, String> {
    match date_time.parse::<DateTime<Utc>>() {
        Ok(time) => Ok(time),
        Err(_) => Err(format!("cannot convert the provided epoch time to datetime: {}", date_time)),
    }
}

///  parse the file path
fn from_file_name_to_date_time(path: &str) -> Result<DateTime<Utc>, String> {

    let file_path = match &path[..1] == "/" {
        true => path[1..].to_owned(),
        false => path.to_owned(),
    };

    println!("file path: {:?}", file_path);

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
        return Err(format!("invalid hour in the file name: {}", vec_file_name[3]));
    };
    let Ok(minute) = vec_file_name[4].parse::<u32>() else {
        return Err(format!("invalid minute in the file name: {}", vec_file_name[4]));
    };
    let Ok(sec) = sec_str.parse::<u32>() else {
        return Err(format!("invalid sec in the file name: {}", sec_str));
    };

    match Utc.with_ymd_and_hms(year, month, day, hour, minute, sec) {
        LocalResult::Single(datetime) => Ok(datetime),
        LocalResult::Ambiguous(_, _) => Err(format!("Ambiguous path: {}", path)),
        _ => Err("Invalid path is provided, so cannot convert it.".to_string())
    }
}

#[cfg(test)]
mod test_generate_file_path {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_generate_file_path() {
        // Assert
        const YEAR: i32  = 1984;
        const MONTH: u32  = 4;
        const DAY: u32  = 4;
        const HOUR: u32  = 12;
        const MIN: u32  = 42;
        const SEC: u32  = 42;
        
        let date_time = Utc.with_ymd_and_hms(YEAR, MONTH, DAY, HOUR, MIN, SEC).unwrap();
        let extension = ".VIDEO";
        
        // Act
        let result = convert_file_name(date_time, extension);

        // Assert
        let expected_date_time = format!(
            "/{}/{}/{}/{}-{}-{}-{}-{}-{}.{}",
            YEAR, MONTH, DAY, YEAR, MONTH, DAY, HOUR, MIN, SEC, "VIDEO"
        );
        assert_eq!(result.unwrap(), expected_date_time);
    }
}

#[cfg(test)]
mod test_epoch_to_datetime {
    use super::*;
    
    #[test]
    fn test_normal_case() {
        // Arrange
        let date_time = 449930090000;
        
        // Acr
        let result = epoch_to_datetime(date_time).unwrap();
        
        // Assert
        assert_eq!(result.year(), 1984);
        assert_eq!(result.month(), 4);
        assert_eq!(result.day(), 4);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 34);
        assert_eq!(result.second(), 50);
    }
}

#[cfg(test)]
mod test_iso_date_time_to_datetime {
    use super::*;
    
    #[test]
    fn test_normal_case() {
        // Arrange
        let date_time = "1984-04-04T12:34:50Z";
        
        // Acr
        let result = iso_date_time_to_datetime(date_time).unwrap();
        
        // Assert
        assert_eq!(result.year(), 1984);
        assert_eq!(result.month(), 4);
        assert_eq!(result.day(), 4);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 34);
        assert_eq!(result.second(), 50);
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
}
