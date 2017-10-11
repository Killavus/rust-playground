pub mod tree;

#[cfg(test)]
mod tests {
    use tree::Tree;

    #[test]
    fn simple_node_insertion() {
        let mut tree: Tree<u32> = Tree::new();

        tree.insert(32);

        assert!(tree.exists(&32));

        tree.insert(18);
        tree.insert(24);

        assert!(tree.exists(&18));
        assert!(tree.exists(&24));
        assert!(!tree.exists(&19));
    }

    #[test]
    fn operations_idempotence() {
        let mut tree: Tree<u8> = Tree::new();

        tree.insert(120u8);
        tree.insert(120u8);

        assert!(tree.exists(&120u8));

        tree.destroy(&120u8);

        assert!(!tree.exists(&120u8));
    }
}
