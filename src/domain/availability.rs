use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
/// Number of hours a week in a time period
pub struct Availability {
    pub hours_weekly: u8,
    pub start_at: chrono::DateTime<chrono::Utc>,
    pub end_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg(test)]
impl Default for Availability {
    fn default() -> Self {
        Self {
            hours_weekly: Default::default(),
            start_at: Default::default(),
            end_at: Default::default(),
        }
    }
}
