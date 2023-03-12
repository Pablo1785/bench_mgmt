use crate::{dao::assignment::AssignmentDAO, service::assignment::AssignmentService};

pub struct AssignmentServiceImpl<A: AssignmentDAO> {
    assignment_dao: A,
}

impl<A: AssignmentDAO> AssignmentService for AssignmentServiceImpl<A> {
    fn try_save(
        &mut self,
        assignment: crate::domain::assignment::Assignment,
    ) -> Result<(), crate::domain::error::Error> {
        todo!()
    }
}
