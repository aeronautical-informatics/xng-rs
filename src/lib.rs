//! A Rust wrapper for the XNG API
//!
//! This crate provides a thin wrapper for the C ABI of [FentISS'](https://fentiss.com/)
//! [Xtratum](https://fentiss.com/products/hypervisor/) Next Generation (XNG) separation kernel. It
//! allows the implementation of bare metal (`no_std`) partitions for the XNG.
//!
//! This is an early effort in an ongoing research effort, this is not validated, qualified or by
//! any meaning _ready_.
#![no_std]
#![deny(missing_docs)]

/// This module contains the raw_bindings to the C ABI of XNG. It is advised to never expose this.
mod raw_bindings {
    #![allow(clippy::redundant_static_lifetimes)]
    #![allow(missing_docs)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod partition;
pub mod port;
pub mod types;
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

/*
impl From<raw_bindings::xReturnCode_t> for Result<(), XngError> {
    fn from(from: raw_bindings::xReturnCode_t) -> Result<(), XngError> {}
}
*/
