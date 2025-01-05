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
        let mut months = self.read_months(year)?.into_iter().collect::<Vec<_>>();
        months.sort();
        Ok(months)
    }

    pub fn days(&self, year: &str, month: &str) -> Result<Vec<String>, errors::ExogenesisError> {
        let mut days = self.read_days(year, month)?.into_iter().collect::<Vec<_>>();
        days.sort();
        Ok(days)
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

    /// add new years data
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

    /// add new months data
    pub fn add_months(
        &mut self,
        year: &str,
        months: Vec<String>,
    ) -> Result<(), errors::ExogenesisError> {
        let mut current_months = self.read_months(year)?;
        let provided_months = months.into_iter().collect::<HashSet<String>>();

        current_months.extend(provided_months);

        let Ok(mut lock) = self.months.write() else {
            return Err(errors::ExogenesisError::WriteLockFailed(
                "months".to_string(),
            ));
        };

        match lock.get_mut(format!("{year}").as_str()) {
            Some(current_month) => {
                current_month.extend(current_months);
            }
            None => {
                lock.insert(year.to_string(), current_months.clone());
            }
        };

        Ok(())
    }

    /// add new months data
    pub fn add_days(
        &mut self,
        year: &str,
        month: &str,
        days: Vec<String>,
    ) -> Result<(), errors::ExogenesisError> {
        let year_month = format!("{year}-{month}");

        let mut current_days = self.read_days(year, month)?;
        let provided_days = days.into_iter().collect::<HashSet<String>>();

        current_days.extend(provided_days);

        let Ok(mut lock) = self.days.write() else {
            return Err(errors::ExogenesisError::WriteLockFailed("days".to_string()));
        };

        match lock.get_mut(&year_month) {
            Some(current_day) => {
                current_day.extend(current_days);
            }
            None => {
                lock.insert(year_month, current_days.clone());
            }
        };

        Ok(())
    }

    fn read_years(&self) -> Result<HashSet<String>, errors::ExogenesisError> {
        match self.years.read() {
            Ok(read) => Ok(read.clone()),
            Err(_) => Err(errors::ExogenesisError::ReadLockFailed("years".to_string())),
        }
    }

    fn read_months(&self, year: &str) -> Result<HashSet<String>, errors::ExogenesisError> {
        let Ok(months) = self.months.read() else {
            return Err(errors::ExogenesisError::ReadLockFailed(
                "months".to_string(),
            ));
        };
        match months.get(year) {
            Some(months) => Ok(months.clone()),
            None => Ok(HashSet::new()),
        }
    }

    fn read_days(
        &self,
        year: &str,
        month: &str,
    ) -> Result<HashSet<String>, errors::ExogenesisError> {
        let Ok(months) = self.days.read() else {
            return Err(errors::ExogenesisError::ReadLockFailed("days".to_string()));
        };
        match months.get(format!("{year}-{month}").as_str()) {
            Some(days) => Ok(days.clone()),
            None => Ok(HashSet::new()),
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
    fn test_add_years() {
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

    #[test]
    fn test_add_months() {
        // Arrange
        let mut mapper = DateMapper::default();
        let year = "1984";
        mapper
            .add_months(year, vec!["3".to_string(), "4".to_string()])
            .unwrap();
        mapper
            .add_months(year, vec!["4".to_string(), "5".to_string()])
            .unwrap();

        // Act
        let new_months = mapper.months(year).unwrap();

        // Assert
        assert_eq!(new_months, vec!["3", "4", "5"]);
    }

    #[test]
    fn test_add_days() {
        // Arrange
        let mut mapper = DateMapper::default();
        let year = "1984";
        let month = "4";
        mapper
            .add_days(year, month, vec!["3".to_string(), "4".to_string()])
            .unwrap();
        mapper
            .add_days(year, month, vec!["4".to_string(), "5".to_string()])
            .unwrap();

        // Act
        let new_days = mapper.days(year, month).unwrap();

        // Assert
        assert_eq!(new_days, vec!["3", "4", "5"]);
    }
}
