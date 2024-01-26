#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, timer::Timer},
};

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

enum State {
    LedOn,
    LedOff,
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();

    board.display_pins.row1.set_high().unwrap();

    let mut led1 = board.display_pins.col1;
    let mut led2 = board.display_pins.col2;

    loop {
        if let Ok(true) = board.buttons.button_a.is_high() {
            let _ = led1.set_high();
        } else {
            let _ = led1.set_low();
        }

        if let Ok(true) = board.buttons.button_b.is_high() {
            let _ = led2.set_high();
        } else {
            let _ = led2.set_low();
        }
    }
}
