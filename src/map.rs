use crate::StackStructure;
use core::fmt::Debug;
use crate::iterators::{StackStructureIterator, StackStructureIteratorRef, StackStructureIteratorRefMut, StackStructureRChunks};
use crate::simple::{StackSimple};

// FOR THE MAP IN THE SEQUENCE // [ordered] map

#[derive(Debug)]
pub struct StackMap<K: Debug + Ord, V: Debug, const N: usize> {
    ss: StackStructure<(K, V), N>, // keep private
}
impl<K: Debug + Ord, V: Debug, const N: usize> StackMap<K, V, N> {
    pub fn new() -> Self {
        Self {
            ss: StackStructure::new(),
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, ()> { // error if ss is full // some if the value existed there previously {
        // keep the ss in the sort sequence, that is how we find things.
        match self.ss.__binary_search_by_key(&key, |t| { &t.0 }) {
            Ok((_virtual_i, internal_array_i)) => {
                // item is found at this location, set element
                //#[cfg(test)] std::println!("ok {:?}, {:?}", virtual_i, internal_array_i);
                Ok(Some(self.ss.main_memory[internal_array_i].element.replace((key, value)).unwrap().1)) // unwrap safe bc the binary search returned ok with this location
            }
            Err((_virtual_i, None)) => {
                //#[cfg(test)] std::println!("err {:?}, None", virtual_i,);
                self.ss.insert(0, (key, value))
                    .map(|()| None)
            }
            Err((_virtual_i, Some(node_before))) => {
                //#[cfg(test)] std::println!("err {:?}, {:?}", virtual_i, node_before);
                match self.ss.__get_new_node_from_free_list() {
                    None => {
                        return Err(()); 
                    }
                    Some(new_node_i) => {
                        self.ss.main_memory[new_node_i].element = Some((key, value));
                        self.ss.__insert_node_after_node(new_node_i, node_before);
                        self.ss.len += 1;
                        Ok(None)
                    }
                }
            }
        }
    }      
    pub fn get(&self, key: &K) -> Option<&V> {  
        match self.ss.__binary_search_by_key(key, |t| &t.0) {
            Ok((_virtual_i, internal_array_index)) => {
                Some(&self.ss.main_memory[internal_array_index].element.as_ref().unwrap().1) // unwrap safe bc binary-search returned Ok  
            }
            Err(_) => None,
        }
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {  
        match self.ss.__binary_search_by_key(key, |t| &t.0) {
            Ok((_virtual_i, internal_array_index)) => {
                Some(&mut self.ss.main_memory[internal_array_index].element.as_mut().unwrap().1) // unwrap safe bc binary-search returned Ok  
            }
            Err(_) => None,
        }
    }
 
    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.ss.__binary_search_by_key(key, |t| &t.0) {
            Ok((_virtual_i, internal_array_index)) => {
                Some(self.ss.__delete_node(internal_array_index).1)  
            }
            Err(_) => None,
        }
    }
    
    pub fn len(&self) -> usize {
        self.ss.len
    }
    
    pub fn iter<'a>(&'a self) -> core::iter::Map<StackStructureIteratorRef<'a, (K, V), N>, fn(&'a (K, V))->(&'a K, &'a V)> {
        fn i<'a, A, B>(t: &'a (A, B)) -> (&'a A, &'a B) { (&t.0, &t.1) }
        self.ss.iter().map(i as fn(&'a (K, V))->(&'a K, &'a V))
    }
    pub fn iter_mut<'a>(&'a mut self) -> core::iter::Map<StackStructureIteratorRefMut<'a, (K, V), N>, fn(&'a mut (K, V))->(&'a K, &'a mut V)> {
        fn im<'a, A, B>(t: &'a mut (A, B)) -> (&'a A, &'a mut B) { (&t.0, &mut t.1) }
        self.ss.iter_mut().map(im as fn(&'a mut (K, V))->(&'a K, &'a mut V))
    }
    pub fn keys<'a>(&'a self) -> core::iter::Map<StackStructureIteratorRef<'a, (K, V), N>, fn(&'a (K, V))->&'a K> {
        fn k<'a, A, B>(t: &'a (A, B)) -> &'a A { &t.0 }
        self.ss.iter().map(k as fn(&'a (K, V)) -> &'a K)
    }
    pub fn values<'a>(&'a self) -> core::iter::Map<StackStructureIteratorRef<'a, (K, V), N>, fn(&'a (K, V))->&'a V> {
        fn v<'a, A, B>(t: &'a (A, B)) -> &'a B { &t.1 }
        self.ss.iter().map(v as fn(&'a (K, V)) -> &'a V)
    }
    pub fn values_mut<'a>(&'a mut self) -> core::iter::Map<StackStructureIteratorRefMut<'a, (K, V), N>, fn(&'a mut (K, V))->&'a mut V> {
        fn vm<'a, A, B>(t: &'a mut (A, B)) -> &'a mut B { &mut t.1 }
        self.ss.iter_mut().map(vm as fn(&'a mut (K, V)) -> &'a mut V)
    }
    pub fn rchunks<'a, const C: usize>(&'a self) -> core::iter::Map<StackStructureRChunks<'a, (K, V), N, C>, fn(StackSimple<&'a (K, V), C>)->StackSimple<(&'a K, &'a V), C>> {  
        fn i<'a, A, B, const C: usize>(ss: StackSimple<&'a (A, B), C>) -> StackSimple<(&'a A, &'a B), C> { 
            ss.into_iter().map(|t| (&t.0, &t.1)).collect()
        }
        self.ss.rchunks::<C>().map(i as fn(StackSimple<&'a (K, V), C>)->StackSimple<(&'a K, &'a V), C>)        
    }   
  
}



impl<K: Ord + Debug, V: Debug, const N: usize> FromIterator<(K, V)> for StackMap<K, V, N> {
    fn from_iter<Iter: IntoIterator<Item=(K, V)>>(iter: Iter) -> Self {
        // must sort them.
        let mut map = Self::new();
        for t in iter {
            map.insert(t.0, t.1).unwrap(); // will panic if not enough capacity!
        }
        map
    }    
}

impl<K: Ord + Debug, V: Debug, const N: usize> IntoIterator for StackMap<K, V, N> {
    type Item = (K, V);
    type IntoIter = StackStructureIterator<(K, V), N>;
    fn into_iter(self) -> Self::IntoIter {
        self.ss.into_iter()
    }
}
