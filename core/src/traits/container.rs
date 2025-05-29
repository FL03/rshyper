/*
    appellation: container <module>
    authors: @FL03
*/

pub trait KeyValue {
    type Key;
    type Value;
}

pub trait RawData {
    type Item;

    fn as_ptr(&self) -> *const Self::Item;

    fn as_mut_ptr(&mut self) -> *mut Self::Item;
}

pub trait RawStore {
    type Data<T>: RawData<Item = T>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
