#![feature(used)]
#![feature(collections)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

extern crate alloc_cortex_m;
extern crate collections;

use collections::vec::Vec;


extern crate iota_trytes as trytes;


// These symbols come from a linker script
extern "C" {
    static mut _heap_start: usize;
    static mut _heap_end: usize;
    static mut _stack_start: usize;
    static mut _sheap: usize;
}


use cortex_m::asm;
use trytes::*;

fn main() {
    //unsafe { hprintln!("stack {:?} sheap {:?} start: {:?} end: {:?}", _stack_start, _sheap, _heap_start, _heap_end) }

    unsafe { alloc_cortex_m::init(&mut _heap_start, &mut _heap_end) }

    let t: Trinary = "ABCD9".chars().collect();


    //hprintln!("trits: {:?}", trits);

    hprintln!("Hello, world! {:?}", t);
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    hprintln!("Oh no.");
    asm::bkpt();
}
