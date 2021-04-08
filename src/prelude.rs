//! The prelude is a collection of all traits and commonly used types in this crate
//!
//! For normal use of this crate it is sufficient to glob import only this moduel, e.g. `use
//! xng_rs::prelude::*`.

pub use cstr_core::{self, CStr};

pub use crate::{
    cstr, partition, port,
    time::{self, Duration},
    XngError,
};
