
extern crate treap;

use treap::Treap;


fn main() {
    let mut t: Treap<i32, String> = Treap::new();

    t.insert(5, "hej".to_string());
    t.insert(10, "foo".to_string());
    t.insert(2, "bar".to_string());
    t.insert(8, "trolol".to_string());
    t.insert(11, "fisk".to_string());

    println!("{:?}", t.insert(2, "bar2".to_string()));

    println!("{:?}", t);
    println!("{:?}", t.get(&2));
    println!("{:?}", t.get(&3));

    for (k, v) in t.iter() {
        println!("key: {}, val: {}", k, v);
    }
}
