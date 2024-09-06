pub fn inplace_radix(arr: &mut [u16]) {
  for bit in (0..15).rev() {
    let mut swap_with = 0;
    let mut block = 0;
    for i in 0..arr.len() {
      let v = arr[i];
      let examine = v >> bit;
      let v_block = examine >> 1;
      if v_block != block {
        dbg!(v, bit, i);
        block = v_block;
        swap_with = i;
      }
      if examine & 1 == 0 {
        arr[i] = arr[swap_with];
        arr[swap_with] = v;
        swap_with += 1;
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::inplace_radix;

  fn check(test_input: &[u16]) {
    let mut test = test_input.to_vec();
    let mut baseline = test_input.to_vec();
    inplace_radix(&mut test);
    baseline.sort();
    assert_eq!(test, baseline);
  }

  #[test]
  fn sort_1_bits_id() {
    check(&[0, 0, 1, 1]);
  }

  #[test]
  fn sort_1_bits_rev() {
    check(&[1, 1, 0, 0]);
  }

  #[test]
  fn sort_2_bits() {
    check(&[0, 3, 1, 2, 0]);
  }

  #[test]
  fn sort_trivial() {
    check(&[0, 3]);
  }
}