mod isol;
use isol::*;

use lazy_db::*;

#[test]
fn lazy_data_new_void() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_void.ld");
    // Create void file
    LazyData::new_void(&path).unwrap();
    // Load void file
    let lazy_data = LazyData::load(path).unwrap();
    // Type must be void
    assert_eq!(lazy_data.lazy_type, LazyType::Void);
}

#[test]
fn lazy_data_string() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_string.ld");
    // Create string file
    let og_string = String::from("Hello world!");
    LazyData::new_string(&path, &og_string).unwrap();
    // Load string file
    let new_string = LazyData::load(path).unwrap().collect_string().unwrap();
    // String must be the same
    assert_eq!(og_string, new_string);
}