#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprintln, RegisterBlock, ITM};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    loop {}
}
