use std::collections::HashMap;

use crate::{dao::assignment::AssignmentDAO, domain::assignment::Assignment};

pub struct AssignmentInMemoryDAO {
    data: HashMap<String, Assignment>,
}

impl AssignmentDAO for AssignmentInMemoryDAO {
    fn get_by_id(&self, id: &str) -> Option<crate::domain::assignment::Assignment> {
        todo!()
    }

    fn upsert(
        &mut self,
        assignment: crate::domain::assignment::Assignment,
    ) -> Option<&mut crate::domain::assignment::Assignment> {
        todo!()
    }

    fn get_all(&self) -> std::collections::HashSet<Assignment> {
        todo!()
    }
}
