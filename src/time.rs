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

/// A notion of time
pub struct Instant(xTime_t);

impl Instant {
    /// Returns an instant corresponding to "now".
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xng_rs::time::Instant;
    ///
    /// let now = Instant::now();
    /// ```
    pub fn now() -> Result<Self, XngError> {
        let mut time = MaybeUninit::uninit();
        unsafe {
            let return_code = XGetSystemTime(time.as_mut_ptr());
            XngError::from(return_code)?;
            Ok(Self(time.assume_init()))
        }
    }

    /// Returns the amount of time elapsed from another instant to this one,
    /// or zero duration if that instant is later than this one.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use xng_rs::time::{Duration, Instant};
    ///
    /// let now = Instant::now();
    /// sleep(Duration::new(1, 0));
    /// let new_now = Instant::now();
    /// println!("{:?}", new_now.saturating_duration_since(now));
    /// println!("{:?}", now.saturating_duration_since(new_now)); // 0ns
    /// ```
    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        let duration_micros = self
            .0
            .checked_sub(earlier.0)
            .map(|micros| core::cmp::max(micros, 0) as u64)
            .unwrap_or_default();
        Duration::from_micros(duration_micros)
    }

    /// Get the duration of time since the boot of the system/
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let duration_since_boot = Instant::now().since_boot();
    /// ```
    pub fn since_boot(&self) -> Result<Duration, TimeError> {
        match self.0 {
            // this unwrap never fails as we check the i64 (xTime_t) to be positive before casting
            // it to the u64 expected from `core::time::Duration::from_micros`
            0..=xTime_t::MAX => Ok(Duration::from_micros(self.0.try_into().unwrap())),
            _ => Err(TimeError::NegativeDuration),
        }
    }
}

/// Error during operations with time
pub enum TimeError {
    /// A negative duration was to be constructed.
    NegativeDuration,
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