pub fn lsb_only(i: usize) -> usize {
    i & i.overflowing_neg().0
}

pub fn make_mask_up_to(i: u32) -> usize {
    (1 << i) - 1
}

pub fn msb(i: usize) -> u32 {
    usize::BITS - i.leading_zeros()
}

pub fn clear_msb(i: usize) -> usize {
    i & make_mask_up_to(msb(i) - 1)
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
    fn make_mask_up_to_test() {
        assert_eq!(make_mask_up_to(0), 0);
        assert_eq!(make_mask_up_to(1), 1);
        assert_eq!(make_mask_up_to(2), 3);
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
