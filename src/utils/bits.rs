pub fn lsb_only(i: usize) -> usize {
  i & i.overflowing_neg().0
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
}