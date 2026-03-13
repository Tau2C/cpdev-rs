#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use cpdev::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{InputConfig, Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{clock::CpuClock, gpio::Input};
use esp_println::println;
use {esp_backtrace as _, esp_println as _};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[unsafe(no_mangle)]
pub extern "Rust" fn _esp_println_timestamp() -> u64 {
    esp_hal::time::Instant::now()
        .duration_since_epoch()
        .as_millis()
}

#[esp_rtos::main]
async fn main(_s: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    esp_rtos::start(timg0.timer0);

    let button = Input::new(
        peripherals.GPIO26,
        InputConfig::default().with_pull(esp_hal::gpio::Pull::Up),
    );
    let led1 = Output::new(peripherals.GPIO25, Level::Low, OutputConfig::default());
    let led3 = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());
    let led2 = Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());

    let mut data = [0 as u8; 1024];
    let code = [0 as u8; 1024];

    let cpdev = CpDev::new(&code, &mut data, Context {}, None, None, None, None, None);

    _s.spawn(cpdev_task(cpdev, button, led1, led2, led3)).ok();

    println!("Test");

    loop {
        Timer::after(Duration::from_secs(2)).await;
    }
}

pub struct Context {}

#[embassy_executor::task]
async fn cpdev_task(
    mut cpdev: CpDev<Context>,
    button: Input<'static>,
    mut led1: Output<'static>,
    mut led2: Output<'static>,
    mut led3: Output<'static>,
) {
    println!("VM initializing...");
    cpdev.set_task_cycle(100);
    cpdev.initialize(RunMode::FIRSTSTART | RunMode::NORMAL);

    let out0 = Variable::new("OUT0", 0, 1);
    let out1 = Variable::new("OUT1", 1, 1);
    let out2 = Variable::new("OUT2", 2, 1);
    let out3 = Variable::new("OUT3", 3, 1);
    let onof = Variable::new("ONOF", 4, 1);

    let mut output: u8;

    while !cpdev.get_run_mode().is_empty() {
        let button = button.is_high();
        unsafe {
            cpdev.write_variable(&onof, button);
        }

        cpdev.run_cycle();

        unsafe {
            output = cpdev.read_variable(&out0);
            led1.set_level(if output == 1 { Level::High } else { Level::Low });
            output = cpdev.read_variable(&out1);
            led2.set_level(if output == 1 { Level::High } else { Level::Low });
            output = cpdev.read_variable(&out2);
            led3.set_level(if output == 1 { Level::High } else { Level::Low });
            output = cpdev.read_variable(&out3);
        }

        if output != 0 {
            led1.set_level(if output == 1 { Level::High } else { Level::Low });
            led2.set_level(if output == 1 { Level::High } else { Level::Low });
            led3.set_level(if output == 1 { Level::High } else { Level::Low });
        }
    }

    cpdev.shutdown();
}
