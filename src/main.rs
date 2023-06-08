#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use panic_rtt_target as _;
use rtic::app;

#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SDIO])]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        (Shared {}, Local {})
    }
}
