#![no_std]
#![allow(non_camel_case_types)]

#[cfg(not(feature = "device-selected"))]
compile_error!(
    "This crate requires you to specify your target device as a feautre.  
    Note that the more specific you pic the device, the more functionality will be available.

    See `Cargo.toml` section 'features' for a comprehensive list of devices (starting with stm32f...)."
);

pub use embedded_hal as hal;

pub use nb;
pub use nb::block;

#[cfg(feature = "stm32f301")]
pub use stm32f3::stm32f301 as stm32;

#[cfg(feature = "stm32f302")]
pub use stm32f3::stm32f302 as stm32;

#[cfg(feature = "stm32f303")]
pub use stm32f3::stm32f303 as stm32;

#[cfg(feature = "stm32f373")]
pub use stm32f3::stm32f373 as stm32;

#[cfg(feature = "stm32f334")]
pub use stm32f3::stm32f3x4 as stm32;

#[cfg(any(
    feature = "stm32f318",
    feature = "stm32f328",
    feature = "stm32f358",
    feature = "stm32f378",
    feature = "stm32f398"
))]
pub use stm32f3::stm32f3x8 as stm32;

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use crate::stm32::interrupt;

#[cfg(feature = "stm32f303")]
pub mod adc;
#[cfg(feature = "device-selected")]
pub mod delay;
#[cfg(feature = "device-selected")]
pub mod flash;
#[cfg(feature = "device-selected")]
pub mod gpio;
#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(feature = "device-selected")]
pub mod prelude;
#[cfg(feature = "device-selected")]
pub mod pwm;
#[cfg(feature = "device-selected")]
pub mod rcc;
#[cfg(feature = "device-selected")]
pub mod serial;
#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub mod time;
#[cfg(feature = "device-selected")]
pub mod timer;
#[cfg(all(
    feature = "stm32-usbd",
    any(
        feature = "stm32f303xb",
        feature = "stm32f303xc",
        feature = "stm32f303xd",
        feature = "stm32f303xe",
    )
))]
pub mod usb;
