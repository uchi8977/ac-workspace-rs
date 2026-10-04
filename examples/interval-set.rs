use procon::template::*;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        queries: [(Usize1, usize); q],
    }

    let mut counts = a.iter().copied().fx_counts();
    let mut set = IntervalSet::new();
    for &x in counts.keys() {
        set.insert(x, x + 1);
    }

    for (i, x) in queries {
        let old = a[i];
        *counts.get_mut(&old).unwrap() -= 1;
        if counts[&old] == 0 {
            set.remove(old, old + 1);
        }

        *counts.entry(x).or_insert(0) += 1;
        if counts[&x] == 1 {
            set.insert(x, x + 1);
        }
        a[i] = x;

        println!("{}", set.mex(0));
    }
}

// new()                 creates an empty set of half-open intervals [l, r)
// insert(l, r)          adds [l, r), merges overlapping or adjacent intervals, and returns whether coverage changed
// remove(l, r)          removes [l, r), splits intervals if needed, and returns whether coverage changed
// get(x)                returns Some((l, r)) for the interval containing x, or None
// contains(x)           checks whether x is covered
// contains_range(l, r)  checks whether all of [l, r) is covered; true for l >= r
// mex(x)                returns the smallest uncovered value >= x; mex(0) gives the usual mex
// clear()               removes all intervals
// is_empty()            checks whether the set is empty
// len()                 returns the number of intervals, not the number of covered values
// iter()                iterates over (l, r) pairs in ascending order of l

// https://atcoder.jp/contests/abc330/tasks/abc330_e
