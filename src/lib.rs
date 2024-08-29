#![no_std] // portant!

use core::fmt::Debug;
use core::mem::MaybeUninit;

mod iterators;
pub use iterators::*;



#[cfg(test)] extern crate std;
#[cfg(test)] use std::*;



#[derive(Debug)]
struct Node<T: Debug> {
    element: Option<T>,  // will be None if this node is free //  // not needed for this to be an option but i like it. it can also be default and then we can save the cost of overwriting the bytes but idk.
    prev: Option<usize>, // none if this is the first item
    next: Option<usize>, // none if this is the last item
}

#[derive(Debug)]
pub struct StackStructure<T: Debug, const N: usize> {
    main_memory: [Node<T>; N],
    head_and_tail: Option<(usize, usize)>,      // None if list is empty// index into the main_memory
    free_list: Option<usize>,                   // points to the first free node. None if list is full.
    len: usize,
}
impl<T: Debug, const N: usize> StackStructure<T, N> {
    pub fn new() -> Self {
        Self {
            main_memory: {
                let mut m: [MaybeUninit<Node<T>>; N] = [const { MaybeUninit::uninit() }; N];
                for (i, element) in (&mut m[..]).into_iter().enumerate() {
                    element.write(
                        Node{
                            element: None,
                            prev: if i == 0   { None } else { Some(i-1) },
                            next: if i == N-1 { None } else { Some(i+1) },
                        }
                    );
                }
                //unsafe { core::mem::transmute::<_, [Node<T>; N]>(m) } // https://github.com/rust-lang/rust/issues/62875
                let done = unsafe { core::ptr::read((&m as *const [MaybeUninit<Node<T>>; N]).cast::<[Node<T>; N]>()) };
                core::mem::forget(m);
                done
            },
            head_and_tail: None,
            free_list: Some(0),
            len: 0,
        }
    }
    
    pub fn insert(&mut self, insertion_index: usize, element: T) -> Result<(), ()> { // err if list is full or if index is out of bounds
        match self.free_list {  // .clone()
            None => {
                return Err(()); // full
            }
            Some(new_node_i) => {
                
                self.free_list = self.main_memory[new_node_i].next;
                if let Some(i) = self.main_memory[new_node_i].next {
                    self.main_memory[i].prev = None; // now this one becomes the first free node // this might not be needed but i like it
                }
                
                self.main_memory[new_node_i].element = Some(element);
                
                match self.head_and_tail {
                    None => {
                        if insertion_index != 0 {
                            return Err(()); // out of bounds
                        }
                        self.head_and_tail = Some((new_node_i, new_node_i));
                        self.main_memory[new_node_i].prev = None; // i think it will always be already None since we using first free index.
                        self.main_memory[new_node_i].next = None;
                    }
                    Some((head, _tail)) => {
                        
                        if insertion_index == 0 {
                            self.main_memory[new_node_i].next = Some(head);
                            self.main_memory[new_node_i].prev = None;
                            self.main_memory[head].prev = Some(new_node_i);
                            self.head_and_tail.as_mut().unwrap().0 = new_node_i; // update the head since we are inserting at the begining
                        
                        } else {
                            let mut current_node_i = head;
                            for _ in 0..insertion_index-1 {
                                match self.main_memory[current_node_i].next {
                                    None => return Err(()), // out of bounds
                                    Some(next_i) => {
                                        current_node_i = next_i;
                                    }
                                }
                            }
                            
                            self.main_memory[new_node_i].prev = Some(current_node_i);
                            
                            match self.main_memory[current_node_i].next {
                                None => {
                                    // tail
                                    self.main_memory[new_node_i].next = None;
                                    self.main_memory[current_node_i].next = Some(new_node_i);
                                    self.head_and_tail.as_mut().unwrap().1 = new_node_i;
                                }
                                Some(next_node_i) => {
                                    // in the middle
                                    self.main_memory[new_node_i].next = Some(next_node_i);
                                    self.main_memory[current_node_i].next = Some(new_node_i);
                                    self.main_memory[next_node_i].prev = Some(new_node_i);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.len += 1;
        Ok(())
    }
    
    pub fn delete(&mut self, deletion_index: usize) -> Result<T, ()> { // error if index out of bounds 
        match self.head_and_tail {
            None => return Err(()), // nothing to delete
            Some((head, _tail)) => {
                let mut node_to_delete_i = head;
                for _ in 0..deletion_index {
                    node_to_delete_i = match self.main_memory[node_to_delete_i].next {
                        None => return Err(()), // index out of bounds
                        Some(i) => i,
                    };
                }
                match self.main_memory[node_to_delete_i].prev {
                    Some(prev_i) => {
                        self.main_memory[prev_i].next = self.main_memory[node_to_delete_i].next;
                    }
                    None => {
                        // node-to-delete is the head so we need to set a new head 
                        match self.main_memory[node_to_delete_i].next {
                            Some(next_i) => {
                                self.head_and_tail.as_mut().unwrap().0 = next_i;
                            }
                            None => {
                                self.head_and_tail = None;
                            }
                        }
                    }
                }
    
                match self.main_memory[node_to_delete_i].next {
                    Some(next_i) => {
                        self.main_memory[next_i].prev = self.main_memory[node_to_delete_i].prev;
                    }
                    None => { 
                        match self.main_memory[node_to_delete_i].prev {
                            Some(prev_i) => {
                                self.head_and_tail.as_mut().unwrap().1 = prev_i;
                            }
                            None => {
                                self.head_and_tail = None;
                            }
                        }
                    }
                }
    
                self.main_memory[node_to_delete_i].next = self.free_list;
                self.free_list = Some(node_to_delete_i);
                
                self.len -= 1;
                Ok(core::mem::take(&mut self.main_memory[node_to_delete_i].element).unwrap())
            }
        }
    }
    
    pub fn get(&self, get_index: usize) -> Result<&T, ()> { // error if index out of bounds
        match self.head_and_tail {
            None => return Err(()), // nothing to get
            Some((head, _tail)) => { // i can optimize this by starting from the tail if get_index > (len/2)
                let mut node_to_get_i = head;
                for _ in 0..get_index {
                    node_to_get_i = match self.main_memory[node_to_get_i].next {
                        None => return Err(()), // index out of bounds
                        Some(i) => i,
                    };
                }
                Ok(self.main_memory[node_to_get_i].element.as_ref().unwrap()) // unwrap is safe here because each element in the list is with a Some value
            }
        }
    }
    
    pub fn get_mut(&mut self, get_index: usize) -> Result<&mut T, ()> { // err if index out of bounds
        match self.head_and_tail {
            None => return Err(()), // nothing to get
            Some((head, _tail)) => { // i can optimize this by starting from the tail if get_index > (len/2)
                let mut node_to_get_i = head;
                for _ in 0..get_index {
                    node_to_get_i = match self.main_memory[node_to_get_i].next {
                        None => return Err(()), // index out of bounds
                        Some(i) => i,
                    };
                }
                Ok(self.main_memory[node_to_get_i].element.as_mut().unwrap()) // unwrap is safe here because each element in the list is with a Some value
            }
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn capacity(&self) -> usize {
        N
    }
    
    pub fn iter<'a>(&'a self) -> StackStructureIteratorRef<'a, T, N> {
        StackStructureIteratorRef{
            ms: self,
            current_node_i: self.head_and_tail.map(|(head, _tail)| head),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> StackStructureIteratorRefMut<'a, T, N> {
        StackStructureIteratorRefMut{
            current_node_i: self.head_and_tail.map(|(head, _tail)| head),
            ms: self,
        }
    }
    
    pub const fn memory_size() -> usize {
        core::mem::size_of::<StackStructure<T, N>>()
    }
    
    // the sequence of the elements in the list must be sorted already before calling this method. otherwise the result is meaningless.
    pub fn binary_search_by_key<'a, K: Ord, F: Fn(&'a T)->K>(&'a self, key: &K, key_of_the_element: F) -> Result<usize, usize> {
        if self.len == 0 {
            return Err(0);
        }
        
        let mut low: usize = 0;
        let mut high: usize = self.len - 1;
        
        // for the ficiency gains so we don't have to traverse the whole list from the begining on each loop
        // the index into the main-memory-array
        let mut main_mem_ptr: usize = self.head_and_tail.as_ref().unwrap().0; // we already returned if self.len == 0
        // the (virtual) index into the user-facing list.
        let mut main_mem_ptr_index: usize = 0;
        
        while low <= high {
            let mid: usize = (low + high) / 2;
            
            let placement_difference: isize = (mid as isize) - (main_mem_ptr_index as isize);
            
            let travel: &mut dyn FnMut(&Node<T>)->Option<usize> = if placement_difference >= 0 {
                &mut |node: &Node<T>| { main_mem_ptr_index += 1; node.next }
            } else {
                &mut |node: &Node<T>| { main_mem_ptr_index -= 1; node.prev }
            };
            
            for _ in 0..placement_difference.abs() {
                main_mem_ptr = travel(&(self.main_memory[main_mem_ptr])).unwrap(); // we are not going out of bounds here. we use the self.len as the starting highd
            }
            
            use core::cmp::Ordering;
            match key_of_the_element(self.main_memory[main_mem_ptr].element.as_ref().unwrap()).cmp(key) { // unwrap because traveling the list is with the lements.
                Ordering::Equal => {
                    return Ok(mid);
                }
                Ordering::Less => {
                    #[cfg(test)] println!("less {:?}", ());
                    low = mid + 1; // while loop condition makes sure we don't use it if it goes out of bounds.
                }
                Ordering::Greater => {
                    #[cfg(test)] println!("greater {:?}", ());
                    high = match mid.checked_sub(1) {
                        Some(good) => good,
                        None => return Err(0),
                    };
                }
            }
        }
        return Err(low);
    }
    
    pub fn binary_search<'a>(&self, key: &T) -> Result<usize, usize> 
    where T: Ord {
        self.binary_search_by_key(&key, |e| { e })
    }
        
}



#[cfg(test)]
mod tests {
    
    
    use super::*;
    
    #[test]
    fn test_1() {
        let mut ms = StackStructure::<u64, 5>::new();
        ms.insert(0, 0);
        ms.insert(1, 1);
        
        println!("{:?}", ms.get(1));
        ms.insert(1, 3);
        println!("{:?}", ms.get(1));
        println!("{:?}", ms.get(2));
    
        *ms.get_mut(1).unwrap() += 1;
        
        
        
        println!("len: {:?}", ms.len());
        println!("ms: {:?}", ms);
        
        for item in ms.iter() {
            println!("item: {:?}", item);        
        }
        
        for item in ms.iter_mut() {
            *item += 1;
            println!("item: {:?}", item);        
        }
        
        for item in ms {
            println!("item: {:?}", item);        
        }
        
        let ms2 = StackStructure::<_, 2>::from_iter([
            "hi",
            "there",        
        ]);
        
        println!("{:?}", StackStructure::<[u8; 200], 100000>::memory_size());  // on a 64-bit-platform this is the sum of the size of the lements + N*40 .
                                                                                // 20,000,000 + 4,000,000
                                            
                                                                                
                                                                                
        let ss = StackStructure::<_, 5>::from_iter([
            "c",
        ]);
                                                                                
                                                                                
        let search_output = ss.binary_search(&"b");
        println!("search_output: {:?}", search_output);
    }

}
