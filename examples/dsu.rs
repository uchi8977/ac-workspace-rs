use procon::template::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(u8, Usize1, Usize1); q],
    }

    let mut dsu = Dsu::new(n);
    for (t, u, v) in queries {
        match t {
            1 => {
                dsu.merge(u, v);
            }
            2 => {
                println!("{}", if dsu.same(u, v) { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}

// new(n)       creates n separate sets
// merge(a, b)  merges the sets containing a and b and returns their new leader
// same(a, b)   checks whether a and b are in the same set
// leader(a)    returns the representative of a's set
// size(a)      returns the size of the set containing a
// groups()     returns all sets as groups

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bn
