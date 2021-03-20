#[derive(Default)]
pub struct TrieNode {
    /// How many "empty strings" hang off this node?
    /// If this trie is an interior node in a larger trie, then these each correspond
    /// to one copy of the same word (the one spelled out by the path from the root).
    pub num_leaves: u32,
    /// You know the deal.
    pub children: [Option<Box<TrieNode>>; 26],
}

/// Panics on chars other than A-Z.
pub fn build_trie<'a>(words: impl Iterator<Item=&'a str>) -> Option<Box<TrieNode>> {
    let mut root = None;
    for w in words {
        insert(&mut root, w.chars());
    }
    root
}

/// Panics on chars other than A-Z.
fn insert(trie: &mut Option<Box<TrieNode>>, mut word: impl Iterator<Item=char>) {
    // Create the node if it doesn't exist.
    if trie.is_none() {
        *trie = Some(Box::new(TrieNode::default()));
    }
    let node = trie.as_mut().unwrap();

    match word.next() {
        None => {
            node.num_leaves += 1;
        }
        Some(c) => {
            assert!('A' <= c && c <= 'Z');
            let idx = c as u8 - b'A';

            insert(&mut node.children[idx as usize], word);
        }
    }
}
