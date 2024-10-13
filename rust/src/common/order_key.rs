use std::cmp::Ordering;

pub trait OrderKeyTrait {
    type TKey;
    fn order_key(&self) -> Self::TKey;
}

pub struct OrderKey<T>(T)
where
    T: OrderKeyTrait;

impl<T> OrderKey<T>
where
    T: OrderKeyTrait,
{
    pub fn new(t: T) -> Self {
        Self(t)
    }
}

impl<T> From<T> for OrderKey<T>
where
    T: OrderKeyTrait,
{
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> PartialEq for OrderKey<T>
where
    T: OrderKeyTrait,
    T::TKey: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.order_key() == other.0.order_key()
    }
}

impl<T> Eq for OrderKey<T>
where
    T: OrderKeyTrait,
    T::TKey: Eq,
{
}

impl<T> PartialOrd for OrderKey<T>
where
    T: OrderKeyTrait,
    T::TKey: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.order_key().partial_cmp(&other.0.order_key())
    }
}

impl<T> Ord for OrderKey<T>
where
    T: OrderKeyTrait,
    T::TKey: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.order_key().cmp(&other.0.order_key())
    }
}