use super::*;
use core::iter::{
    DoubleEndedIterator,
    FusedIterator,
    Extend,
};



pub struct StackStructureIterator<T: Debug, const N: usize> {
    ms: StackStructure<T, N>,
}

pub struct StackStructureIteratorRef<'a, T: Debug, const N: usize> {
    ms: &'a StackStructure<T, N>,
    current_nodes_i_forward_and_backward: Option<(usize, usize)>,  // none if there are no more items
    number_of_items_served: usize,
}

pub struct StackStructureIteratorRefMut<'a, T: Debug, const N: usize> {
    ms: &'a mut StackStructure<T, N>,
    current_nodes_i_forward_and_backward: Option<(usize, usize)>,  // none if there are no more items
    number_of_items_served: usize,
}



impl<T: Debug, const N: usize> StackStructure<T, N> {
    pub fn iter<'a>(&'a self) -> StackStructureIteratorRef<'a, T, N> {
        StackStructureIteratorRef{
            ms: self,
            current_nodes_i_forward_and_backward: self.head_and_tail,
            number_of_items_served: 0,
        }
    }
    pub fn iter_mut<'a>(&'a mut self) -> StackStructureIteratorRefMut<'a, T, N> {
        StackStructureIteratorRefMut{
            current_nodes_i_forward_and_backward: self.head_and_tail,
            ms: self,
            number_of_items_served: 0,
        }
    }
}

impl<T: Debug, const N: usize> FromIterator<T> for StackStructure<T, N> {
    fn from_iter<Iter: IntoIterator<Item=T>>(iter: Iter) -> Self {
        let mut ms = Self::new();
        for item in iter {
            ms.push(item).expect("The size of the StackStructure must be set big enough for this iterator"); // will panic if size is not big enough, or maybe silent error and break?
        }
        ms
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

impl<T: Debug, const N: usize> Iterator for StackStructureIterator<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.ms.len() {
            0 => None,
            _ => Some(self.ms.delete(0).unwrap()),
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) { // hint for the remaining length of the iterator
        (self.ms.len(), Some(self.ms.len()))
    }
}

impl<'a, T: Debug, const N: usize> Iterator for StackStructureIteratorRef<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_nodes_i_forward_and_backward {
            None => return None,
            Some((ref mut forward_i, ref backward_i)) => {
                let item = self.ms.main_memory[*forward_i].element.as_ref().unwrap();
                if forward_i == backward_i {
                    self.current_nodes_i_forward_and_backward = None;
                } else {
                    *forward_i = self.ms.main_memory[*forward_i].next.unwrap(); // unwrap bc this scope only happens if forward_i != backward_i so there will always be a Some(prev) in this specific scope  
                }
                self.number_of_items_served += 1;
                Some(item)
            } 
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.ms.len() - self.number_of_items_served;
        (len, Some(len))
    }
}

impl<'a, T: Debug, const N: usize> Iterator for StackStructureIteratorRefMut<'a, T, N> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current_nodes_i_forward_and_backward {
            None => return None,
            Some((ref mut forward_i, ref backward_i)) => {
                let item = self.ms.main_memory[*forward_i].element.as_mut().unwrap();
                if forward_i == backward_i {
                    self.current_nodes_i_forward_and_backward = None;
                } else {
                    *forward_i = self.ms.main_memory[*forward_i].next.unwrap(); // unwrap bc this scope only happens if forward_i != backward_i so there will always be a Some(prev) in this specific scope  
                }
                self.number_of_items_served += 1;
                unsafe { Some(&mut *(item as *mut T)) }
            } 
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.ms.len() - self.number_of_items_served;
        (len, Some(len))
    }
}

impl<T: Debug, const N: usize> DoubleEndedIterator for StackStructureIterator<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.ms.len() {
            0 => None,
            _ => Some(self.ms.delete(self.ms.len() - 1).unwrap()),
        }
    }
}

impl<'a, T: Debug, const N: usize> DoubleEndedIterator for StackStructureIteratorRef<'a, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_nodes_i_forward_and_backward {
            None => return None,
            Some((ref forward_i, ref mut backward_i)) => {
                let item = self.ms.main_memory[*backward_i].element.as_ref().unwrap();
                if forward_i == backward_i {
                    self.current_nodes_i_forward_and_backward = None;
                } else {
                    *backward_i = self.ms.main_memory[*backward_i].prev.unwrap(); // unwrap bc this scope only happens if forward_i != backward_i so there will always be a Some(prev) in this specific scope  
                }
                self.number_of_items_served += 1;
                Some(item)
            } 
        }        
    }
}

impl<'a, T: Debug, const N: usize> DoubleEndedIterator for StackStructureIteratorRefMut<'a, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.current_nodes_i_forward_and_backward {
            None => return None,
            Some((ref forward_i, ref mut backward_i)) => {
                let item = self.ms.main_memory[*backward_i].element.as_mut().unwrap();
                if forward_i == backward_i {
                    self.current_nodes_i_forward_and_backward = None;
                } else {
                    *backward_i = self.ms.main_memory[*backward_i].prev.unwrap(); // unwrap bc this scope only happens if forward_i != backward_i so there will always be a Some(prev) in this specific scope  
                }
                self.number_of_items_served += 1;
                unsafe { Some(&mut *(item as *mut T)) }
            } 
        }        
    }
}

// must implement the size_hint Iterator method on the Iterator plementations for these structs, the ExactSizeIterator::len method uses the size_hint iterator method and the low and high must be the same
impl<T: Debug, const N: usize> ExactSizeIterator for StackStructureIterator<T, N> {}
impl<'a, T: Debug, const N: usize> ExactSizeIterator for StackStructureIteratorRef<'a, T, N> {}
impl<'a, T: Debug, const N: usize> ExactSizeIterator for StackStructureIteratorRefMut<'a, T, N> {}


impl<'a, T: Debug, const N: usize> FusedIterator for StackStructureIterator<T, N> {}
impl<'a, T: Debug, const N: usize> FusedIterator for StackStructureIteratorRef<'a, T, N> {}
impl<'a, T: Debug, const N: usize> FusedIterator for StackStructureIteratorRefMut<'a, T, N> {}

impl<T: Debug, const N: usize> Extend<T> for StackStructure<T, N> {
    fn extend<Iter: IntoIterator<Item=T>>(&mut self, iter: Iter) {
        for item in iter {
            self.push(item);
        }
    }
}



pub struct RChunks<'a, T: Debug, const N: usize, const C: usize> {
    iterator: StackStructureIteratorRef<'a, T, N>,
    default_value: T,
}
impl<'a, T: Debug, const N: usize, const C: usize> Iterator for RChunks<'a, T, N, C> {
    type Item = VariableSizeImmutableArray<&'a T, C>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator.len() == 0 {
            return None;
        }
        let mut chunk_data: [MaybeUninit<&T>; C] = [const { MaybeUninit::uninit() }; C];
        let chunk_len: usize = core::cmp::min(self.iterator.len(), C);
        for i in 0..chunk_len {
            chunk_data[chunk_data.len() - 1 - i - (C - chunk_len)].write(
                self.iterator.next_back().unwrap() // unwrap cause we checked that the chunk_len is never greater than the iterator len
            );
        }
        for i in chunk_len..C {
            chunk_data[i].write(&self.default_value);
        }
        //unsafe { core::mem::transmute::<_, [Node<T>; N]>(m) } // https://github.com/rust-lang/rust/issues/62875
        let done = unsafe { core::ptr::read((&chunk_data as *const [MaybeUninit<&T>; C]).cast::<[&'a T; C]>()) };
        core::mem::forget(chunk_data);
        
        Some(VariableSizeImmutableArray::new(done, chunk_len).unwrap()) // unwrap ok bc we set chunk_len to the min(C, iterator.len()). 
        
    }
}

impl<'a, T: Debug + Default, const N: usize> StackStructure<T, N> {
    pub fn rchunks<const C: usize>(&'a self) -> RChunks<'a, T, N, C> {
        RChunks::<'a, T, N, C>{
            iterator: self.iter(),
            default_value: T::default(),
        }
    }
    /*
    pub fn rchunks_mut<const C: usize>(&self) -> RChunksMut<'a, T, N, C> {
        
    }
    */
}
