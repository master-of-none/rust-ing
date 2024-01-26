#![no_std]
#![no_main]

mod life;
use life::*;

use nanorand::Rng;

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer};

//use core::time::Duration;
use panic_rtt_target as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    // Game of Life board
    // let mut life_board = [
    //     [0, 0, 0, 0, 0],
    //     [0, 0, 1, 0, 0],
    //     [0, 1, 1, 1, 0],
    //     [0, 0, 1, 0, 0],
    //     [0, 0, 0, 0, 0],
    // ];
    let mut life_board = [[0; 5]; 5];
    generate_random_board(&mut life_board);

    for r in 0..5 {
        for c in 0..5 {
            rprint!("{}", life_board[r][c]);
        }
        rprintln!();
    }
    loop {
        display.show(&mut timer, life_board, 1000);
        life(&mut life_board);
        if done(&life_board) {
            break;
        }
    }
    panic!("Done")
}

fn generate_random_board(life_board: &mut [[u8; 5]]) {
    // let current_time = Duration::from_secs(2);
    // let seed = current_time.as_secs();
    // let mut rng = nanorand::Pcg64::new_seed(seed.into());
    let mut rng = nanorand::Pcg64::new_seed(10);
    for r in 0..5 {
        for c in 0..5 {
            let b: bool = rng.generate();
            // life_board[r][c] = rng.generate_range(0..2);
            life_board[r][c] = b as u8;
        }
    }
}
