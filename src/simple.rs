use core::ops::*;
use core::slice::SliceIndex;
use core::mem::MaybeUninit;
use core::iter::{FusedIterator, ExactSizeIterator};

// use the slice-index syntax to view this StackSimple &StackSimple[..] outputs &[T]
// C is the max-size and capacity of the StackSimple. len is the current len of the valid-items in the StackSimple.
// this struct takes the place of when need an owned variable sized immutable array on the stack
// using this struct, an array is created on the stack with the max-length and the unused slots are hidden

#[derive(Debug)]
pub struct StackSimple<T, const C: usize> {
    data: [MaybeUninit<T>; C], // private field portant // space for the data // C must be constant-generic bc size of the stack must be known at compile time
    len: usize, 
}

impl<T, const C: usize> StackSimple<T, C> {
    pub fn new() -> Self { 
        Self {
            data: [const { MaybeUninit::uninit() }; C],
            len: 0,
        }
    }
    // if you want the stack-simple to have bigger capacity then the length of the given array, then use StackSimple::<T, C/*set capacity here*/>::from_iter(array) but make sure capacity is greater than the length of the array.
    pub fn from_array(a: [T; C]) -> Self {
        Self {
            len: a.len(),
            data: array_as_maybeuninit(a),            
        }
    }
    /// caller must make sure that the first len number of items is initialized with maybeuninit.write and caller must make sure that the remaining elements are not initialized yet.
    /// len must be <= data.len()
    pub(crate) unsafe fn from_maybe_uninit_data_and_len(data: [MaybeUninit<T>; C], len: usize) -> Self {
        Self {
            data,
            len
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn push(&mut self, value: T) -> Result<(), ()> { // err if full
        if self.len == C {
            return Err(());
        }
        self.data[self.len].write(value);
        self.len += 1;
        Ok(())
    }
    pub fn pop(&mut self) -> Option<T> { // some with the last element if the list was not empty before this call
        if self.len == 0 {
            return None;
        }
        
        let mut y = MaybeUninit::uninit();
        
        core::mem::swap(&mut self.data[self.len - 1] , &mut y);
        
        self.len -= 1;
        Some(unsafe { y.assume_init() }) //unsafe ok bc the len tells us where the valid values are. 
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.data[index].as_ptr() }) // unsafe ok bc we checked the index is within range
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &mut *self.data[index].as_mut_ptr() }) // unsafe ok bc we checked the index is within range
    }
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        (&self[..]).into_iter()
    }
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        (&mut self[..]).into_iter()
    }
    // panics if chunk_size == 0
    pub fn rchunks(&self, chunk_size: usize) -> StackSimpleRChunks<'_, T, C> {
        StackSimpleRChunks::new(self, chunk_size)
    }
}


impl<T, const C: usize> FromIterator<T> for StackSimple<T, C> {
    fn from_iter<Iter: IntoIterator<Item=T>>(iter: Iter) -> Self {
        let mut simple = Self::new();
        for item in iter {
            match simple.push(item) {
                Ok(()) => {},
                Err(()) => break,
            }
        }
        simple
    }    
}

// into-iterator

pub struct StackSimpleIterator<T, const C: usize> {
    s: StackSimple<T, C>,
    number_of_items_uninitialized_at_the_begining: usize,
}
impl<T, const C: usize> Iterator for StackSimpleIterator<T, C> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.s.len - self.number_of_items_uninitialized_at_the_begining == 0 {
            return None;
        }
        let mut y = MaybeUninit::uninit();
        core::mem::swap(&mut self.s.data[0 + self.number_of_items_uninitialized_at_the_begining], &mut y);
        self.number_of_items_uninitialized_at_the_begining += 1;
        Some(unsafe { y.assume_init() })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.s.len - self.number_of_items_uninitialized_at_the_begining;
        (len, Some(len))
    }
}
impl<T, const C: usize> DoubleEndedIterator for StackSimpleIterator<T, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.s.len - self.number_of_items_uninitialized_at_the_begining == 0 {
            return None;
        }
        Some(self.s.pop().unwrap()) // unwrap cause we checked there is at least one item
    }
}
impl<T, const C: usize> ExactSizeIterator for StackSimpleIterator<T, C> {}
impl<T, const C: usize> FusedIterator     for StackSimpleIterator<T, C> {}

impl<T, const C: usize> IntoIterator for StackSimple<T, C> {
    type Item = T;
    type IntoIter = StackSimpleIterator<T, C>;
    fn into_iter(self) -> Self::IntoIter {
        StackSimpleIterator{
            s: self,
            number_of_items_uninitialized_at_the_begining: 0,
        }
    }
} 

// rchunks

pub struct StackSimpleRChunks<'a, T, const C: usize> {
    s: &'a StackSimple<T, C>,
    chunk_size: usize,
    number_of_items_served: usize,
}
impl<'a, T, const C: usize> StackSimpleRChunks<'a, T, C> {
    pub fn new(s: &'a StackSimple<T, C>, chunk_size: usize) -> Self {
        if chunk_size == 0 {
            panic!("chunk size cannot be 0");
        }
        Self {
            s,
            chunk_size,
            number_of_items_served: 0,
        }
    }
}
impl<'a, T, const C: usize> Iterator for StackSimpleRChunks<'a, T, C> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let end = self.s.len - self.number_of_items_served;
        if end == 0 {
            return None;
        }
        let start = end.saturating_sub(self.chunk_size);
        self.number_of_items_served += end - start;
        Some(&self.s[start..end])
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let number_of_items_left = self.s.len - self.number_of_items_served;
        let mut number_of_chunks_left = number_of_items_left / self.chunk_size;
        if number_of_items_left % self.chunk_size != 0 {
            number_of_chunks_left += 1;
        }
        (number_of_chunks_left, Some(number_of_chunks_left))
    }

}

impl<'a, T, const C: usize> ExactSizeIterator for StackSimpleRChunks<'a, T, C> {}
impl<'a, T, const C: usize> FusedIterator for StackSimpleRChunks<'a, T, C> {}




// extend
impl<T, const C: usize> Extend<T> for StackSimple<T, C> {
    fn extend<Iter: IntoIterator<Item=T>>(&mut self, iter: Iter) {
        for item in iter {
            self.push(item).unwrap(); // will panic if not enough room!
        }
    }
}



// index traits

impl<T, const C: usize> core::ops::Index<usize> for StackSimple<T, C> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data[index].as_ptr() } // unsafe ok bc we checked the index is within range
    }
}
impl<T, const C: usize> core::ops::Index<Range<usize>> for StackSimple<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        if index.start >= self.len || index.end > self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_ref(&self.data[index]) }
    }
}
impl<T, const C: usize> core::ops::Index<RangeFrom<usize>> for StackSimple<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        if index.start >= self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_ref(&self.data[index.start..self.len]) }
    }
}
impl<T, const C: usize> core::ops::Index<RangeTo<usize>> for StackSimple<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        if index.end > self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_ref(&self.data[index]) }
    }
}
impl<T, const C: usize> core::ops::Index<RangeFull> for StackSimple<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, _index: RangeFull) -> &Self::Output {
        unsafe { slice_assume_init_ref(&self.data[..self.len]) }
    }
}
impl<T, const C: usize> core::ops::Index<RangeInclusive<usize>> for StackSimple<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        if *index.start() >= self.len || *index.end() >= self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_ref(&self.data[index]) }
    }
}
impl<T, const C: usize> core::ops::Index<RangeToInclusive<usize>> for StackSimple<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        if index.end >= self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_ref(&self.data[index]) }
    }
}


// indexmut


impl<T, const C: usize> core::ops::IndexMut<usize> for StackSimple<T, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data[index].as_mut_ptr() } // unsafe ok bc we checked the index is within range
    }
}
impl<T, const C: usize> core::ops::IndexMut<Range<usize>> for StackSimple<T, C> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        if index.start >= self.len || index.end > self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_mut(&mut self.data[index]) }
    }
}
impl<T, const C: usize> core::ops::IndexMut<RangeFrom<usize>> for StackSimple<T, C> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        if index.start >= self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_mut(&mut self.data[index.start..self.len]) }
    }
}
impl<T, const C: usize> core::ops::IndexMut<RangeTo<usize>> for StackSimple<T, C> {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        if index.end > self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_mut(&mut self.data[index]) }
    }
}
impl<T, const C: usize> core::ops::IndexMut<RangeFull> for StackSimple<T, C> {
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        unsafe { slice_assume_init_mut(&mut self.data[..self.len]) }
    }
}
impl<T, const C: usize> core::ops::IndexMut<RangeInclusive<usize>> for StackSimple<T, C> {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        if *index.start() >= self.len || *index.end() >= self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_mut(&mut self.data[index]) }
    }
}
impl<T, const C: usize> core::ops::IndexMut<RangeToInclusive<usize>> for StackSimple<T, C> {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        if index.end >= self.len {
            panic!("index out of bounds");
        }
        unsafe { slice_assume_init_mut(&mut self.data[index]) }
    }
}







// taken from nightly https://doc.rust-lang.org/1.80.1/src/core/mem/maybe_uninit.rs.html#967
pub const unsafe fn slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    // SAFETY: casting `slice` to a `*const [T]` is safe since the caller guarantees that
    // `slice` is initialized, and `MaybeUninit` is guaranteed to have the same layout as `T`.
    // The pointer obtained is valid since it refers to memory owned by `slice` which is a
    // reference and thus guaranteed to be valid for reads.
    unsafe { &*(slice as *const [MaybeUninit<T>] as *const [T]) }
}
pub unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    // SAFETY: similar to safety notes for `slice_get_ref`, but we have a
    // mutable reference which is also guaranteed to be valid for writes.
    unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) }
}

// https://stackoverflow.com/a/78699200
/*
fn slice_as_maybeuninit<'a, T>(s: &'a [T]) -> &'a [MaybeUninit<T>] {
    // SAFETY:
    //  - `MaybeUninit<T>` is guaranteed to have the same layout as `T`.
    //  - Slices with compatible elements layout have compatible layout,
    //    since slices are have the same layout as the backing array
    //    and array lay all elements consecutively.
    //  - It is always safe to treat initialized values as possibly-initialized.
    unsafe { &*(s as *const [T] as *const [MaybeUninit<T>]) }
}
*/
fn array_as_maybeuninit<T, const C: usize>(a: [T; C]) -> [MaybeUninit<T>; C] {
    // can't use transmute bc const generic size issue// https://github.com/rust-lang/rust/issues/62875
    let d = unsafe { core::ptr::read((&a as *const [T; C]).cast::<[MaybeUninit<T>; C]>()) };
    core::mem::forget(a);
    d
}
