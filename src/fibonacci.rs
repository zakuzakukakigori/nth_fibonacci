pub fn calculate(_n: usize) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_yields_0_given_0() {
        assert_eq!(calculate(0), 0)
    }
}
