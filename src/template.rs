use core::{convert::Infallible, marker::PhantomData};
use num_traits::{Zero, Bounded, PrimInt, Unsigned};
use std::{
    hash::Hash,
    ops::{Add, BitAnd, BitOr, BitXor},
};

pub use ac_library::{
    dsu::Dsu,
    segtree::{Additive, Max, Min, Multiplicative, Segtree},
    string::z_algorithm_arbitrary,
    ModInt1000000007 as Mint1,
    ModInt998244353 as Mint9,
};
pub use itertools::Itertools;
pub use num_integer::{gcd, lcm};
pub use proconio::{
    input,
    input_interactive,
    marker::{Chars, Isize1, Usize1},
};
pub use rand::{
    seq::{IteratorRandom, SliceRandom},
    rng, Rng,
};
pub use rustc_hash::{FxHashMap, FxHashSet};
pub use std::{
    cmp::Reverse,
    collections::{btree_map, BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    f64::consts::PI,
    mem::swap,
    process::exit,
    time::Instant,
};

pub const DIR4: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];
pub const DIR8: [(usize, usize); 8] = [(!0, !0), (!0, 0), (!0, 1), (0, !0), (0, 1), (1, !0), (1, 0), (1, 1)];

#[derive(Debug, Clone)]
pub struct FxHashMultiSet<T: Eq + Hash> {
    map: FxHashMap<T, usize>,
    len: usize,
}

impl<T: Eq + Hash> FxHashMultiSet<T> {
    pub fn default() -> Self {
        Self {
            map: FxHashMap::default(),
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
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(val, &cnt)| itertools::repeat_n(val, cnt))
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
    S: Copy + BitAnd<Output = S> + Bounded,
{
    type S = S;
    fn identity() -> Self::S {
        S::max_value()
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
    fn counts_btree(self) -> BTreeMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut counts = BTreeMap::new();
        self.for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
    fn counts_fx(self) -> FxHashMap<Self::Item, usize>
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

    let mut values = Vec::new();
    let mut ranks = vec![0; n];

    for (k, &i) in ord.iter().enumerate() {
        if k == 0 || v[i] != v[ord[k - 1]] {
            values.push(v[i]);
        }
        ranks[i] = values.len() - 1;
    }

    (ranks, values)
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
    while i * i <= n {
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
    let mut ret = T::one();
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