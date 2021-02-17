#![crate_type = "lib"]

#[repr(align(16))] //~ ERROR the attribute `repr(align(...))` on functions is unstable
fn requires_alignment() {}
