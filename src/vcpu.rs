//! This module contains functions for the virtual CPU

use crate::raw_bindings;

/// Type representing the id of a virtual CPU
pub type VCpuId = raw_bindings::xVCpuId_t;

/// Instruct the callers virtual CPU to yield its computation time in the current slot to XNG
pub fn finish_slot() {
    unsafe { raw_bindings::XWaitUntilNextScheduleSlot() };
}

/* These symbols are not yet provided in SKE

/// Get the callers CPU id
pub fn cpu_id()->VCpuId{
    unsafe {raw_bindings::XGetMyVCpuId()}
}

/// Halt the VCpu
pub fn halt_cpu(cpu: VCpuId)->Result<(), XngError>{
    let return_code = unsafe{ raw_bindings::xHaltVCpu(cpu)};
    todo!();
}

/// Suspend the VCpu
pub fn suspend(cpu: VCpuId)->Result<(), XngError>{
    let return_code = unsafe{ raw_bindings::XSuspendVCpu(cpu)};
    todo!();
}

/// Resume the VCpu
pub fn resume(cpu: VCpuId)->Result<(), XngError>{
    let return_code = unsafe{ raw_bindings::XResumeVCpu(cpu)};
    todo!();
}
 */

/*
 * What is xMemAddr_t for?
 * xReturnCode_t XResetVCpu(xVCpuId_t, xMemAddr_t);
 */
