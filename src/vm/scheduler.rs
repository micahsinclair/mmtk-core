#[cfg(feature = "jikesrvm")]
use std::mem::transmute;

#[cfg(feature = "jikesrvm")]
use ::vm::jtoc::*;

#[cfg(feature = "jikesrvm")]
use ::vm::JTOC_BASE;

#[cfg(feature = "jikesrvm")]
pub fn stop_all_mutators() {
    unsafe {
        transmute::<usize, fn()>((JTOC_BASE + BLOCK_ALL_MUTATORS_FOR_GC_METHOD_JTOC_OFFSET).as_usize())();
    }
}

#[cfg(not(feature = "jikesrvm"))]
pub fn stop_all_mutators() {
    unimplemented!()
}

#[cfg(feature = "jikesrvm")]
pub fn resume_mutators() {
    unimplemented!()
}

#[cfg(not(feature = "jikesrvm"))]
pub fn resume_mutators() {
    unimplemented!()
}