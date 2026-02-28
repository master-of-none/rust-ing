#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::rmt::Rmt;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use panic_rtt_target as _;
use smart_leds::{RGB8, SmartLedsWrite};
// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();

    // GPIO8 is the onboard WS2812 RGB LED on ESP32-C6-DevKitM-1
    let mut buffer = smart_led_buffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, &mut buffer);

    let on = RGB8 { r: 0, g: 20, b: 0 }; // dim green
    let off = RGB8 { r: 0, g: 0, b: 0 };

    loop {
        led.write([on].iter().cloned()).unwrap();
        blocking_delay(Duration::from_millis(500));

        led.write([off].iter().cloned()).unwrap();
        blocking_delay(Duration::from_millis(500));
    }
}
fn blocking_delay(duration: Duration) {
    let delay_start = Instant::now();
    while delay_start.elapsed() < duration {}
}
