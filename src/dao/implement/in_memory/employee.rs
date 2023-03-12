use std::collections::HashMap;

use crate::{dao::employee::EmployeeDAO, domain::employee::Employee};

pub struct EmployeeInMemoryDAO {
    data: HashMap<String, Employee>,
}

impl EmployeeDAO for EmployeeInMemoryDAO {
    fn get_by_id(&self, id: &str) -> Option<crate::domain::employee::Employee> {
        todo!()
    }

    fn upsert(
        &mut self,
        employee: crate::domain::employee::Employee,
    ) -> Option<&mut crate::domain::employee::Employee> {
        todo!()
    }

    fn get_all(&self) -> std::collections::HashSet<Employee> {
        todo!()
    }
}
