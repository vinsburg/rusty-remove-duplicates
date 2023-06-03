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

    let mut index_of_current_item = index_list.len();
    while index_of_current_item > 0 {
        index_of_current_item -= 1;
        let index = index_list[index_of_current_item];
        list.remove(index);
    }
}