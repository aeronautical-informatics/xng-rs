//! This module adds some notion of time
//!
//! It deviates from the XNG API significantly in order to rensemble the time types from Rusts std
//! library.
//!
//! There are two basic types in this module, `Duration` and `Instant`. `Duration` is our
//! substitute for `xTimeSpan_t`, while `Instant` replaces  `xTime_t`.

pub use core::time::Duration;
use core::{convert::TryInto, mem::MaybeUninit};

use crate::{
    raw_bindings::{xTime_t, XGetSystemTime},
    XngError,
};

/// Get the duration of time since the boot of the system/
///
/// # Examples
///
/// ```no_run
/// use xng_rs::prelude::*;
///
/// let duration_since_boot = time::since_boot();
/// ```
pub fn since_boot() -> Result<Duration, XngError> {
    let mut time = MaybeUninit::uninit();
    let time = unsafe {
        let return_code = XGetSystemTime(time.as_mut_ptr());
        XngError::from(return_code)?;
        time.assume_init()
    };

    Ok(duration_from_xtime_t(time)?)
}

/// Convert a xtime_t to a `Duration`
///
/// This API is not to be published
// TODO ^ is that clever?
pub(crate) fn duration_from_xtime_t(time: xTime_t) -> Result<Duration, TimeError> {
    if time.is_negative() {
        Err(TimeError::InfiniteTime.into())
    } else {
        Ok(Duration::from_micros(time.try_into().unwrap())) // this should never fail
    }
}

/// Error during operations with time
#[derive(Debug)]
pub enum TimeError {
    /// An instant has the value infinity. This should not happen in the foreseable future!
    InfiniteTime,
}

/// Extension trait that adds convenience methods to the `i64` type
pub trait DurationFromInt {
    /// Duration in secs
    fn secs(self) -> Duration;

    /// Duration in milliseconds
    fn ms(self) -> Duration;

    /// Duration in microseconds
    fn us(self) -> Duration;
}

impl DurationFromInt for u64 {
    fn secs(self) -> Duration {
        Duration::from_secs(self)
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self)
    }

    fn us(self) -> Duration {
        Duration::from_micros(self)
    }
}

impl DurationFromInt for u32 {
    fn secs(self) -> Duration {
        Duration::from_secs(self.into())
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self.into())
    }

    fn us(self) -> Duration {
        Duration::from_micros(self.into())
    }
}

impl DurationFromInt for u16 {
    fn secs(self) -> Duration {
        Duration::from_secs(self.into())
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self.into())
    }

    fn us(self) -> Duration {
        Duration::from_micros(self.into())
    }
}

impl DurationFromInt for u8 {
    fn secs(self) -> Duration {
        Duration::from_secs(self.into())
    }

    fn ms(self) -> Duration {
        Duration::from_millis(self.into())
    }

    fn us(self) -> Duration {
        Duration::from_micros(self.into())
    }
}

/*
impl From<u32> for Hertz {
    fn from(t: u32) -> Self {
        t.hz()
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

/// Time unit
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct MilliSeconds(pub u32);
*/
