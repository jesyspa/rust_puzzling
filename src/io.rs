use std::fmt::Debug;
use std::str::FromStr;

pub fn readline(s: &mut String) {
    s.clear();
    std::io::stdin().read_line(s).unwrap();
    while s.ends_with(&[' ', '\n', '\r', '\t'][..]) {
        s.pop();
    }
}

pub fn read<F: FromStr>(s: &mut String) -> F
where
    <F as FromStr>::Err: Debug,
{
    readline(s);
    s.parse().unwrap()
}

pub fn reads<F: FromStr>(s: &mut String) -> impl '_ + Iterator<Item = F>
where
    <F as FromStr>::Err: Debug,
{
    readline(s);
    s.split_whitespace().map(|r| r.parse::<F>().unwrap())
}

pub fn readpair<F: FromStr>(s: &mut String) -> (F, F)
where
    <F as FromStr>::Err: Debug,
{
    let mut it = reads::<F>(s);
    (it.next().unwrap(), it.next().unwrap())
}

pub fn readtriple<F: FromStr>(s: &mut String) -> (F, F, F)
where
    <F as FromStr>::Err: Debug,
{
    let mut it = reads::<F>(s);
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}

pub fn readvec<F: FromStr>(s: &mut String) -> Vec<F>
where
    <F as FromStr>::Err: Debug,
{
    reads::<F>(s).collect()
}
