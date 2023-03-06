use std::{io::{self, BufRead}, cmp::min};
fn main() {
    let mut lines: Vec<Vec<i64>> = io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .trim()
                .split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    let nm = lines.remove(0);
    let n = nm[0] as usize;
    let m = nm[1] as usize;

    let mut nodes: Vec<(i64)> = vec![0; n];
    let mut graph: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

    let mut ogw: i64 = 0; // original graph weight

    for line in lines {
        let i = line[0] as usize;
        let j = line[1] as usize;

        graph[i][j] -= 1;
        graph[j][i] -= 1;

        adj[i].push(j);
        adj[j].push(i);
    }

    let time = 0;
    let mut visited = vec![false; n + 1];
    let mut tin = vec![-1; n + 1];
    let mut low = vec![-1; n + 1];
    for i in 0..n {
        if !visited[i] {
            visited[i] = true;
            time += 1;
            tin = time;
            low = time;
            for to in adj[i] {
                if to == p {
                    continue;
                }
                if visited[to] {
                    low[v] = min(low[v], tin[to]);
                } else {
                    dfs(to, v);
                    low[v] = min(low[v], low[to]);
                    if low[to > tin[v]]
                }
            }
        }
    }
}

// fn bfs_modded(graph: &Vec<Vec<usize>>, nodes: &Vec<i64>) {
//     let Q:Vec<usize> = Vec::new();

//     while Q.len() > 0 {
//         let cur = Q.pop().unwrap();
//         for v in graph[cur] {
//             if (nodes[v-1])
//         }
//     }

//     while Q.len() > 0 {
//         let current = Q.pop():unw;

//     }
// }
