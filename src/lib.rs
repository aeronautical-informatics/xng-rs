//! A Rust wrapper for the XNG API
//!
//! This crate provides a thin wrapper for the C ABI of [FentISS'](https://fentiss.com/)
//! [Xtratum](https://fentiss.com/products/hypervisor/) Next Generation (XNG) separation kernel. It
//! allows the implementation of bare metal (`no_std`) partitions for XNG.
//!
//!
//! # About the Project
//!
//! This is by no means ready - it is an ongoing progress. While we've already used this together
//! with FentISS' Separation Kernel Emulator (SKE), it was __not__ throughfully tested. While
//! we are engaged with FentISS, there is no official support for this neither from FentISS nor
//! from us. However, if you encounter any problems, please open up an issue. The chances are that
//! we care and try to fix the issue.
#![no_std]
#![deny(missing_docs)]

/// This module contains the raw_bindings to the C ABI of XNG. It is advised to never expose this.
#[allow(clippy::redundant_static_lifetimes)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod raw_bindings;

pub mod prelude;

pub mod partition;
pub mod port;
pub mod time;
pub mod vcpu;

/// The XNG error type
///
/// Every failable function in this crate will return a Result<(), XngError>. This enum can
/// represent all error conditions which may occure during runtime
#[derive(Debug)]
pub enum XngError {
    /// System’s operational status unaffected by request.
    NoAction,
    /// The request cannot be performed immediately.
    NotAvailable,
    /// Parameter specified in request invalid.
    InvalidParam,
    /// Parameter specified in request incompatible with current configuration.
    InvalidConfig,
    /// Request incompatible with mode of operation.
    InvalidMode,
    /// A function returned a return code which we do not know
    UnknownReturnCode(raw_bindings::xReturnCode_t),
    /// The buffer is too big
    BufTooBig {
        /// The size of the buffer
        buf_size: usize,
        /// The maximum allowed size
        max_allowed: usize,
    },
    /// The buffer is too small
    BufTooSmall {
        /// The size of the buffer
        buf_size: usize,
        /// The maximum allowed size
        min_required: usize,
    },
    /// A time error occured
    TimeError(time::TimeError),
}

impl From<time::TimeError> for XngError {
    fn from(te: time::TimeError) -> Self {
        XngError::TimeError(te)
    }
}

impl XngError {
    fn from(from: raw_bindings::xReturnCode_t) -> Result<(), Self> {
        match from {
            raw_bindings::xNoError => Ok(()),
            raw_bindings::xNoAction => Err(XngError::NoAction),
            raw_bindings::xNotAvailable => Err(XngError::NotAvailable),
            raw_bindings::xInvalidParam => Err(XngError::InvalidParam),
            raw_bindings::xInvalidConfig => Err(XngError::InvalidConfig),
            raw_bindings::xInvalidMode => Err(XngError::InvalidMode),
            code => Err(XngError::UnknownReturnCode(code)),
        }
    }
}

/// An Xng Error with trace information
pub struct XngErrorTrace {
    error: XngError,
    _line: u32,
}

impl From<XngErrorTrace> for XngError {
    fn from(error_trace: XngErrorTrace) -> Self {
        error_trace.error
    }
}

/// Convert a `xReturnCode_t` to an `Result<(), XngError>`
///
// TODO make this work with no_std
#[macro_export]
macro_rules! to_traceable_error {
    ($return_code:expr) => {{
        println!("Error in file {}, line {}", file!(), line!());
        XngError::from($return_code)
    }};
}

/// Create a NULL terminated string in C representation
///
/// Use this where you would write `"Some string literal"` in C. Will panic if interior NULL bytes
/// are in the string.
#[macro_export]
macro_rules! cstr {
    ($s:expr) => {{
        let a = concat!($s, '\0');
        $crate::prelude::CStr::from_bytes_with_nul(a.as_bytes())
            .expect("Interior NULL bytes are not allowed in cstr literals")
    }};
}
