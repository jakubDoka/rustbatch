pub struct ComponentArray<T> {
    inner: Vec<Option<T>>,
    ids: Vec<usize>,
    len: usize,
    id: usize,
}

impl<T> ComponentArray<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: vec![],
            ids: vec![],
            len: 0,
            id: 0,
        }
    }

    #[inline]
    pub fn insert(&mut self, component: T) -> usize {
        self.len = self.inner.len() - 1;
        if self.ids.is_empty() {
            self.inner.push(Some(component));
            return self.len
        }
        self.id = self.ids[self.len];
        self.inner[self.id] = Some(component);
        self.ids.truncate(self.len);

        self.id
    }

    #[inline]
    pub fn get(&mut self, id: usize) -> Option<T> {
        std::mem::replace(&mut self.inner[id], None)
    }

    pub fn put_back(&mut self, id: usize, component: T) {
        self.inner[id] = Some(component);
    }

    #[inline]
    pub fn get_fer(&self, id: usize) -> &Option<T> {
        &self.inner[id]
    }

    #[inline]
    pub fn get_mut(&mut self, id: usize) -> &mut Option<T> {
        &mut self.inner[id]
    }
}
