#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{USB, I2C0};
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::{gpio, i2c};
use embassy_time::Timer;
use embassy_usb_logger;
use gpio::{Level, Output, Input, Pull};
use {defmt_rtt as _, panic_probe as _};

use embassy_time::Duration;

mod msa301;

const BMA_530_ADDRESS: u8 = 0x18;

bind_interrupts!(struct Irqs {
	I2C0_IRQ => InterruptHandler<I2C0>;
	USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
	embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
	let p = embassy_rp::init(Default::default());

	// setup logging
	let driver = Driver::new(p.USB, Irqs);
	spawner.spawn(logger_task(driver)).unwrap();

	// i2c pins
    let sda = p.PIN_16;
    let scl = p.PIN_17;

	let mut i2c = i2c::I2c::new_async(p.I2C0, scl, sda, Irqs, i2c::Config::default());

	let led_0 = Output::new(p.PIN_25, Level::Low);
	let led_1 = Output::new(p.PIN_26, Level::High);
	let mut input_0 = Input::new(p.PIN_7, Pull::None);
	
	spawner.spawn(blink_light(led_0, Duration::from_millis(500))).unwrap();

	input_0.wait_for_high().await;
	spawner.spawn(blink_light(led_1, Duration::from_hz(30))).unwrap();
	
	// read somethin from i2c
	let mut buf = [0_u8; 1];
	i2c.write_read_async(BMA_530_ADDRESS, [0x02], &mut buf).await.unwrap();

	if buf[0] == 0x0F {
		info!("BMA-530 is chillin");
	}

	loop {
		info!("Hello There!");
	}
}


#[embassy_executor::task(pool_size = 2)]
async fn blink_light(mut light: Output<'static>, half_period: Duration) {
	loop {
		light.set_high();
		Timer::after(half_period).await;

		light.set_low();
		Timer::after(half_period).await;
	}
}