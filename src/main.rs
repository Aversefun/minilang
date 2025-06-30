//! programming language with a miniscule binary interpreter
#![warn(missing_docs, clippy::missing_docs_in_private_items)]
#![no_std]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![deny(clippy::panic)]

fn main() {}

/// Tiny panic handler that just aborts the process.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}
