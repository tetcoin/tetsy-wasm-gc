use std::error;
use std::fmt;
use tetsy_wasm::elements::Error as TetsyWasmError;

/// The error type for garbage collecting webassembly bytecode.
#[derive(Debug)]
pub struct Error(TetsyWasmError);

impl error::Error for Error {
    fn description(&self) -> &str {
        "webassembly garbage collection failed"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub fn from(tetsy: TetsyWasmError) -> Error {
    Error(tetsy)
}
