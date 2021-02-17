//! Ports allow communication between the partitions in XNG
//!
//! # Types of Ports
//!
//! There are two different types of ports which differ slightly in semantics.
//!
//! A __Sampling Port__ retains the last message (if any). It allows for single producer multiple
//! consumer (SPMC). The maximum size of a message is capped at `N`, shorter messages however are
//! permissible.
//!
//! In contrast, a __Queuing Port__ retains the last `M` messages, of which each might be up to `N`
//! bytes big. The messages are guaranteed to to be served in FIFO order. This type of port is
//! single producer single consumer (SPSC), so only two partitions can use one Queueing Port.

use crate::raw_bindings;

mod sampling;

pub use sampling::*;

/// The direction of a port
#[derive(PartialEq, Eq)]
enum PortDirection {
    /// This port is a source
    Source = raw_bindings::xSourcePort as isize,
    /// This port is a destination
    Destination = raw_bindings::xDestinationPort as isize,
}

/// Check if a message is valid
///
/// Returns true if the message was valid
fn validity_to_bool(validity: raw_bindings::xValidity_t) -> bool {
    match validity {
            raw_bindings::xInvalidMessage => false,
            raw_bindings::xValidMessage => true,
            _=> panic!("The function `XReadSamplingMessage` broke it's contract, the value of `xValidity_t` is neither {} nor {}, but {}",
             raw_bindings::xInvalidMessage       ,
              raw_bindings::xValidMessage,
              validity),
        }
}
