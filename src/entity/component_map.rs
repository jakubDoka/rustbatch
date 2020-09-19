use crate::entity::id_generator::IDType;
use std::collections::HashMap;
use crate::entity::FastHash;
use std::ops::{Deref, DerefMut};

pub struct ComponentMap<V, K: IDType = u64> {
    inner: HashMap<K, V, FastHash>
}

impl<K: IDType, V> Deref for ComponentMap<K, V> {
    type Target = HashMap<K, V, FastHash>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K: IDType, V> DerefMut for ComponentMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}