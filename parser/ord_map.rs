pub trait OrdMapItem<I> {
    fn name(&self) -> Option<I>;
}

#[derive(Clone, Debug)]
pub struct OrdMap<K, T>
where
    T: OrdMapItem<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    vec: Vec<T>,
    map: std::collections::HashMap<K, usize>,
}

impl<K, T> Default for OrdMap<K, T>
where
    T: OrdMapItem<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn default() -> Self {
        Self {
            vec: Default::default(),
            map: Default::default(),
        }
    }
}

impl<K, T> From<Vec<T>> for OrdMap<K, T>
where
    T: OrdMapItem<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    fn from(vec: Vec<T>) -> Self {
        let mut map = std::collections::HashMap::new();
        // TODO remove for loop use for_each and filter
        for (i, item) in vec.iter().enumerate() {
            if let Some(name) = item.name() {
                map.insert(name, i);
            }
        }

        Self { vec, map }
    }
}

impl<K, T> OrdMap<K, T>
where
    T: OrdMapItem<K>,
    K: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn push(&mut self, item: T) -> Result<(), K> {
        if let Some(name) = item.name().clone() {
            if self.map.contains_key(&name) {
                return Err(name);
            }
            self.map.insert(name, self.vec.len());
        }
        self.vec.push(item);
        Ok(())
    }

    pub fn get(&self, name: &K) -> Option<&T> {
        self.map.get(name).map(|index| &self.vec[*index])
    }

    pub fn keys(&self) -> std::collections::hash_map::Keys<'_, K, usize> {
        self.map.keys()
    }
}
