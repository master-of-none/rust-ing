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
    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();

    let mut state = State::LedOn;

    loop {
        state = match state {
            State::LedOn => {
                board.display_pins.row1.set_high().unwrap();
                rprintln!("High");
                State::LedOff
            }
            State::LedOff => {
                board.display_pins.row1.set_low().unwrap();
                rprintln!("Low");
                State::LedOn
            }
        };
        timer.delay_ms(500u16);
    }
}
