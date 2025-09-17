/*
    graph
    This problem requires you to implement a basic graph function
*/

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct Graph<T: Eq + Hash + Clone + Ord + fmt::Display> {
    adj: HashMap<T, HashSet<T>>,
}

impl<T: Eq + Hash + Clone + Ord + fmt::Display> Graph<T> {
    pub fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    pub fn add_node(&mut self, u: T) {
        self.adj.entry(u).or_insert_with(HashSet::new);
    }

    /// 无向边；若题目要求“有向图”，删除插入 v->u 的那一行即可
    pub fn add_edge(&mut self, u: T, v: T) {
        self.adj.entry(u.clone()).or_insert_with(HashSet::new).insert(v.clone());
        self.adj.entry(v).or_insert_with(HashSet::new).insert(u);
    }

    /// 返回排序后的邻居列表，确保断言次序可预测
    pub fn neighbors(&self, u: &T) -> Vec<T> {
        let mut out: Vec<T> = self
            .adj
            .get(u)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default();
        out.sort();
        out
    }

    /// 是否存在从 start 到 goal 的路径（BFS）
    pub fn has_path_bfs(&self, start: &T, goal: &T) -> bool {
        if start == goal {
            return true;
        }
        if !self.adj.contains_key(start) || !self.adj.contains_key(goal) {
            return false;
        }

        let mut seen: HashSet<T> = HashSet::new();
        let mut q: VecDeque<T> = VecDeque::new();

        seen.insert(start.clone());
        q.push_back(start.clone());

        while let Some(u) = q.pop_front() {
            if let Some(ns) = self.adj.get(&u) {
                for v in ns {
                    if seen.insert(v.clone()) {
                        if v == goal {
                            return true;
                        }
                        q.push_back(v.clone());
                    }
                }
            }
        }
        false
    }
}

impl<T: Eq + Hash + Clone + Ord + fmt::Display> fmt::Display for Graph<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 收集并排序所有键
        let mut keys: Vec<&T> = self.adj.keys().collect();
        keys.sort();

        // 按引用 + 下标遍历，避免 move，便于判断是否最后一项
        for (idx, k) in keys.iter().enumerate() {
            // k: &&T  →  *k: &T
            let mut ns: Vec<&T> = self.adj[*k].iter().collect();
            ns.sort();

            write!(f, "{}:", k)?;
            for (i, n) in ns.iter().enumerate() {
                if i == 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", n)?;
                if i + 1 != ns.len() {
                    write!(f, " ")?;
                }
            }

            // 不是最后一个键则换行
            if idx + 1 != keys.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
