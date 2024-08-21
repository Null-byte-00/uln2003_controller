#![no_std]
#![no_main]

use arduino_hal::{hal::port::PB1, port::{mode::{self, Output}, Pin}, Pins};
use panic_halt as _;


pub struct Motor {
    last_index: i32,
    steper1: Pin<mode::Output>,
    steper2: Pin<mode::Output>,
    steper3: Pin<mode::Output>,
    steper4: Pin<mode::Output>,
}

impl Motor {
    fn new() -> Self {
        let dp = arduino_hal::Peripherals::take().unwrap();
        let pins = arduino_hal::pins!(dp);
        return Motor{
            steper1: pins.d9.into_output().downgrade(),
            steper2: pins.d10.into_output().downgrade(),
            steper3: pins.d11.into_output().downgrade(),
            steper4: pins.d12.into_output().downgrade(),
            last_index: 0
        };
    }

    fn move_clockwise(&mut self) {
        match self.last_index {
            0=>{
                self.steper4.set_low();
                self.steper1.set_high()
            },
            1=>{
                self.steper1.set_low();
                self.steper2.set_high();
            },
            2=>{
                self.steper2.set_low();
                self.steper3.set_high();
            },
            3=>{
                self.steper3.set_low();
                self.steper4.set_high();
            },
            _=>{}
        }
        if self.last_index >= 3 {
            self.last_index = 0;
        } else {
            self.last_index += 1;
        }
    }
}

/* 
fn step_clockwise(pins: Pins) {
    let mut steper1 = pins.d9.into_output();
    let mut steper2 = pins.d10.into_output();
    let mut steper3 = pins.d11.into_output();
    let mut steper4 = pins.d12.into_output();
}
*/

#[arduino_hal::entry]
fn main() -> ! {
    //let dp = arduino_hal::Peripherals::take().unwrap();
    //let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    //let mut led = pins.d13.into_output();

    let mut motor = Motor::new();

    loop {
        motor.move_clockwise();
        arduino_hal::delay_ms(10);
    }
}
