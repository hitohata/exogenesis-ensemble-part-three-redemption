//! This is the return data type that is defined in the API doc.

pub mod return_data_types {
    use serde::Serialize;

    /// The years of the videos
    #[derive(Serialize, Debug)]
    pub struct YearsVideos {
        pub years: Vec<String>,
    }

    /// The months of the videos
    #[derive(Serialize, Debug)]
    pub struct MonthsVideos {
        pub months: Vec<String>,
    }

    /// The days of the videos
    #[derive(Serialize, Debug)]
    pub struct DaysVideos {
        pub days: Vec<String>,
    }

    /// The video object's name
    #[derive(Serialize, Debug)]
    pub struct VideoObjects {
        pub objects: Vec<String>,
    }
}
