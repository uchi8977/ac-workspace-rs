// https://github.com/uchi8977/ac-workspace-rs
use core::{convert::Infallible, marker::PhantomData};
use num_traits::{PrimInt, Unsigned, Zero};
use std::{
    hash::Hash,
    ops::{Add, BitOr, BitXor},
};

pub use ac_library::{
    ModInt998244353 as Mint9, ModInt1000000007 as Mint1,
    dsu::Dsu,
    segtree::{Additive, Max, Min, Multiplicative, Segtree},
    string::z_algorithm_arbitrary,
};
pub use itertools::Itertools;
pub use num_integer::Integer;
pub use proconio::{
    input, input_interactive,
    marker::{Chars, Isize1, Usize1},
};
pub use rand::{
    Rng, rng,
    seq::{IteratorRandom, SliceRandom},
};
pub use rustc_hash::{FxHashMap, FxHashSet};
pub use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque, btree_map},
    f64::consts::PI,
    mem::swap,
    process::exit,
    time::Instant,
};

pub const DIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];
pub const DIR8: [(usize, usize); 8] = [
    (!0, !0),
    (!0, 0),
    (!0, 1),
    (0, !0),
    (0, 1),
    (1, !0),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone)]
pub struct FxHashMultiSet<T: Eq + Hash> {
    map: FxHashMap<T, usize>,
    len: usize,
}

impl<T: Eq + Hash> FxHashMultiSet<T> {
    pub fn insert(&mut self, val: T) {
        self.len += 1;
        *self.map.entry(val).or_insert(0) += 1;
    }
    pub fn remove(&mut self, val: &T) -> bool {
        if let Some(cnt) = self.map.get_mut(val) {
            self.len -= 1;
            *cnt -= 1;
            if *cnt == 0 {
                self.map.remove(val);
            }
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.len = 0;
        self.map.clear();
    }
    pub fn count(&self, val: &T) -> usize {
        *self.map.get(val).unwrap_or(&0)
    }
    pub fn contains(&self, val: &T) -> bool {
        self.map.contains_key(val)
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(val, &cnt)| itertools::repeat_n(val, cnt))
    }
}

impl<T: Eq + Hash> Default for FxHashMultiSet<T> {
    fn default() -> Self {
        Self {
            map: FxHashMap::default(),
            len: 0,
        }
    }
}

impl<T: Eq + Hash> FromIterator<T> for FxHashMultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = FxHashMultiSet::default();
        for x in iter {
            set.insert(x);
        }
        set
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a FxHashMultiSet<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn Iterator<Item = &'a T> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

#[derive(Debug, Clone)]
pub struct BTreeMultiSet<T: Ord> {
    map: BTreeMap<T, usize>,
    len: usize,
}

impl<T: Ord> BTreeMultiSet<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
        }
    }
    pub fn insert(&mut self, val: T) {
        self.len += 1;
        *self.map.entry(val).or_insert(0) += 1;
    }
    pub fn remove(&mut self, val: &T) -> bool {
        if let Some(cnt) = self.map.get_mut(val) {
            self.len -= 1;
            *cnt -= 1;
            if *cnt == 0 {
                self.map.remove(val);
            }
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.len = 0;
        self.map.clear();
    }
    pub fn count(&self, val: &T) -> usize {
        *self.map.get(val).unwrap_or(&0)
    }
    pub fn contains(&self, val: &T) -> bool {
        self.map.contains_key(val)
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(val, &cnt)| itertools::repeat_n(val, cnt))
    }
    pub fn range<R>(&self, range: R) -> impl DoubleEndedIterator<Item = &T>
    where
        R: std::ops::RangeBounds<T>,
    {
        self.map
            .range(range)
            .flat_map(|(val, &cnt)| itertools::repeat_n(val, cnt))
    }
}

impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = BTreeMultiSet::new();
        for x in iter {
            set.insert(x);
        }
        set
    }
}

impl<'a, T: Ord> IntoIterator for &'a BTreeMultiSet<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn DoubleEndedIterator<Item = &'a T> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.iter())
    }
}

#[derive(Debug, Clone)]
pub struct IntervalSet<T: Ord + Copy> {
    map: BTreeMap<T, T>,
}

impl<T: Ord + Copy> IntervalSet<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
    pub fn insert(&mut self, mut l: T, mut r: T) -> bool {
        if l >= r {
            return false;
        }
        if let Some((&a, &b)) = self.map.range(..=l).next_back() {
            if r <= b {
                return false;
            }
            if l <= b {
                self.map.remove(&a);
                l = a;
            }
        }
        while let Some((&a, &b)) = self.map.range(l..=r).next() {
            self.map.remove(&a);
            r = r.max(b);
        }
        self.map.insert(l, r);
        true
    }
    pub fn remove(&mut self, l: T, r: T) -> bool {
        if l >= r {
            return false;
        }
        let mut changed = false;
        if let Some((a, b)) = self.get(l) {
            self.map.remove(&a);
            if a < l {
                self.map.insert(a, l);
            }
            if r < b {
                self.map.insert(r, b);
                return true;
            }
            changed = true;
        }
        while let Some((&a, &b)) = self.map.range(l..r).next() {
            self.map.remove(&a);
            changed = true;
            if r < b {
                self.map.insert(r, b);
                break;
            }
        }
        changed
    }
    pub fn get(&self, x: T) -> Option<(T, T)> {
        self.map
            .range(..=x)
            .next_back()
            .filter(|&(_, &r)| x < r)
            .map(|(&l, &r)| (l, r))
    }
    pub fn contains(&self, x: T) -> bool {
        self.get(x).is_some()
    }
    pub fn contains_range(&self, l: T, r: T) -> bool {
        l >= r || self.get(l).is_some_and(|(_, b)| r <= b)
    }
    pub fn mex(&self, x: T) -> T {
        self.get(x).map_or(x, |(_, r)| r)
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (T, T)> + ExactSizeIterator {
        self.map.iter().map(|(&l, &r)| (l, r))
    }
}

impl<T: Ord + Copy> Default for IntervalSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Copy> FromIterator<(T, T)> for IntervalSet<T> {
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        let mut set = Self::new();
        for (l, r) in iter {
            set.insert(l, r);
        }
        set
    }
}

pub struct Dst<M: ac_library::segtree::Monoid> {
    table: Vec<Vec<M::S>>,
}

impl<M: ac_library::segtree::Monoid> Dst<M> {
    pub fn new(n: usize) -> Self {
        vec![M::identity(); n].into()
    }
    pub fn get(&self, p: usize) -> M::S {
        self.table[0][p].clone()
    }
    pub fn get_slice(&self) -> &[M::S] {
        &self.table[0]
    }
    pub fn prod<R>(&self, range: R) -> M::S
    where
        R: std::ops::RangeBounds<usize>,
    {
        use std::ops::Bound;

        let l = match range.start_bound() {
            Bound::Included(&l) => l,
            Bound::Excluded(&l) => l.checked_add(1).expect("range bound overflow"),
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Included(&r) => r.checked_add(1).expect("range bound overflow"),
            Bound::Excluded(&r) => r,
            Bound::Unbounded => self.len(),
        };
        assert!(l <= r && r <= self.len());
        if l == r {
            return M::identity();
        }
        let r = r - 1;
        if l == r {
            return self.get(l);
        }
        let k = (usize::BITS - 1 - (l ^ r).leading_zeros()) as usize;
        M::binary_operation(&self.table[k][l], &self.table[k][r])
    }
    pub fn all_prod(&self) -> M::S {
        self.prod(..)
    }
    pub fn max_right<F>(&self, l: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(l <= self.len());
        assert!(f(&M::identity()));
        let (mut lo, mut hi) = (l, self.len());
        while lo < hi {
            let mid = lo + (hi - lo) / 2 + 1;
            if f(&self.prod(l..mid)) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }
    pub fn min_left<F>(&self, r: usize, f: F) -> usize
    where
        F: Fn(&M::S) -> bool,
    {
        assert!(r <= self.len());
        assert!(f(&M::identity()));
        let (mut lo, mut hi) = (0, r);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if f(&self.prod(mid..r)) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        self.table[0].len()
    }
}

impl<M: ac_library::segtree::Monoid> Default for Dst<M> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<M: ac_library::segtree::Monoid> From<Vec<M::S>> for Dst<M> {
    fn from(v: Vec<M::S>) -> Self {
        let levels = (usize::BITS - v.len().saturating_sub(1).leading_zeros()) as usize;
        let mut table = vec![v];
        for k in 1..levels {
            let half = 1usize << k;
            let mut row = table[0].clone();
            for block in row.chunks_mut(half * 2) {
                let mid = half.min(block.len());
                for i in (0..mid - 1).rev() {
                    block[i] = M::binary_operation(&block[i], &block[i + 1]);
                }
                for i in mid + 1..block.len() {
                    block[i] = M::binary_operation(&block[i - 1], &block[i]);
                }
            }
            table.push(row);
        }
        Self { table }
    }
}

impl<M: ac_library::segtree::Monoid> FromIterator<M::S> for Dst<M> {
    fn from_iter<I: IntoIterator<Item = M::S>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<M: ac_library::segtree::Monoid> Clone for Dst<M> {
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
        }
    }
}

pub struct BitOrMonoid<S>(Infallible, PhantomData<fn() -> S>);
impl<S> ac_library::segtree::Monoid for BitOrMonoid<S>
where
    S: Copy + BitOr<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a | *b
    }
}

pub struct BitAndMonoid<S>(Infallible, PhantomData<fn() -> S>);
impl<S> ac_library::segtree::Monoid for BitAndMonoid<S>
where
    S: PrimInt,
{
    type S = S;
    fn identity() -> Self::S {
        !S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a & *b
    }
}

pub struct BitXorMonoid<S>(Infallible, PhantomData<fn() -> S>);
impl<S> ac_library::segtree::Monoid for BitXorMonoid<S>
where
    S: Copy + BitXor<Output = S> + Zero,
{
    type S = S;
    fn identity() -> Self::S {
        S::zero()
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        *a ^ *b
    }
}

pub trait ChOrd: PartialOrd + Copy {
    fn chmax(&mut self, val: Self) -> bool {
        if val > *self {
            *self = val;
            true
        } else {
            false
        }
    }
    fn chmin(&mut self, val: Self) -> bool {
        if val < *self {
            *self = val;
            true
        } else {
            false
        }
    }
}

impl<T: PartialOrd + Copy> ChOrd for T {}

pub trait ItertoolsExt: Iterator {
    fn btree_counts(self) -> BTreeMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut counts = BTreeMap::new();
        self.for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
    fn fx_counts(self) -> FxHashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut counts = FxHashMap::default();
        self.for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
}

impl<I> ItertoolsExt for I where I: Iterator + ?Sized {}

pub fn coord_compress<T>(v: &[T]) -> (Vec<usize>, Vec<T>)
where
    T: Ord + Copy,
{
    let n = v.len();
    let mut ord = (0..n).collect_vec();
    ord.sort_unstable_by_key(|&i| &v[i]);

    let mut unique_sorted = Vec::new();
    let mut compressed = vec![0; n];

    for (k, &i) in ord.iter().enumerate() {
        if k == 0 || v[i] != v[ord[k - 1]] {
            unique_sorted.push(v[i]);
        }
        compressed[i] = unique_sorted.len() - 1;
    }

    (compressed, unique_sorted)
}

pub fn cumsum<T>(v: &[T]) -> Vec<T>
where
    T: Copy + Zero + Add<Output = T>,
{
    let mut ret = vec![T::zero(); v.len() + 1];
    for i in 0..v.len() {
        ret[i + 1] = v[i] + ret[i];
    }
    ret
}

pub fn prime_factors<T>(mut n: T) -> Vec<T>
where
    T: PrimInt + Unsigned,
{
    let mut ret = vec![];
    let mut i = T::one() + T::one();
    while i <= n / i {
        while n % i == T::zero() {
            ret.push(i);
            n = n / i;
        }
        i = i + T::one();
    }
    if n > T::one() {
        ret.push(n);
    }
    ret
}

pub fn powmod<T, U>(mut base: T, mut exp: U, modu: T) -> T
where
    T: PrimInt + Unsigned,
    U: PrimInt + Unsigned,
{
    let mut ret = T::one() % modu;
    base = base % modu;
    while exp != U::zero() {
        if (exp & U::one()) == U::one() {
            ret = (ret * base) % modu;
        }
        exp = exp >> 1;
        base = base * base % modu;
    }
    ret
}
