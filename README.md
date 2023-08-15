# lazy-db
A simple, bare-bones and lazily loaded database for small projects

## Examples
---
### Some basic usage
Here is a really basic `LazyDB` that holds some information about a hypothetical person named *'Dave'*
```rust
use lazy_db::*;

let path = "example_db"; // path to the database
let database = LazyDB::init_db(path).unwrap(); // initialise the database

// Writing to the database with a concise macro
// The individual containers are separated by `/` while the `LazyData` is separted with `::`.
// The assigning `=` sign indicates the `LazyData` that is being written to the path
// The function after the `=` sign is formatted like this: new_<primative_type>
write_database!((&database) /people/Dave::fav_colour = new_string("Blue")).unwrap();
write_database!((&database) /people/Dave::age = new_u8(21)).unwrap();
write_database!((&database) /people/Dave::unemployed = new_bool(true)).unwrap();

// Reading from the database with a concise macro
// Same path as before
let fav_colour: String = search_database!((&database) /people/Dave::fav_colour).unwrap();
let age: u8 = search_database!((&database) /people/Dave::age).unwrap();
let unemployed: bool = search_database!((&database) /people/Dave::unemployed).unwrap();
```