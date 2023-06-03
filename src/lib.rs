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

    for index in index_list {
        list.remove(index);
    }
}