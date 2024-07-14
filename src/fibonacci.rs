pub fn calculate(n: usize) -> u64 {
    n.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_yields_0_given_0() {
        assert_eq!(calculate(0), 0)
    }

    #[test]
    fn it_yields_1_given_1() {
        assert_eq!(calculate(1), 1)
    }
}
