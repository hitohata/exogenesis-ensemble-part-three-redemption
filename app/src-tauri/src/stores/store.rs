use std::collections::{HashMap, HashSet};
use std::sync::RwLock;
use crate::errors;

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
    pub fn years(&self) -> Result<Vec<String>, errors::ExogenesisError> {
        match self.years.read() {
            Ok(read) => Ok(read.iter().collect::<Vec<String>>()),
            Err(_) => Err(errors::ExogenesisError::ReadLockFailed("years".to_string()))
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