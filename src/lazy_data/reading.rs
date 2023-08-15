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
}