use crate::domain::position::Position;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Project {
    pub id: String,
    pub required_positions: Vec<Position>,
}

impl core::hash::Hash for Project {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// TIP: Struct methods can be defined using the `impl MyStruct {}` block
impl Project {
    // TIP: `Self` in trait and struct definitions refers to a given struct's type;
    //      Below signature is equivalent to: `fn new(id: String) -> Project`
    pub fn new(id: String) -> Self {
        Project {
            id,
            required_positions: vec![],
        }
    }
}

#[cfg(test)]
impl Default for Project {
    fn default() -> Self {
        Self {
            id: Default::default(),
            required_positions: Default::default(),
        }
    }
}
