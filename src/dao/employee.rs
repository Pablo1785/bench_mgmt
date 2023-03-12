use std::collections::HashSet;

use crate::domain::employee::Employee;

pub trait EmployeeDAO {
    // TIP: Option type signifies the possibility for a value to not exist. This is often handled via the `null` type in other languages
    // TIP: Rust does a lot of static type inference; you can pass a &String type as an argument and it will be implicitly used as a &str
    fn get_by_id(&self, id: &str) -> Option<Employee>;

    fn get_all(&self) -> HashSet<Employee>;

    fn upsert(&mut self, employee: Employee) -> Option<&mut Employee>;
}
