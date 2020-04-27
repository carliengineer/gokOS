// languages have "runtime system" provides an environment in which programs run, addresses issues including layout of memory, how the program accesses variables,
// mechanisms for passing parameters between procedures, interfacing with the operating system, so it  is reposible things like ( java: garbage collection, go: threads)
// rust starts with crt0 (C runtime zero), Our freestanding executable does not have access to the Rust runtime and crt0. 
// implementing the start language item wouldn't help, since it would still require crt0. Instead, we need to overwrite the crt0 entry point directly.#![no_main]
// we need to remove main function as well. The reason is that a main doesn't make sense without an underlying runtime that calls it. 
#![no_std]
#![no_main]
//since we don't use std lib, we need our own panic handler
use core::panic::PanicInfo;

/// This is panic handler function called on panic, never returns, it is called diverging function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// instead of main,  we are now overwriting the operating system entry point with our own _start function
//By using the #[no_mangle] attribute we disable the name mangling to ensure that the Rust compiler really outputs a function with the name _start.
//Without the attribute, the compiler would generate some cryptic _ZN3blog_os4_start7hb173fedf945531caE symbol to give every function an unique name.
//We also have to mark the function as extern "C" to tell the compiler that it should use the C calling convention for this function
pub extern "C" fn _start() -> !{
    loop {}
}