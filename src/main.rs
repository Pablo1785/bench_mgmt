use std::collections::HashMap;

// TIP: Variables are immutable by default.
// ```
// let a = 10;
// let b = a + 20;
// println!("`a` equals {}", a);
// println!("`b` equals {}", b);
// ```


// TIP: You cannot change immutable variables BUT you can reassign new values to them with `let`.
// ```
// let a = 10;
// a = 20;  // This will NOT compile!
// ```
//
// ```
// let a = 10;
// let a = 20;  // OK!
// ```


// TIP: Mutable variables can be declared with `mut`.
// ```
// let mut a = 10;
// a += 20;
// println!("`a` equals {}", a);
// ```


// TIP: Variables are always initialized at their declaration and dropped at the end of the current scope.
// ```
// {
//     let a = 10;   
// }
// println!("`a` equals {}", a);  // This will NOT compile!
// ```

// TIP: Rust is expression-based, which means almost any language construct evaluates to something.
//      Therefore almost every statement in Rust has an expression form that can return a value and be used in-line.
// ```
// let b = {
//    let a = 10;
//    return a + 20;
// };  // Notice the semicolon 
// 
// let b = if true {
//     return 10;
// } else {
//     return 20;   
// };
// 
// let mut counter = 0;
// let b = loop {
//     if i == 5 { break 50; } // TIP: `loop` creates infinite loops and can use `break` to return a value
//     counter += 1;  
// };
// ```


// TIP: Instead of using `return` at the end of a code block you can omit the semicolon to return
// ```
// let b = if true {
//     10
// } else {
//     20    
// };

// TIP: Structs are similar to Golang structs, Python dataclasses or Plain Old Java Objects.
// ```
// let emp = Employee { id: "Joseph Joestar".to_string(), availabilities: vec![], competences: vec![] };
// println!("My name is {}", emp.id);
// ```
struct Employee {
    id: String,

    availabilities: Vec<Availability>,

    competences: Vec<Role>,
}

#[derive(Clone)]
/// Part of a 40 hour work week in a time period
struct Availability {
    fraction: f64,
    start_at: chrono::DateTime<chrono::Utc>,
    end_at: Option<chrono::DateTime<chrono::Utc>>
}

#[derive(Clone)]
struct Position {
    id: String,
    
    required_availability: Availability,
    required_competences: Vec<Role>,
}

#[derive(Clone)]
enum Role {
    Designer,

    // TIP: Every enum variant can have a different set of named fields
    Developer { avg_project_delay_days: u8, is_cobol_ninja: bool },
    Finance,
    HR,
    Legal,

    // TIP: Every enum variant can have a different set of unnamed fields AKA be a tuple
    Marketing(usize, String),
    ProductManager,
    ProjectManager,
    OfficeManager,
    QA,
}

impl Role {
    fn can_maintain_a_banking_app(&self) -> bool {
        match self {
            // TIP: Pattern matching is very convenient in tandem with enums
            // TIP: `*<variable name>` is used for dereferencing
            Role::Developer { avg_project_delay_days, is_cobol_ninja } => avg_project_delay_days > &10 && *is_cobol_ninja,
            _ => false
        }
    }

    fn get_brand_power_level(&self) -> usize {
        match self {
            // TIP: Add additional restrictions on `match` branches with `if` guards
            Role::Marketing(level_of_funny, social_media_of_the_day) if social_media_of_the_day.to_lowercase().split_whitespace().collect::<String>() == "tiktok" => *level_of_funny,
            Role::Marketing(level_of_funny, _) => level_of_funny / 2,
            _ => 0
        }
    }
}

#[derive(Clone)]
struct Project {
    id: String,
    required_positions: Vec<Position>,
}

// TIP: Struct methods can be defined using the `impl MyStruct {}` block
impl Project {
    // TIP: `Self` in trait and struct definitions refers to a given struct's type;
    //      Below signature is equivalent to: `fn new(id: String) -> Project`
    fn new(id: String) -> Self {
        Project { id, required_positions: vec![] }
    }
}


struct ProjectRepository {
    data: HashMap<String, Project>
}

impl ProjectRepository {
    // TIP: Option type signifies the possibility for a value to not exist. This is often handled via the `null` type in other languages
    fn get_by_id(self, id: String) -> Option<Project> {
        // TIP: Pattern matching
        if let Some(project) = self.data.get(&id) {
            // TIP: Return by omitting the semicolon in the last line
            Some(project.clone())
        } else {
            // TIP: Return by keyword
            return None
        }

        // TIP: Can be shortened to:
        // self.data.get(&id).cloned()
    }

    // TIP: Indicate failures via the Result type. This is the default way of handling errors in Rust
    // TIP: All Rust expressions evaluate to a value. Empty expressions, such as functions without return values evaluate to an empty tuple (or, the unit type) - `()`
    fn try_insert(mut self, project: Project) -> Result<(), AlreadyExistsError<String>> {
        if self.data.contains_key(&project.id) {
            Err(AlreadyExistsError::new(project.id))
        } else {
            // EXC: Remove `.clone()` to see a borrow checker error
            self.data.insert(project.id.clone(), project);
            Ok(())
        }
    }
}

struct AlreadyExistsError<T> {
    value: T
}

/// TIP: You can easily create conversion methods between your types using the `From<>` trait
/// ```
/// impl From<T> for AlreadyExistsError<T> {
///     fn from(value: T) -> Self {
///         AlreadyExistsError { value }
///     }
/// }
/// 
/// let a = "my string";
/// let err = AlreadyExistsError::from(a);
/// ```
/// 
/// It automagically generates a complementary `.into()` method on the other type by implementing the `Into<>` trait.
/// ```
/// let a = 5;
/// let err: AlreadyExistsError<i64> = a.into()
/// ```
/// 
/// Implementing `Into<>` does not provide `From<>` on the other type.
/// 
/// TIP: Implementing `TryFrom<>` also provides `From<>`, as well as providing `TryInto<>` and `Into<>` on the other type.
impl<T> AlreadyExistsError<T> {
    fn new(value: T) -> Self {
        let b = {
            let a = 10;
            a + 20
        };
        AlreadyExistsError { value }
    }
}

fn main() {
    let my_project = Project::new("cool one".to_string());

    // TIP: Turbofish operator (::<Type1,Type2,...>) can be used to specify types for generics
    // TIP: Variables are immutable by default. To perform mutations you need to declare them with `mut` keyword
    let mut data = HashMap::<String, Project>::new();

    println!("Hello, {}!", my_project.id);
}
