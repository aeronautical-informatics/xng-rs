//! Functions related to the partitionining system
//!
//!

use core::mem::MaybeUninit;

use crate::{raw_bindings, XngError};

/// One partitions id type
pub type PartitionId = raw_bindings::xPartitionId_t;

/// Get the current partitions ID
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), xng_rs::XngError> {
/// use xng_rs::partition;
///
/// let my_id = partition::my_id()?;
/// # Ok(())}
/// ```
pub fn my_id() -> Result<PartitionId, XngError> {
    let mut id = MaybeUninit::uninit();

    unsafe {
        let return_code = raw_bindings::XGetMyPartitionId(id.as_mut_ptr());
        XngError::from(return_code)?;
        Ok(id.assume_init())
    }
}

/// Get the other partitions ID
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), xng_rs::XngError> {
/// use xng_rs::partition;
///
/// let my_id = partition::my_id()?;
/// # Ok(())}
/// ```
pub fn id() {}

/// Halt a partition
///
/// # Arguments
///
/// * partition - Th
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), xng_rs::XngError> {
/// use xng_rs::partition;
///
/// // Let suicide our execution
/// let my_id = partition::my_id()?;
/// partition::halt(my_id)?;
/// # Ok(())}
/// ```
pub fn halt(partition: PartitionId) -> Result<(), XngError> {
    let return_code = unsafe { raw_bindings::XHaltPartition(partition) };
    XngError::from(return_code)
}
