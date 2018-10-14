#![feature(test)]

extern crate rand;
extern crate test;
extern crate treap;

use rand::{SeedableRng, StdRng, XorShiftRng};
use test::Bencher;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::iter::{Extend, FromIterator};

use treap::TreapMap;

#[bench]
fn bench_treap_insert(b: &mut Bencher) {
    let seed = [1, 2, 3, 5];
    let rng: XorShiftRng = SeedableRng::from_seed(seed);

    b.iter(|| {
        let mut t = TreapMap::new_with_rng(rng.clone());
        for i in 0..500i32 {
            t.insert(i, i);
        }
        t
    })
}

#[bench]
fn bench_btree_insert(b: &mut Bencher) {
    b.iter(|| {
        let mut t = BTreeMap::<i32, i32>::new();
        for i in 0..500 {
            t.insert(i, i);
        }
        t
    })
}

#[bench]
fn bench_hash_insert(b: &mut Bencher) {
    b.iter(|| {
        let mut t = HashMap::<i32, i32>::new();
        for i in 0..500 {
            t.insert(i, i);
        }
        t
    })
}

#[bench]
fn bench_treap_find(b: &mut Bencher) {
    use rand::Rng;
    let mut nums: Vec<_> = (1..1000).into_iter().collect();
    let seed = [1, 2, 3, 5];
    let mut rng: XorShiftRng = SeedableRng::from_seed(seed);
    rng.shuffle(&mut nums);
    let mut t = TreapMap::new_with_rng(rng);
    t.extend((&nums).into_iter().map(|k| (*k, *k)));

    b.iter(|| {
        let mut sum = 0u32;
        for i in &nums[1..300] {
            sum += *t.get(&i).unwrap();
        }
        sum
    })
}

#[bench]
fn bench_btree_find(b: &mut Bencher) {
    use rand::Rng;
    let mut nums: Vec<_> = (1..1000).into_iter().collect();
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.shuffle(&mut nums);
    let t = BTreeMap::from_iter((&nums).into_iter().map(|k| (*k, *k)));

    b.iter(|| {
        let mut sum = 0u32;
        for i in &nums[0..300] {
            sum += *t.get(i).unwrap();
        }
        sum
    })
}

#[bench]
fn bench_hash_find(b: &mut Bencher) {
    use rand::Rng;
    let mut nums: Vec<_> = (1..1000).into_iter().collect();
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    rng.shuffle(&mut nums);
    let t: HashMap<u32, u32> = HashMap::from_iter((&nums).into_iter().map(|k| (*k, *k)));

    b.iter(|| {
        let mut sum = 0u32;
        for i in &nums[1..300] {
            sum += *t.get(&i).unwrap();
        }
        sum
    })
}
