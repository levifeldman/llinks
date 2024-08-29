// no-std

use core::fmt::Debug;
use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct MainStructure<T: Debug, const N: usize> {
    main_memory: [Node<T>; N],
    head_and_tail: Option<(usize, usize)>,      // None if list is empty// index into the main_memory
    free_list: Option<usize>,                   // points to the first free node. None if list is full.
    len: usize,
}
impl<T: Debug, const N: usize> MainStructure<T, N> {
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
                    Some((head, tail)) => {
                        
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
            Some((head, tail)) => {
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
            Some((head, tail)) => {
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
            Some((head, tail)) => {
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
    
    pub fn iter<'a>(&'a self) -> MainStructureIteratorRef<'a, T, N> {
        MainStructureIteratorRef{
            ms: self,
            current_node_i: self.head_and_tail.map(|(head, tail)| head),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> MainStructureIteratorRefMut<'a, T, N> {
        MainStructureIteratorRefMut{
            current_node_i: self.head_and_tail.map(|(head, tail)| head),
            ms: self,
        }
    }
    
}

#[derive(Debug)]
struct Node<T: Debug> {
    element: Option<T>,  // will be None if this node is free //  // not needed for this to be an option but i like it. it can also be default and then we can save the cost of overwriting the bytes but idk.
    prev: Option<usize>, // none if this is the first item
    next: Option<usize>, // none if this is the last item
}


struct MainStructureIteratorRef<'a, T: Debug, const N: usize> {
    ms: &'a MainStructure<T, N>,
    current_node_i: Option<usize>,  // none if there are no more items
}
impl<'a, T: Debug, const N: usize> Iterator for MainStructureIteratorRef<'a, T, N> {
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

struct MainStructureIteratorRefMut<'a, T: Debug, const N: usize> {
    ms: &'a mut MainStructure<T, N>,
    current_node_i: Option<usize>,  // none if there are no more items
}
impl<'a, T: Debug, const N: usize> Iterator for MainStructureIteratorRefMut<'a, T, N> {
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

pub struct MainStructureIterator<T: Debug, const N: usize> {
    ms: MainStructure<T, N>,
}
impl<T: Debug, const N: usize> Iterator for MainStructureIterator<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.ms.len() {
            0 => None,
            _ => Some(self.ms.delete(0).unwrap()),
        }
    }
}

impl<T: Debug, const N: usize> IntoIterator for MainStructure<T, N> {
    type Item = T;
    type IntoIter = MainStructureIterator<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        MainStructureIterator{
            ms: self
        }
    }
} 

impl<T: Debug, const N: usize> FromIterator<T> for MainStructure<T, N> {
    fn from_iter<GenericIterator: IntoIterator<Item=T>>(iter: GenericIterator) -> Self {
        let mut ms = Self::new();
        for item in iter {
            ms.insert(ms.len(), item);
        }
        ms
    }
    
}


#[test]
fn test_1() {
    let mut ms = MainStructure::<u64, 5>::new();
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

}
