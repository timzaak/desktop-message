pub mod server;
mod mqtt;
mod http;
pub mod discovery; // Made public for api_discovery

use std::fmt::Display;



#[repr(C)]
#[derive(Debug)]
pub enum ErrorCode { // Already public, no change needed here based on instruction
    Ok = 0,
    BadConfig = 1,
    StartServerError = 2,
    InvalidServerPoint = 3,
    ServerHasInit = 4,
    MDNSInitFailure = 5,
    OutOfAllocatedBounds = 6,
}
impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Changed to {:?} to use Debug derive, as per typical Display for enums. Or keep as self.to_string() if specific string representations are defined. For now, using Debug.
    }
}
