pub struct Node<T>
where
    T: PartialOrd,
{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialOrd,
{
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        let next = if value < self.value {
            &mut self.left
        } else {
            &mut self.right
        };

        match next {
            &mut Some(ref mut node) => {
                node.insert(value);
            }
            &mut None => {
                let new_node = Node::new(value);
                let boxed_node = Some(Box::new(new_node));

                *next = boxed_node;
            }
        }
    }
}