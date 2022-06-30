//! This module contains functions for the virtual CPU

use crate::bindings;

/// Type representing the id of a virtual CPU
pub struct VCpuId(bindings::xVCpuId_t);

/// Yields the computation time of the current vCpu to the hypervisor until the start of a new
/// slot.
pub fn wait_until_next_schedule_slot() {
    unsafe { bindings::XWaitUntilNextScheduleSlot() };
}

/// Definition of the  vCpu's current state type
#[repr(u32)]
pub enum VCpuState {
    /// The vCpu is eligible to run but it has not been selected by the scheduler
    Ready = bindings::xVCpuReady,

    /// The vCpu is running
    Running = bindings::xVCpuRunning,

    /// The vCpu is not eligible by the scheduler to run. A vCpu reset operation is required to
    /// pass the vCpu to a ready state
    Idle = bindings::xVCpuIdle,

    /// The vCpu is not eligible by the scheduler to run. The vCpu can set to ready state either
    /// by:
    ///    1) Resetting the vCpu
    ///       OR
    ///    2) Resuming the vCpu
    Suspended = bindings::xVCpuSuspended,

    /// The vCpu yields its computation time to the hypervisor until the end of the current slot
    Waiting = bindings::xVCpuWaiting,
}

/// Status of the current schedule slot when vCpu is in xVCpuRunning state
pub struct VCpuSchedStatus {
    /// Current slot's identifier
    slot_id: bindings::xcfSlotId_t,
    slot_start: core::time::Duration,
    slot_duration: core::time::Duration,
}

/* These symbols are not yet provided in SKE

/// Get the callers CPU id
pub fn cpu_id()->VCpuId{
    unsafe {bindings::XGetMyVCpuId()}
}

/// Halt the VCpu
pub fn halt_cpu(cpu: VCpuId)->Result<(), XngError>{
    let return_code = unsafe{ bindings::xHaltVCpu(cpu)};
    todo!();
}

/// Suspend the VCpu
pub fn suspend(cpu: VCpuId)->Result<(), XngError>{
    let return_code = unsafe{ bindings::XSuspendVCpu(cpu)};
    todo!();
}

/// Resume the VCpu
pub fn resume(cpu: VCpuId)->Result<(), XngError>{
    let return_code = unsafe{ bindings::XResumeVCpu(cpu)};
    todo!();
}
 */

/*
 * What is xMemAddr_t for?
 * xReturnCode_t XResetVCpu(xVCpuId_t, xMemAddr_t);
 */
