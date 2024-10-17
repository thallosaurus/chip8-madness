#![no_std]
#![no_main]

use core::cell::RefCell;
use core::str;

use arduino_hal::prelude::*;
use arduino_hal::{hal::Usart, usart::Baudrate};
use chip8::app;
use panic_halt as _;

type CoreClock = arduino_hal::clock::MHz16;

//pub const TESTROM: &[u8] = include_bytes!("../../chip8-test-rom/test_opcode.ch8");
pub const IBM_LOGO: &[u8] = include_bytes!("../../chip8-roms/roms/IBM Logo.ch8");

#[arduino_hal::entry]
fn main() -> ! {

    /// Import

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

     // Setup Serial
    let mut led = pins.d13.into_output();
    let mut serial = Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        Baudrate::<CoreClock>::new(57600)
    );

    let mut a = RefCell::new(chip8::app::AppState::new(IBM_LOGO));
    
    loop {
        //led.set_high();
        let b = a.borrow_mut();
        b.step();
        //arduino_hal::delay_ms(500);
        
        for i in b.display_iter() {
            match i {
                chip8::display::DisplayStates::On => {
                    ufmt::uwrite!(&mut serial, "X").unwrap();
                },
                chip8::display::DisplayStates::Off => {
                    ufmt::uwriteln!(&mut serial, "-").unwrap();
                },
                chip8::display::DisplayStates::NewLine => {
                    ufmt::uwriteln!(&mut serial, "\r").unwrap();
                },
            }
        }
        //arduino_hal::delay_ms(1000);
        //led.set_low();
        //arduino_hal::delay_ms(100);
    }
}