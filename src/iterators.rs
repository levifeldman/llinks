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
            match ms.push(item) {
                Ok(()) => {},
                Err(()) => break,
            }
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
            self.push(item).unwrap(); // will panic if not enough room!
        }
    }
}



pub struct StackStructureRChunks<'a, T: Debug, const N: usize, const C: usize> {
    iterator: StackStructureIteratorRef<'a, T, N>,
}
impl<'a, T: Debug, const N: usize, const C: usize> Iterator for StackStructureRChunks<'a, T, N, C> {
    type Item = StackSimple<&'a T, C>;
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
        Some(unsafe { StackSimple::from_maybe_uninit_data_and_len(chunk_data, chunk_len) }) // unsafe ok bc we just wrote to the first len items in the chunk_data
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut number_of_chunks_left = self.iterator.len() / C;
        if self.iterator.len() % C != 0 {
            number_of_chunks_left += 1;
        }
        (number_of_chunks_left, Some(number_of_chunks_left))
    }
}
impl<'a, T: Debug, const N: usize, const C: usize> ExactSizeIterator for StackStructureRChunks<'a, T, N, C> {}
impl<'a, T: Debug, const N: usize, const C: usize> FusedIterator for StackStructureRChunks<'a, T, N, C> {}


impl<'a, T: Debug, const N: usize> StackStructure<T, N> {
    // this method creates a reference for each element in the chunk at the same time. don't create large chunks.
    // the chunk size is reserved at compile time on the stack though. and the size is the size of a reference times the number of references in the chunk.
    pub fn rchunks<const C: usize>(&'a self) -> StackStructureRChunks<'a, T, N, C> {
        StackStructureRChunks::<'a, T, N, C>{
            iterator: self.iter(),
        }
    }
}
