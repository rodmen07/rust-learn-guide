fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let a = "alpha";
    let b = "alphabet soup";
    println!("Longest: {}", longest(a, b));
}
