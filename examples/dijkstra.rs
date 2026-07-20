use procon::template::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v, w) in edges {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let mut dist = vec![!0; n];
    let mut heap = BinaryHeap::new();
    dist[0] = 0;
    heap.push((Reverse(0), 0));

    while let Some((Reverse(d), v)) = heap.pop() {
        if d != dist[v] {
            continue;
        }
        for &(to, w) in &graph[v] {
            let nd = d + w;
            if dist[to].chmin(nd) {
                heap.push((Reverse(nd), to));
            }
        }
    }

    for d in dist {
        if d == !0 {
            println!("-1");
        } else {
            println!("{d}");
        }
    }
}

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bl
