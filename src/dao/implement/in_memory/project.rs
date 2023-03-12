use std::borrow::Borrow;
use std::collections::HashMap;

use crate::dao;
use crate::domain;
use crate::domain::error::Error;

pub struct ProjectInMemoryDAO {
    data: HashMap<String, domain::project::Project>,
}

impl ProjectInMemoryDAO {
    pub fn new() -> Self {
        ProjectInMemoryDAO {
            data: HashMap::new(),
        }
    }

    pub fn with_data(data: HashMap<String, domain::project::Project>) -> Self {
        ProjectInMemoryDAO { data }
    }
}

impl dao::project::ProjectDAO for ProjectInMemoryDAO {
    fn get_by_id(&self, id: &str) -> Option<domain::project::Project> {
        // TIP: Pattern matching
        if let Some(project) = self.data.get(id) {
            // TIP: Return by omitting the semicolon in the last line
            Some(project.clone())
        } else {
            // TIP: Return by keyword
            return None;
        }

        // TIP: Can be shortened to:
        // self.data.get(&id).cloned()
    }

    fn upsert(&mut self, project: domain::project::Project) -> Result<(), Error> {
        self.data
            .entry(project.id.clone())
            .and_modify(|existing_project| {
                // TIP: Dereference a '&mut' value wiht '*' to modify it
                // TIP: You can '.clone()' values to easily appease the borrow-checker; excessive usage of '.clone()' may be a sign that your program is not structured very well
                *existing_project = domain::project::Project { ..project.clone() }
            })
            .or_insert(project);
        Ok(())
    }

    fn try_insert(
        &mut self,
        project: domain::project::Project,
    ) -> Result<(), domain::error::Error> {
        todo!()
    }

    fn get_all(&self) -> std::collections::HashSet<domain::project::Project> {
        self.data.values().cloned().collect()
    }
}
