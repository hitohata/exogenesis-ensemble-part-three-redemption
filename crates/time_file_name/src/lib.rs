//! this is a shared crate for this app.
//! This crate handles file name and datetime
//! The local file name will be /yyyy/MM/dd/yyyy-MM-dd-hh-mm-ss.{extension}

pub mod file_datetime;
pub mod file_path;

#[cfg(test)]
/// This test is of checking if the date time and path are re-convert
mod test {
    use crate::file_datetime::PathDateTime;
    use crate::file_path::FilePath;
    use chrono::{TimeZone, Utc};

    #[test]
    fn time_to_time() {
        // Arrange
        let date_time = Utc.with_ymd_and_hms(1984, 4, 4, 12, 34, 50).unwrap();

        // Act
        let path = FilePath::new()
            .generate_file_path(date_time.to_rfc3339().as_str(), "video")
            .unwrap();
        let result = PathDateTime::parse(path.as_str()).unwrap();

        // Assert
        assert_eq!(result.unix_time, date_time.timestamp_millis());
    }

    #[test]
    fn string_to_string() {
        // Arrange
        let date_time = "1984-04-04T12:34:50+00:00";

        // Act
        let path = FilePath::new()
            .generate_file_path(date_time, "video")
            .unwrap();
        let result = PathDateTime::parse(path.as_str()).unwrap();

        // Assert
        assert_eq!(result.iso_string, date_time);
    }
}
