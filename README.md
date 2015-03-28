
# treap-rs

[![Build Status](https://travis-ci.org/mbudde/treap-rs.svg)](https://travis-ci.org/mbudde/treap-rs)

A randomized treap implementation.

[Documentation](http://mbudde.github.io/treap-rs/treap/index.html)

## Example

```rust

extern crate treap;

use treap::TreapMap;

fn main() {
    let mut t = TreapMap::new();

    for i in 0..10 {
        t.insert(i, i);
    }

    for (k, v) in &mut t {
        *v = *v * *v;
    }

    assert_eq!(t.get(&5), Some(&25));
    assert_eq!(t.remove(&3), Some(9));
}
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
treap = "*"
```

and this to your crate root:

```rust
extern crate treap;
```
