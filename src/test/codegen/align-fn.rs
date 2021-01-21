// compile-flags: -C no-prepopulate-passes -Z mir-opt-level=0

#![crate_type = "lib"]

// CHECK: align 16
#[no_mangle]
#[repr(align(16))]
pub fn fn_align() {}
