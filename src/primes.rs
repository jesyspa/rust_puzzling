use std::ops::{Index, IndexMut};

use crate::num::{three, two, Num};

pub struct PrimeTable<T>(Vec<T>);
pub struct PrimeIter<'a, T> {
    table: &'a mut PrimeTable<T>,
    ix: usize,
}

impl<T> Index<usize> for PrimeTable<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for PrimeTable<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Num> Default for PrimeTable<T> {
    fn default() -> Self {
        PrimeTable(vec![two(), three()])
    }
}

impl<T: Num> PrimeTable<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn last(&self) -> T {
        self.0[self.0.len() - 1]
    }

    fn gen_next(&mut self) -> T {
        let mut n = self.last();
        loop {
            n += two();
            for &i in self.0.iter() {
                if i * i > n {
                    self.0.push(n);
                    return n;
                }
                if n % i == T::ZERO {
                    break;
                }
            }
        }
    }
}

impl<'a, T: Num> Iterator for PrimeIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ix += 1;
        if self.ix <= self.table.len() {
            Some(self.table[self.ix - 1])
        } else {
            Some(self.table.gen_next())
        }
    }
}

impl<'a, T: Num> IntoIterator for &'a mut PrimeTable<T> {
    type Item = T;
    type IntoIter = PrimeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        PrimeIter { table: self, ix: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primes_once() {
        let mut table = PrimeTable::<i32>::new();
        let primes: Vec<_> = table.into_iter().take(5).collect();
        assert_eq!(&primes, &[2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_primes_twice_longer() {
        let mut table = PrimeTable::<i32>::new();
        let primes: Vec<_> = table.into_iter().take(3).collect();
        assert_eq!(&primes, &[2, 3, 5]);
        let primes: Vec<_> = table.into_iter().take(5).collect();
        assert_eq!(&primes, &[2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_primes_twice_shorter() {
        let mut table = PrimeTable::<i32>::new();
        let primes: Vec<_> = table.into_iter().take(5).collect();
        assert_eq!(&primes, &[2, 3, 5, 7, 11]);
        let primes: Vec<_> = table.into_iter().take(3).collect();
        assert_eq!(&primes, &[2, 3, 5]);
    }
}
