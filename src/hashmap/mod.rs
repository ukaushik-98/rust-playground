use std::{
    cell::Cell,
    collections::HashMap,
    iter::zip,
    sync::{Arc, Mutex},
    thread,
};

// returning a hashmap doesn't work because the data is expected to outlive the bounds of the function
// instead of the map being a ref to vec, ownership could be passed - see foo3
fn foo(keys: Vec<&str>, data: Vec<Vec<i32>>) /* ->  HashMap<String, &Vec<i32>> */
{
    let mut data_map: HashMap<String, &Vec<i32>> = HashMap::new();
    for (i, x) in zip(keys, &data) {
        let key = format!("key_{}", i);
        data_map.insert(key, x);
    }
    for (key, value) in data_map.iter() {
        println!("{}: {:?}", key, value)
    }
    // data_map
}

// returning hashmap of refs by relying on parameterized vecs
// notice the lifetimes
fn foo2<'a>(keys: &'a Vec<&str>, data: &'a Vec<Vec<i32>>) -> HashMap<&'a str, &'a Vec<i32>> {
    let mut data_map: HashMap<&'a str, &'a Vec<i32>> = HashMap::new();
    for (i, x) in zip(keys, data) {
        data_map.insert(i, x);
    }
    data_map
}

fn foo3(keys: Vec<&str>, data: Vec<Vec<i32>>) -> HashMap<String, Vec<i32>> {
    let mut data_map: HashMap<String, Vec<i32>> = HashMap::new();
    for (i, x) in zip(keys, data) {
        let key = format!("key_{}", i);
        data_map.insert(key, x);
    }
    data_map
}

// fn foo4() {
//     let mut keys = vec!["key1", "key2", "key3"];
//     let ref_keys = &mut keys;
//     for k in keys.iter() {}
//     ref_keys.push("key5");
// }

pub mod test {
    use super::*;

    #[test]
    fn foo_test() {
        let keys = vec!["key1", "key2", "key3"];
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        foo(keys, data);
    }

    #[test]
    fn foo2_test() {
        let keys = vec!["key1", "key2", "key3"];
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let data_map = foo2(&keys, &data);
        for (k, v) in data_map.iter() {
            println!("{}: {:?}", k, v)
        }
    }

    #[test]
    fn foo3_test() {
        let keys = vec!["key1", "key2", "key3"];
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let data_map = foo3(keys, data);
        for (k, v) in data_map.iter() {
            println!("{}: {:?}", k, v)
        }
    }
}
