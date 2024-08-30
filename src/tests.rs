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


#[test]
fn test_iter() {

    let numbers = StackStructure::<_, 6>::from_iter([1, 2, 3, 4, 5, 6]);
        
    let mut iter = numbers.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(Some(&4), iter.next());
    assert_eq!(Some(&5), iter.next());
    assert_eq!(Some(&6), iter.next());
    assert_eq!(None, iter.next());
    
    let mut iter = numbers.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&6), iter.next_back());
    assert_eq!(Some(&5), iter.next_back());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    assert_eq!(Some(&4), iter.next());
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next_back());
    
}


#[test]
fn test_chunk() {
    let x = VariableSizeImmutableArray::new([1,2,3,4,5], 4).unwrap();
    println!("chunk RangeFull: {:?}", &x[..]);
}

#[test]
fn test_rchunks() {
    let x = StackStructure::<_, 5>::from_iter([1,2,3,4,5]);
    let rchunks = x.rchunks::<2>();
    for chunk in rchunks {
        println!("{:?}", &chunk[..]);
        for item in &chunk[..1] {
            println!("{:?}", item);
        }
    }
}
