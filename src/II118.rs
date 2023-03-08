struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut union_find = UnionFind::new(edges.len());
        for edge in edges {
            let (x, y) = (edge[0] as usize - 1, edge[1] as usize - 1);
            if union_find.merge(x, y) {
                return edge;
            }
        }
        vec![]
    }
}
struct UnionFind {
    fa: Vec<usize>,
    cnt: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            fa: (0..n).collect(),
            cnt: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.fa[x] == x {
            x
        } else {
            self.fa[x] = self.find(self.fa[x]);
            self.fa[x]
        }
    }
    fn merge(&mut self, x: usize, y: usize) -> bool {
        let (rootx, rooty) = (self.find(x), self.find(y));
        if rootx == rooty {
            true
        } else {
            self.fa[rootx] = rooty;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_II118() {
        assert_eq!(
            vec![2, 3],
            dbg!(Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 3]
            ]))
        );
    }
}
