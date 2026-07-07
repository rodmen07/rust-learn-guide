fn greeting() -> &'static str {
    "Hello, Rust learner!"
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn returns_the_expected_message() {
        assert_eq!(greeting(), "Hello, Rust learner!");
    }
}
