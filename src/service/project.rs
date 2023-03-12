use std::collections::{HashMap, HashSet};

use crate::domain::{
    assignment::Assignment, employee::Employee, error::Error, position::Position, project::Project,
};

pub trait ProjectService {
    fn find_matching_projects(&self, employee: &Employee) -> HashSet<Project>;

    fn find_matching_employees(&self, project: &Project) -> HashMap<Position, HashSet<Employee>>;

    /// This should fail for invalid assignments
    fn try_assign(
        &self,
        employee: Employee,
        position: Position,
        project: Project,
    ) -> Result<Assignment, Error>;

    fn try_save(&mut self, project: Project) -> Result<(), Error>;
}
