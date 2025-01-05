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
        match self.years.read() {
            Ok(read) => Ok(read.clone().into_iter().collect::<Vec<_>>()),
            Err(_) => Err(errors::ExogenesisError::ReadLockFailed("years".to_string()))
        }
    }

    pub fn months(&self, year: &str) -> Result<Vec<String>, errors::ExogenesisError> {
        let Ok(months) = self.months.read() else {
            return  Err(errors::ExogenesisError::ReadLockFailed("months".to_string()))
        };
        match months.get(year) {
            Some(months) => Ok(months.clone().into_iter().collect::<Vec<_>>()),
            None => Ok(Vec::new())
        }
    }
    
    pub fn days(&self, year: &str, month: &str) -> Result<Vec<String>, errors::ExogenesisError> {
        let Ok(months) = self.days.read() else {
            return  Err(errors::ExogenesisError::ReadLockFailed("days".to_string()))
        };
        match months.get(format!("{year}-{month}").as_str()) {
            Some(months) => Ok(months.clone().into_iter().collect::<Vec<_>>()),
            None => Ok(Vec::new())
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