use rust_puzzling::io::*;

struct Table(Vec<f64>);

impl Table {
  fn new(rows: usize) -> Self {
    let mut t = Table(Vec::with_capacity(rows * (rows+1) / 2));
    for n in 1..=rows {
      let mut last = 0.;
      for k in 0..=n {
        last += (0..=k).map(|i| t.compute_probability(n, k, i)).sum::<f64>() / ((n + 1) as f64);
        t.0.push(last);
      }
    }
    t
  }

  fn get_cumul(&self, n: usize, k: usize) -> f64 {
    if k == 0 { return 0.; }
    if k > n { return 1.; }
    self.0[n * (n - 1) / 2 + k - 1]
  }

  fn compute_probability(&self, n: usize, k: usize, i: usize) -> f64 {
    let j = n - i;
    self.get_cumul(i, k+1) * self.get_cumul(j, k) + self.get_cumul(j, k+1) * self.get_cumul(i, k) - 2. * self.get_cumul(i, k) * self.get_cumul(j, k)
  }

  fn expectation(&self, n: usize) -> f64 {
    let row_start = n * (n - 1) / 2;
    (0..n).map(|k| (self.0[row_start + k + 1] - self.0[row_start + k]) * (k as f64)).sum()
  }
}

fn main() {
  let mut s = String::new();
  for _ in 0..read(&mut s).unwrap() {
    let n = read(&mut s).unwrap();
    println!("{}", Table::new(n).expectation(n));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_float_eq(x: f64, y: f64) {
    assert!((x - y).abs() < 0.0000001, "got: {}, expected: {}", x, y);
  }

  #[test]
  fn test_construction() {
    let mut t = Table::new(4);
    assert_float_eq(t.0[0], H)
  }
}