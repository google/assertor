pub(crate) mod map {
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    /// Difference for a single key in a Map-like data structure.
    pub(crate) struct MapValueDiff<K: Eq + Hash + Debug, V: PartialEq + Debug> {
        pub(crate) key: K,
        pub(crate) left_value: V,
        pub(crate) right_value: V,
    }

    /// Disjoint representation and commonalities between two Map-like data structures.
    pub(crate) struct MapComparison<K: Eq + Hash + Debug, V: PartialEq + Debug> {
        pub(crate) exclusive_left: Vec<(K, V)>,
        pub(crate) exclusive_right: Vec<(K, V)>,
        pub(crate) different_values: Vec<MapValueDiff<K, V>>,
        pub(crate) common: Vec<(K, V)>,
    }

    // TODO: how would this look like for the `BTreeMap`?
    impl<K: Eq + Hash + Debug, V: PartialEq + Debug> MapComparison<K, V> {
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
        use std::collections::HashMap;

        use crate::diff::map::MapComparison;

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
}

pub(crate) mod iter {
    use std::fmt::Debug;

    /// Differences between two Sequence-like structures.
    pub(crate) struct SequenceComparison<T: PartialEq + Debug> {
        pub(crate) order_preserved: bool,
        pub(crate) exclusive_left: Vec<T>,
        pub(crate) exclusive_right: Vec<T>,
    }

    pub(crate) enum SequenceOrderComparison {
        Relative,
        Exact,
    }

    impl<T: PartialEq + Debug> SequenceComparison<T> {
        pub(crate) fn are_same(&self) -> bool {
            self.exclusive_left.is_empty() && self.exclusive_right.is_empty()
        }

        pub(crate) fn contains_all(&self) -> bool {
            self.exclusive_right.is_empty()
        }

        pub(crate) fn are_equal(&self) -> bool {
            self.are_same() && self.order_preserved
        }

        pub(crate) fn from_iter<
            ICL: Iterator<Item = T> + Clone,
            ICR: Iterator<Item = T> + Clone,
        >(
            left: ICL,
            right: ICR,
            sequence_order: SequenceOrderComparison,
        ) -> SequenceComparison<T> {
            match sequence_order {
                SequenceOrderComparison::Exact => {
                    Self::strict_order_comparison(left.clone(), right.clone())
                }
                SequenceOrderComparison::Relative => {
                    Self::relative_order_comparison(left.clone(), right.clone())
                }
            }
        }

        pub(self) fn strict_order_comparison<
            ICL: Iterator<Item = T> + Clone,
            ICR: Iterator<Item = T> + Clone,
        >(
            mut actual_iter: ICL,
            mut expected_iter: ICR,
        ) -> SequenceComparison<T> {
            let mut extra: Vec<T> = vec![];
            let mut missing: Vec<T> = vec![];
            let mut order_preserved = true;
            let move_element = |el: T, source: &mut Vec<T>, target: &mut Vec<T>| {
                if let Some(idx) = source.iter().position(|e: &T| e.eq(&el)) {
                    source.remove(idx);
                } else {
                    target.push(el);
                }
            };
            loop {
                match (actual_iter.next(), expected_iter.next()) {
                    (Some(actual_elem), Some(expect_elem)) => {
                        if actual_elem.eq(&expect_elem) {
                            continue;
                        }
                        order_preserved = false;
                        move_element(expect_elem, &mut extra, &mut missing);
                        move_element(actual_elem, &mut missing, &mut extra);
                    }
                    (None, Some(expect_elem)) => {
                        move_element(expect_elem, &mut extra, &mut missing);
                    }
                    (Some(actual_elem), None) => {
                        move_element(actual_elem, &mut missing, &mut extra);
                    }
                    (None, None) => break,
                }
            }
            SequenceComparison {
                order_preserved,
                exclusive_left: extra,
                exclusive_right: missing,
            }
        }

        pub(self) fn relative_order_comparison<
            ICL: Iterator<Item = T> + Clone,
            ICR: Iterator<Item = T> + Clone,
        >(
            mut actual_iter: ICL,
            mut expected_iter: ICR,
        ) -> SequenceComparison<T> {
            let mut actual_value = actual_iter.next();
            let mut expected_value = expected_iter.next();
            let mut missing: Vec<T> = vec![];
            let mut extra: Vec<T> = vec![];
            loop {
                if expected_value.is_none() {
                    extra.extend(actual_iter);
                    break;
                }
                if actual_value.is_none() {
                    missing.push(expected_value.unwrap());
                    missing.extend(expected_iter);
                    break;
                }
                if actual_value.eq(&expected_value) {
                    actual_value = actual_iter.next();
                    expected_value = expected_iter.next();
                } else {
                    extra.push(actual_value.unwrap());
                    actual_value = actual_iter.next();
                }
            }
            let order_preserved = missing.is_empty();

            // check out of order elements.
            if !missing.is_empty() {
                for extra_elem in extra.iter() {
                    if let Some(idx) = missing.iter().position(|m: &T| m.eq(extra_elem)) {
                        missing.remove(idx);
                    }
                }
            }

            SequenceComparison {
                order_preserved,
                exclusive_left: extra,
                exclusive_right: missing,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        // TODO: add quickcheck and parameterized (test_case / rstest) tests; for now covered with public API tests
    }
}
