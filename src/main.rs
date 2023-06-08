#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use panic_rtt_target as _;
use rtic::app;

#[app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SDIO])]
mod app {
    use rtic_monotonics::systick::*;
    use stm32f4xx_hal::prelude::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let dp = cx.device;
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(16_000_000.Hz()).freeze();

        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(cx.core.SYST, 50_000_000, systick_mono_token);

        (Shared {}, Local {})
    }
}
