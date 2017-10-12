mod node;

pub struct Tree<T>
    where T: Ord
{
    root: Option<node::Node<T>>,
}

impl<T> Tree<T>
    where T: Ord
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

    pub fn destroy(&mut self, needle: &T) {
        if let Some(ref mut root_node) = self.root {}
    }
}
