use core::ops::*;
use core::slice::SliceIndex;


// use the slice-index syntax to view this VariableSizeImmutableArray &VariableSizeImmutableArray[..] outputs &[T]
// C is the max-size and capacity of the VariableSizeImmutableArray. len is the current len of the valid-items in the VariableSizeImmutableArray.
// this struct takes the place of when need an owned variable sized immutable array on the stack
// using this struct, an array is created on the stack with the max-length and the unused slots are hidden

#[derive(Debug)]
pub struct VariableSizeImmutableArray<T, const C: usize> {
    data: [T; C], // private field portant // space for the data // C must be constant-generic bc size of the stack must be known at compile time
    len: usize, 
}

impl<T, const C: usize> VariableSizeImmutableArray<T, C> {
    pub fn len(&self) -> usize {
        self.len
    }
    // this function will return an error if len > C 
    pub fn new(data: [T; C], len: usize) -> Result<Self, ()> { // error if len > C
        if len > C {
            return Err(());
        }
        Ok(Self {
            data,
            len,
        })
    }
}

impl<T, const C: usize> core::ops::Index<usize> for VariableSizeImmutableArray<T, C> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        &self.data[index]
    }
}
impl<T, const C: usize> core::ops::Index<Range<usize>> for VariableSizeImmutableArray<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: Range<usize>) -> &Self::Output {
        if index.start >= self.len || index.end > self.len {
            panic!("index out of bounds");
        }
        &self.data[index]
    }
}
impl<T, const C: usize> core::ops::Index<RangeFrom<usize>> for VariableSizeImmutableArray<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        if index.start >= self.len {
            panic!("index out of bounds");
        }
        &self.data[index.start..self.len]
    }
}
impl<T, const C: usize> core::ops::Index<RangeTo<usize>> for VariableSizeImmutableArray<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        if index.end > self.len {
            panic!("index out of bounds");
        }
        &self.data[index]
    }
}
impl<T, const C: usize> core::ops::Index<RangeFull> for VariableSizeImmutableArray<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, _index: RangeFull) -> &Self::Output {
        &self.data[..self.len]
    }
}
impl<T, const C: usize> core::ops::Index<RangeInclusive<usize>> for VariableSizeImmutableArray<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        if *index.start() >= self.len || *index.end() >= self.len {
            panic!("index out of bounds");
        }
        &self.data[index]
    }
}
impl<T, const C: usize> core::ops::Index<RangeToInclusive<usize>> for VariableSizeImmutableArray<T, C> {
    type Output = <Range<usize> as SliceIndex<[T]>>::Output;
    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        if index.end >= self.len {
            panic!("index out of bounds");
        }
        &self.data[index]
    }
}
