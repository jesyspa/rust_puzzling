use rust_puzzling::io::read;

fn main() {
  let mut s = String::new();
  let mut n = read::<usize>(&mut s);
  let mut v = Vec::new();
  while n != 0 {
    let q = (n - 1) / 2;
    let a = n - 2 * q;
    v.push(['4', '7'][a - 1]);
    n = q;
  }
  println!("{}", v.into_iter().rev().collect::<String>());
}