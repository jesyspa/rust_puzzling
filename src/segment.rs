use std::ops::Add;
use std::iter::FromIterator;

use crate::bits::{clear_msb, make_mask_up_to, msb};

fn left_child(i: usize) -> usize {
    2 * clear_msb(i)
}

fn right_child(i: usize) -> usize {
    2 * clear_msb(i) + 1
}

fn leading_ones_run(i: usize) -> u32 {
    let k1 = msb(i);
    let k2 = msb(!(i | !make_mask_up_to(k1)));
    k1 - k2
}

fn split_on_leading_ones(i: usize) -> (u32, usize) {
    let k = leading_ones_run(i);
    (k, i & make_mask_up_to(msb(i) - k))
}

fn lower_bound_impl(node: usize, num_leaves: usize) -> usize {
    if node < num_leaves {
        node
    } else {
        let (k, i) = split_on_leading_ones(node);
        i << k
    }
}

fn upper_bound_impl(node: usize, num_leaves: usize) -> usize {
    if node < num_leaves {
        node + 1
    } else {
        let (k, i) = split_on_leading_ones(node);
        (i + 1) << k
    }
}

pub struct SegmentTree<T>(pub Vec<T>);

impl<T> FromIterator<T> for SegmentTree<T>
where
    T: Add<Output = T> + Copy + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::build_tree(iter.into_iter().collect())
    }
}

impl<T> SegmentTree<T>
where
    T: Add<Output = T> + Copy + Default,
{
    pub fn new(data: &[T]) -> Self {
        let mut v = Vec::with_capacity(data.len().next_power_of_two() * 2);
        for &e in data {
            v.push(e);
        }
        Self::build_tree(v)
    }

    fn recompute(v: &[T], node: usize) -> T {
        v[left_child(node)] + v[right_child(node)]
    }

    fn build_tree(mut v: Vec<T>) -> Self {
        let real_elems = v.len();
        while v.len() < real_elems.next_power_of_two() {
            v.push(T::default());
        }

        let target_size = 2 * v.len() - 1;
        while v.len() < target_size {
            let node = Self::recompute(&v, v.len());
            v.push(node);
        }
        Self(v)
    }

    fn is_leaf(&self, i: usize) -> bool {
        2 * i < self.0.len()
    }

    fn root(&self) -> usize {
        self.0.len() - 1
    }

    pub fn num_leaves(&self) -> usize {
        (self.0.len() + 1) / 2
    }

    fn parent(&self, n: usize) -> usize {
        (n / 2) | self.num_leaves()
    }

    fn lower_bound(&self, node: usize) -> usize {
        lower_bound_impl(node, self.num_leaves())
    }

    fn upper_bound(&self, node: usize) -> usize {
        upper_bound_impl(node, self.num_leaves())
    }

    fn query_range_impl(&self, node: usize, i: usize, j: usize) -> T {
        if self.is_leaf(node) {
            return if i <= node && node < j {
                self.0[node]
            } else {
                T::default()
            };
        }
        let lb = self.lower_bound(node);
        let ub = self.upper_bound(node);
        if i <= lb && ub <= j {
            return self.0[node];
        }
        if j < lb || ub < i {
            return T::default();
        }
        self.query_range_impl(left_child(node), i, j)
            + self.query_range_impl(right_child(node), i, j)
    }

    /* All ranges are closed-open. */
    pub fn query_range(&self, i: usize, j: usize) -> T {
        self.query_range_impl(self.root(), i, j)
    }

    pub fn update(&mut self, mut i: usize, t: T) {
        self.0[i] = t;
        while i != self.root() {
            i = self.parent(i);
            self.0[i] = Self::recompute(&self.0, i);
        }
    }

    pub fn get_leaf(&self, i: usize) -> T {
        self.0[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leading_ones_run_test() {
        assert_eq!(leading_ones_run(1), 1);
        assert_eq!(leading_ones_run(2), 1);
        assert_eq!(leading_ones_run(3), 2);
        assert_eq!(leading_ones_run(5), 1);
        assert_eq!(leading_ones_run(8), 1);
        assert_eq!(leading_ones_run(12), 2);
        assert_eq!(leading_ones_run(13), 2);
        assert_eq!(leading_ones_run(15), 4);
    }

    #[test]
    fn bounds_impl_test() {
        let num_leaves = 8;
        for i in 0..num_leaves {
            assert_eq!(lower_bound_impl(i, num_leaves), i);
            assert_eq!(upper_bound_impl(i, num_leaves), i + 1);
        }
        assert_eq!(lower_bound_impl(8, num_leaves), 0);
        assert_eq!(upper_bound_impl(8, num_leaves), 2);
        assert_eq!(lower_bound_impl(10, num_leaves), 4);
        assert_eq!(upper_bound_impl(10, num_leaves), 6);
        assert_eq!(lower_bound_impl(14, num_leaves), 0);
        assert_eq!(upper_bound_impl(14, num_leaves), 8);
    }

    #[test]
    fn new_test() {
        let seg = SegmentTree::new(&[0, 1, 2, 3]);
        assert_eq!(&seg.0, &[0, 1, 2, 3, 1, 5, 6]);
    }

    #[test]
    fn not_pow2_new_test() {
        let seg = SegmentTree::new(&[0, 1, 2, 3, 4, 5]);
        assert_eq!(&seg.0, &[0, 1, 2, 3, 4, 5, 0, 0, 1, 5, 9, 0, 6, 9, 15]);
    }

    #[test]
    fn query_range_test() {
        let seg = SegmentTree::new(&[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(seg.query_range(0, 8), 28);
        assert_eq!(seg.query_range(0, 4), 6);
        assert_eq!(seg.query_range(3, 7), 18);
    }

    #[test]
    fn update_test() {
        let mut seg = SegmentTree::new(&[0, 1, 2, 3]);
        seg.update(2, 5);
        assert_eq!(&seg.0, &[0, 1, 5, 3, 1, 8, 9]);
        seg.update(0, 3);
        assert_eq!(&seg.0, &[3, 1, 5, 3, 4, 8, 12]);
    }
}
