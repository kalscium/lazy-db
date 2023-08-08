use super::*;

impl LazyData {
    /// ### **Lazy Action**
    /// ( Only returns internal ofile field of `Lazy Data` )
    /// 
    /// ---
    /// Collects the `LazyData` as a Lazy `OFile`.
    /// 
    /// Returns `LDBError::IncorrectType` if the LazyData type is not `LazyType::Binary`
    pub fn collect_ofile(self) -> Result<OFile, LDBError> {
        if self.lazy_type != LazyType::Binary { return Err(LDBError::IncorrectType(self.lazy_type, LazyType::Binary)) }
        Ok(self.ofile)
    }
}