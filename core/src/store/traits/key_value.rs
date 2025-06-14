/*
    appellation: key_value <module>
    authors: @FL03
*/

/// A [`RawEntry`] represents a single record within a key-value store, providing access to the
/// key and value of the entry.
pub trait RawEntry<'a> {
    type Key;
    type Value;

    private!();
    /// Returns a reference to the key of the entry.
    fn key(&self) -> &Self::Key;
    /// Returns a reference to the value of the entry.
    fn value(&self) -> Option<&Self::Value>;
    /// Returns a mutable reference to the value of the entry.
    fn value_mut(&mut self) -> Option<&mut Self::Value>;
    /// if the entry is vacant, insert the given value
    fn or_insert(self, value: Self::Value) -> &'a mut Self::Value;
}
/// The [`RawKeyValue`] trait defines the base interface common to all key-value stores
pub trait RawKeyValue<K, V> {
    /// the entry for the key-value pair
    type Entry<'a>: RawEntry<'a, Key = K, Value = V>
    where
        Self: 'a;

    private!();
    /// returns the [`Entry`] for the given key
    fn entry(&mut self, key: K) -> Self::Entry<'_>;
}
/// the [`KeyValue`] trait extends the [`RawKeyValue`] trait to provide additional methods for
/// manipulating key-value stores.
pub trait KeyValue<K, V>: RawKeyValue<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
}

#[cfg(feature = "std")]
mod impl_std {
    use super::*;
    use core::hash::{BuildHasher, Hash};
    use std::collections::hash_map::{self, HashMap};

    impl<'a, K, V> RawEntry<'a> for hash_map::Entry<'a, K, V>
    where
        K: Eq + Hash,
    {
        type Key = K;
        type Value = V;

        seal!();

        fn key(&self) -> &Self::Key {
            hash_map::Entry::key(self)
        }

        fn value(&self) -> Option<&Self::Value> {
            match self {
                hash_map::Entry::Occupied(entry) => Some(entry.get()),
                hash_map::Entry::Vacant(_) => None,
            }
        }

        fn value_mut(&mut self) -> Option<&mut Self::Value> {
            match self {
                hash_map::Entry::Occupied(entry) => Some(entry.get_mut()),
                hash_map::Entry::Vacant(_) => None,
            }
        }

        fn or_insert(self, value: Self::Value) -> &'a mut Self::Value {
            self.or_insert(value)
        }
    }

    impl<K, V, S> RawKeyValue<K, V> for HashMap<K, V, S>
    where
        K: Eq + Hash,
        S: BuildHasher,
    {
        type Entry<'a>
            = hash_map::Entry<'a, K, V>
        where
            Self: 'a;

        seal!();

        fn entry(&mut self, key: K) -> Self::Entry<'_> {
            HashMap::entry(self, key)
        }
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::*;
    use alloc::collections::btree_map::{self, BTreeMap};

    impl<'a, K, V> RawEntry<'a> for btree_map::Entry<'a, K, V>
    where
        K: Ord,
    {
        type Key = K;
        type Value = V;

        seal!();

        fn key(&self) -> &Self::Key {
            match self {
                btree_map::Entry::Occupied(entry) => entry.key(),
                btree_map::Entry::Vacant(_) => panic!("Cannot get key from a vacant entry"),
            }
        }

        fn value(&self) -> Option<&Self::Value> {
            match self {
                btree_map::Entry::Occupied(entry) => Some(entry.get()),
                btree_map::Entry::Vacant(_) => None,
            }
        }

        fn value_mut(&mut self) -> Option<&mut Self::Value> {
            match self {
                btree_map::Entry::Occupied(entry) => Some(entry.get_mut()),
                btree_map::Entry::Vacant(_) => None,
            }
        }

        fn or_insert(self, value: Self::Value) -> &'a mut Self::Value {
            self.or_insert(value)
        }
    }

    impl<K, V> RawKeyValue<K, V> for BTreeMap<K, V>
    where
        K: Ord,
    {
        type Entry<'a>
            = btree_map::Entry<'a, K, V>
        where
            Self: 'a;

        seal!();

        fn entry(&mut self, key: K) -> Self::Entry<'_> {
            BTreeMap::entry(self, key)
        }
    }
}
