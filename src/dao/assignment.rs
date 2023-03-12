use std::collections::HashSet;

use crate::domain::assignment::Assignment;

pub trait AssignmentDAO {
    // TIP: Option type signifies the possibility for a value to not exist. This is often handled via the `null` type in other languages
    // TIP: Rust does a lot of static type inference; you can pass a &String type as an argument and it will be implicitly used as a &str
    fn get_by_id(&self, id: &str) -> Option<Assignment>;

    fn get_all(&self) -> HashSet<Assignment>;

    fn upsert(&mut self, assignment: Assignment) -> Option<&mut Assignment>;
}
