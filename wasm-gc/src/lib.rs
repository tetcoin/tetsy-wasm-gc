extern crate parity_wasm;
#[macro_use]
extern crate log;
extern crate rustc_demangle;

mod gc;
mod error;

use std::path::Path;
use parity_wasm::elements::{
    Module,
    Serialize,
    Deserialize
};

use gc::garbage_collect;
pub use error::Error;

/// Garbage collects the webassembly bytecode from `input_path` and saves it to `output_path`.
pub fn garbage_collect_file<I, O>(input_path: I, output_path: O) -> Result<(), Error>
where
    I: AsRef<Path>,
    O: AsRef<Path>,
{
    _gc_file(input_path.as_ref(), output_path.as_ref())
}

fn _gc_file(input: &Path, output: &Path) -> Result<(), Error> {
    let mut module = parity_wasm::deserialize_file(input).map_err(error::from)?;
    garbage_collect(&mut module);
    parity_wasm::serialize_to_file(output, module).map_err(error::from)?;

    Ok(())
}

/// Garbage collects given webassembly bytecode.
pub fn garbage_collect_slice(mut bytecode: &[u8]) -> Result<Vec<u8>, Error> {
    let mut module = Module::deserialize(&mut bytecode).map_err(error::from)?;
    garbage_collect(&mut module);

    let mut output = Vec::new();
    module.serialize(&mut output).map_err(error::from)?;
    Ok(output)
}