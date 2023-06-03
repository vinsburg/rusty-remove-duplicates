use std::collections::HashSet;

pub fn remove_duplicate(list: &Vec<i32>) -> Vec<i32>{
    let mut set: HashSet<i32> = HashSet::new();
    let mut new_list: Vec<i32> = Vec::new();

    for &item in list {
        if !set.contains(&item) {
            set.insert(item);
            new_list.push(item);
        }
    }

    new_list
}