
extern crate treap;

use treap::Treap;


fn main() {
    let mut t: Treap<i32, &str> = Treap::new();

    t.insert(5, "hej");
    t.insert(10, "foo");
    t.insert(2, "bar");
    t.insert(8, "trolol");
    t.insert(11, "fisk");
    t.delete(&8);

    println!("{:?}", t.insert(2, "bar2"));

    println!("{:?}", t);
    println!("{:?}", t.get(&2));
    println!("{:?}", t.get(&3));

    for (k, v) in t.iter() {
        println!("key: {}, val: {}", k, v);
    }
}
