use std::cmp::Ordering;

use rust_puzzling::io::*;

fn main() {
    let mut s = String::new();
    for i in 0..read::<usize>(&mut s).unwrap() {
        let n = read::<usize>(&mut s).unwrap();
        let mut v1: Vec<(String, usize, usize)> = Vec::with_capacity(n);

        for _ in 0..n {
            let (c, d, u) = readtriple::<String>(&mut s).unwrap();
            let d: usize = d.parse().unwrap();
            let u: usize = u.parse().unwrap();
            v1.push((c, d, u));
        }

        let mut v2 = v1.clone();
        v1.sort_by(|(_, d1, u1), (_, d2, u2)| {
            let ord = d1.cmp(d2);
            if ord == Ordering::Equal {
                u1.cmp(u2)
            } else {
                ord
            }
        });
        v2.sort_by(|(c1, _, u1), (c2, _, u2)| {
            let ord = c1.cmp(c2);
            if ord == Ordering::Equal {
                u1.cmp(u2)
            } else {
                ord
            }
        });

        println!(
            "Case #{}: {}",
            i + 1,
            v1.iter()
                .map(|(_, _, u)| u)
                .zip(v2.iter().map(|(_, _, u)| u))
                .filter(|(u1, u2)| u1 == u2)
                .count()
        );
    }
}
