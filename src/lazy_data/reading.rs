use super::*;

macro_rules! incorrect_type {
    ($original:expr, $expected:pat) => {
        if let $expected = $original {}
        else {
            return Err(LDBError::IncorrectType($original, stringify!($expected).to_string()));
        };
    }
}

impl LazyData {
    /// ### Lazy Action
    /// ( Only returns internal ofile field of `Lazy Data` )
    /// 
    /// ---
    /// Collects the `LazyData` as a Lazy `OFile`.
    /// 
    /// Returns `LDBError::IncorrectType` if the LazyData type is not `LazyType::Binary`
    pub fn collect_ofile(self) -> Result<OFile, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::Binary);
        Ok(self.ofile)
    }

    /// ### Expensive Action
    /// ( Loads the entire file's data into memory )
    /// 
    /// ---
    /// Collects the `LazyData` as a `String`
    /// 
    /// Returns `LDBError::IncorrectType` if LazyData type is not `LazyType::String`
    /// Returns `LDBError::IOError` if there is an io error while reading file contents
    pub fn collect_string(self) -> Result<String, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::String);
        // Expensive and best to be avoided if possible
        let bytes = unwrap_result!(self.ofile.to_bytes() => |e| if let OFileError::IOError(e) = e { Err(LDBError::IOError(e)) } else { panic!("OFile read should only return an IO error") });
        
        if let Ok(x) = String::from_utf8(bytes.to_vec()) {
            Ok(x)
        } else {
            Err(LDBError::InvalidUTF8String(bytes))
        }
    }
}