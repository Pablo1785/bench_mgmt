fn main() {
    // Crashes in Rust are called panics; you can invoke a panic anywhere with this macro
    panic!("This will crash your program"); // this panics

    // Some methods and functions also panic
    // Option type can contain something or nothing; calling unwrap() will panic if it contains nothing
    let o1 = Some(32);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    o2.unwrap(); // this panics

    // Result can contain something or error; unwrap() will panic if its an error
    let r1: Result<i32, &str> = Ok(22);
    r1.unwrap(); // this is fine

    let r2: Result<i32, &str> = Err("msg");
    r2.unwrap(); // this panics

    // Under the hood both of these types are enums with two variants
    enum MyOption<T> {
        Some(T),
        None,
    }
    impl<T> MyOption<T> {
        fn unwrap(self) -> T {
            match self {
                // Capitalized 'S' Self refers to the type we are implementing methods for
                Self::Some(val) => val,
                Self::None => panic!(),
            }
        }
    }

    // Similarily Result
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    // Functions that can fail typically return Result
    let s = std::str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", s);

    let s = std::str::from_utf8(&[195, 40]);
    println!("{:?}", s);

    // Match can help you unpack enums
    match s {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }

    // 'if let' can be used too
    if let Ok(value) = s {
        println!("{}", value);
    } else {
        println!("Error encountered");
    }

    // The question mark operator returns early if encountered a None or an Error
    fn failable<T: std::fmt::Display, E>(r: Result<T, E>) -> Result<(), E> {
        let value = r?;
        println!("{}", value);
        Ok(())
    }

    // Above is equivalent to
    fn failable2<T: std::fmt::Display, E>(r: Result<T, E>) -> Result<(), E> {
        match r {
            Ok(value) => println!("{}", value),
            Err(err) => return Err(err),
        }
        Ok(())
    }
}
