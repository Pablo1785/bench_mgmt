fn main() {
    // Traits are somehting multiple types have in common
    trait Signed {
        fn is_negative(self) -> bool;
    }

    // You can implement:
    // 1. One of your trait on anybody's type, e.x. the above 'Signed' for i32
    // 2. Anybody's trait on one of your types, e.x. Iterator for your custom collection
    // BUT you can't implement foreign traits on foreign types, e.x. Iterator for i32

    // These are called 'orphan rules'; you may notice they are mentioned in error messages

    struct Number {
        is_odd: bool,
        value: i32,
    }

    // Rule 1
    impl Signed for Number {
        fn is_negative(self) -> bool {
            self.value < 0
        }
    }

    // Rule 1
    impl Signed for i32 {
        fn is_negative(self) -> bool {
            self < 0
        }
    }

    // Rule 2
    // 'impl' is always 'for' a type so inside the block 'Self' means that type
    impl std::ops::Neg for Number {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Number {
                value: -self.value,
                ..self
            }
        }
    }

    struct CoolNumber {
        is_odd: bool,
        value: i32,
    }
    // Trait methods can take 'self' by reference or mutable reference
    impl std::clone::Clone for CoolNumber {
        fn clone(&self) -> Self {
            Self { ..*self }
        }
    }

    // Some traits are marker traits; Copy implicitly clones values on move
    impl Copy for CoolNumber {}

    // Values are moved by default if they do not implement Copy
    let n = Number {
        is_odd: true,
        value: 3,
    };
    let b = n;
    let c = n; // error

    let n = CoolNumber {
        is_odd: true,
        value: 3,
    };
    let b = n;
    let c = n; // this is fine

    // Some traits are so simple they can be implemented automatically via derive
    #[derive(Clone, Copy)]
    struct DeriveNumber {
        is_odd: bool,
        value: i32,
    }

    let n = DeriveNumber {
        is_odd: true,
        value: 3,
    };
    let a = n;
    let b = n; // this is fine
}
