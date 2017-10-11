pub mod tree;

#[cfg(test)]
mod tests {
    use tree::Tree;

    #[test]
    fn inserts_node() {
        let mut tree: Tree<u32> = Tree::new();

        tree.insert(32);
        tree.insert(18);
        tree.insert(44);
    }
}
