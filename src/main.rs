mod dao;
mod domain;
mod service;

use std::collections::HashMap;

use crate::domain::project::Project;

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
// let emp = Employee { id: "Joseph Joestar".to_string(), availabilities: HashSet::new(), competences: HashSet::new() };
// println!("My name is {}", emp.id);
// ```

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

fn main() {
    let my_project = Project::new("cool one".to_string());

    // TIP: Turbofish operator (::<Type1,Type2,...>) can be used to specify types for generics
    // TIP: Variables are immutable by default. To perform mutations you need to declare them with `mut` keyword
    let mut data = HashMap::<String, Project>::new();

    println!("Hello, {}!", my_project.id);
}
