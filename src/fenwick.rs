use crate::bits::lsb_only;
use std::ops::AddAssign;

fn get_parent(i: usize) -> usize {
    i - lsb_only(i)
}

fn get_next(i: usize) -> usize {
    i + lsb_only(i)
}

pub struct Fenwick<T>(Vec<T>);

impl<T> FromIterator<T> for Fenwick<T>
where
    T: AddAssign<T> + Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v: Vec<T> = iter.into_iter().collect();
        for i in 1..=v.len() {
            let k = get_next(i);
            if k <= v.len() {
                let x = v[i - 1];
                v[k - 1] += x;
            }
        }
        Self(v)
    }
}

impl<T> Fenwick<T>
where
    T: AddAssign<T> + Copy,
{
    pub fn new(data: &[T]) -> Self {
        Self::from_iter(data.iter().cloned())
    }

    pub fn query(&self, mut i: usize) -> T {
        let mut result = self.0[i - 1];
        i = get_parent(i);
        while i > 0 {
            result += self.0[i - 1];
            i = get_parent(i);
        }
        result
    }

    pub fn update(&mut self, mut i: usize, t: T) {
        while i <= self.0.len() {
            self.0[i - 1] += t;
            i = get_next(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_parent_test() {
        for i in 1..5 {
            let n = 1 << i;
            assert_eq!(get_parent(n), 0);
            assert_eq!(get_parent(n + 1), n);
        }

        assert_eq!(get_parent(7), 6);
        assert_eq!(get_parent(11), 10);
    }

    #[test]
    fn get_next_test() {
        for i in 1..5 {
            let n = 1 << i;
            assert_eq!(get_next(n), n << 1);
        }

        assert_eq!(get_next(3), 4);
        assert_eq!(get_next(6), 8);
        assert_eq!(get_next(10), 12);
    }

    #[test]
    fn from_iter_test() {
        let fen = Fenwick::from_iter([1, 2, 3, 4, 5, 6, 7].into_iter());
        assert_eq!(&fen.0, &[1, 3, 3, 10, 5, 11, 7]);
    }

    #[test]
    fn new_test() {
        let fen = Fenwick::new(&[1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(&fen.0, &[1, 2, 1, 4, 1, 2, 1]);
    }

    #[test]
    fn query_test() {
        let fen = Fenwick::new(&[1, 2, 3, 4, 5, 6, 7]);
        for i in 1..=fen.0.len() {
            assert_eq!(fen.query(i), i * (i + 1) / 2);
        }
    }

    #[test]
    fn update_test() {
        let mut fen = Fenwick::new(&[1, 1, 1, 1, 1, 1, 1]);
        fen.update(3, 2);
        assert_eq!(&fen.0, &[1, 2, 3, 6, 1, 2, 1]);
        fen.update(7, 10);
        assert_eq!(&fen.0, &[1, 2, 3, 6, 1, 2, 11]);
        fen.update(1, 100);
        assert_eq!(&fen.0, &[101, 102, 3, 106, 1, 2, 11]);
    }
}
