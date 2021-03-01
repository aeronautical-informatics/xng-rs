//! This module contains handles for the console

use crate::{
    prelude::CStr,
    //raw_bindings::XWriteConsole,
    XngError,
};

/// The maximum allowed length of a string to be written to the console
pub use crate::raw_bindings::xMaxStringLength as MAX_STRING_LEN;

/*
/// Writes a string to the console
///
/// The length of the string must be smaller or equal than `MAX_STRING_LEN`.
pub fn write(write: &CStr)->Result<(),XngError>{
    let return_code = unsafe {
        XWriteConsole(write.as_ptr(), write.to_bytes_with_nul().len())
    };
    XngError::from(return_code)?;

    Ok(())
}
*/

/// Create a NULL terminated string in C representation
///
/// Use this where you would write `"Some string literal"` in C.
#[macro_export]
macro_rules! cprint {
    ($t:tt) => {{
        let a = concat!(env!("CARGO_BIN_NAME"), ": ", $s, "\0");
        $crate::prelude::CStr::from_bytes_with_nul(a.as_bytes())
            .expect("Interior NULL bytes are not allowed in cstr literals")
    }};
}
