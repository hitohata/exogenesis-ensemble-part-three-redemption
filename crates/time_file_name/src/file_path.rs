//! This module is for converting from the datetime to the file path

use chrono::{DateTime, Datelike, Timelike, Utc};

/// This struct is designed for generating the file path
/// This main function is `generate_file_path`, but this struct must be created beforehand.
///
/// # Example
/// When the input is an ISO format string
/// ```rust
/// # use time_file_name::file_path::FilePath;
/// # fn main() {
///  let date_time = "1984-04-04T12:34:50Z";
///  let file_path = FilePath::new().generate_file_path(date_time, "video");
///  assert_eq!(file_path.unwrap(), "/1984/4/4/1984-4-4-12-34-50.video");
/// # }
/// ```
/// # Example
/// When the input is an epoch timestamp
/// ```rust
/// # use time_file_name::file_path::FilePath;
/// # fn main() {
///  let date_time = 449930090000_u128;
///  let file_path = FilePath::new().generate_file_path(date_time, "video");
///  assert_eq!(file_path.unwrap(), "/1984/4/4/1984-4-4-12-34-50.video");
/// # }
/// ```
pub struct FilePath {}

impl FilePath {
    pub fn new() -> Self {
        FilePath {}
    }

    /// Generate a file path
    /// Acceptable type is u128, epoch time, and &str, the ISO 8061 string.
    #[allow(private_bounds)]
    pub fn generate_file_path<DateTimeType>(
        &self,
        date_time: DateTimeType,
        extension: &str,
    ) -> Result<String, String>
    where
        Self: GenerateFile<DateTimeType>,
    {
        self.generate_file_path_from_datetime(date_time, extension)
    }

    /// create the file path from the date time.
    /// It is /yyyy/MM/dd/yyyy-MM-dd-hh-mm-ss.{extension}
    fn convert_file_name(
        &self,
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

    /// Check if the extension is not empty
    fn check_extension(&self, extension: &str) -> Result<(), String> {
        match extension.len() < 1 {
            true => Err("Invalid extension".to_string()),
            false => Ok(()),
        }
    }

    /// take an epoch time as an argument, then returns the datetime struct
    fn epoch_to_datetime(&self, epoch_time: u128) -> Result<DateTime<Utc>, String> {
        match DateTime::from_timestamp_millis(epoch_time as i64) {
            Some(datetime) => Ok(datetime),
            None => Err("Cannot convert the provided epoch time".to_string()),
        }
    }

    /// take the ISO 8061 ,
    fn iso_date_time_to_datetime(&self, date_time: &str) -> Result<DateTime<Utc>, String> {
        match date_time.parse::<DateTime<Utc>>() {
            Ok(time) => Ok(time),
            Err(_) => Err(format!(
                "cannot convert the provided epoch time to datetime: {}",
                date_time
            )),
        }
    }
}

trait GenerateFile<DateTimeType> {
    /// A path is created by the date time.
    fn generate_file_path_from_datetime(
        &self,
        date_time: DateTimeType,
        extension: &str,
    ) -> Result<String, String>;
}

impl GenerateFile<u128> for FilePath {
    /// The date time must be the epoch time.
    fn generate_file_path_from_datetime(
        &self,
        date_time: u128,
        extension: &str,
    ) -> Result<String, String> {
        self.check_extension(extension)?;
        let date_time_chrono = self.epoch_to_datetime(date_time)?;
        self.convert_file_name(date_time_chrono, extension)
    }
}

impl GenerateFile<&str> for FilePath {
    /// date time must be a ISO 8061 date time string
    fn generate_file_path_from_datetime(
        &self,
        date_time: &str,
        extension: &str,
    ) -> Result<String, String> {
        self.check_extension(extension)?;
        let date_time_chrono = self.iso_date_time_to_datetime(date_time)?;
        self.convert_file_name(date_time_chrono, extension)
    }
}

#[cfg(test)]
mod test_generate_file_path {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_generate_file_path() {
        // Assert
        const YEAR: i32 = 1984;
        const MONTH: u32 = 4;
        const DAY: u32 = 4;
        const HOUR: u32 = 12;
        const MIN: u32 = 42;
        const SEC: u32 = 42;

        let date_time = Utc
            .with_ymd_and_hms(YEAR, MONTH, DAY, HOUR, MIN, SEC)
            .unwrap();
        let extension = ".VIDEO";

        // Act
        let result = FilePath::new().convert_file_name(date_time, extension);

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
        let result = FilePath::new().epoch_to_datetime(date_time).unwrap();

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
        let result = FilePath::new()
            .iso_date_time_to_datetime(date_time)
            .unwrap();

        // Assert
        assert_eq!(result.year(), 1984);
        assert_eq!(result.month(), 4);
        assert_eq!(result.day(), 4);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 34);
        assert_eq!(result.second(), 50);
    }
}
