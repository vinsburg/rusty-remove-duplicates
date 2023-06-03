#![cfg_attr(feature = "strict", deny(warnings))]

use std::vec::Vec;
use remove_duplicate;

#[test]
fn test_empty_list() {
    let list: Vec<i32> = Vec::new();
    let read_list: Vec<i32> = remove_duplicate::remove_duplicate(&list);
    assert_eq!(read_list, list);
}