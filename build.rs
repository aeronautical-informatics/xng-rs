extern crate bindgen;

use std::env;
use std::path::PathBuf;

const BASE_HEADER: &[(u8, &str)] = &[
    (10, "xalCpuIfc.h"),
    (10, "xalTypesIfc.h"),
    (10, "xArchTypeIfc.h"),
    (10, "xBoardTypeIfc.h"),
    (10, "xcfTypeIfc.h"),
    (10, "xCommPortIfc.h"),
    (10, "xConsoleIfc.h"),
    (10, "xLegacyIfc.h"),
    (10, "xPartitionIfc.h"),
    (10, "xQueuingPortIfc.h"),
    (10, "xSamplingPortIfc.h"),
    (10, "xScheduleIfc.h"),
    (10, "xTraceIfc.h"),
    (10, "xTypeIfc.h"),
    (10, "xVClockIfc.h"),
    (10, "xVCpuIfc.h"),
    (10, "xVIrqCtrlIfc.h"),
    (10, "xVTimerIfc.h"),
];

#[allow(dead_code)]
const SKE_HEADER: &[(u8, &str)] = &[
    // "skeconfig.h",
    // "ske_external_ifc.h",
    // "ske_init.h",
    // "ske_irq.h",
    // "ske_partition.h",
    // "ske_queuing_port.h",
    // "ske_sampling_port.h",
    // "ske_time.h",
    // "ske_trace.h",
    // "ske_types.h",
    // "ske_yield.h",
    (0, "types.h"),
    (10, "xalStdio.h"),
    (10, "xalString.h"),
    (5, "xalTypes.h"),
];

#[allow(dead_code)]
const XNG_HEADER: &[(u8, &str)] = &[
    (10, "xalSpinLockIfc.h"),
    (10, "xArchHealthMonitorIfc.h"),
    (10, "xArchLegacyIfc.h"),
    (10, "xArchTraceIfc.h"),
    (10, "xAsmIfc.h"),
    (10, "xcfArchTypeIfc.h"),
    (10, "xcfBoardTypeIfc.h"),
    (10, "xConfigurationIfc.h"),
    (10, "xc/xalDivIfc.h"),
    (10, "xc/xalStdio.h"),
    (10, "xc/xalString.h"),
    (10, "xc/xalTypes.h"),
    (10, "xIoPortIfc.h"),
    (10, "xIpviIfc.h"),
    (10, "xre-armv7a-vmsa-tz/xalCpuIfc.h"),
    (10, "xre-armv7a-vmsa-tz/xalSpinLockIfc.h"),
    (10, "xre-armv7a-vmsa-tz/xalTypesIfc.h"),
    (10, "xSystemCfgIfc.h"),
    // TODO these belongs to BASE_HEADER, but it includes xArchHealthMonitorIfc.h
    // which is currently only provided by XNG, thus it breaks the compilation to
    // have this in SKE builds
    (10, "xHypervisorIfc.h"),
    (5, "xHealthMonitorIfc.h"),
];

fn main() {
    let headers_to_include = BASE_HEADER.iter();

    #[cfg(feature = "std")]
    let headers_to_include = headers_to_include.chain(SKE_HEADER.iter());

    #[cfg(not(feature = "std"))]
    let headers_to_include = headers_to_include.chain(XNG_HEADER.iter());

    let mut sorted_includes: Vec<_> = headers_to_include.collect();
    sorted_includes.sort();
    let wrapper: String = sorted_includes
        .into_iter()
        .map(|(_, header)| format!("#include \"{header}\"\n"))
        .collect();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header_contents("wrapper.h", &wrapper)
        .use_core()
        .allowlist_function("(x|X).*")
        .allowlist_type("(x|X).*")
        .allowlist_var("(x|X).*")
        .blocklist_type("__.*")
        .ctypes_prefix("cty")
        .generate_comments(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
