#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CodeRange {
    /// Start byte of the range.
    pub start: u32,

    /// End byte of the range.
    pub end: u32,

    /// Address of the source code to which the range applies.
    ///
    /// Stored as only the address to improve the performance of hashing.
    pub address: usize,
}

impl CodeRange {
    pub fn new(start: u32, end: u32, src: &str) -> Self {
        let raw_ptr = src as *const str;
        let thin_ptr = raw_ptr as *const u8;
        let address = thin_ptr as usize;
        Self {
            start,
            end,
            address,
        }
    }

    /// Returns whether the code range applies to the given source code.
    pub fn applies_to(&self, source: &str) -> bool {
        let raw_ptr = source as *const str;
        let thin_ptr = raw_ptr as *const u8;
        let address = thin_ptr as usize;
        self.address == address
    }
}
