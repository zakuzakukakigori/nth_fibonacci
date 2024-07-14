pub fn calculate(n: usize) -> u64 {
    if n == 0 { return 0 }
    
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_yields_fibonacci_sequence() {
        let sequence: Vec<u64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

        let result: Vec<_> = sequence
            .iter()
            .enumerate()
            .map(|(i, _)| calculate(i))
            .collect();

        assert_eq!(sequence, result);
    }
}
