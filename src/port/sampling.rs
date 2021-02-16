use core::{ffi::c_void, mem::MaybeUninit};

use super::{validity_to_bool, PortDirection};
use crate::{raw_bindings, XngError};

// TODO check if the port name is nullterminated

pub type SamplingPortId = raw_bindings::xSamplingPortId_t;
pub type Time = raw_bindings::xTime_t;

/// Keeps the last (if any) sent value
pub struct SamplingReceiver<const N: usize> {
    port_id: SamplingPortId,
}

// TODO Pin port_buffer, likely it is necessary
impl<const N: usize> SamplingReceiver<N> {
    /// Creates a communication port operating in sampling mode
    ///
    /// # Arguments
    ///
    /// * `port_name` - The name of this port. Best be used with the byte string literal, e.g.
    /// `b"my_port\0"`. DO NOT FORGET THE TRAILLING `\0`!
    pub fn new(port_name: &[u8], time: i64) -> Result<Self, XngError> {
        let mut port_id = MaybeUninit::uninit();

        let return_code = unsafe {
            raw_bindings::XCreateSamplingPort(
                port_name.as_ptr() as *mut i8, // TODO fix to non mut pointer
                N as u32,
                PortDirection::Destination as u32,
                time,
                port_id.as_mut_ptr(),
            )
        };

        XngError::from(return_code)?;
        let port_id = unsafe { port_id.assume_init() };

        Ok(Self { port_id })
    }

    /// read a message
    ///
    /// Returns Ok(Some(read_bytes)) on success
    pub fn recv(&self, buf: &mut [u8; N]) -> Result<Option<usize>, XngError> {
        let mut bytes_read = MaybeUninit::uninit();
        let mut validity = MaybeUninit::uninit();

        let return_code = unsafe {
            raw_bindings::XReadSamplingMessage(
                self.port_id,
                buf.as_mut_ptr() as *mut c_void,
                bytes_read.as_mut_ptr(),
                validity.as_mut_ptr(),
            )
        };

        XngError::from(return_code)?;
        unsafe {
            Ok(match validity_to_bool(validity.assume_init()) {
                true => Some(bytes_read.assume_init() as usize),
                false => None,
            })
        }
    }

    /// Get the id of this sampling port
    pub fn id(&self) -> SamplingPortId {
        self.port_id
    }

    /// Get status of the port
    pub fn status(&self) -> Result<SamplingPortStatus, XngError> {
        SamplingPortStatus::new(self.port_id)
    }
}

/// Keeps the last (if any) sent value
pub struct SamplingSender<const N: usize> {
    port_id: raw_bindings::xSamplingPortId_t,
}

// TODO Pin port_buffer, likely it is necessary
impl<const N: usize> SamplingSender<N> {
    /// Creates a communication port operating in sampling mode
    ///
    /// # Arguments
    ///
    /// * `port_name` - The name of this port. Best be used with the byte string literal, e.g.
    /// `b"my_port\0"`. DO NOT FORGET THE TRAILLING `\0`!
    pub fn new(port_name: &[u8], time: i64) -> Result<Self, XngError> {
        let mut port_id = MaybeUninit::uninit();

        let return_code = unsafe {
            raw_bindings::XCreateSamplingPort(
                port_name.as_ptr() as *mut i8, // TODO fix to non mut pointer
                N as u32,                      // fix to usize
                PortDirection::Source as u32,  // TODO fix to usize
                time,
                port_id.as_mut_ptr(),
            )
        };

        XngError::from(return_code)?;
        let port_id = unsafe { port_id.assume_init() };

        Ok(Self { port_id })
    }

    /// read a message
    ///
    /// Returns Ok(read_bytes) on success
    pub fn send(&self, buf: &[u8]) -> Result<(), XngError> {
        if buf.len() > N {
            return Err(XngError::BufTooBig(buf.len()));
        }

        let return_code = unsafe {
            raw_bindings::XWriteSamplingMessage(
                self.port_id,
                buf.as_ptr() as *mut c_void, // TODO fix to non mut pointer
                buf.len() as u32,            // TODO fix to usize
            )
        };
        XngError::from(return_code)
    }

    /// Get the id of this sampling port
    pub fn id(&self) -> SamplingPortId {
        self.port_id
    }

    /// Get status of the port
    pub fn status(&self) -> Result<SamplingPortStatus, XngError> {
        SamplingPortStatus::new(self.port_id)
    }
}

/// The current status of a Sampling Port
#[derive(Debug)]
pub struct SamplingPortStatus {
    pub refresh_period: Time,

    /// Timestamp of last message
    pub last_message_ts: Time,

    pub last_message_size: usize,

    pub last_message_valid: bool,
}

impl SamplingPortStatus {
    fn new(id: SamplingPortId) -> Result<SamplingPortStatus, XngError> {
        let mut status_struct = MaybeUninit::uninit();

        let status_struct = unsafe {
            let return_code = raw_bindings::XGetSamplingPortStatus(id, status_struct.as_mut_ptr());
            XngError::from(return_code)?;
            status_struct.assume_init()
        };

        Ok(Self {
            refresh_period: status_struct.refreshPeriod,
            last_message_ts: status_struct.lastMessageTimestamp,
            last_message_size: status_struct.lastMessageSize as usize,
            last_message_valid: validity_to_bool(status_struct.lastMessageValidity),
        })
    }
}
