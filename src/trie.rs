use crate::{internal_data_structure::naive_trie::NaiveTrie, Louds};

pub mod trie;
pub mod trie_builder;

#[derive(Clone, Debug)]
pub struct Trie<Label> {
    louds: Louds,

    /// (LoudsNodeNum - 2) -> TrieLabel
    trie_labels: Vec<TrieLabel<Label>>,
}

pub struct TrieBuilder<Label> {
    naive_trie: NaiveTrie<Label>,
}

#[derive(Clone, Debug)]
struct TrieLabel<Label> {
    label: Label,
    is_terminal: bool,
}
