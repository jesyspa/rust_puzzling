use rust_puzzling::io::{readpair, readtriple};
use rust_puzzling::union_find::*;

struct Mst {
    weight: i32,
    edges: Vec<(usize, usize)>,
}

fn process(s: &mut String, n: usize, m: usize) -> Option<Mst> {
    let mut v: Vec<(i32, usize, usize)> = (0..m)
        .map(|_| {
            let (x, y, w) = readtriple::<i32>(s).unwrap();
            (w, x as usize, y as usize)
        })
        .collect();
    v.sort();

    let mut uf = UnionFind::new(n);
    let mut mst = Mst {
        weight: 0,
        edges: Vec::with_capacity(n - 1),
    };

    for (w, x, y) in v.into_iter() {
        if uf.rep(x) != uf.rep(y) {
            mst.weight += w;
            mst.edges.push((x.min(y), x.max(y)));
            uf.union(x, y);
        }
    }

    if mst.edges.len() < n - 1 {
        None
    } else {
        mst.edges.sort();
        Some(mst)
    }
}

fn main() {
    let mut s = String::new();
    loop {
        let (n, m) = readpair::<usize>(&mut s).unwrap();
        if n == 0 && m == 0 {
            return;
        }
        if let Some(mst) = process(&mut s, n, m) {
            println!("{}", mst.weight);
            for (a, b) in mst.edges.into_iter() {
                println!("{} {}", a, b);
            }
        } else {
            println!("Impossible");
        }
    }
}
