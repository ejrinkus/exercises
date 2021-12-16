#[derive(Clone, Debug, Default, Ord)]
struct BinaryNode<V: Debug + Default + Ord> {
    value: V,
    left_child: Option<BinaryNode>,
    right_child: Option<BinaryNode>,
}

impl fmt::Display for BinaryNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.left_child.is_some() && self.right_child.is_some() {
            return write!(
                f,
                "{ {} -> (left={}, right={})",
                self.value,
                self.left_child.unwrap(),
                self.right_child.unwrap()
            );
        }
        if self.right_child.is_some() {
            return write!(
                f,
                "{ {} -> (left=None, right={})",
                self.value,
                self.right_child.unwrap()
            );
        }
        if self.left_child.is_some() {
            return write!(
                f,
                "{ {} -> (left={}, right=None)",
                self.value,
                self.left_child.unwrap()
            );
        }
        return write!(f, "{ {} -> (left=None, right=None)", self.value);
    }
}

struct BinaryHeap {}
