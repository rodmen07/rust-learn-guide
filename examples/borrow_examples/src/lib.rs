pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

pub fn append_value(v: &mut Vec<i32>, value: i32) {
    v.push(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_longest_first() {
        let a = "short";
        let b = "much longer";
        assert_eq!(longest(a, b), b);
    }

    #[test]
    fn returns_longest_second() {
        let a = "long enough";
        let b = "tiny";
        assert_eq!(longest(a, b), a);
    }

    #[test]
    fn append_mutates_vector() {
        let mut v = vec![1, 2];
        append_value(&mut v, 3);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
