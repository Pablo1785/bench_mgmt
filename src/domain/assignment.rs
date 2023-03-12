use uuid::Uuid;

use super::{employee::Employee, position::Position, project::Project};

/// Non-exhaustive macro is here to restrict construction and update of this struct like: Assignment { ... } and Assignment { ..other }
/// This ensures that the callers have to create it with a constructor and handle invalid data
#[non_exhaustive]
#[derive(Debug)]
pub struct Assignment {
    pub id: String,
    pub employee: Employee,
    pub position: Position,
    pub project: Project,
}

impl Assignment {
    pub fn new(employee: Employee, position: Position, project: Project) -> Self {
        Assignment {
            id: Uuid::new_v4().to_string(),
            employee,
            position,
            project,
        }
    }
}

#[cfg(test)]
impl Default for Assignment {
    fn default() -> Self {
        Self {
            id: Default::default(),
            employee: Default::default(),
            position: Default::default(),
            project: Default::default(),
        }
    }
}
