use rust_puzzling::io::*;

fn main() {
    let mut s = String::new();
    for t in 0..read::<usize>(&mut s).unwrap() {
      let (m, n, p) = readtriple::<usize>(&mut s).unwrap();
      let mut vo = vec![0; n];
      let mut vj = vec![0; n];
      for i in 0..m {
        for (j, v) in reads::<usize>(&mut s).unwrap().enumerate() {
          if i+1 == p {
            vj[j] = v;
          } else {
            vo[j] = vo[j].max(v);
          }
        }
      }
      let total = (0..n).map(|j| if vj[j] < vo[j] { vo[j] - vj[j] } else { 0 }).sum::<usize>();
      println!("Case #{}: {}", t+1, total);
    }
}