use rust_puzzling::segment::*;
use rust_puzzling::io::*;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct MinInt(i64);

impl Default for MinInt {
    fn default() -> Self {
        MinInt(i64::max_value())
    }
}
impl Add for MinInt {
    type Output = MinInt;

    fn add(self, rhs: Self) -> Self::Output {
        MinInt(self.0.min(rhs.0))
    }
}

fn one_case(s: &mut String) -> i64 {
    let n = read::<usize>(s).unwrap();
    let st =
        SegmentTree::from_iter([MinInt(0)].into_iter().chain(
            reads::<i64>(s).unwrap().scan(0, |s, v| {
                *s += v;
                Some(MinInt(*s))
            })));
    let mut total = 0;
    for i in 0..n {
        for j in i+1..n+1 {
            if st.query_range(i+1, j+1).0 >= st.get_leaf(i).0 {
                total += st.get_leaf(j).0 - st.get_leaf(i).0;
            }
        }
    }
    total
}

fn main() {
    let mut s = String::new();
    for t in 0..read::<usize>(&mut s).unwrap() {
        println!("Case #{}: {}", t+1, one_case(&mut s));
    }
}
