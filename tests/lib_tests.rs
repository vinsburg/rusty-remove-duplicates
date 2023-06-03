#![cfg_attr(feature = "strict", deny(warnings))]

use std::vec::Vec;
use remove_duplicate;

#[test]
fn test_empty_list() {
    let list: Vec<i32> = Vec::new();
    let read_list: Vec<i32> = remove_duplicate::remove_duplicate(&list);
    assert_eq!(read_list, list);
}


#[test]
fn test_two_member_list() {
    let list: Vec<i32> = vec![1, 2];
    let read_list: Vec<i32> = remove_duplicate::remove_duplicate(&list);
    assert_eq!(read_list, list);
}


#[test]
fn test_multiple_duplicates() {
    let list: Vec<i32> = vec![1, 1, 2, 2, 3, 3];
    let expected_list: Vec<i32> = vec![1, 2, 3];
    let read_list: Vec<i32> = remove_duplicate::remove_duplicate(&list);
    assert_eq!(read_list, expected_list);
}