pub fn calculate(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for single_step in single_steps(n) {
        (a, b) = (
            a.wrapping_mul(b.wrapping_mul(2).wrapping_sub(a)),
            b.wrapping_pow(2).wrapping_add(a.wrapping_pow(2)),
        );

        if single_step {
            (a, b) = (b, a.wrapping_add(b));
        }
    }
    a
}

fn single_steps(n: usize) -> impl Iterator<Item = bool> {
    let left_bound = usize::BITS - n.leading_zeros();

    (0..left_bound).rev().map(move |i| {
        let mask = 1_usize << i;

        (n & mask) != 0
    })
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

    #[test]
    fn it_yields_big_wrapping_number() {
        assert_eq!(331_1503_4269_4199_0459, calculate(10_usize.pow(9)));
    }
}
