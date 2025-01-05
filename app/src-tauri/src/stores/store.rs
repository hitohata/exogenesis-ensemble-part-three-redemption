use crate::errors;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

pub struct DateMapper {
    /// the years
    years: RwLock<HashSet<String>>,
    /// the months
    /// the key of the hashmap is the ${year}";
    months: RwLock<HashMap<String, HashSet<String>>>,
    /// the days
    /// the key of the hashmap is the ${year}-{month}";
    days: RwLock<HashMap<String, HashSet<String>>>,
    /// the objects
    /// the key of the hashmap is the ${year}-{month}-{day}";
    objects: RwLock<HashMap<String, HashSet<String>>>,
}

impl DateMapper {
    /// get years
    pub fn years(&self) -> Result<Vec<String>, errors::ExogenesisError> {
        let mut years = self.read_years()?.into_iter().collect::<Vec<_>>();
        years.sort();
        Ok(years)
    }

    pub fn months(&self, year: &str) -> Result<Vec<String>, errors::ExogenesisError> {
        let Ok(months) = self.months.read() else {
            return Err(errors::ExogenesisError::ReadLockFailed(
                "months".to_string(),
            ));
        };
        match months.get(year) {
            Some(months) => Ok(months.clone().into_iter().collect::<Vec<_>>()),
            None => Ok(Vec::new()),
        }
    }

    pub fn days(&self, year: &str, month: &str) -> Result<Vec<String>, errors::ExogenesisError> {
        let Ok(days) = self.days.read() else {
            return Err(errors::ExogenesisError::ReadLockFailed("days".to_string()));
        };
        match days.get(format!("{year}-{month}").as_str()) {
            Some(days) => Ok(days.clone().into_iter().collect::<Vec<_>>()),
            None => Ok(Vec::new()),
        }
    }

    pub fn objects(
        &self,
        year: &str,
        month: &str,
        day: &str,
    ) -> Result<Vec<String>, errors::ExogenesisError> {
        let Ok(objects) = self.objects.read() else {
            return Err(errors::ExogenesisError::ReadLockFailed(
                "objects".to_string(),
            ));
        };
        match objects.get(format!("{year}-{month}-{day}").as_str()) {
            Some(objects) => Ok(objects.clone().into_iter().collect::<Vec<_>>()),
            None => Ok(Vec::new()),
        }
    }

    /// add a new year data
    pub fn add_years(&mut self, years: Vec<String>) -> Result<(), errors::ExogenesisError> {
        let mut current_years = self.read_years()?;
        let provided_years: HashSet<String> = years.into_iter().collect();

        current_years.extend(provided_years);

        match self.years.write() {
            Ok(mut lock) => {
                *lock = current_years;
                Ok(())
            }
            Err(_) => Err(errors::ExogenesisError::WriteLockFailed(
                "years".to_string(),
            )),
        }
    }

    fn read_years(&self) -> Result<HashSet<String>, errors::ExogenesisError> {
        match self.years.read() {
            Ok(read) => Ok(read.clone()),
            Err(_) => Err(errors::ExogenesisError::ReadLockFailed("years".to_string())),
        }
    }
}

impl Default for DateMapper {
    fn default() -> Self {
        Self {
            years: RwLock::new(HashSet::new()),
            months: RwLock::new(HashMap::new()),
            days: RwLock::new(HashMap::new()),
            objects: RwLock::new(HashMap::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_year() {
        // Arrange
        let mut mapper = DateMapper::default();
        mapper
            .add_years(vec!["1984".to_string(), "1985".to_string()])
            .unwrap();
        mapper
            .add_years(vec!["1985".to_string(), "1986".to_string()])
            .unwrap();

        // Act
        let new_years = mapper.years().unwrap();

        // Assert
        assert_eq!(new_years, vec!["1984", "1985", "1986"]);
    }
}
