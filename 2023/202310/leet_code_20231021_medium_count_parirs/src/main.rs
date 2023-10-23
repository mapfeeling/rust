#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<usize>,
    count: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            count: vec![1; n],
        }
    }
    fn find(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            return idx;
        }
        self.parent[idx] = self.find(self.parent[idx]);
        self.parent[idx]
    }
    fn union(&mut self, idx0: usize, idx1: usize) {
        let p0 = self.find(idx0);
        let p1 = self.find(idx1);
        if p0 != p1 {
            if self.count[p0] > self.count[p1] {
                self.parent[p1] = p0;
                self.count[p0] += self.count[p1];
            } else {
                self.parent[p0] = p1;
                self.count[p1] += self.count[p0];
            }
        }
    }
}

struct Solution {}

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut uf = UnionFind::new(n as usize);
        for edge in edges {
            uf.union(edge[0] as usize, edge[1] as usize);
        }
        (0..n as usize)
            .map(|x| {
                let p = uf.find(x);
                n as i64 - uf.count[p]
            })
            .sum::<i64>()
            / 2
    }
}

fn main() {//{0, 2}, {0, 5}, {2, 4}, {1, 6}, {5, 4}}
    let s = Solution::count_pairs(7, vec![vec![0,2], vec![0,5], vec![2,4], vec![1,6], vec![5,4]]);
    println!("{:?}", s);
    println!("Hello, world!");
}
