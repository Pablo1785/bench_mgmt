fn main() {
    // This is how you declare a range; ranges are evaluated lazily, like Python generators
    let natural_numbers = 1..;
    natural_numbers.contains(&1000); // true

    // Ranges include the first element and exclude the last
    (1..3).contains(&1); // true
    (1..3).contains(&2); // true
    (1..3).contains(&3); // false

    // To make ranges inclusive use '..='
    (0..=20).contains(&20); // true
    (0..20).contains(&20); // false

    // 'for' iterates over anything that implements Iterator
    for i in vec![1, 2, 3] {
        println!("this is number {}", i);
    }

    for i in &[1, 2, 3] {
        println!("this is number from slice {}", i);
    }

    for i in &[1, 2, 3].map(|val| val * val) {
        println!("this is number from slice {}", i);
    }

    for c in "rust".chars() {
        println!("give me a {}", c);
    }

    for c in "Rust"
        .chars()
        .filter(|val| val.is_lowercase())
        .flat_map(|val| val.to_uppercase())
    {
        println!("give me a {}", c);
    }
}
