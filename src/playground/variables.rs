fn main() {
    // let introduces a variable binding:
    let x;
    x = 42;

    // This can be written as a single line
    let x = 42;

    // You can annotate variable types
    let x: i32;
    x = 42;

    // This can also be done on a single line
    let x: i32 = 42;

    // You cannot access uninitialized variables;
    let a: i32;
    a + 10;

    // But this is fine
    let a;
    a = 42;
    a + 10;

    // When in doubt, use i32 - equivalent to a 'long' in C or 'int' in Java

    // _ is a throwaway variable name; this does nothing
    // try omitting 'let' and see what happens
    let _ = 42;

    // This calls the function and ignores its result;
    _ = get_char();

    // This is how you declare and use tuples
    let pair = ('a', 22);
    pair.0; // -> 'a'
    pair.1; // -> 22

    // or with explicit type annotations
    let pair: (char, i32) = ('a', 22);

    // You can destructure tuples
    let (l, r) = pair;
    assert_eq!(l, 'a');
    assert_eq!(r, 22);

    // You can throwaway parts of tuples in assignment
    let (_, r) = pair;

    // fn declares a function; this function returns nothing
    #[rustfmt::skip]
    fn this_function_is_only_here_to_switch_off_autoformatting() {
        // Semicolons mark the end of a statement; statements can span multiple lines
        let x = vec![1, 2, 3, 4]
            .iter()
            .map(|x| x + 3)
            .fold(0, |x, y| x + y);
    }

    // You can omit the semicolon on the last expression to return its value
    // You will soon find out why this is useful
    fn get_num() -> i32 {
        4
    }

    let x = "out";
    // you can create a new scope by opening a block anywhere
    {
        // this is a different 'x'
        let x = "in";
        println!("{}", x); // prints "in"
    }
    println!("{}", x); // prints "out"

    // Blocks are also expressions that evaluate to a value
    let x = {
        let a = 1;
        let b = 2;
        a + b // the final expression without ';' is also called a 'tail'
    };

    // Below functions are equivalent
    fn return_thing() -> i32 {
        return 10;
    }
    fn implicit_return_thing() -> i32 {
        10
    }

    // This is why return by omitting ';' is useful
    fn get_conditional(is_val: bool) -> i32 {
        if is_val {
            5
        } else {
            10
        }
    }

    // The above worked because 'if's are also expressions
    let x = if true {
        let a = 1;
        let b = 2;
        a + b
    } else {
        let a = 10;
        let b = 20;
        a + b
    };

    // Similarly, 'match's are also expressions
    let is_val = true;
    let x = match is_val {
        true => 3,
        false => 30,
    };

    // You navigate namespaces using '::' and access properties and methods with '.'
    let a = Example { a: 'a', b: 23 };
    a.a;
    a.b;

    // Accessing constructs from other namespaces looks like this
    let x = std::cmp::min(1, 20);

    // This means roughly: 'crate_name::file_name::function_name'
    // Crates are things that combine multiple modules
    // Modules can span an entire file or part of a file with a 'mod modulename {}' block

    // Types are namespaces too
    let x = "abcd".len();
    let x = str::len("abcd");
}

fn get_char() -> char {
    'a'
}

struct Example {
    a: char,
    b: i32,
}
