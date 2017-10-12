use std::ops::DerefMut;
use std::ops::Deref;
use std::cmp::Ordering;

pub struct Node<T>
    where T: Ord
{
    pub value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
    where T: Ord
{
    pub fn exists(&self, value: &T) -> bool {
        let traversal_result = self.find_node_or_parent(value);
        traversal_result.value == *value
    }

    pub fn insert(&mut self, value: T) {
        let parent = self.find_node_or_parent_mut(&value);

        if parent.value == value {
            return;
        }

        match value.cmp(&parent.value) {
            Ordering::Less => {
                parent.left = Some(Box::new(Node::new(value)));
            }
            Ordering::Greater => {
                parent.right = Some(Box::new(Node::new(value)));
            }
            _ => {
                panic!("Parent has the same value as inserted node despite sanity check.");
            }
        }
    }

    pub fn destroy(&mut self, value: &T) {}

    fn parent_for(&self, value: &T) -> Option<&Node<T>> {
        if self.value == *value {
            return None;
        }

        let mut cursor = Some(self);

        while cursor.is_some() {
            let node = cursor.unwrap();
            let next = node.step(value);

            if let Some(next_node) = next && next_node.value == *value {
                return cursor;
            } else {
                cursor = next;
            }
        }

        None
    }

    //    fn parent_for_mut(&self, value: &T) -> Option<&mut Node<T>> {}

    fn find_node_or_parent(&self, value: &T) -> &Node<T> {
        let mut cursor = Some(self);

        loop {
            let node = cursor.unwrap();
            let next_node = node.step(value);
            if node.value == *value || next_node.is_none() {
                return node;
            }

            cursor = next_node;
        }
    }

    fn find_node_or_parent_mut(&mut self, value: &T) -> &mut Node<T> {
        let mut cursor = Some(self);

        loop {
            let node = cursor.unwrap();
            let next_node = node.step_mut(value);
            if node.value == *value || next_node.is_none() {
                return node;
            }

            cursor = next_node;
        }
    }

    fn step(&self, value: &T) -> Option<&Node<T>> {
        match value.cmp(&self.value) {
            Ordering::Less => self.left.as_ref().and_then(|node| Some(node.deref())),
            Ordering::Greater => self.right.as_ref().and_then(|node| Some(node.deref())),
            Ordering::Equal => Some(&self),
        }
    }

    fn step_mut(&mut self, value: &T) -> Option<&mut Node<T>> {
        match value.cmp(&self.value) {
            Ordering::Less => {
                self.left
                    .as_ref()
                    .and_then(|node| Some(node.deref_mut()))
            }
            Ordering::Greater => {
                self.right
                    .as_ref()
                    .and_then(|node| Some(node.deref_mut()))
            }
            Ordering::Equal => Some(&mut self),
        }
    }

    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}
