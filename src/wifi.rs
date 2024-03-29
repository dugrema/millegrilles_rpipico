//! This example test the RP Pico W on board LED.
//!
//! It does not work with the RP Pico board. See blinky.rs.

use core::cell::{Cell, RefCell};
use core::str;
use cortex_m::interrupt::Mutex;

use cyw43::Control;
use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, Peripherals};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
use embedded_hal_1::digital::OutputPin;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn wifi_task(runner: cyw43::Runner<'static, Output<'static, PIN_23>, PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

pub struct WifiHandler<'a> {
    control: Control<'a>,
}

impl<'a> WifiHandler<'a> {
    pub async fn blink(&mut self, delay: Duration) {
        debug!("led on!");
        self.control.gpio_set(0, true).await;
        Timer::after(delay).await;

        debug!("led off!");
        self.control.gpio_set(0, false).await;
    }

    pub async fn scan(&mut self) {
        let mut scanner = self.control.scan().await;
        while let Some(bss) = scanner.next().await {
            if let Ok(ssid_str) = str::from_utf8(&bss.ssid) {
                info!("scanned {} == {:x}", ssid_str, bss.bssid);
            }
        }
    }

    // pub async fn connect(&mut self, ssid: &str, passphrase: &str) -> Result<(), >{
    //     let resultat = self.control.join_wpa2(ssid, passphrase).await;
    // }
}

#[embassy_executor::task]
pub async fn run(pin_23: PIN_23, pin_25: PIN_25, pio0: PIO0, pin_24: PIN_24, pin_29: PIN_29, dma_ch0: DMA_CH0) -> ! {
    debug!("Start wifi");

    // let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");
    debug!("Code wifi importe fw len: {}, clm len: {}", fw.len(), clm.len());

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 230321) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    // let pwr = Output::new(p.PIN_23, Level::Low);
    // let cs = Output::new(p.PIN_25, Level::High);
    // let mut pio = Pio::new(p.PIO0, Irqs);
    // let spi = PioSpi::new(&mut pio.common, pio.sm0, pio.irq0, cs, p.PIN_24, p.PIN_29, p.DMA_CH0);

    let pwr = Output::new(pin_23, Level::Low);
    let cs = Output::new(pin_25, Level::High);
    let mut pio = Pio::new(pio0, Irqs);
    let spi = PioSpi::new(&mut pio.common, pio.sm0, pio.irq0, cs, pin_24, pin_29, dma_ch0);

    let (_net_device, mut control, runner) = {
        static STATE: StaticCell<cyw43::State> = StaticCell::new();
        let state = STATE.init(cyw43::State::new());
        cyw43::new(state, pwr, spi, fw).await
    };
    debug!("Recuperer spawner");
    let spawner = Spawner::for_current_executor().await;
    unwrap!(spawner.spawn(wifi_task(runner)));

    debug!("Init controle");
    control.init(clm).await;
    debug!("Set wifi powersave");
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let mut wifi_handler = WifiHandler { control };

    // let delay = Duration::from_secs(5);
    // loop {
    //     debug!("wifi Blink");
    //     wifi_handler.blink(delay.clone()).await;
    //     Timer::after(delay).await;
    // }

    let delay_blink = Duration::from_secs(1);
    loop {
        debug!("wifi Scan");
        wifi_handler.scan().await;
        debug!("wifi Blink");
        for i in 1..10 {
            wifi_handler.blink(delay_blink.clone()).await;
            Timer::after(delay_blink).await;
        }
    }
}

