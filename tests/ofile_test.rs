mod isol;
use isol::*;

use std::fs;
use lazy_db::ofile::*;

#[test]
fn isol_ofile_read() {
    // Set up reading situation
    let tmp = new_env();
    let path = format!("{tmp}/test_file.bin");
    let contents: [u8; 16] = [8, 37, 23, 94, 12, 77, 54, 88, 46, 100, 255, 233, 142, 157, 177, 200];
    fs::write(&path, contents).unwrap();

    // Actual reading
    let mut ofile = OFile::new(path).unwrap();
    for b in contents {
        let read = ofile.read().unwrap();
        assert_eq!(b, read);
    }
}

#[test]
fn isol_ofile_write() {
    // set up needed stuff for test
    let tmp = new_env();
    let path = format!("{tmp}/test_file.bin");
    let contents: [u8; 16] = [8, 37, 23, 94, 12, 77, 54, 88, 46, 100, 255, 233, 142, 157, 177, 200];

    // Actual writing
    let mut ofile = OFile::new(path.clone()).unwrap();
    for b in contents {
        ofile.write(b).unwrap();
    } ofile.finish().unwrap();

    // Test if writing was successful
    let new_contents = fs::read(path).unwrap();
    for (i, b) in contents.iter().enumerate() {
        println!("og: {}, read: {}", b, new_contents[i]);
        assert_eq!(*b, new_contents[i]);
    }
}

#[test]
fn isol_ofile_modify() {
    // set up stuff for test
    let tmp = new_env();
    let path = format!("{tmp}/to_modify.bin");
    let original = [1u8, 12, 32, 48, 96, 34, 87, 26];
    let expected: Vec<u8> = original.iter().map(|b| b * 2).collect();

    // write initial
    fs::write(&path, original).unwrap();

    // Actual modification
    let mut ofile = OFile::new(path.clone()).unwrap();
    for _ in 0..original.len() {
        let read = ofile.read().unwrap();
        // println!("{read}");
        ofile.write(read * 2).unwrap();
    }; ofile.finish().unwrap();

    // Test if writing was successful
    let new_contents = fs::read(path).unwrap();
    // new_contents.iter().for_each(|b| println!("{b}"));
    for (i, b) in expected.iter().enumerate() {
        println!("expected: {}, read: {}", b, new_contents[i]);
        assert_eq!(*b, new_contents[i]);
    }
}

#[test]
fn isol_ofile_modify_in_depth() {
    // set up stuff for test
    let tmp = new_env();
    let path = format!("{tmp}/to_modify.bin");
    let original = [1u8, 12, 32, 48, 96, 34, 87, 26];
    let expected = [1u8, 12, 64, 96, 192, 68, 174, 52];

    // write initial
    fs::write(&path, original).unwrap();

    // Actual modification
    let mut ofile = OFile::new(path.clone()).unwrap();
    for i in 0..original.len() {
        let read = ofile.read().unwrap();
        // println!("{read}");
        if i > 1 { ofile.write(read * 2).unwrap() };
    }; ofile.finish().unwrap();

    // Test if writing was successful
    let new_contents = fs::read(path).unwrap();
    // new_contents.iter().for_each(|b| println!("{b}"));
    for (i, b) in expected.iter().enumerate() {
        println!("expected: {}, read: {}", b, new_contents[i]);
        assert_eq!(*b, new_contents[i]);
    }
}