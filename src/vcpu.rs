//! This module contains handles for the VCpu.

use crate::raw_bindings;

pub type VCpuId = raw_bindings::xVCpuId_t;

/// Allows the current vCpu to yield its computation time to XNG
pub fn finish_slot() {
    unsafe { raw_bindings::XWaitUntilNextScheduleSlot() };
}

/* Symbols not provided?
/// Get my CPU id
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
