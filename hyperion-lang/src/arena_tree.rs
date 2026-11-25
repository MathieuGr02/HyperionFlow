#[derive(Debug)]
pub struct Ref(usize);

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

    pub fn add(&mut self, item: T) -> Ref {
        let index = self.data.len();
        self.data.push(item);
        Ref(index)
    }

    pub fn set_root(&mut self, root: Ref) {
        self.root = Some(root);
    }

    pub fn get(&self, index: Ref) -> Option<&T> {
        self.data.get(index.0)
    }
}
