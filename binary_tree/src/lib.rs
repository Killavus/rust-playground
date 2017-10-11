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
}
