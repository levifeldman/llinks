use crate::StackStructure;
use core::fmt::Debug;



// FOR THE MAP IN THE SEQUENCE // [ordered] map

pub struct StackMap<K: Debug + Ord, V: Debug, const N: usize> {
    ss: StackStructure<(K, V), N>, // keep private
}
impl<K: Debug + Ord, V: Debug, const N: usize> StackMap<K, V, N> {
    fn new() -> Self {
        Self {
            ss: StackStructure::new(),
        }
    }
    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, ()> { // error if ss is full // some if the value existed there previously {
        // keep the ss in the sort sequence, that is how we find things.
        // this impl is not the most possible ficient but it is good for the now.
        match self.ss.binary_search_by_key(&&key, |t| { &t.0 }) {
            Ok(i) => {
                let old = self.ss.set(i, (key, value)).unwrap(); // set returns the old // unwrap bc i is what we got back from binary-search as an Ok
                Ok(Some(old.1))
            }
            Err(i) => {
                self.ss.insert(i, (key, value))?;
                Ok(None)
            }
        }
    }
}
