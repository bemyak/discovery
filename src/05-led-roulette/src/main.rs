#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

enum Action {
    On,
    Off,
}

impl Action {
    fn toggle(&mut self) {
        match self {
            Action::On => *self = Action::Off,
            Action::Off => *self = Action::On,
        }
    }
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u8;

    let mut i = 0_u8;
    let mut action = Action::On;

    leds[0].on().ok();

    loop {
        match action {
            Action::On => {
                let i = i.wrapping_add(1);
                let j = i as usize % leds.len();
                leds[j].on().ok();
            }
            Action::Off => {
                let j = i as usize % leds.len();
                leds[j].off().ok();
                i = i.wrapping_add(1);
            }
        }
        action.toggle();
        delay.delay_ms(half_period);
    }
}
