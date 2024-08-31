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
    
    println!("{:?}", core::mem::size_of::<StackStructure::<[u8; 200], 100000>>());  // on a 64-bit-platform this is the sum of the size of the lements + N*40 .
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
        
    println!("numbers: {:?}", numbers);
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
    let x = StackSimple::from_array([1,2,3,4,5]);
    println!("chunk RangeFull: {:?}", &x[..]);
    
    let mut iter = x.into_iter();
    for _ in 0..3 {
        let v = iter.next_back().unwrap();
        println!("{:?}", v);
        if v == 3 {
            break;
        }
    }
    for i in iter {
        println!("{:?}", i);
    }
    
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


#[test]
fn test_map() {
    let mut map = StackMap::<_, _, 40>::from_iter([
        (0, "hi"),
        (1, "tell"),
    ]);
    
    
    map.insert(4, "three").unwrap();
    map.insert(3, "three").unwrap();
    map.insert(5, "three").unwrap();
    map.insert(2, "two").unwrap();
    for i in map.iter() {
        println!("i: {:?}", i);
    }
    //println!("map: {:?}", &map);
    
    for i in 0..5 {
        map.insert(i, "loop").unwrap();
    }
    for i in map.iter() {
        println!("i: {:?}", i);
    }
    
    for i in map.values_mut() {
        *i = "change";
    }
    
    for i in map.iter() {
        println!("i: {:?}", i);
    }
    
    println!("map len: {:?}", map.len());

    
    map.remove(&600);
    
    for i in map.iter() {
        println!("i: {:?}", i);
    }
    println!("map len: {:?}", map.len());

}


#[test]
fn test_simple() {
    let mut x = StackSimple::from_array([1,2,5,6,8]);
    
    for chunk in x.rchunks(3) {
        println!("{:?}", chunk);
    }
    
    x[3] += 1;
    
    let mut iter = x.rchunks(3); 
    
    for _ in 0..iter.len() {
        println!("chunks left: {:?}", iter.len());
        println!("{:?}", iter.next().unwrap());
    }
    

    
}

#[test]
fn test_map_rchunks() {
    let x = StackMap::<u64, u64, 10>::from_iter([
        (0,0),
        (1,1),
        (2,2),
        (3,3),
        (4,4)
    ]);
    
    for c in x.rchunks::<2>() {
        println!("{:?}", &c[..]);
    }
    
}
