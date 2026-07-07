pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("{}", greeting("Rust learner"));
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn returns_name() {
        assert_eq!(greeting("Alice"), "Hello, Alice!");
    }
}
