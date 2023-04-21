use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) struct MapValueDiff<K: Eq + Hash + Debug, V: Eq + Debug> {
    key: K,
    left_value: V,
    right_value: V,
}

pub(crate) struct MapComparison<K: Eq + Hash + Debug, V: Eq + Debug> {
    pub(crate) exclusive_left_keys: Vec<K>,
    pub(crate) exclusive_right_keys: Vec<K>,
    pub(crate) different_values: Vec<MapValueDiff<K, V>>,
    pub(crate) common: Vec<(K, V)>,
}

pub(crate) struct SequenceComparison<V: Eq + Debug, C: IntoIterator<Item=V>> {
    pub(crate) exclusive_left: C,
    pub(crate) exclusive_right: C,
    pub(crate) common: Vec<(V, usize)>,
}

pub(crate) struct SetComparison<V: Eq + Debug + Hash, C: IntoIterator<Item=V>> {
    pub(crate) exclusive_left: C,
    pub(crate) exclusive_right: C,
    pub(crate) common: HashSet<V>,
}

pub(crate) enum SubSequenceComparison {
    Found(usize, usize),
    NotFound,
}

pub(crate) enum ElementContainment {
    Found(usize),
    NotFound,
}

impl<V: Eq + Debug, C: IntoIterator<Item=V>> SequenceComparison<V, C> {
    fn is_equal(&self) -> bool {
        self.exclusive_left.is_empty() && self.exclusive_right.is_empty()
    }
}
