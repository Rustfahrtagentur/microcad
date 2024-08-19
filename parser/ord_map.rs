pub trait OrdMapItem<I> {
    fn name(&self) -> Option<I>;
}

#[derive(Clone, Default, Debug)]
pub struct OrdMap<I, T>
where
    T: OrdMapItem<I>,
    I: std::cmp::Eq,
    I: std::hash::Hash,
    I: Clone,
{
    vec: Vec<T>,
    map: std::collections::HashMap<I, usize>,
}

impl<T, I> OrdMap<I, T>
where
    T: OrdMapItem<I>,
    I: std::cmp::Eq,
    I: std::hash::Hash,
    I: Clone,
{
    pub fn new(vec: Vec<T>) -> Self {
        let mut map = std::collections::HashMap::new();
        // TODO remove for loop use for_each and filter
        for (i, item) in vec.iter().enumerate() {
            if let Some(name) = item.name() {
                map.insert(name, i);
            }
        }

        Self { vec, map }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.vec.iter()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn push(&mut self, item: T) -> Result<(), I> {
        if let Some(name) = item.name().clone() {
            if self.map.contains_key(&name) {
                return Err(name);
            }
            self.map.insert(name, self.vec.len());
        }
        self.vec.push(item);
        Ok(())
    }
}
