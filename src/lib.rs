use std::collections::HashSet;

pub fn remove_duplicate(list: &mut Vec<i32>) {
    let mut set: HashSet<i32> = HashSet::new();
    let mut index_list: Vec<usize> = Vec::new();

    for (index, &item) in list.iter().enumerate() {
        if !set.contains(&item) {
            set.insert(item);
        } else {
            index_list.push(index);
        }
    }

    while index_list.len() > 0 {
        let index: usize = index_list.pop().unwrap();
        list.remove(index);
    }
}