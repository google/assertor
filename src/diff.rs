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
        pub(crate) extra: Vec<(K, V)>,
        pub(crate) missing: Vec<(K, V)>,
        pub(crate) different_values: Vec<MapValueDiff<K, V>>,
        pub(crate) common: Vec<(K, V)>,
    }

    // TODO: how would this look like for the `BTreeMap`?
    impl<K: Eq + Hash + Debug, V: PartialEq + Debug> MapComparison<K, V> {
        pub(crate) fn from_hash_maps<'a>(
            left: &'a HashMap<K, V>,
            right: &'a HashMap<K, V>,
        ) -> MapComparison<&'a K, &'a V> {
            let mut extra = vec![];
            let mut missing = vec![];
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
                        extra.push((key, value));
                    }
                }
            }

            for (key, value) in right {
                if !left.contains_key(key) {
                    missing.push((key, value));
                }
            }

            MapComparison {
                extra,
                missing,
                different_values,
                common,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;

        use crate::diff::map::MapComparison;
        use test_case::test_case;

        //          left              right             extra               missing             common               name
        #[test_case(vec![],           vec![],           vec![],             vec![],             vec![] ;             "empty maps")]
        #[test_case(vec![("123", 2)], vec![],           vec![(&"123", &2)], vec![],             vec![] ;             "extra entry")]
        #[test_case(vec![],           vec![("123", 2)], vec![],             vec![(&"123", &2)], vec![] ;             "missing entry")]
        #[test_case(vec![("123", 2)], vec![("123", 2)], vec![],             vec![],             vec![(&"123", &2)] ; "common entry")]
        fn map_diff(
            left: Vec<(&str, i32)>,
            right: Vec<(&str, i32)>,
            extra: Vec<(&&str, &i32)>,
            missing: Vec<(&&str, &i32)>,
            common: Vec<(&&str, &i32)>,
        ) {
            let l: HashMap<&str, i32> = left.into_iter().collect();
            let r: HashMap<&str, i32> = right.into_iter().collect();
            let result = MapComparison::from_hash_maps(&l, &r);
            assert_eq!(common, result.common);
            assert_eq!(extra, result.extra);
            assert_eq!(missing, result.missing);
        }
    }
}

pub(crate) mod iter {
    use std::fmt::Debug;

    /// Differences between two Sequence-like structures.
    pub(crate) struct SequenceComparison<T: PartialEq + Debug> {
        pub(crate) order_preserved: bool,
        pub(crate) extra: Vec<T>,
        pub(crate) missing: Vec<T>,
    }

    pub(crate) enum SequenceOrderComparison {
        Relative,
        Strict,
    }

    impl<T: PartialEq + Debug> SequenceComparison<T> {
        pub(crate) fn are_same(&self) -> bool {
            self.extra.is_empty() && self.missing.is_empty()
        }

        pub(crate) fn contains_all(&self) -> bool {
            self.missing.is_empty()
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
                SequenceOrderComparison::Strict => {
                    Self::strict_order_comparison(left.clone(), right.clone())
                }
                SequenceOrderComparison::Relative => {
                    Self::relative_order_comparison(left.clone(), right.clone())
                }
            }
        }

        pub(self) fn strict_order_comparison<ICL: Iterator<Item = T>, ICR: Iterator<Item = T>>(
            mut actual_iter: ICL,
            mut expected_iter: ICR,
        ) -> SequenceComparison<T> {
            let mut extra = vec![];
            let mut missing = vec![];
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
                extra,
                missing,
            }
        }

        pub(self) fn relative_order_comparison<ICL: Iterator<Item = T>, ICR: Iterator<Item = T>>(
            mut actual_iter: ICL,
            mut expected_iter: ICR,
        ) -> SequenceComparison<T> {
            let mut missing: Vec<T> = vec![];
            let mut extra: Vec<T> = vec![];
            let mut actual_value = actual_iter.next();
            let mut expected_value = expected_iter.next();
            loop {
                if expected_value.is_none() {
                    if let Some(actual) = actual_value {
                        extra.push(actual);
                    }
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
                extra,
                missing,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::SequenceComparison;
        use crate::diff::iter::SequenceOrderComparison;
        use test_case::test_case;

        //          left                    right          extra             missing       order   name
        #[test_case(vec![1, 2],             vec![],        vec![&1, &2],     vec![],       true  ; "empty right operand")]
        #[test_case(vec![],                 vec![1, 2],    vec![],           vec![&1, &2], false ; "empty left operand")]
        #[test_case(vec![1, 2, 3],          vec![1, 3],    vec![&2],         vec![],       true  ; "extra and relative order")]
        #[test_case(vec![1, 2, 3],          vec![1, 3, 4], vec![&2],         vec![&4],     false ; "not found, both extra and missing")]
        #[test_case(vec![1, 2],             vec![1, 2, 4], vec![],           vec![&4],     false ; "not found, extra prefix")]
        #[test_case(vec![1, 2],             vec![0, 1, 2], vec![&1, &2],     vec![&0],     false ; "not found, extra suffix")]
        #[test_case(vec![1, 2, 3],          vec![3, 1],    vec![&1, &2],     vec![],       false ; "all found, out of order")]
        #[test_case(vec![1, 2, 3],          vec![1, 2, 3], vec![],           vec![],       true  ; "equal")]
        #[test_case(vec![1, 2, 3, 4, 5, 6], vec![1, 3, 6], vec![&2, &4, &5], vec![],       true  ; "order preserved relatively")]
        #[test_case(vec![1, 2, 3, 4],       vec![1, 2, 3], vec![&4],         vec![],       true  ; "prefix sub-sequence")]
        #[test_case(vec![1, 2, 3, 4],       vec![2, 3, 4], vec![&1],         vec![],       true  ; "suffix sub-sequence")]
        fn relative_order_comparison(
            left: Vec<i32>,
            right: Vec<i32>,
            expected_extra: Vec<&i32>,
            expected_missing: Vec<&i32>,
            expected_order: bool,
        ) {
            let result = SequenceComparison::from_iter(
                left.iter(),
                right.iter(),
                SequenceOrderComparison::Relative,
            );
            assert_eq!(expected_extra, result.extra);
            assert_eq!(expected_missing, result.missing);
            assert_eq!(expected_order, result.order_preserved);
        }

        //          left                    right          extra             missing       order   name
        #[test_case(vec![1, 2],             vec![],        vec![&1, &2],     vec![],       true  ; "empty right operand")]
        #[test_case(vec![],                 vec![1, 2],    vec![],           vec![&1, &2], true  ; "empty left operand")]
        #[test_case(vec![1, 2, 3],          vec![1, 3],    vec![&2],         vec![],       false ; "extra and relative order")]
        #[test_case(vec![1, 2, 3],          vec![2, 3, 4], vec![&1],         vec![&4],     false ; "not found, both extra and missing")]
        #[test_case(vec![1, 2],             vec![1, 2, 4], vec![],           vec![&4],     true  ; "not found, extra prefix")]
        #[test_case(vec![1, 2],             vec![0, 1, 2], vec![],           vec![&0],     false ; "not found, extra suffix")]
        #[test_case(vec![1, 2, 3],          vec![3, 1],    vec![&2],         vec![],       false ; "all found, out of order")]
        #[test_case(vec![1, 2, 3],          vec![1, 2, 3], vec![],           vec![],       true  ; "equal")]
        #[test_case(vec![1, 2, 3, 4, 5, 6], vec![1, 3, 6], vec![&2, &4, &5], vec![],       false ; "order preserved relatively")]
        #[test_case(vec![1, 2, 3, 4, 5, 6], vec![3, 4, 5], vec![&1, &2, &6], vec![],       false ; "order preserved strictly")]
        #[test_case(vec![1, 2, 3, 4],       vec![1, 2, 3], vec![&4],         vec![],       true  ; "prefix sub-sequence")]
        #[test_case(vec![1, 2, 3, 4],       vec![2, 3, 4], vec![&1],         vec![],       false ; "suffix sub-sequence")]
        fn strict_order_comparison(
            left: Vec<i32>,
            right: Vec<i32>,
            expected_extra: Vec<&i32>,
            expected_missing: Vec<&i32>,
            expected_order: bool,
        ) {
            let result = SequenceComparison::from_iter(
                left.iter(),
                right.iter(),
                SequenceOrderComparison::Strict,
            );
            assert_eq!(expected_extra, result.extra);
            assert_eq!(expected_missing, result.missing);
            assert_eq!(expected_order, result.order_preserved);
        }
    }
}
