use core::{ffi::c_void, mem::MaybeUninit};

use cstr_core::CStr;

use super::{validity_to_bool, PortDirection};
use crate::{
    raw_bindings,
    time::{duration_from_xtime_t, Duration},
    XngError,
};

/// The type of a sampling ports id
pub type SamplingPortId = raw_bindings::xSamplingPortId_t;

/// Keeps the last (if any) sent value
pub struct SamplingReceiver<const N: usize> {
    port_id: SamplingPortId,
}

impl<const N: usize> SamplingReceiver<N> {
    /// Creates a communication port operating in sampling mode
    ///
    /// # Arguments
    ///
    /// * `port_name` - The name of this port. Use the `cstr!("Hello world")` macro to create
    /// values from literals.
    /// * `ttl` - Time to live of the message. The message will be valid for `ttl` microseconds
    /// after it was written. Naturally, a duration below one microsecond is not supported.
    pub fn new<T: Into<Duration>>(port_name: &CStr, ttl: T) -> Result<Self, XngError> {
        let mut port_id = MaybeUninit::uninit();

        let return_code = unsafe {
            raw_bindings::XCreateSamplingPort(
                port_name.as_ptr() as *mut i8, // TODO fix to non mut pointer
                N as u32,
                PortDirection::Destination as u32,
                ttl.into().as_micros() as raw_bindings::xTime_t,
                port_id.as_mut_ptr(),
            )
        };

        XngError::from(return_code)?;
        let port_id = unsafe { port_id.assume_init() };

        Ok(Self { port_id })
    }

    /// Receives a message
    ///
    /// Returns `Ok(Some(read_bytes))` if a valid message was available, `Ok(None)` if no message
    /// was available and `Err(XngError)` if an error occure
    pub fn recv<'a>(&self, buf: &'a mut [u8]) -> Result<Option<(&'a mut [u8], bool)>, XngError> {
        // if buf is smaller than N bytes, we can not fit a full message in it; abort
        if buf.len() < N {
            return Err(XngError::BufTooSmall {
                buf_size: buf.len(),
                min_required: N,
            });
        }

        let mut bytes_read = MaybeUninit::uninit();
        let mut validity = MaybeUninit::uninit();

        let return_code = unsafe {
            raw_bindings::XReadSamplingMessage(
                self.port_id,
                buf.as_mut_ptr() as *mut c_void,
                bytes_read.as_mut_ptr(), // TODO make this usize
                validity.as_mut_ptr(),
            )
        };

        // retrieve possible error
        let error = XngError::from(return_code);
        // handle NotAvailable special, as export the semantics of it via Option
        if let Err(XngError::NotAvailable) = error {
            return Ok(None);
        }
        // yield any other error
        error?;

        // No error, give back the result together with the validity
        Ok(Some(unsafe {
            (
                &mut buf[..bytes_read.assume_init() as usize],
                validity_to_bool(validity.assume_init()),
            )
        }))
    }

    /// Get the id of this sampling port
    // TODO should this really be exposed?
    pub fn id(&self) -> SamplingPortId {
        self.port_id
    }

    /// Get status of the port
    pub fn status(&self) -> Result<SamplingPortStatus, XngError> {
        SamplingPortStatus::new(self.port_id)
    }
}

/// Allows to store one message in the port
pub struct SamplingSender<const N: usize> {
    port_id: raw_bindings::xSamplingPortId_t,
}

impl<const N: usize> SamplingSender<N> {
    /// Creates a communication port operating in sampling mode
    ///
    /// # Arguments
    ///
    /// * `port_name` - The name of this port. Use the `csrt!("Hello world")` macro to create
    /// values from literals.
    pub fn new(port_name: &CStr) -> Result<Self, XngError> {
        let mut port_id = MaybeUninit::uninit();

        let return_code = unsafe {
            raw_bindings::XCreateSamplingPort(
                port_name.as_ptr() as *mut i8, // TODO fix to non mut pointer
                N as u32,                      // fix to usize
                PortDirection::Source as u32,  // TODO fix to usize
                1 as raw_bindings::xTime_t,
                port_id.as_mut_ptr(),
            )
        };

        XngError::from(return_code)?;
        let port_id = unsafe { port_id.assume_init() };

        Ok(Self { port_id })
    }

    /// Send a message
    ///
    /// Returns `Ok(())` on success. `buf` must be smaller or equal in size to `N`.
    pub fn send(&self, buf: &[u8]) -> Result<(), XngError> {
        // if buf is bigger than N bytes, we can not fit the send the whole buffer; abort
        if buf.len() > N {
            return Err(XngError::BufTooBig {
                buf_size: buf.len(),
                max_allowed: N,
            });
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
    /// Refresh period as defined via XCF
    pub refresh_period: Duration,

    /// Timestamp of last message - None if no message ever was received priorly
    pub last_message_ts: Option<Duration>,

    /// Size in bytes of the last message which was received
    pub last_message_size: usize,

    /// Whether the last message was valid
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
            refresh_period: duration_from_xtime_t(status_struct.refreshPeriod)?,
            last_message_ts: duration_from_xtime_t(status_struct.lastMessageTimestamp).ok(),
            last_message_size: status_struct.lastMessageSize as usize,
            last_message_valid: validity_to_bool(status_struct.lastMessageValidity),
        })
    }
}
