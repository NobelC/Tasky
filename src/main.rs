fn greeting() -> String {
    "Hello from tasky".to_string()
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saluda() {
        assert_eq!(greeting(), "Hello from tasky");
    }
}
