use mockall::automock;

use crate::domain::{
    availability::Availability, employee::Employee, position::Position, project::Project,
};

#[automock]
pub trait EmployeeService {
    fn is_employee_available_during(&self, employee: &Employee, availability: Availability)
        -> bool;

    fn is_employee_matching_position(&self, employee: &Employee, position: &Position) -> bool;

    fn is_employee_matching_project(&self, employee: &Employee, project: &Project) -> bool;
}
