#![no_std] // portant!

use core::fmt::Debug;
use core::mem::MaybeUninit;

mod iterators;
pub use iterators::*;

mod simple;
pub use simple::*;

mod map;
pub use map::*;

#[cfg(test)] extern crate std;
#[cfg(test)] use std::*;
#[cfg(test)] mod tests;




#[derive(Debug)]
struct Node<T: Debug> {
    element: Option<T>,  // will be None if this node is free //  // not needed for this to be an option but i like it. it can also be default and then we can save the cost of overwriting the bytes but idk.
    prev: Option<usize>, // none if this is the first item
    next: Option<usize>, // none if this is the last item
}

#[derive(Debug)]
pub struct StackStructure<T: Debug, const N: usize> {
    pub(crate) main_memory: [Node<T>; N],
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
    
    fn __get_new_node_from_free_list(&mut self) -> Option<usize/*internal-array-index*/> { // None if full
        match self.free_list {
            None => {
                return None; // full
            }
            Some(new_node_i) => {
                self.free_list = self.main_memory[new_node_i].next;
                if let Some(i) = self.main_memory[new_node_i].next {
                    self.main_memory[i].prev = None; // now this one becomes the first free node // this might not be needed but i like it
                }
                Some(new_node_i)
            }
        }
    }
    
    fn __insert_node_after_node(&mut self, new_node_i: usize, current_node_i: usize) {
        self.main_memory[new_node_i].prev = Some(current_node_i);
        match self.main_memory[current_node_i].next {
            None => {
                // tail
                self.main_memory[new_node_i].next = None;
                self.head_and_tail.as_mut().unwrap().1 = new_node_i;
            }
            Some(next_node_i) => {
                // in the middle
                self.main_memory[new_node_i].next = Some(next_node_i);
                self.main_memory[next_node_i].prev = Some(new_node_i);
            }
        }
        self.main_memory[current_node_i].next = Some(new_node_i);
    }

    
    // optimize to start from tail if len - insertion_index < len / 2
    pub fn insert(&mut self, insertion_index: usize, element: T) -> Result<(), ()> { // err if list is full or if index is out of bounds
        match self.__get_new_node_from_free_list() {
            None => {
                return Err(()); // full
            }
            Some(new_node_i) => {
                
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
                            
                            self.__insert_node_after_node(new_node_i, current_node_i);
                        }
                    }
                }
            }
        }
        self.len += 1;
        Ok(())
    }
    
    pub fn push(&mut self, element: T) -> Result<(), ()> { // err if list is full or if index is out of bounds
        self.insert(self.len, element) // later i can optimize the insert method to start from the tail if the index is closer to len than it is to 0.
    }
    
    fn __delete_node(&mut self, node_to_delete_i: usize) -> T {
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
        self.main_memory[node_to_delete_i].element.take().unwrap()
    }
    
    // optimize to start from tail if len - insertion_index < len / 2
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
                Ok(self.__delete_node(node_to_delete_i))
            }
        }
    }
    
    // optimize to start from tail if len - insertion_index < len / 2
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
    
    // optimize to start from tail if len - insertion_index < len / 2
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
    
    // optimize to start from tail if len - insertion_index < len / 2
    pub fn set(&mut self, set_index: usize, value: T) -> Result<T, ()> { // error if index out of bounds // returns old value
        match self.head_and_tail {
            None => return Err(()), // nothing to get
            Some((head, _tail)) => { // i can optimize this by starting from the tail if get_index > (len/2)
                let mut node_to_get_i = head;
                for _ in 0..set_index {
                    node_to_get_i = match self.main_memory[node_to_get_i].next {
                        None => return Err(()), // index out of bounds
                        Some(i) => i,
                    };
                }
                Ok(self.main_memory[node_to_get_i].element.replace(value).unwrap()) // unwrap is safe here because each element in the list is with a Some value
            }
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
        
    pub(crate) fn __binary_search_by_key<'a, K: Ord, F: Fn(&'a T)->K>(&'a self, key: K, key_of_the_element: F) 
    -> Result<(usize/*virtual-index*/, usize/*internal-array-index*/), (usize/*virtual-index*/, Option<usize>/*None means insert at virtual-index-~0, Some means the node that comes before a potential sorted insert*/)> // ok is the item is found at this location 
    {
        if self.len == 0 {
            return Err((0, None));
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
            
            //#[cfg(test)] println!("main_mem_ptr before compare {:?}", main_mem_ptr);
           
            
            use core::cmp::Ordering;
            match key_of_the_element(self.main_memory[main_mem_ptr].element.as_ref().unwrap()).cmp(&key) { // unwrap because traveling the list is with the lements.
                Ordering::Equal => {
                    //#[cfg(test)] println!("equal {:?}", ());
                    return Ok((mid, main_mem_ptr));
                }
                Ordering::Less => {
                    //#[cfg(test)] println!("less {:?}", ());
                    low = mid + 1; // while loop condition makes sure we don't use it if it goes out of bounds.
                }
                Ordering::Greater => {
                    //#[cfg(test)] println!("greater {:?}", ());
                    high = match mid.checked_sub(1) {
                        Some(good) => good,
                        None => return Err((0, None)),
                    };
                    if high < low {
                        return Err((low, Some(self.main_memory[main_mem_ptr].prev.unwrap())));
                    }
                }
            }
        }
        return Err((low, Some(main_mem_ptr)));
    }
    
    // the sequence of the elements in the list must be sorted already before calling this method. otherwise the result is meaningless.
    pub fn binary_search_by_key<'a, K: Ord, F: Fn(&'a T)->K>(&'a self, key: K, key_of_the_element: F) -> Result<usize, usize>   
    {  
        self.__binary_search_by_key(key, key_of_the_element)
            .map(    |(virtual_i, _)| virtual_i)
            .map_err(|(virtual_i, _)| virtual_i)
    }
        
    pub fn binary_search(&self, key: &T) -> Result<usize, usize> 
    where T: Ord {
        //#[inline_always]
        fn same<T>(t: &T) -> &T { t }
        self.binary_search_by_key(key, same)
    }
        
}
