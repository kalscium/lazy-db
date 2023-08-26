use super::*;

macro_rules! incorrect_type {
    ($original:expr, $expected:pat) => {
        if let $expected = $original {}
        else {
            return Err(LDBError::IncorrectType($original, stringify!($expected).to_string()));
        };
    }
}

macro_rules! collect_number {
    (($name:ident) $type:ty = $lazy_type:pat) => {
        /// ### Expensive Action
        /// ( Loads the entire file's data into memory  )
        /// 
        /// ---
        /// Collects the `LazyData` as an unsigned integer.
        /// 
        /// Returns `LDBError::IncorrectType` if the LazyData type is not the correct unsigned integer
        pub fn $name(self) -> Result<$type, LDBError> {
            incorrect_type!(self.lazy_type, $lazy_type);

            // Expensive and best to be avoided if possible
            let bytes = self.wrapper.read_to_end()?;
            const LENGTH: usize = <$type>::BITS as usize / 8usize;

            // Check if the size is correct
            if bytes.len() != LENGTH {
                return Err(LDBError::InvalidNumberByteLength(bytes.len() as u8,
                    stringify!($lazy_type).to_string()))
            };

            // Convert to number
            let value = <$type>::from_be_bytes(unsafe { *(bytes.as_ptr() as *const [u8; LENGTH]) });

            Ok(value)
        }
    };

    (signed ($name:ident) $type:ty = $lazy_type:pat) => {
        /// ### Expensive Action
        /// ( Loads the entire file's data into memory  )
        /// 
        /// ---
        /// Collects the `LazyData` as a signed integer.
        /// 
        /// Returns `LDBError::IncorrectType` if the LazyData type is not the correct signed integer
        pub fn $name(self) -> Result<$type, LDBError> {
            incorrect_type!(self.lazy_type, $lazy_type);

            // Expensive and best to be avoided if possible
            let bytes = self.wrapper.read_to_end()?;
            const LENGTH: usize = <$type>::BITS as usize / 8usize;

            // Check if the size is correct
            if bytes.len() != LENGTH {
                return Err(LDBError::InvalidNumberByteLength(bytes.len() as u8,
                    stringify!($lazy_type).to_string()))
            };

            // Convert to number
            let value = <$type>::from_be_bytes(unsafe { *(bytes.as_ptr() as *const [u8; LENGTH]) });

            Ok(value)
        }
   }
}

macro_rules! collect_array {
    (($name:ident) $type:ty = $lazy_type:ident) => {
        collect_array!(($name, <$type>::BITS as usize / 8usize) $type = $lazy_type);
    };

    (($name:ident, $bytes:expr) $type:ty = $lazy_type:ident) => {
        /// ### Expensive Action
        /// ( Loads the entire file's data into memory  )
        /// 
        /// ---
        /// Collects the `LazyData` as an array of values of a single type
        /// 
        /// Returns `LDBError::IncorrectType` if the LazyData type is not the correct array type
        pub fn $name(mut self) -> Result<Box<[$type]>, LDBError> {
            incorrect_type!(self.lazy_type, LazyType::Array);

            // Read array-type
            let array_type =
                LazyType::try_from(self.wrapper.read(1)?[0])?;
            incorrect_type!(array_type, LazyType::$lazy_type);

            const LENGTH: usize = $bytes;
            let mut result = Vec::<$type>::new();
            loop {
                let bytes = match self.wrapper.read_opt(LENGTH)? {
                    Some(x) => x,
                    None => break,
                };
                // Convert to number
                let value = <$type>::from_be_bytes(unsafe { *(bytes.as_ptr() as *const [u8; LENGTH]) });
                result.push(value);
            }

            Ok(result.into_boxed_slice())
        }
    };
}

impl LazyData {
    /// ### Expensive Action
    /// ( Reads all of the contents of the file and stores it on the heap )
    /// 
    /// ---
    /// Collects the `LazyData` as a `Box<[u8]>`.
    /// 
    /// Returns `LDBError::IncorrectType` if the LazyData type is not `LazyType::Binary`
    pub fn collect_binary(self) -> Result<Box<[u8]>, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::Binary);
        self.wrapper.read_to_end()
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
        let bytes = self.wrapper.read_to_end()?;
        
        if let Ok(x) = String::from_utf8(bytes.to_vec()) {
            Ok(x)
        } else {
            Err(LDBError::InvalidUTF8String(bytes))
        }
    }
    
    // Unsigned numbers
    collect_number!((collect_u8) u8 = LazyType::U8);
    collect_number!((collect_u16) u16 = LazyType::U16);
    collect_number!((collect_u32) u32 = LazyType::U32);
    collect_number!((collect_u64) u64 = LazyType::U64);
    collect_number!((collect_u128) u128 = LazyType::U128);

    // Signed numbers
    collect_number!(signed (collect_i8) i8 = LazyType::I8);
    collect_number!(signed (collect_i16) i16 = LazyType::I16);
    collect_number!(signed (collect_i32) i32 = LazyType::I32);
    collect_number!(signed (collect_i64) i64 = LazyType::I64);
    collect_number!(signed (collect_i128) i128 = LazyType::I128);

    // Arrays
    collect_array!((collect_u8_array) u8 = U8);
    collect_array!((collect_u16_array) u16 = U16);
    collect_array!((collect_u32_array) u32 = U32);
    collect_array!((collect_u64_array) u64 = U64);
    collect_array!((collect_u128_array) u128 = U128);
    collect_array!((collect_i8_array) i8 = I8);
    collect_array!((collect_i16_array) i16 = I16);
    collect_array!((collect_i32_array) i32 = I32);
    collect_array!((collect_i64_array) i64 = I64);
    collect_array!((collect_i128_array) i128 = I128);
    collect_array!((collect_f32_array, 4) f32 = F32);
    collect_array!((collect_f64_array, 8) f64 = F64);

    /* Floating point numbers */

    /// ### Expensive Action
    /// ( Loads the entire file's data into memory  )
    /// 
    /// ---
    /// Collects the `LazyData` as an `f32`.
    /// 
    /// Returns `LDBError::IncorrectType` if the LazyData type is not ``LazyFloat::F32`
    pub fn collect_f32(self) -> Result<f32, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::F32);

        // Expensive and best to be avoided if possible
        let bytes = self.wrapper.read_to_end()?;

        // Check if the size is correct
        if bytes.len() != 4 {
            return Err(LDBError::InvalidNumberByteLength(bytes.len() as u8,
                stringify!($lazy_type).to_string()))
        };

        // Convert to number
        let value = f32::from_be_bytes(unsafe { *(bytes.as_ptr() as *const [u8; 4]) });

        Ok(value)
    }

    /// ### Expensive Action
    /// ( Loads the entire file's data into memory  )
    /// 
    /// ---
    /// Collects the `LazyData` as an `f64`.
    /// 
    /// Returns `LDBError::IncorrectType` if the LazyData type is ``LazyFloatType::F64`
    pub fn collect_f64(self) -> Result<f64, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::F64);

        // Expensive and best to be avoided if possible
        let bytes = self.wrapper.read_to_end()?;

        // Check if the size is correct
        if bytes.len() != 8 {
            return Err(LDBError::InvalidNumberByteLength(bytes.len() as u8,
                stringify!($lazy_type).to_string()))
        };

        // Convert to number
        let value = f64::from_be_bytes(unsafe { *(bytes.as_ptr() as *const [u8; 8]) });

        Ok(value)
    }

    /// ### Inexpensive Action
    /// ( Just reads the type field of `LazyData` )
    /// 
    /// ---
    /// Collects the `LazyData` as a boolean
    /// 
    /// Returns `LDBError::IncorrectType` if the type is not of boolean
    pub fn collect_bool(self) -> Result<bool, LDBError> {
        match self.lazy_type {
            LazyType::True => Ok(true),
            LazyType::False => Ok(false),
            _ => Err(LDBError::IncorrectType(self.lazy_type, String::from("Boolean"))),
        }
    }

    /// ### Inexpensive Action
    /// ( Loads `LazyData` specified at path )
    /// 
    /// ---
    /// Collects the `LazyData` as a path and converts that into another `LazyData`.
    pub fn collect_link(self, database: LazyDB) -> Result<LazyData, LDBError> {
        incorrect_type!(self.lazy_type, LazyType::Link);

        // Loads string as a path
        // Expensive and best to be avoided if possible
        let bytes = self.wrapper.read_to_end()?;
        
        let string = if let Ok(x) = String::from_utf8(bytes.to_vec()) {
            x
        } else {
            return Err(LDBError::InvalidUTF8String(bytes))
        };

        database.as_container()?.read_data(string)
    }
}