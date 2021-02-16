mod sampling;

pub use sampling::*;

use crate::raw_bindings;

/// The direction of a port
#[derive(PartialEq, Eq)]
enum PortDirection {
    /// This port is a source
    Source = raw_bindings::xSourcePort as isize,
    /// This port is a destination
    Destination = raw_bindings::xDestinationPort as isize,
}

/// Check if a message was valid
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
