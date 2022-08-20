use std::fmt::Debug;
use std::str::FromStr;

pub fn readline(s: &mut String) -> bool {
    s.clear();
    let b = std::io::stdin().read_line(s).is_ok();
    while s.ends_with(&[' ', '\n', '\r', '\t'][..]) {
        s.pop();
    }
    b
}

pub fn read<F: FromStr>(s: &mut String) -> Option<F>
where
    <F as FromStr>::Err: Debug,
{
    if readline(s) {
        s.parse().ok()
    } else {
        None
    }
}

pub fn reads<F: FromStr>(s: &mut String) -> Option<impl '_ + Iterator<Item = F>>
where
    <F as FromStr>::Err: Debug,
{
    if readline(s) {
        Some(s.split_whitespace().map(|r| r.parse::<F>().unwrap()))
    } else {
        None
    }
}

pub fn readpair<F: FromStr>(s: &mut String) -> Option<(F, F)>
where
    <F as FromStr>::Err: Debug,
{
    let mut it = reads::<F>(s)?;
    Some((it.next()?, it.next()?))
}

pub fn readtriple<F: FromStr>(s: &mut String) -> Option<(F, F, F)>
where
    <F as FromStr>::Err: Debug,
{
    let mut it = reads::<F>(s)?;
    Some((it.next()?, it.next()?, it.next()?))
}

pub fn readvec<F: FromStr>(s: &mut String) -> Option<Vec<F>>
where
    <F as FromStr>::Err: Debug,
{
    Some(reads::<F>(s)?.collect())
}
