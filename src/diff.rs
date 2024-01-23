// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub(crate) mod map {
    use crate::diff::iter::{SequenceComparison, SequenceOrderComparison};
    use std::collections::{BTreeMap, HashMap};
    use std::fmt::Debug;
    use std::hash::Hash;

    /// Difference for a single key in a Map-like data structure.
    pub(crate) struct MapValueDiff<K: Debug, V: PartialEq + Debug> {
        pub(crate) key: K,
        pub(crate) actual_value: V,
        pub(crate) expected_value: V,
    }

    /// Disjoint and commonalities representation between two Map-like data structures.
    pub(crate) struct MapComparison<K: Eq + Debug, V: PartialEq + Debug> {
        pub(crate) extra: Vec<(K, V)>,
        pub(crate) missing: Vec<(K, V)>,
        pub(crate) different_values: Vec<MapValueDiff<K, V>>,
        pub(crate) common: Vec<(K, V)>,
        pub(crate) key_order_comparison: Option<SequenceComparison<K>>,
    }

    pub trait MapLike<K: Eq, V> {
        type It<'a>: Iterator<Item = &'a K>
        where
            K: 'a,
            V: 'a,
            Self: 'a;

        fn get(&self, k: &K) -> Option<&V>;
        fn contains(&self, k: &K) -> bool {
            self.get(k).is_some()
        }

        fn keys_iter<'a>(&'a self) -> Self::It<'a>
        where
            K: 'a,
            V: 'a;

        fn keys_ordered(&self) -> bool;

        fn len(&self) -> usize {
            self.keys_iter().count()
        }

        fn keys<'a>(&'a self) -> Vec<&'a K>
        where
            K: 'a,
            V: 'a,
        {
            self.keys_iter().collect()
        }
        fn entries(&self) -> Vec<(&K, &V)>;
    }

    pub trait OrderedMapLike<K: Eq + Ord, V>: MapLike<K, V> {}

    impl<K: Eq + Ord, V> MapLike<K, V> for BTreeMap<K, V> {
        type It<'a> = std::collections::btree_map::Keys<'a, K, V> where K: 'a, V: 'a;

        fn get(&self, k: &K) -> Option<&V> {
            self.get(k)
        }

        fn keys_iter<'a>(&'a self) -> Self::It<'a>
        where
            K: 'a,
            V: 'a,
        {
            self.keys()
        }

        fn keys_ordered(&self) -> bool {
            true
        }

        fn entries(&self) -> Vec<(&K, &V)> {
            self.into_iter().collect()
        }
    }

    impl<K: Eq + Ord, V> OrderedMapLike<K, V> for BTreeMap<K, V> {}

    impl<K: Eq + Hash, V> MapLike<K, V> for HashMap<K, V> {
        type It<'a> = std::collections::hash_map::Keys<'a, K, V> where K: 'a, V: 'a;

        fn get(&self, k: &K) -> Option<&V> {
            self.get(k)
        }

        fn keys_iter<'a>(&'a self) -> Self::It<'a>
        where
            K: 'a,
            V: 'a,
        {
            self.keys()
        }

        fn keys_ordered(&self) -> bool {
            false
        }

        fn entries(&self) -> Vec<(&K, &V)> {
            self.into_iter().collect()
        }
    }

    impl<K: Eq + Debug, V: PartialEq + Debug> MapComparison<K, V> {
        pub(crate) fn from_map_like<'a, M1, M2>(
            actual: &'a M1,
            expected: &'a M2,
            order_comparison: Option<SequenceOrderComparison>,
        ) -> MapComparison<&'a K, &'a V>
        where
            M1: MapLike<K, V>,
            M2: MapLike<K, V>,
        {
            let mut extra = vec![];
            let mut missing = vec![];
            let mut different_values = vec![];
            let mut common = vec![];

            for (key, value) in actual.entries() {
                match expected.get(key) {
                    Some(rv) if value == rv => {
                        common.push((key, value));
                    }
                    Some(rv) => different_values.push(MapValueDiff {
                        key,
                        actual_value: value,
                        expected_value: rv,
                    }),
                    None => {
                        extra.push((key, value));
                    }
                }
            }

            for (key, value) in expected.entries() {
                if !actual.contains(key) {
                    missing.push((key, value));
                }
            }

            let key_order_comparison = order_comparison
                .filter(|_| actual.keys_ordered() && expected.keys_ordered())
                .map(|comparison| {
                    SequenceComparison::from_iter(
                        actual.keys().into_iter(),
                        expected.keys().into_iter(),
                        comparison,
                    )
                });

            MapComparison {
                extra,
                missing,
                different_values,
                common,
                key_order_comparison,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::collections::{BTreeMap, HashMap};

        use crate::diff::iter::SequenceOrderComparison;
        use crate::diff::map::MapComparison;
        use test_case::test_case;
        /*
                    expected          actual            extra               missing             common               name
        */
        #[test_case(vec![],           vec![],           vec![],             vec![],             vec![] ;             "empty maps")]
        #[test_case(vec![("123", 2)], vec![],           vec![(&"123", &2)], vec![],             vec![] ;             "extra entry")]
        #[test_case(vec![],           vec![("123", 2)], vec![],             vec![(&"123", &2)], vec![] ;             "missing entry")]
        #[test_case(vec![("123", 2)], vec![("123", 2)], vec![],             vec![],             vec![(&"123", &2)] ; "common entry")]
        fn unordered_map_diff(
            left: Vec<(&str, i32)>,
            right: Vec<(&str, i32)>,
            extra: Vec<(&&str, &i32)>,
            missing: Vec<(&&str, &i32)>,
            common: Vec<(&&str, &i32)>,
        ) {
            let l: HashMap<&str, i32> = left.into_iter().collect();
            let r: HashMap<&str, i32> = right.into_iter().collect();
            let result = MapComparison::from_map_like(&l, &r, None);
            assert_eq!(common, result.common);
            assert_eq!(extra, result.extra);
            assert_eq!(missing, result.missing);
        }

        /*
                    expected                                    actual                        extra             missing common                              order_preserved  order_extra  order_missing  name
        */
        #[test_case(vec![(1, 1), (2, 2), (3, 3), (4, 4)],       vec![(1, 1), (2, 2), (3, 3)], vec![(&4, &4)],   vec![], vec![(&1, &1), (&2, &2), (&3, &3)], true,            vec![&4],    vec![]       ; "prefix sub-sequence")]
        #[test_case(vec![(1, 1), (2, 2), (3, 3), (4, 4)],       vec![(2, 2), (3, 3), (4, 4)], vec![(&1, &1)],   vec![], vec![(&2, &2), (&3, &3), (&4, &4)], true,            vec![&1],    vec![]       ; "suffix sub-sequence")]
        fn relative_key_order_map_diff(
            left: Vec<(i32, i32)>,
            right: Vec<(i32, i32)>,
            extra: Vec<(&i32, &i32)>,
            missing: Vec<(&i32, &i32)>,
            common: Vec<(&i32, &i32)>,
            order_preserved: bool,
            order_extra: Vec<&i32>,
            order_missing: Vec<&i32>,
        ) {
            let l: BTreeMap<i32, i32> = left.into_iter().collect();
            let r: BTreeMap<i32, i32> = right.into_iter().collect();
            let result =
                MapComparison::from_map_like(&l, &r, Some(SequenceOrderComparison::Relative));
            assert_eq!(common, result.common);
            assert_eq!(extra, result.extra);
            assert_eq!(missing, result.missing);
            let order_comparison = result.key_order_comparison.unwrap();
            assert_eq!(order_preserved, order_comparison.order_preserved);
            assert_eq!(order_extra, order_comparison.extra);
            assert_eq!(order_missing, order_comparison.missing);
        }

        /*
                    expected                                    actual                        extra             missing common                              order_preserved  order_extra  order_missing  name
        */
        #[test_case(vec![(1, 1), (2, 2), (3, 3), (4, 4)],       vec![(1, 1), (2, 2), (3, 3)], vec![(&4, &4)],   vec![], vec![(&1, &1), (&2, &2), (&3, &3)], true,            vec![&4],    vec![]       ; "prefix sub-sequence")]
        #[test_case(vec![(1, 1), (2, 2), (3, 3), (4, 4)],       vec![(2, 2), (3, 3), (4, 4)], vec![(&1, &1)],   vec![], vec![(&2, &2), (&3, &3), (&4, &4)], false,           vec![&1],    vec![]       ; "suffix sub-sequence")]
        fn strict_key_order_map_diff(
            left: Vec<(i32, i32)>,
            right: Vec<(i32, i32)>,
            extra: Vec<(&i32, &i32)>,
            missing: Vec<(&i32, &i32)>,
            common: Vec<(&i32, &i32)>,
            order_preserved: bool,
            order_extra: Vec<&i32>,
            order_missing: Vec<&i32>,
        ) {
            let l: BTreeMap<i32, i32> = left.into_iter().collect();
            let r: BTreeMap<i32, i32> = right.into_iter().collect();
            let result =
                MapComparison::from_map_like(&l, &r, Some(SequenceOrderComparison::Strict));
            assert_eq!(common, result.common);
            assert_eq!(extra, result.extra);
            assert_eq!(missing, result.missing);
            let order_comparison = result.key_order_comparison.unwrap();
            assert_eq!(order_preserved, order_comparison.order_preserved);
            assert_eq!(order_extra, order_comparison.extra);
            assert_eq!(order_missing, order_comparison.missing);
        }

        #[test]
        fn ordered_unordered_key_order_comparison() {
            let actual = BTreeMap::from([(1, 1)]);
            let expected = HashMap::from([(2, 2)]);
            let comparison = MapComparison::from_map_like(
                &actual,
                &expected,
                Some(SequenceOrderComparison::Strict),
            );
            assert!(comparison.key_order_comparison.is_none());
        }

        #[test]
        fn unordered_ordered_key_order_comparison() {
            let actual = HashMap::from([(2, 2)]);
            let expected = BTreeMap::from([(1, 1)]);
            let comparison = MapComparison::from_map_like(
                &actual,
                &expected,
                Some(SequenceOrderComparison::Strict),
            );
            assert!(comparison.key_order_comparison.is_none());
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
        pub(crate) fn contains_exactly(&self) -> bool {
            self.extra.is_empty() && self.missing.is_empty()
        }

        pub(crate) fn contains_all(&self) -> bool {
            self.missing.is_empty()
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

        /*
                    expected                actual         extra             missing       order   name
        */
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

        //          expected                actual         extra             missing       order
        // name
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
