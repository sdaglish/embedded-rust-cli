#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

// use panic_rtt_target as _;
use panic_reset as _;
use rtic::app;

#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SDIO])]
mod app {
    use rtic_monotonics::systick::*;
    use stm32f4xx_hal::prelude::*;

    use stm32f4xx_hal::{
        pac::USART2,
        prelude::*,
        serial::{config::Config, Rx, Serial, Tx},
    };

    use heapless::spsc::{Consumer, Producer, Queue};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        serial_debug_tx: Tx<USART2>,
        serial_debug_rx: Rx<USART2>,

        serial_debug_producer: Producer<'static, u8, 256>,
        serial_debug_consumer: Consumer<'static, u8, 256>,
    }

    #[init(local = [serial_debug_queue: Queue<u8, 256> = Queue::new()])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let dp = cx.device;
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(16_000_000.Hz()).freeze();

        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 50_000_000, systick_mono_token);

        // Uart Rx is PA3, Tx is PA2
        let gpioa = dp.GPIOA.split();
        let tx = gpioa.pa2.into_alternate();
        let rx = gpioa.pa3.into_alternate();

        let (serial_debug_tx, mut serial_debug_rx) = Serial::new(
            dp.USART2,
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            &clocks,
        )
        .unwrap()
        .split();

        serial_debug_rx.listen();

        let (serial_debug_producer, serial_debug_consumer) = cx.local.serial_debug_queue.split();

        (
            Shared {},
            Local {
                serial_debug_tx,
                serial_debug_rx,
                serial_debug_producer,
                serial_debug_consumer,
            },
        )
    }

    #[task(binds = USART2, local = [serial_debug_rx, serial_debug_producer])]
    fn usart_rx(cx: usart_rx::Context) {
        let rx = cx.local.serial_debug_rx;
        if let Ok(byte) = rx.read() {
            cx.local.serial_debug_producer.enqueue(byte).ok();
        }
    }
}
