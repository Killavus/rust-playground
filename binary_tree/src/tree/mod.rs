mod node;

pub struct Tree<T>
    where T: PartialOrd
{
    root: Option<node::Node<T>>,
}

impl<T> Tree<T>
    where T: PartialOrd
{
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut node) => {
                node.insert(value);
            }
            None => {
                self.root = Some(node::Node::new(value));
            }
        }
    }

    pub fn exists(&self, needle: &T) -> bool {
        match self.root {
            Some(ref node) => node.exists(needle),
            None => false,
        }
    }
}
