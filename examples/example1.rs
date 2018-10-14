extern crate treap;

use treap::TreapMap;

fn main() {
    let mut t = TreapMap::new();

    t.insert(5, "hej");
    t.insert(10, "foo");
    t.insert(2, "bar");
    t.insert(8, "trolol");
    t.insert(11, "fisk");
    t.remove(&8);

    for i in 2..19 {
        t.insert(i, "test");
    }

    println!("{:?}", t.insert(2, "bar2"));

    println!("{:?}", t.get(&2));
    println!("{:?}", t.get(&3));

    for (k, v) in &t {
        println!("key: {}, val: {}", k, v);
    }

    println!("in order:");
    for (k, v) in t.iter_ordered() {
        println!("key: {}, val: {}", k, v);
    }

    for (k, v) in t {
        println!("i own key: {}, val: {}", k, v);
    }

    let mut r = TreapMap::new();
    r.extend(vec![(1, 200), (2, 120), (3, 330)].into_iter());

    for (k, v) in &mut r {
        *v = *v + *k;
    }
    println!("{:?}", r.get(&2));
}
