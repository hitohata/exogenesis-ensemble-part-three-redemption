use crate::stores::date;

/// the root state of this app
pub struct Store {
    #[allow(dead_code)] // TODO: remove
    date: date::DateStates,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            date: date::DateStates::default(),
        }
    }
}
