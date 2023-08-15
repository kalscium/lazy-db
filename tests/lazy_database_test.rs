mod isol;
use isol::*;
use lazy_db::*;

#[test]
fn lazy_database_database() {
    let tmp = new_env();
    let path = tmp.get_path().join("database");
    let og_string = String::from("Hello world!");

    // Writing to the database
    let database = LazyDB::init(&path).unwrap();
    write_database!((database) data = new_string(&og_string)).unwrap(); // Writes to database with macro

    // Read from the database
    let database = LazyDB::load_dir(path).unwrap();
    let new_string = search_database!((database) ("data")).unwrap().collect_string().unwrap(); // Reads from database with macro

    // Must be equal
    assert_eq!(og_string, new_string);
}

#[test]
fn lazy_database_compile() {
    let tmp = new_env();
    let path = tmp.get_path().join("database");
    let og_string = String::from("Hello world!");

    // Writing to the database and compiling
    let database = LazyDB::init_db(path).unwrap();
    write_database!((&database) ("data") = new_string(&og_string)).unwrap(); // Writes to database with macro
    let path = database.compile().unwrap();

    // Read from the database
    let database = LazyDB::load_db(path).unwrap();
    let new_string = search_database!((database) data).unwrap().collect_string().unwrap(); // Reads from database with macro

    // Must be equal
    assert_eq!(og_string, new_string);
}

#[test]
fn lazy_database_compile_nested() {
    let tmp = new_env();
    let path = tmp.get_path().join("database");
    let og_string = String::from("Hello world!");

    // Writing to the database and compiling
    let database = LazyDB::init_db(path).unwrap();
    write_database!((&database) /("nested")::data = new_string(&og_string)).unwrap(); // Writes to database with macro
    let path = database.compile().unwrap();

    // Read from the database
    let database = LazyDB::load_db(path).unwrap();
    let new_string = search_database!((database) /nested::("data")).unwrap().collect_string().unwrap(); // Reads from database with macro

    // Must be equal
    assert_eq!(og_string, new_string);
}

fn _lazy_database_stress_test() {
    let tmp = new_env();
    let path = tmp.get_path().join("stressed_database");
    let database = LazyDB::init_db(path).unwrap();

    for i in 0..1000000u32 {
        write_database!((&database) /root::(i.to_string()) = new_u64(gen_random())).unwrap();
    } database.compile().unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
}