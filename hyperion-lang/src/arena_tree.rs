use std::fmt::Display;

/// Reference index of a value in the [`ArenaTree<T>`]
#[derive(Debug, Clone, Copy)]
pub struct Ref(usize);

impl Ref {
    pub fn zero() -> Self {
        Ref(0)
    }
}

impl Display for Ref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The Arena Tree type.
/// Represents a N-Ary tree structure where the nodes are stored in a flat vector
///
/// The user of this type is responsible for storing the returned references of the items stored in the tree.
/// As such, the user should set the root after its done building the tree
#[derive(Debug)]
pub struct ArenaTree<T> {
    root: Option<Ref>,
    data: Vec<T>,
}

impl<T> ArenaTree<T> {
    pub fn new() -> Self {
        ArenaTree {
            root: None,
            data: Vec::new(),
        }
    }

    /// Add new element to the tree.
    /// Returns the reference of its location in the arena
    pub fn add(&mut self, item: T) -> Ref {
        let index = self.data.len();
        self.data.push(item);
        Ref(index)
    }

    /// Replace value at index with new value
    pub fn replace(&mut self, index: Ref, value: T) {
        let _ = std::mem::replace(&mut self.data[index.0], value);
    }

    /// Set the root reference of the arena tree
    pub fn set_root(&mut self, root: Ref) {
        self.root = Some(root);
    }

    /// Get the value at the index in the arena tree
    pub fn get(&self, index: Ref) -> Option<&T> {
        self.data.get(index.0)
    }

    pub fn get_mut(&mut self, index: Ref) -> Option<&mut T> {
        self.data.get_mut(index.0)
    }

    pub fn get_unchecked(&self, index: Ref) -> &T {
        self.data.get(index.0).as_ref().unwrap()
    }

    pub fn get_root(&self) -> Option<&Ref> {
        self.root.as_ref()
    }

    pub fn get_root_value(&self) -> Option<&T> {
        if let Some(root) = self.get_root() {
            return self.data.get(root.0);
        }

        None
    }
}
