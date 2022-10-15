use rust_puzzling::io::*;

fn main() {
    let mut s = String::new();
    for t in 0..read::<usize>(&mut s).unwrap() {
        let rh = {
            let (a, b) = readpair::<i64>(&mut s).unwrap();
            (a + b) * (a + b)
        };
        let f = |(x, y): (i64, i64)| {
            let v = x * x + y * y;
            if v <= rh {
                Some(v)
            } else {
                None
            }
        };

        let n = read::<usize>(&mut s).unwrap();
        let mut sr = (0..n)
            .filter_map(|_| f(readpair::<i64>(&mut s).unwrap()))
            .collect::<Vec<_>>();
        sr.sort();

        let m = read::<usize>(&mut s).unwrap();
        let mut sy = (0..m)
            .filter_map(|_| f(readpair::<i64>(&mut s).unwrap()))
            .collect::<Vec<_>>();
        sy.sort();

        let scr;
        let scy;
        if sr.is_empty() || sy.is_empty() {
            scr = sr.len();
            scy = sy.len();
        } else {
            scr = sr.iter().take_while(|&&v| v < sy[0]).count();
            scy = sy.iter().take_while(|&&v| v < sr[0]).count();
        }
        println!("Case #{}: {} {}", t + 1, scr, scy);
    }
}
