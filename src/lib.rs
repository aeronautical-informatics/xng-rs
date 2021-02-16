#![no_std]

pub mod raw_bindings {
    #![allow(clippy::redundant_static_lifetimes)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod partition;
pub mod port;
pub mod vcpu;

#[derive(Debug)]
pub enum XngError {
    MaxStringLength,
    InfiniteTimeValue,
    InvalidCpuId,

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

    UnknownReturnCode(raw_bindings::xReturnCode_t),
    BufTooBig(usize),
}

impl XngError {
    fn from(from: raw_bindings::xReturnCode_t) -> Result<(), Self> {
        match from {
            raw_bindings::xNoError => Ok(()),
            //raw_bindings::xInvalidCpuId => Err(XngError::InvalidCpuId),
            raw_bindings::xNoAction => Err(XngError::NoAction),
            raw_bindings::xNotAvailable => Err(XngError::NotAvailable),
            raw_bindings::xInvalidParam => Err(XngError::InvalidParam),
            raw_bindings::xInvalidConfig => Err(XngError::InvalidConfig),
            raw_bindings::xInvalidMode => Err(XngError::InvalidMode),
            code => Err(XngError::UnknownReturnCode(code)),
        }
    }
}
