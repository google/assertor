use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub(crate) struct MapValueDiff<K: PartialEq + Hash + Debug, V: Eq + Debug> {
    pub(crate) key: K,
    pub(crate) left_value: V,
    pub(crate) right_value: V,
}

pub(crate) struct MapComparison<K: PartialEq + Hash + Debug, V: Eq + Debug> {
    pub(crate) exclusive_left: Vec<(K, V)>,
    pub(crate) exclusive_right: Vec<(K, V)>,
    pub(crate) different_values: Vec<MapValueDiff<K, V>>,
    pub(crate) common: Vec<(K, V)>,
}

impl<K: Eq + PartialEq + Hash + Debug, V: Eq + Debug> MapComparison<K, V> {
    pub(crate) fn from_hash_maps<'a>(
        left: &'a HashMap<K, V>,
        right: &'a HashMap<K, V>,
    ) -> MapComparison<&'a K, &'a V> {
        let mut exclusive_left = vec![];
        let mut exclusive_right = vec![];
        let mut different_values = vec![];
        let mut common = vec![];

        for (key, value) in left {
            match right.get(key) {
                Some(rv) if value == rv => {
                    common.push((key, value));
                }
                Some(rv) => different_values.push(MapValueDiff {
                    key,
                    left_value: value,
                    right_value: rv,
                }),
                None => {
                    exclusive_left.push((key, value));
                }
            }
        }

        for (key, value) in right {
            if !left.contains_key(key) {
                exclusive_right.push((key, value));
            }
        }

        MapComparison {
            exclusive_left,
            exclusive_right,
            different_values,
            common,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::diff::MapComparison;
    use std::collections::HashMap;

    #[test]
    fn diff_empty_maps() {
        let left: HashMap<&str, i32> = HashMap::from([]);
        let right: HashMap<&str, i32> = HashMap::from([]);
        let result = MapComparison::from_hash_maps(&left, &right);
        assert!(result.common.is_empty());
        assert!(result.exclusive_left.is_empty());
        assert!(result.exclusive_right.is_empty());
    }

    #[test]
    fn map_diff_left_exclusive() {
        let left: HashMap<&str, i32> = HashMap::from([("123", 2)]);
        let right: HashMap<&str, i32> = HashMap::from([]);
        let result = MapComparison::from_hash_maps(&left, &right);
        assert!(result.common.is_empty());
        assert_eq!(result.exclusive_left, vec![(&"123", &2)]);
        assert!(result.exclusive_right.is_empty());
    }

    #[test]
    fn map_diff_right_exclusive() {
        let left: HashMap<&str, i32> = HashMap::from([]);
        let right: HashMap<&str, i32> = HashMap::from([("123", 2)]);
        let result = MapComparison::from_hash_maps(&left, &right);
        assert!(result.common.is_empty());
        assert!(result.exclusive_left.is_empty());
        assert_eq!(result.exclusive_right, vec![(&"123", &2)]);
    }

    #[test]
    fn map_diff_common() {
        let left: HashMap<&str, i32> = HashMap::from([("123", 2)]);
        let right: HashMap<&str, i32> = HashMap::from([("123", 2)]);
        let result = MapComparison::from_hash_maps(&left, &right);
        assert_eq!(result.common, vec![(&"123", &2)]);
        assert!(result.exclusive_left.is_empty());
        assert!(result.exclusive_right.is_empty());
    }
}
