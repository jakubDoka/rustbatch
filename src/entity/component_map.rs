use crate::entity::id_generator::IDType;
use std::collections::HashMap;
use crate::entity::FastHash;
use std::ops::{Deref, DerefMut};

pub struct ComponentMap<V, K: IDType = u64> {
    inner: HashMap<K, V, FastHash>
}

impl<V, K: IDType> Deref for ComponentMap<V, K> {
    type Target = HashMap<K, V, FastHash>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V, K: IDType> DerefMut for ComponentMap<V, K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}