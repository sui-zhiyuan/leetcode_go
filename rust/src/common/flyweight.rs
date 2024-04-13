use std::collections::HashMap;

pub struct Flyweight<TP, TR>
where
    TP: Eq + std::hash::Hash,
{
    cache: HashMap<TP, TR>,
    f: Box<dyn FnMut(TP) -> TR>,
}

impl<TP, TR> Flyweight<TP, TR>
where
    TP: Eq + std::hash::Hash + Copy,
{
    pub fn new(f: Box<dyn FnMut(TP) -> TR>) -> Self {
        Self {
            cache: HashMap::new(),
            f,
        }
    }

    pub fn get(&mut self, p: TP) -> &TR {
        self.cache.entry(p).or_insert_with(|| (self.f)(p))
    }
}
