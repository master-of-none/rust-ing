#![no_std]
#![no_main]

mod life;
use core::ops::RangeFrom;

use embedded_hal::{blocking::delay::DelayMs, digital::v2::InputPin};
use life::*;

use nanorand::Rng;

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer, pac::TIMER0};

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
    let mut seeds = 9..; // Testing
    life_board = generate_random_board(&mut life_board, seeds.next().unwrap());
    rprintln!("Initial Board");
    print_board(&life_board);

    loop {
        if let Ok(true) = board.buttons.button_a.is_low() {
            handle_button_a(&mut life_board, &mut display, &mut timer, &mut seeds);
        }
        // if let Ok(true) = board.buttons.button_a.is_low() {
        //     let seed = seeds.next().unwrap();
        //     rprintln!("{}", seed);
        //     life_board = generate_random_board(&mut life_board, seed);
        //     display.show(&mut timer, life_board, 1000);
        //     timer.delay_ms(100u16);
        //     rprintln!("Pressed 'A' board continues");
        // print_board(&another_board);
        // life(&mut another_board);
        // if done(&another_board) {
        //     break;
        // }
        else if let Ok(true) = board.buttons.button_b.is_low() {
            handle_button_b(&mut life_board, &mut display, &mut timer);
        } else {
            display.show(&mut timer, life_board, 1000);
            timer.delay_ms(100u16);
            rprintln!("GamePlay");
            print_board(&life_board);
            life(&mut life_board);
            if done(&life_board) {
                timer.delay_ms(5000u16);
                // if let Ok(true) = board.buttons.button_a.is_low() {
                //     rprintln!("Board A continues after Game");
                //     handle_button_a(&mut life_board, &mut display, &mut timer, &mut seeds);
                // } else if let Ok(true) = board.buttons.button_b.is_low() {
                //     rprintln!("Board B continues after Game");
                //     handle_button_b(&mut life_board, &mut display, &mut timer);
                // } else {
                let seed = seeds.next().unwrap();
                rprintln!("{}", seed);
                life_board = generate_random_board(&mut life_board, seed);
                //}
            }
        }
    }

    //panic!("Done")
}

fn generate_random_board(life_board: &mut [[u8; 5]; 5], seed: u128) -> [[u8; 5]; 5] {
    // let current_time = Duration::from_secs(2);
    // let seed = current_time.as_secs();
    // let mut rng = nanorand::Pcg64::new_seed(seed.into());
    let mut rng = nanorand::Pcg64::new_seed(seed);
    for r in 0..5 {
        for c in 0..5 {
            let b: bool = rng.generate();
            // life_board[r][c] = rng.generate_range(0..2);
            life_board[r][c] = b as u8;
        }
    }
    *life_board
}

fn complement(life_board: &mut [[u8; 5]], complement_board: &mut [[u8; 5]]) {
    for r in 0..5 {
        for c in 0..5 {
            if life_board[r][c] == 0 {
                complement_board[r][c] = 1;
            } else {
                complement_board[r][c] = 0
            }
        }
    }
}

fn print_board(board_print: &[[u8; 5]]) {
    for r in 0..5 {
        for c in 0..5 {
            rprint!("{}", board_print[r][c]);
        }
        rprintln!();
    }
    rprintln!();
}

fn handle_button_a(
    life_board: &mut [[u8; 5]; 5],
    display: &mut Display,
    timer: &mut Timer<TIMER0>,
    seeds: &mut RangeFrom<u128>,
) {
    let seed = seeds.next().unwrap();
    rprintln!("{}", seed);
    *life_board = generate_random_board(life_board, seed);
    display.show(timer, *life_board, 1000);
    timer.delay_ms(100u16);
    rprintln!("Pressed 'A' board continues");
}

fn handle_button_b(
    life_board: &mut [[u8; 5]; 5],
    display: &mut Display,
    timer: &mut Timer<TIMER0>,
) {
    let mut complement_board = [[0; 5]; 5];
    complement(life_board, &mut complement_board);
    rprintln!("Pressed 'B' board continues");
    print_board(&complement_board);
    display.show(timer, complement_board, 1000);
    timer.delay_ms(500u16);

    // let seed = seeds.next().unwrap();
    // rprintln!("{}", seed);
    // *life_board = generate_random_board(life_board, seed);
    // display.show(timer, *life_board, 1000);
    // timer.delay_ms(100u16);
    // rprintln!("Pressed 'A' board continues");
}
