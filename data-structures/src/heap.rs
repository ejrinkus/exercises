use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug, Default)]
struct BinaryNode<V: Debug + Default + Ord> {
    value: V,
    left_child: Option<Box<BinaryNode<V>>>,
    right_child: Option<Box<BinaryNode<V>>>,
}

impl<V: Debug + Default + Ord> Ord for BinaryNode<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<V: Debug + Default + Ord> PartialOrd for BinaryNode<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<V: Debug + Default + Ord> PartialEq for BinaryNode<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: Debug + Default + Ord> Eq for BinaryNode<V> {}

impl<V: Debug + Default + Ord> Display for BinaryNode<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.left_child.is_some() && self.right_child.is_some() {
            return write!(
                f,
                "{{ {:?} -> (left={}, right={})}}",
                self.value,
                *self.left_child.as_ref().unwrap(),
                *self.right_child.as_ref().unwrap()
            );
        }
        if self.right_child.is_some() {
            return write!(
                f,
                "{{ {:?} -> (left=None, right={})}}",
                self.value,
                *self.right_child.as_ref().unwrap()
            );
        }
        if self.left_child.is_some() {
            return write!(
                f,
                "{{ {:?} -> (left={}, right=None)}}",
                self.value,
                *self.left_child.as_ref().unwrap()
            );
        }
        return write!(f, "{{ {:?} -> (left=None, right=None)}}", self.value);
    }
}

struct BinaryHeap {}
