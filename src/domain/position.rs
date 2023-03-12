use std::collections::HashSet;

use crate::domain::{availability::Availability, role::Role};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub id: String,

    pub required_availability: Availability,
    pub required_competences: HashSet<Role>,
}

impl core::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
impl Default for Position {
    fn default() -> Self {
        Self {
            id: Default::default(),
            required_availability: Default::default(),
            required_competences: Default::default(),
        }
    }
}
