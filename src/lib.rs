//! > **A simple, bare-bones and lazily loaded database for small projects**
//! 
//! ## Concepts
//! This will be quite short as `lazy-db` is *bare-bones*.
//! - ### `LazyDB`
//!     - An obstraction of a `LazyContainer` that makes sure that incompatible databases won't be corrupted by incompatible versions of `lazy-db`
//!     - The actual 'Lazy Database'
//! - ### `LazyData`
//!     - Binary files that only support the primative types; nothing more nothing less
//! - ### `LazyContainer`
//!     - A collection of `LazyData`, think of it like an object from `OOP` or a directory in a file system
//!     - An abstaction of the underlying filesystem directory
//! 
//! ## Examples
//! ### Some basic usage
//! Here is a really basic `LazyDB` that holds some information about a hypothetical person named *'Dave'*
//! ```rust
//! let path = "example_db"; // path to the database
//! let database = LazyDB::init_db(path).unwrap(); // initialise the database
//! 
//! // Writing to the database with a concise macro
//! // The individual containers are separated by `/` while the `LazyData` is separted with `::`.
//! // The assigning `=` sign indicates the `LazyData` that is being written to the path
//! // The function after the `=` sign is formatted like this: new_<primative_type>
//! write_database!((&database) /people/Dave::fav_colour = new_string("Blue")).unwrap();
//! write_database!((&database) /people/Dave::age = new_u8(21)).unwrap();
//! write_database!((&database) /people/Dave::unemployed = new_bool(true)).unwrap();
//! 
//! // Reading from the database with a concise macro
//! // Same path as before
//! let fav_colour: String = search_database!((&database) /people/Dave::fav_colour).unwrap();
//! let age: u8 = search_database!((&database) /people/Dave::age).unwrap();
//! let unemployed: bool = search_database!((&database) /people/Dave::unemployed).unwrap();
//! ```

pub mod error;
pub mod lazy_type;
pub mod lazy_data;
pub mod version;
pub mod lazy_database;
pub mod lazy_container;
pub mod lazy_trait;
mod lazy_archive;

// Prelude
pub use crate::{
    error::LDBError,
    lazy_type::*,
    lazy_data::*,
    lazy_database::*,
    lazy_container::*,
    lazy_trait::*,
};

pub const VERSION: version::Version = version::Version::new(0, 1, 0);

#[macro_export]
macro_rules! const_eval {
    (($type:ty) $code:expr) => {{
        const RESULT: $type = $code;
        RESULT
    }};
}

#[macro_export]
macro_rules! unwrap_result {
    ($result:expr => $wrapper:expr) => {{
        let result = $result;
        if let Err(e) = result {
            return $wrapper(e);
        } result.unwrap()
    }}
}