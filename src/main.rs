//! This example test the RP Pico W on board LED.
//!
//! It does not work with the RP Pico board. See blinky.rs.

#![no_std]
#![no_main]

mod wifi;
mod messages;
mod lib_mgcrypto;

// use defmt::*;
use embassy_executor::Spawner;

use cortex_m_rt::entry;
use defmt::{debug, info, unwrap};
use embassy_executor::{Executor, InterruptExecutor};
use embassy_rp::interrupt;
use embassy_rp::interrupt::{InterruptExt, Priority};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0};
use embassy_time::{Duration, Instant, Timer, TICK_HZ};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::{spawn_core1, Stack};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use millegrilles_cryptographie::messages_structs::{MessageMilleGrillesBufferHeapless, CONST_NOMBRE_CERTIFICATS_MAX, CONST_BUFFER_MESSAGE_MIN};
// use messages::FormatMessageTest1;
use crate::lib_mgcrypto::{test_buffer_heapless, test_build_into_u8, test_hachage_1, test_signer_into};
// use {defmt_rtt as _, panic_probe as _};

static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
// static EXECUTOR_MED: InterruptExecutor = InterruptExecutor::new();
static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();


static mut CORE1_STACK: Stack<65536> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
// static CHANNEL: Channel<CriticalSectionRawMutex, LedState, 1> = Channel::new();



#[interrupt]
unsafe fn SWI_IRQ_2() {
    EXECUTOR_HIGH.on_interrupt()
}

#[embassy_executor::task]
async fn low_priority_loop() {
    let delay = Duration::from_secs(20);
    loop {
        debug!("Main - loop");
        Timer::after(delay).await;
    }
}

// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//
//     let p = embassy_rp::init(Default::default());
//     debug!("Run wifi");
//     unwrap!(spawner.spawn(wifi::run(
//         p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0
//     )));
// }

#[embassy_executor::task]
async fn core0_task() {
    loop {
        info!("Hello from core 0");
        Timer::after_secs(10).await;
    }
}

#[embassy_executor::task]
async fn core1_task() {
    loop {
        info!("Hello from core 1");
        Timer::after_secs(10).await;
    }
}

// #[entry]
// fn main() -> ! {
//     debug!("Hello world");
//
//     let p = embassy_rp::init(Default::default());
//
//     debug!("Spawn core 1");
//     spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
//         let executor1 = EXECUTOR1.init(Executor::new());
//         debug!("Spawning core 1");
//         executor1.run(|spawner| unwrap!(spawner.spawn(core1_task())));
//         // executor1.run(|spawner| unwrap!(
//         //     spawner.spawn(wifi::run(
//         //         p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0
//         //     ))
//         // ));
//     });
//
//     debug!("Spawn core0");
//     let executor0 = EXECUTOR0.init(Executor::new());
//     executor0.run(|spawner| unwrap!(spawner.spawn(core0_task())));
// }

// #[entry]
// fn main() -> ! {
//     debug!("Hello world");
//
//     let p = embassy_rp::init(Default::default());
//
//     debug!("Spawn core 1");
//     spawn_core1(p.CORE1, unsafe { &mut CORE1_STACK }, move || {
//
//         debug!("Preparer executor high priority");
//         // High-priority executor: SWI_IRQ_1, priority level 2
//         interrupt::SWI_IRQ_2.set_priority(Priority::P2);
//         let spawner = EXECUTOR_HIGH.start(interrupt::SWI_IRQ_2);
//         // let mut wifi_handler = wifi::setup(spawner).await;
//         // unwrap!(spawner.spawn(wifi::run(&mut p)));
//         unwrap!(spawner.spawn(wifi::run(
//             p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0
//         )));
//
//         debug!("Preparer executor low priority");
//         // Low priority executor: runs in thread mode, using WFE/SEV
//         let executor = EXECUTOR_LOW.init(Executor::new());
//         executor.run(|spawner| {
//             unwrap!(spawner.spawn(low_priority_loop()));
//         });
//
//         // let executor1 = EXECUTOR1.init(Executor::new());
//         // debug!("Spawning core 1");
//         // // executor1.run(|spawner| unwrap!(spawner.spawn(core1_task())));
//         // executor1.run(|spawner| unwrap!(
//         //     spawner.spawn(wifi::run(
//         //         p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0
//         //     ))
//         // ));
//     });
//
//     debug!("Spawn core0");
//     let executor0 = EXECUTOR0.init(Executor::new());
//     executor0.run(|spawner| unwrap!(spawner.spawn(core0_task())));
// }

#[embassy_executor::task]
async fn run_high() {
    loop {
        info!("        [high] tick!");
        Timer::after_ticks(673740).await;
    }
}

#[entry]
fn main() -> ! {
    let p = embassy_rp::init(Default::default());

    debug!("Preparer executor high priority");
    // High-priority executor: SWI_IRQ_1, priority level 2
    interrupt::SWI_IRQ_2.set_priority(Priority::P2);
    let spawner = EXECUTOR_HIGH.start(interrupt::SWI_IRQ_2);
    unwrap!(spawner.spawn(run_high()));
    // unwrap!(spawner.spawn(wifi::run(
    //     p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0
    // )));

    debug!("Preparer executor low priority");
    // Low priority executor: runs in thread mode, using WFE/SEV
    let executor = EXECUTOR_LOW.init(Executor::new());
    executor.run(|spawner| {
        unwrap!(spawner.spawn(run_low()));
        // unwrap!(spawner.spawn(wifi::run(
        //     p.PIN_23, p.PIN_25, p.PIO0, p.PIN_24, p.PIN_29, p.DMA_CH0
        // )))
    });
}

// #[entry]
// fn main() -> ! {
//     debug!("Start json test");
//     let contenu = r#"{
//         "valeur_text": "Du texte parsed",
//         "valeur_num": 12
//     }"#;
//     // unwrap!(messages::test_parse(contenu));
//     messages::test_parse(contenu);
//     debug!("Fin json test");
//     let valeur = FormatMessageTest1 {
//         valeur_text: "Du texte static",
//         valeur_num: -192,
//     };
//     messages::test_stringify(valeur);
//
//     panic!("Done!");
// }

//
// #[embassy_executor::task]
// async fn run_high() {
//     loop {
//         info!("        [high] tick!");
//         Timer::after_ticks(673740).await;
//     }
// }
//
// #[embassy_executor::task]
// async fn run_med() {
//     loop {
//         let start = Instant::now();
//         info!("    [med] Starting long computation");
//
//         // Spin-wait to simulate a long CPU computation
//         cortex_m::asm::delay(125_000_000); // ~1 second
//
//         let end = Instant::now();
//         let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
//         info!("    [med] done in {} ms", ms);
//
//         Timer::after_ticks(53421).await;
//     }
// }

#[embassy_executor::task]
async fn run_low() {
    loop {
        // let start = Instant::now();
        // info!("[low] Starting long computation");

        // // Spin-wait to simulate a long CPU computation
        // cortex_m::asm::delay(250_000_000); // ~2 seconds
        //
        // let end = Instant::now();
        // let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
        // info!("[low] done in {} ms", ms);

        // Timer::after_ticks(82983).await;
        run_tests();

        Timer::after_secs(5).await;
    }
}

fn run_tests() {
    let start = Instant::now();
    info!("[low] Debut hachage");
    test_hachage_1();
    let end = Instant::now();
    let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
    info!("[low] done in {} ms", ms);

    let start = Instant::now();
    info!("[low] Debut signer");
    test_signer_into();
    let end = Instant::now();
    let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
    info!("[low] done in {} ms", ms);

    let mut buffer: MessageMilleGrillesBufferHeapless<CONST_BUFFER_MESSAGE_MIN, CONST_NOMBRE_CERTIFICATS_MAX> =
        MessageMilleGrillesBufferHeapless::new();

    let start = Instant::now();
    info!("[low] Debut test verification message MilleGrilles");
    test_buffer_heapless(&mut buffer);
    let end = Instant::now();
    let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
    info!("[low] done in {} ms", ms);

    let start = Instant::now();
    info!("[low] Debut test build message MilleGrilles");
    test_build_into_u8(&mut buffer);
    let end = Instant::now();
    let ms = end.duration_since(start).as_ticks() * 1000 / TICK_HZ;
    info!("[low] done in {} ms", ms);
}

// static EXECUTOR_HIGH: InterruptExecutor = InterruptExecutor::new();
// static EXECUTOR_MED: InterruptExecutor = InterruptExecutor::new();
// static EXECUTOR_LOW: StaticCell<Executor> = StaticCell::new();
//
// #[interrupt]
// unsafe fn SWI_IRQ_1() {
//     EXECUTOR_HIGH.on_interrupt()
// }
//
// #[interrupt]
// unsafe fn SWI_IRQ_0() {
//     EXECUTOR_MED.on_interrupt()
// }
//
// #[entry]
// fn main() -> ! {
//     info!("Hello World!");
//
//     let _p = embassy_rp::init(Default::default());
//
//     // High-priority executor: SWI_IRQ_1, priority level 2
//     interrupt::SWI_IRQ_1.set_priority(Priority::P2);
//     let spawner = EXECUTOR_HIGH.start(interrupt::SWI_IRQ_1);
//     unwrap!(spawner.spawn(run_high()));
//
//     // Medium-priority executor: SWI_IRQ_0, priority level 3
//     interrupt::SWI_IRQ_0.set_priority(Priority::P3);
//     let spawner = EXECUTOR_MED.start(interrupt::SWI_IRQ_0);
//     unwrap!(spawner.spawn(run_med()));
//
//     // Low priority executor: runs in thread mode, using WFE/SEV
//     let executor = EXECUTOR_LOW.init(Executor::new());
//     executor.run(|spawner| {
//         unwrap!(spawner.spawn(run_low()));
//     });
// }