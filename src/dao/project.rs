use std::collections::HashSet;

use mockall::automock;

use crate::domain::{self, project::Project};

#[automock]
pub trait ProjectDAO {
    // TIP: Option type signifies the possibility for a value to not exist. This is often handled via the `null` type in other languages
    // TIP: Rust does a lot of static type inference; you can pass a &String type as an argument and it will be implicitly used as a &str
    fn get_by_id(&self, id: &str) -> Option<Project>;

    fn get_all(&self) -> HashSet<Project>;

    fn upsert(&mut self, project: Project) -> Result<(), domain::error::Error>;

    // TIP: Indicate failures via the Result type. This is the default way of handling errors in Rust
    // TIP: All Rust expressions evaluate to a value. Empty expressions, such as functions without return values evaluate to an empty tuple (or, the unit type) - `()`
    fn try_insert(&mut self, project: Project) -> Result<(), domain::error::Error>;
}
