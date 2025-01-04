//! Entities

/// Collection
/// <https://hitohata.github.io/ExogenesisEnsemble-Part3-Redemption/project/docs/technical-information/DynamoDB-Definition/#collection>
pub mod collection {
    use std::collections::{HashMap, HashSet};
    use time_file_name::file_datetime::PathDateTime;

    pub struct CollectionItem {
        pub year: String,
        pub unix_time: i64,
        pub is_unzipped: bool,
        pub vault: String,
        /// This is a S3 bucket prefix name
        pub key_name: String,
    }

    impl CollectionItem {
        /// create a new item
        pub fn new_object(key_name: &str, vault: &str) -> Result<Self, String> {
            let path_date_time = PathDateTime::parse(key_name)?;

            Ok(CollectionItem {
                year: path_date_time.year.to_string(),
                unix_time: path_date_time.unix_time,
                is_unzipped: false,
                vault: vault.to_string(),
                key_name: key_name.to_string(),
            })
        }
    }

    #[cfg(test)]
    impl CollectionItem {
        /// dummy data from datetime
        /// the vault is just a string, "vault".
        pub fn dummy_object(key_name: &str) -> CollectionItem {
            CollectionItem::new_object(&key_name, "vault").unwrap()
        }
    }

    /// This is a helper struct for lookup items
    pub(crate) struct LookUpItems {
        pub years: Vec<String>,
        /// (year, [months])
        pub months: Vec<(usize, Vec<String>)>,
        /// (year, month, [days])
        #[allow(dead_code)]
        pub days: Vec<(usize, usize, Vec<String>)>,
        /// (year, month, days, [objects])
        pub objects: Vec<(usize, usize, usize, Vec<String>)>,
    }

    impl LookUpItems {
        pub(crate) fn new(collections: &Vec<CollectionItem>) -> Result<Self, String> {
            let mut years_hash: HashSet<String> = HashSet::new();
            let mut months_hash: HashMap<String, HashSet<String>> = HashMap::new();
            let mut days_hash: HashMap<String, HashSet<String>> = HashMap::new();
            let mut objects_hash: HashMap<String, HashSet<String>> = HashMap::new();

            for collection in collections {
                let time = PathDateTime::parse(collection.key_name.as_str())?;

                let year = time.year;
                let month = time.month;
                let day = time.day;

                years_hash.insert(year.to_string());

                months_hash
                    .entry(year.to_string())
                    .and_modify(|set| {
                        set.insert(month.to_string());
                    })
                    .or_insert(HashSet::from([month.to_string()]));

                let year_month = format!("{}-{}", year, month);

                days_hash
                    .entry(year_month.to_string())
                    .and_modify(|set| {
                        set.insert(day.to_string());
                    })
                    .or_insert(HashSet::from([day.to_string()]));

                let year_month_day = format!("{}-{}-{}", year, month, day);

                objects_hash
                    .entry(year_month_day.to_string())
                    .and_modify(|set| {
                        set.insert(collection.key_name.to_string());
                    })
                    .or_insert(HashSet::from([collection.key_name.to_string()]));
            }

            let years = years_hash.into_iter().collect::<Vec<String>>();

            let mut months = Vec::new();

            for (key, val) in months_hash.iter() {
                let Ok(year_usize) = key.parse::<usize>() else {
                    return Err("year parse failed".to_string());
                };
                months.push((year_usize, val.clone().into_iter().collect()));
            }

            let mut days = Vec::new();

            for (key, val) in days_hash.iter() {
                let name = key.split('-').collect::<Vec<&str>>();
                let Ok(year_usize) = name[0].parse::<usize>() else {
                    return Err("year parse failed".to_string());
                };
                let Ok(month_usize) = name[1].parse::<usize>() else {
                    return Err("year parse failed".to_string());
                };

                days.push((year_usize, month_usize, val.clone().into_iter().collect()));
            }

            let mut objects = Vec::new();

            for (key, val) in objects_hash.iter() {
                let name = key.split('-').collect::<Vec<&str>>();

                let Ok(year_usize) = name[0].parse::<usize>() else {
                    return Err("year parse failed".to_string());
                };
                let Ok(month_usize) = name[1].parse::<usize>() else {
                    return Err("year parse failed".to_string());
                };
                let Ok(day_usize) = name[2].parse::<usize>() else {
                    return Err("year parse failed".to_string());
                };

                objects.push((
                    year_usize,
                    month_usize,
                    day_usize,
                    val.clone().into_iter().collect(),
                ));
            }

            Ok(Self {
                years,
                months,
                days,
                objects,
            })
        }
    }

    #[cfg(test)]
    mod test {
        use crate::dynamodb::entities::collection::{CollectionItem, LookUpItems};

        #[test]
        fn test_collection() {
            // Arrange
            let date_times = vec![
                "1984/04/04/1984-04-04-12-34-50.MOV",
                "1984/04/04/1984-04-04-12-34-51.MOV",
                "1984/04/05/1984-04-05-12-34-50.MOV",
                "1984/05/04/1984-05-04-12-34-50.MOV",
                "1985/04/04/1985-04-04-12-34-50.MOV",
            ];

            let collections: Vec<CollectionItem> = date_times
                .iter()
                .map(|data| CollectionItem::dummy_object(data))
                .collect();

            // Act
            let look_up_items = LookUpItems::new(&collections).unwrap();

            // Arrange
            let mut years = look_up_items.years;
            years.sort();

            assert_eq!(years, ["1984", "1985"]);

            let mut months = look_up_items.months;
            months.sort();

            let expected_month = [
                (1984, vec!["4".to_string(), "5".to_string()]),
                (1985, vec!["4".to_string()]),
            ];

            assert_eq!(months.len(), expected_month.len());

            for (month, data) in std::iter::zip(months, expected_month) {
                let mut month_list = month.1;
                month_list.sort();
                assert_eq!((month.0, month_list), data);
            }

            let mut days = look_up_items.days;
            days.sort();

            let expected_days = [
                (1984, 4, vec!["4".to_string(), "5".to_string()]),
                (1984, 5, vec!["4".to_string()]),
                (1985, 4, vec!["4".to_string()]),
            ];

            assert_eq!(days.len(), expected_days.len());

            for (day, data) in std::iter::zip(days, expected_days) {
                let mut day_list = day.2;
                day_list.sort();

                assert_eq!((day.0, day.1, day_list), data);
            }

            let mut objects = look_up_items.objects;
            objects.sort();

            let expected_objects = [
                (
                    1984,
                    4,
                    4,
                    vec![
                        "1984/04/04/1984-04-04-12-34-50.MOV".to_string(),
                        "1984/04/04/1984-04-04-12-34-51.MOV".to_string(),
                    ],
                ),
                (
                    1984,
                    4,
                    5,
                    vec!["1984/04/05/1984-04-05-12-34-50.MOV".to_string()],
                ),
                (
                    1984,
                    5,
                    4,
                    vec!["1984/05/04/1984-05-04-12-34-50.MOV".to_string()],
                ),
                (
                    1985,
                    4,
                    4,
                    vec!["1985/04/04/1985-04-04-12-34-50.MOV".to_string()],
                ),
            ];

            assert_eq!(objects.len(), expected_objects.len());

            for (object, data) in std::iter::zip(objects, expected_objects) {
                let mut object_list = object.3;
                object_list.sort();

                assert_eq!((object.0, object.1, object.2, object_list), data);
            }
        }
    }
}
