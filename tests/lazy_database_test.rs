mod isol;
use isol::*;
use lazy_db::*;

#[test]
fn lazy_data_database() {
    let tmp = new_env();
    let path = tmp.get_path().join("database");
    let og_string = String::from("Hello world!");

    { // Writing to the database
        let database = LazyDB::init(&path).unwrap();
        let file = database.as_container().unwrap().data_writer("data").unwrap(); // Get file-wrapper
        LazyData::new_string(file, &og_string).unwrap(); // Writes string
    }

    // Read from the database
    let database = LazyDB::load_dir(path).unwrap();
    let new_string = database.as_container().unwrap().read_data("data").unwrap().collect_string().unwrap();

    // Must be equal
    assert_eq!(og_string, new_string);
}

#[test]
fn lazy_data_compile() {
    let tmp = new_env();
    let mut path = tmp.get_path().join("database");
    let og_string = String::from("Hello world!");

    { // Writing to the database and compiling
        let database = LazyDB::init_db(&path).unwrap();
        let file = database.as_container().unwrap().data_writer("data").unwrap(); // Get file-wrapper
        LazyData::new_string(file, &og_string).unwrap(); // Writes string
        let database = database.compile().unwrap();
        path = database;
    }

    // Read from the database
    let database = LazyDB::load_db(path).unwrap();
    let new_string = database.as_container().unwrap().read_data("data").unwrap().collect_string().unwrap();

    // Must be equal
    assert_eq!(og_string, new_string);
}

#[test]
fn lazy_data_compile_nested() {
    let tmp = new_env();
    let mut path = tmp.get_path().join("database");
    let og_string = String::from("Hello world!");

    { // Writing to the database and compiling
        let database = LazyDB::init_db(&path).unwrap();
        let file = database.as_container().unwrap().new_container("nested").unwrap().data_writer("data").unwrap(); // Get file-wrapper
        LazyData::new_string(file, &og_string).unwrap(); // Writes string
        let database = database.compile().unwrap();
        path = database;
    }

    // Read from the database
    let database = LazyDB::load_db(path).unwrap();
    let new_string = database.as_container().unwrap().read_container("nested").unwrap().read_data("data").unwrap().collect_string().unwrap();

    // Must be equal
    assert_eq!(og_string, new_string);
}