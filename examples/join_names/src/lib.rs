pub fn join_names<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    format!("{} {}", a.as_ref(), b.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_str_and_str() {
        assert_eq!(join_names("a", "b"), "a b");
    }

    #[test]
    fn joins_string_and_str() {
        assert_eq!(join_names(String::from("hello"), "world"), "hello world");
    }

    #[test]
    fn joins_string_and_string() {
        assert_eq!(join_names(String::from("x"), String::from("y")), "x y");
    }
}
