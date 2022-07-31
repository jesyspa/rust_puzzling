pub fn lsb_only(i: usize) -> usize {
    i & i.overflowing_neg().0
}

pub fn msb(i: usize) -> u32 {
    usize::BITS - i.leading_zeros()
}

pub fn clear_msb(i: usize) -> usize {
    let keep_count = msb(i) - 1;
    let mask = (1 << keep_count) - 1;
    i & mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lsb_only_test() {
        for i in 1..5 {
            let n = 1 << i;
            assert_eq!(lsb_only(n), n);
            assert_eq!(lsb_only(n + 1), 1);
        }
    }

    #[test]
    fn msb_test() {
        for i in 1..5 {
            let n = 1 << i;
            assert_eq!(msb(n), i + 1);
            assert_eq!(msb(n + 1), i + 1);
        }
    }

    #[test]
    fn clear_msb_test() {
        for i in 1..5 {
            let n = 1 << i;
            assert_eq!(clear_msb(n), 0);
            assert_eq!(clear_msb(n + 1), 1);
        }
        assert_eq!(clear_msb(7), 3);
        assert_eq!(clear_msb(13), 5);
    }
}
