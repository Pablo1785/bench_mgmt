use thiserror::Error;

use super::{employee::Employee, position::Position, project::Project};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Entity with id {0} already exists")]
    AlreadyExistsError(String),
    #[error("Employee {employee:?} does not match position {position:?}")]
    EmployeeDoesNotMatchPositionError {
        employee: Employee,
        position: Position,
    },
    #[error("Employee {employee:?} does not match project {project:?}")]
    EmployeeDoesNotMatchProjectError {
        employee: Employee,
        project: Project,
    },
    #[error("Employee {employee:?} does not match project {project:?} nor position {position:?}")]
    EmployeeDoesNotMatchProjectNorPositionError {
        employee: Employee,
        project: Project,
        position: Position,
    },
}
