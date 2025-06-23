use std::collections::HashMap;

pub(crate) const DICT_LEN: usize = 32;

pub(crate) static DICTIONARY: [(char, u16); DICT_LEN] = [
    (' ', 321),
    ('!', 59),
    (',', 999),
    ('.', 15),
    ('?', 17),
    ('\n', 888),
    ('a', 0),
    ('b', 8),
    ('c', 6),
    ('d', 4),
    ('e', 2),
    ('f', 12),
    ('g', 34),
    ('h', 56),
    ('i', 78),
    ('j', 90),
    ('k', 70),
    ('l', 58),
    ('m', 36),
    ('n', 14),
    ('o', 136),
    ('p', 358),
    ('q', 570),
    ('r', 16),
    ('s', 72),
    ('t', 91),
    ('u', 18),
    ('v', 71),
    ('w', 3568),
    ('x', 5780),
    ('y', 9653),
    ('z', 975)
];

#[derive(Default, Debug, PartialEq, Eq)]
/// A prefix tree. trie:) sounds like a fairy tale character or something,
/// but the effect is really fairy tale: it makes decoding O(N). wohoo!
pub(crate) struct Trie {
    /// Nodes of the trie.
    /// *Big note: the index 0 should be the root which does not contain any letter—only edges.*
    pub(crate) nodes: Vec<TrieNode>
}

/// Struct representing a node in a Trie (prefix tree)
#[derive(Default, Debug, PartialEq, Eq)]
pub(crate) struct TrieNode {
    /// Set to the letter it represents if the node is a leaf node. Set to `None` otherwise.
    pub(crate) letter: Option<char>,
    /// 0 -> 9
    pub(crate) children: [Option<usize>; 10]
}

impl TrieNode {
    pub(crate) fn new(letter: Option<char>) -> Self {
        Self { letter, ..Default::default() }
    }
}

impl Trie {
    /// Create a new Trie from a given dictionary (array of tuples of `char` and `u16`, i.e the
    /// character associated and its code).
    pub(crate) fn from_dict(dict: &[(char, u16)]) -> Self {
        let mut result = Vec::new();

        result.push(TrieNode::new(None)); // the starting point (root) node

        // For each dictionary entry, add a new leaf node to the tree.
        for (c, n) in dict {

            // get the list of digits
            // TODO: use log10? but this may be faster
            // or even use strings for each of the codes but that may add too much overhead as
            // well..
            let mut residue: u16 = *n;
            let mut digits = Vec::new();
            if *n != 0 {
                while residue > 0 {
                    digits.push(residue % 10);
                    residue = residue / 10;
                }
            } else {
                digits.push(0);
            }
            digits.reverse();

            append_trie_node(&mut result, &digits, *c);

        };
        Self {
            nodes: result
        }
    }

}

/// Append a trie leaf to a trie. Creates new nodes when they don't exist.
/// Note: A leaf is defined as a node without any children. Or, with a letter assigned to it.
///
/// `edges` must be the *sorted* key, e.g 2354. This function will then traverse through edge 0, 2,
/// 3, 5 (creating new nodes if they don't already exist), and then finally a "4" leaf.
fn append_trie_node(tree: &mut Vec<TrieNode>, edges: &[u16], c: char) {
    // we use index-based pointers to refer to a single trie node.
    let mut curr_parent: usize = 0;
    for i in 0..edges.len()-1 {
        let edge = edges[i] as usize;

        // If an edge exists, traverse to it by setting it as our new parent.
        if let Some(new_p) = tree[curr_parent].children[edge] {
            curr_parent = new_p
        } else {
            // Otherwise, we have to create the node first.

            // 0-based indexing consequence: new index = length of vec rn = the old index + 1
            let new_node_idx = tree.len();
            tree.push(TrieNode::new(None));

            tree[curr_parent].children[edge] = Some(new_node_idx);
            curr_parent = new_node_idx;
        }
    }

    // curs ed
    tree[curr_parent].children[edges[edges.len()-1] as usize] = Some(tree.len());

    // afinalllly, add the leaf.
    tree.push(TrieNode::new(Some(c)));
}

/// Get a new hashmap for reverse lookup
pub fn get_encoder_dict_map() -> HashMap<char, u16>{
    HashMap::from(DICTIONARY)
}

#[cfg(test)]
mod test {
    use crate::words::{Trie, TrieNode};

    #[test]
    fn test_build_trie() {
        let example_dict = [('a', 0), ('b', 1), ('c', 23), ('d', 24)];
        let tree = Trie::from_dict(&example_dict);
        let correct_tree = Trie {
            nodes: vec![
                // idx 0
                TrieNode {
                    letter: None,
                    children: [Some(1), Some(2), Some(3), None, None, None, None, None, None, None]
                },
                // idx 1
                TrieNode {
                    letter: Some('a'),
                    ..Default::default()
                },
                // idx 2
                TrieNode {
                    letter: Some('b'),
                    ..Default::default()

                },
                // idx 3
                TrieNode {
                    letter: None,
                    children: [None, None, None, Some(4), Some(5), None, None, None, None, None]
                },
                // idx 4
                TrieNode {
                    letter: Some('c'),
                    children: [None, None, None, None, None, None, None, None, None, None]
                },
                // idx 4
                TrieNode {
                    letter: Some('d'),
                    children: [None, None, None, None, None, None, None, None, None, None]
                }
            ]
        };

        assert_eq!(tree, correct_tree);

    }

}
