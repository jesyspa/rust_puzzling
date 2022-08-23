#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Node {
    parent: usize,
    size: usize,
}

impl Node {
    pub fn new(n: usize) -> Self {
        Node { parent: n, size: 1 }
    }
}

pub struct UnionFind(Vec<Node>);

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind(vec![Node::new(n); n])
    }

    fn parent(&self, i: usize) -> Option<usize> {
        let k = self.0[i].parent;
        if k == self.0.len() {
            None
        } else {
            Some(k)
        }
    }

    pub fn rep(&mut self, i: usize) -> usize {
        if let Some(j) = self.parent(i) {
            let k = self.rep(j);
            if j != k {
                self.0[j].size -= self.0[i].size;
                self.0[i].parent = k;
            }
            k
        } else {
            i
        }
    }

    pub fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.rep(i);
        j = self.rep(j);
        if i == j {
            return;
        }
        // i is parent, j is child.
        if self.0[i].size < self.0[j].size {
            std::mem::swap(&mut i, &mut j);
        }
        self.0[j].parent = i;
        self.0[i].size += self.0[j].size;
    }

    pub fn num_partitions(&mut self) -> usize {
        let mut s = std::collections::HashSet::new();
        for i in 0..self.0.len() {
            s.insert(self.rep(i));
        }
        s.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_without_internals() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(2, 3);
        assert_eq!(uf.rep(0), 0);
        assert_eq!(uf.rep(1), 0);
        assert_eq!(uf.rep(2), 2);
        assert_eq!(uf.rep(3), 2);
    }

    #[test]
    fn test_union_deep() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(0, 2);
        assert_eq!(uf.rep(0), 0);
        assert_eq!(uf.rep(1), 0);
        assert_eq!(uf.rep(2), 0);
        assert_eq!(uf.rep(3), 0);
    }

    #[test]
    fn test_size_accounted() {
        let mut uf = UnionFind::new(4);
        uf.union(1, 2);
        uf.union(0, 1);
        assert_eq!(uf.rep(0), 1);
        assert_eq!(uf.rep(1), 1);
        assert_eq!(uf.rep(2), 1);
        assert_eq!(uf.rep(3), 3);
    }

    #[test]
    fn test_update_occurs() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);
        assert_eq!(
            uf.0,
            &[
                Node { parent: 6, size: 2 },
                Node { parent: 0, size: 1 },
                Node { parent: 6, size: 2 },
                Node { parent: 2, size: 1 },
                Node { parent: 6, size: 1 },
                Node { parent: 6, size: 1 }
            ]
        );
        uf.union(1, 2);
        uf.union(4, 0);
        assert_eq!(
            uf.0,
            &[
                Node { parent: 6, size: 5 },
                Node { parent: 0, size: 1 },
                Node { parent: 0, size: 2 },
                Node { parent: 2, size: 1 },
                Node { parent: 0, size: 1 },
                Node { parent: 6, size: 1 }
            ]
        );
        for i in 0..6 {
            uf.rep(i);
        }
        assert_eq!(
            uf.0,
            &[
                Node { parent: 6, size: 5 },
                Node { parent: 0, size: 1 },
                Node { parent: 0, size: 1 },
                Node { parent: 0, size: 1 },
                Node { parent: 0, size: 1 },
                Node { parent: 6, size: 1 }
            ]
        );
    }

    #[test]
    fn test_num_partitions() {
        let mut uf = UnionFind::new(6);
        assert_eq!(uf.num_partitions(), 6);
        uf.union(0, 1);
        uf.union(1, 2);
        assert_eq!(uf.num_partitions(), 4);
        uf.union(2, 3);
        uf.union(4, 0);
        assert_eq!(uf.num_partitions(), 2);
    }
}
