fn main() {
    // Struct declaration
    struct Number {
        is_odd: bool,
        value: i32,
    }

    // Struct initialization
    let x = Number {
        is_odd: true,
        value: 33,
    };

    // Field order does not matter
    let x = Number {
        value: 33,
        is_odd: true,
    };

    fn print_number(n: Number) {
        // Match arms are patterns
        match n.value {
            1 => println!("One"),
            2 => println!("Two"),
            // Matches have to be exhaustive; this is why below 'catch-all' arm is necessary
            _ => println!("Not one or two: {}", n.value),
        }
    }

    // You can match deeply nested structures to easily use necessary values
    fn get_even(n: Number) -> i32 {
        match n {
            Number {
                is_odd: true,
                value,
            } => value * 2,
            Number {
                is_odd: false,
                value,
            } => value,
        }
    }

    // You can declare methods on your own types like so
    impl Number {
        fn is_positive(self) -> bool {
            self.value > 0
        }
    }
    let negative_two = Number {
        is_odd: false,
        value: -2,
    };
    println!("Is it positive? {}", negative_two.is_positive());

    // Variable bindings are immutable by default; they cannot be reassigned
    negative_two = Number {
        is_odd: true,
        value: 3,
    }; // error

    // ...and their interior cannot be mutated either
    negative_two.is_odd = true; // error

    // Use 'mut' to make variables mutable
    let mut mutable_num = Number {
        is_odd: true,
        value: 0,
    };
    mutable_num.value += 1;

    // Structs can be generic
    struct Pair<T> {
        a: T,
        b: T,
    }

    // Functions can be generic too
    fn foobar<T>(arg: T) {
        // work with arg
    }
    fn foobar2<L, R>(left: L, right: R) {
        // work with left and right
    }

    let a = Pair { a: true, b: false };
    let b = Pair { a: 23, b: 12 };

    // Use the turbofish ('::<>') operator to specify generic types
    foobar::<i32>(23);

    // ...but its not always necessary
    foobar(23);

    // std vectors are generic; they use arrays behind the scenes that grow when necessary
    let mut v1 = Vec::new(); // vector of integers
    v1.push(23);

    let mut v2 = Vec::new(); // vector of booleans
    v2.push(true);

    // You can initialize vectors with vec! macro
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false, true];

    // You will encounter macros all over the language; they look like:
    // name!()
    // name!{}
    // name![]
    // Macros expand to regular code at compile time;
    // Print is a macro
    println!("Hello world!");

    // It has the same effect as the code below
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello world!\n").unwrap();
}
