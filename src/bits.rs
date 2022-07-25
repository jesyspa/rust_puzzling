pub fn lsb_only(i: usize) -> usize {
  i & i.overflowing_neg().0
}

pub fn is_pow2(i: usize) -> bool {
  i & (i - 1) == 0
}

pub fn nearest_pow2_up(mut i: usize) -> usize {
  while !is_pow2(i) {
    i += lsb_only(i);
  }
  i
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lsb_only_test() {
    for i in 1..5 {
      let n = 1 << i;
      assert_eq!(lsb_only(n), n);
      assert_eq!(lsb_only(n+1), 1);
    }
  }

  #[test]
  fn is_pow2_test() {
    for i in 1..5 {
      let n = 1 << i;
      assert!(is_pow2(n));
      assert!(!is_pow2(n+1));
    }
  }

  #[test]
  fn nearest_pow2_up_test() {
    for i in 1..5 {
      let n = 1 << i;
      assert_eq!(nearest_pow2_up(n), n);
      assert_eq!(nearest_pow2_up(n+1), n << 1);
    }
    assert_eq!(nearest_pow2_up(7), 8);
    for i in 9..16 {
      assert_eq!(nearest_pow2_up(i), 16);
    }
  }
}