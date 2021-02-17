//! Various types which do not belong to a specific module

use crate::raw_bindings;

/// The type of a point in time
pub type Time = raw_bindings::xTime_t;
