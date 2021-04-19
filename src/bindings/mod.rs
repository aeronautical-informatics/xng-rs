#![allow(clippy::redundant_static_lifetimes)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

#[cfg(target_pointer_width = "32")]
include! {"32.rs"}

#[cfg(target_pointer_width = "64")]
include! {"64.rs"}
