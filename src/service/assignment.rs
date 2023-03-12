use crate::domain::{assignment::Assignment, error::Error};

pub trait AssignmentService {
    /// This should fail for existing or invalid assignments
    fn try_save(&mut self, assignment: Assignment) -> Result<(), Error>;
}
