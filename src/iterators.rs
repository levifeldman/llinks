use super::*;


pub struct StackStructureIteratorRef<'a, T: Debug, const N: usize> {
    pub(crate) ms: &'a StackStructure<T, N>,
    pub(crate) current_node_i: Option<usize>,  // none if there are no more items
}
impl<'a, T: Debug, const N: usize> Iterator for StackStructureIteratorRef<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node_i {
            None => return None,
            Some(current_node_i) => {
                let item = self.ms.main_memory[current_node_i].element.as_ref().unwrap();
                self.current_node_i = self.ms.main_memory[current_node_i].next;
                Some(item)
            } 
        }
    }
}

pub struct StackStructureIteratorRefMut<'a, T: Debug, const N: usize> {
    pub(crate) ms: &'a mut StackStructure<T, N>,
    pub(crate) current_node_i: Option<usize>,  // none if there are no more items
}
impl<'a, T: Debug, const N: usize> Iterator for StackStructureIteratorRefMut<'a, T, N> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_node_i {
            None => return None,
            Some(current_node_i) => {
                let item = self.ms.main_memory[current_node_i].element.as_mut().unwrap();
                self.current_node_i = self.ms.main_memory[current_node_i].next;
                unsafe { Some(&mut *(item as *mut T)) }
            } 
        }
    }
}

pub struct StackStructureIterator<T: Debug, const N: usize> {
    pub(crate) ms: StackStructure<T, N>,
}
impl<T: Debug, const N: usize> Iterator for StackStructureIterator<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.ms.len() {
            0 => None,
            _ => Some(self.ms.delete(0).unwrap()),
        }
    }
}

impl<T: Debug, const N: usize> IntoIterator for StackStructure<T, N> {
    type Item = T;
    type IntoIter = StackStructureIterator<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        StackStructureIterator{
            ms: self
        }
    }
} 

impl<T: Debug, const N: usize> FromIterator<T> for StackStructure<T, N> {
    fn from_iter<GenericIterator: IntoIterator<Item=T>>(iter: GenericIterator) -> Self {
        let mut ms = Self::new();
        for item in iter {
            ms.insert(ms.len(), item).expect("size of the StackStructure must be big enough for this iterator"); // will panic if size is not big enough, or maybe silent error and break?
        }
        ms
    }
    
}
