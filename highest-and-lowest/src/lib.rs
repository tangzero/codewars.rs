fn high_and_low(numbers: &str) -> String {
    let numbers: Vec<i32> = numbers.split_whitespace().map(|n| n.parse().unwrap()).collect();
    format!("{} {}", numbers.iter().max().unwrap(), numbers.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }
}
