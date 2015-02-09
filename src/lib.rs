
//! Randomized Treap
//!
//! A treap is a variation of a binary tree. Each inserted key is assigned a priority and the
//! resulting binary tree has the invariant that it is a binary search tree with respect to the
//! keys and a max-heap with respect to the priorities.
//!
//! This implementation is randomized meaning that the priorities are assigned at random. The treap
//! has an expected depth of O(log n).

#![feature(core)]

extern crate rand;

pub use map::TreapMap;
pub use set::TreapSet;

mod node;
pub mod map;
pub mod set;
