#![allow(non_camel_case_types)]
type uid_t = u32;

extern "C" {
    fn geteuid() -> uid_t;
    fn getuid() -> uid_t;
}

pub fn effective() -> uid_t {
    unsafe { geteuid() }
}

#[allow(dead_code)]
pub fn real() -> uid_t {
    unsafe { getuid() }
}
