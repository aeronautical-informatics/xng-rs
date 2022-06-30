//! Functions related to the partitionining system
//!
//!

use core::mem::MaybeUninit;
use cstr_core::CStr;

use crate::{bindings, XngError};

/// One partitions id type
pub type PartitionId = bindings::xPartitionId_t;

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
        let return_code = bindings::XGetMyPartitionId(id.as_mut_ptr());
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
pub fn id(port_name: &CStr) -> Result<PartitionId, XngError> {
    let mut id = MaybeUninit::uninit();

    unsafe {
        let return_code =
            bindings::XGetPartitionId(port_name.as_ptr() as *mut cty::c_char, id.as_mut_ptr());
        XngError::from(return_code)?;
        Ok(id.assume_init())
    }
}

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
    let return_code = unsafe { bindings::XHaltPartition(partition) };
    XngError::from(return_code)
}

struct PartitionStatus {
    /// The start condition field indicates the way the partition was started
    start_condition: u8,

    /// Times the partition has been re-started. This counter is zeroed when the hypervisor is
    /// reset
    restarts: bindings::xPartitionRestartRange_t,

    /// When vCpuState is xVCpuRunning, vCpuSchedStatus contains the status of the current slot
    vcpu_state: crate::vcpu::VCpuState,

    ///
    vcpu_sched_state: bindings::xVCpuSchedStatus_t,
}
