use std::collections::HashSet;

use crate::domain::{availability::Availability, role::Role};

#[derive(Debug)]
pub struct Employee {
    pub id: String,

    pub availabilities: HashSet<Availability>,

    pub competences: HashSet<Role>,
}

#[cfg(test)]
impl Default for Employee {
    fn default() -> Self {
        Self {
            id: Default::default(),
            availabilities: Default::default(),
            competences: Default::default(),
        }
    }
}
