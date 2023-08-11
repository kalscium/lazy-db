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

#[test]
fn lazy_data_signed() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_signed.ld");
    // Create i32 file
    let og_i32 = 34563435i32;
    LazyData::new_i32(&path, og_i32).unwrap();
    // Load i32 file
    let new_i32 = LazyData::load(path).unwrap().collect_i32().unwrap();
    // Two values must be the same
    assert_eq!(og_i32, new_i32);
}

#[test]
fn lazy_data_unsigned() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_unsigned.ld");
    // Create i32 file
    let og_u32 = 34563435u32;
    LazyData::new_u32(&path, og_u32).unwrap();
    // Load i32 file
    let new_u32 = LazyData::load(path).unwrap().collect_u32().unwrap();
    // Two values must be the same
    assert_eq!(og_u32, new_u32);
}

#[test]
fn lazy_data_f32() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_f32.ld");
    // Create f32 file
    let og_f32 = 123.123f32;
    LazyData::new_f32(&path, og_f32).unwrap();
    // Load f32 file
    let new_f32 = LazyData::load(path).unwrap().collect_f32().unwrap();
    // Two values must be the same
    assert_eq!(og_f32, new_f32);
}

#[test]
fn lazy_data_f64() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_f64.ld");
    // Create f32 file
    let og_f64 = 123.2345345123f64;
    LazyData::new_f64(&path, og_f64).unwrap();
    // Load f32 file
    let new_f64 = LazyData::load(path).unwrap().collect_f64().unwrap();
    // Two values must be the same
    assert_eq!(og_f64, new_f64);
}

#[test]
fn lazy_data_binary() {
    let tmp = new_env();
    let path = tmp.get_path().join("new_binary.ld");
    // Create binary file
    let og_bin = Box::new([12u8, 234, 48, 128]);
    LazyData::new_binary(&path, og_bin.as_ref()).unwrap();
    // Load binary file
    let new_bin = LazyData::load(path).unwrap().collect_binary().unwrap();
    // Two values must be the same
    assert_eq!(*og_bin, *new_bin);
}