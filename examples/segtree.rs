use procon::template::*;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut seg = Segtree::<Max<usize>>::new(n);
    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! {
                    pos: Usize1,
                    x: usize,
                }
                seg.set(pos, x);
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                println!("{}", seg.prod(l..r));
            }
            _ => unreachable!(),
        }
    }
}

// new(n)           creates a tree of length n filled with identity elements
// from(v)          builds a tree from the given vector
// set(p, x)        sets the value at position p to x
// get(p)           returns the value at position p
// prod(l..r)       returns the product over the range [l, r)
// all_prod()       returns the product over the entire range
// max_right(l, f)  finds the maximum r such that f(prod(l..r)) is true
// min_left(r, f)   finds the minimum l such that f(prod(l..r)) is true

// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bf