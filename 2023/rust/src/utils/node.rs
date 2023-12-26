use std::hash::Hash;

pub struct Node<'a> {
    pub index: usize,
    pub prev: Option<&'a Node<'a>>,
}

impl Node<'_> {
    pub fn new(index: usize) -> Self {
        Self { index, prev: None }
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Node<'_> {}

impl Hash for Node<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.prev.hash(state);
    }
}
