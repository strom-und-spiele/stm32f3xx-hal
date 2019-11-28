//! # Work in Progress
//! API for the ADC1 and ADC2 (Analog to Digital Converter) on the 303xB/C/D/E
//!
//! # Examples
//! check `adc.rs` in the examples folder
//!
use cortex_m::asm;
use embedded_hal::adc::{Channel, OneShot};

use crate::rcc::{Clocks, AHB};

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
use crate::gpio::{gpioa, gpiob, gpioc, gpiod, gpioe, gpiof, Analog};

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
use crate::stm32::{ADC1, ADC1_2, ADC2, ADC3, ADC3_4, ADC4};

/// ADC configuration
pub struct Adc<ADC> {
    pub rb: ADC,
    clocks: Clocks,
    prescale: Prescale,
    operation_mode: Option<OperationMode>,
}

#[derive(Clone, Copy, Debug)]
/// ADC sampling time
///
/// each channel can be sampled with a different sample time.
/// the total conversion time is
/// 12.5 ADC clock cycles + sample time (T_x + .5)
///
/// TODO: there are boundaries on how this can be set depending on the hardware.
pub enum SampleTime {
    T_1,
    T_2,
    T_4,
    T_7,
    T_19,
    T_61,
    T_181,
    T_601,
}

impl SampleTime {
    /// Get the default timer
    pub fn default() -> Self {
        SampleTime::T_19
    }

    /// Conversion to bits for SMP
    fn bitcode(&self) -> u8 {
        match self {
            SampleTime::T_1 => 0b000,
            SampleTime::T_2 => 0b001,
            SampleTime::T_4 => 0b010,
            SampleTime::T_7 => 0b011,
            SampleTime::T_19 => 0b100,
            SampleTime::T_61 => 0b101,
            SampleTime::T_181 => 0b110,
            SampleTime::T_601 => 0b111,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
/// ADC operation mode
pub enum OperationMode {
    OneShot,
    // TODO all the other modes
}

#[derive(Clone, Copy, Debug)]
/// ADC prescale
pub enum Prescale {
    // HCLK_1 needs some requirements to be met
    HCLK_2 = 2,
    HCLK_4 = 4,
}

impl Prescale {
    /// Get default prescaler
    fn default() -> Self {
        Prescale::HCLK_2
    }

    /// Conversion to bits for CKMODE in ADCx_CCR
    fn bitcode(&self) -> u8 {
        match self {
            Prescale::HCLK_2 => 0b10,
            Prescale::HCLK_4 => 0b11,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// ADC data register alignment
pub enum Align {
    /// Right alignment of output data
    Right,
    /// Left alignment of output data
    Left,
}

impl Align {
    /// Default: right alignment
    pub fn default() -> Self {
        Align::Right
    }
}

impl Align {
    /// Conversion to bits for ALIGN in ADCx_CFGR
    fn bitvalue(&self) -> bool {
        match self {
            Align::Right => false,
            Align::Left => true,
        }
    }
}

macro_rules! adc_pins {
    ($ADC:ident, $($pin:ty => $chan:expr),+ $(,)*) => {
        $(
            impl Channel<$ADC> for $pin {
                type ID = u8;

                fn channel() -> u8 { $chan }
            }
        )+
    };
}

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
adc_pins!(ADC1,
    gpioa::PA0<Analog> => 1_u8,
    gpioa::PA1<Analog> => 2_u8,
    gpioa::PA2<Analog> => 3_u8,
    gpioa::PA3<Analog> => 4_u8,
    gpiof::PF4<Analog> => 5_u8,
    // Channels 6 to 10 are shared channels (i.e. ADC12_INx)
    gpioc::PC0<Analog> => 6_u8,
    gpioc::PC1<Analog> => 7_u8,
    gpioc::PC2<Analog> => 8_u8,
    gpioc::PC3<Analog> => 9_u8,
    gpiof::PF2<Analog> => 10_u8,
);

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
adc_pins!(ADC2,
    gpioa::PA4<Analog> => 1_u8,
    gpioa::PA5<Analog> => 2_u8,
    gpioa::PA6<Analog> => 3_u8,
    gpioa::PA7<Analog> => 4_u8,
    gpioc::PC4<Analog> => 5_u8,
    gpioc::PC5<Analog> => 11_u8,
    gpiob::PB2<Analog> => 12_u8,
    // Channels 6 to 10 are shared channels (i.e. ADC12_INx)
    gpioc::PC0<Analog> => 6_u8,
    gpioc::PC1<Analog> => 7_u8,
    gpioc::PC2<Analog> => 8_u8,
    gpioc::PC3<Analog> => 9_u8,
    gpiof::PF2<Analog> => 10_u8,
);

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
adc_pins!(ADC3,
    gpiob::PB1<Analog> => 1_u8,
    gpioe::PE9<Analog> => 2_u8,
    gpioe::PE13<Analog> => 3_u8,
    gpiob::PB13<Analog> => 5_u8,
    gpiob::PB0<Analog> => 12_u8,
    gpioe::PE7<Analog> => 13_u8,
    gpioe::PE10<Analog> => 14_u8,
    gpioe::PE11<Analog> => 15_u8,
    gpioe::PE12<Analog> => 16_u8,
    // Shared channels (i.e. ADC34_INx)
    gpioe::PE8<Analog> => 6_u8,
    gpiod::PD10<Analog> => 7_u8,
    gpiod::PD11<Analog> => 8_u8,
    gpiod::PD12<Analog> => 9_u8,
    gpiod::PD13<Analog> => 10_u8,
    gpiod::PD14<Analog> => 11_u8,
);

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
adc_pins!(ADC4,
    gpioe::PE14<Analog> => 1_u8,
    gpioe::PE15<Analog> => 2_u8,
    gpiob::PB12<Analog> => 3_u8,
    gpiob::PB14<Analog> => 4_u8,
    gpiob::PB15<Analog> => 5_u8,
    gpiob::PB8<Analog> => 12_u8,
    gpiob::PB9<Analog> => 13_u8,
    // Shared channels (i.e. ADC34_INx)
    gpioe::PE8<Analog> => 6_u8,
    gpiod::PD10<Analog> => 7_u8,
    gpiod::PD11<Analog> => 8_u8,
    gpiod::PD12<Analog> => 9_u8,
    gpiod::PD13<Analog> => 10_u8,
    gpiod::PD14<Analog> => 11_u8,
);

macro_rules! adc_hal {
    ($(
            $ADC:ident: ($init:ident, $ADC_COMMON:ident),
    )+) => {
        $(
            impl Adc<$ADC> {

                /// Init a new ADC
                ///
                /// Enables the clock, performs a calibration and enables the ADC
                pub fn $init(
                    rb: $ADC,
                    adc_common : &mut $ADC_COMMON,
                    ahb: &mut AHB,
                    clocks: Clocks,
                ) -> Self {
                    let mut this_adc = Self {
                        rb,
                        clocks,
                        prescale : Prescale::default(),
                        operation_mode: None,
                    };
                    this_adc.enable_clock(ahb, adc_common);
                    this_adc.set_align(Align::default());
                    this_adc.calibrate();
                    // ADEN bit cannot be set during ADCAL=1 and 4 ADC clock cycle after the ADCAL
                    // bit is cleared by hardware
                    this_adc.wait_adc_clk_cycles(4);
                    this_adc.enable();
                    return this_adc;
                }

                /// sets up adc in one shot mode for a single channel
                pub fn setup_oneshot(&mut self) {
                    // stop and clear overrun events
                    self.rb.cr.modify(|_, w| w.adstp().set_bit());
                    self.rb.isr.modify(|_, w| w.ovr().clear_bit());

                    self.rb.cfgr.modify(|_, w| w
                        .cont()     .clear_bit()
                        .ovrmod()   .clear_bit()
                    );

                    self.rb.sqr1.modify(|_, w|
                        // NOTE(unsafe): set the sequence length to 1
                        unsafe { w.l3().bits(0) }
                    );

                    self.operation_mode = Some(OperationMode::OneShot);
                }

                fn set_align(&self, align: Align) {
                    self.rb.cfgr.modify(|_, w| w.align().bit(align.bitvalue()));
                }

                fn enable(&mut self) {
                    self.rb.cr.modify(|_, w| w.aden().set_bit());
                    while self.rb.isr.read().adrdy().bit_is_clear() {}
                }

                fn disable(&mut self) {
                    self.rb.cr.modify(|_, w| w.aden().clear_bit());
                }


                /// Calibrate according to 15.3.8 in the Reference Manual
                fn calibrate(&mut self) {
                    if !self.advregen_enabled() {
                        self.advregen_enable();
                        self.wait_advregen_startup();
                    }

                    self.disable();

                    self.rb.cr.modify(|_, w| w
                        // NOTE: needs to be adopted if implementing differential input
                        .adcaldif().clear_bit()
                        .adcal()   .set_bit());

                    while self.rb.cr.read().adcal().bit_is_set() {}
                }


                fn wait_adc_clk_cycles(&self, cycles: u32) {
                    let adc_clk_cycle = self.clocks.hclk().0 / (self.prescale as u32);
                    asm::delay(adc_clk_cycle * cycles);
                }


                fn advregen_enabled(&self) -> bool {
                    return self._get_new_advregen() == 0b01;
                }

                fn advregen_enable(&mut self){
                    // need to go though 00 first
                    self._set_new_advregen(0b00);
                    self._set_new_advregen(0b01);
                }

                /// returns ADVREGEN[1:0]
                /// (deeppwd got merged as high bit in advregen - see ref manual)
                fn _get_new_advregen(&self) -> u8 {
                    return
                        (self.rb.cr.read().deeppwd().bit() as u8) << 1 |
                        (self.rb.cr.read().advregen().bit() as u8);
                }

                /// sets ADVREGEN[1:0]
                /// (deeppwd got merged as high bit in advregen - see ref manual)
                fn _set_new_advregen(&mut self, val: u8) {
                    self.rb.cr.modify(|_, w| { w
                        .deeppwd().bit((val & 0x02) != 0)
                            .advregen().bit((val & 0x01) != 0)
                    });
                }

                #[cfg(any(
                    feature = "stm32f303xb",
                    feature = "stm32f303xc",
                    feature = "stm32f303xd",
                    feature = "stm32f303xe",
                ))]
                fn wait_advregen_startup(&self) {
                    const MAX_STARTUP_TIME_US: u32 = 10;
                    asm::delay(MAX_STARTUP_TIME_US / (self.clocks.sysclk().0 /1_000_000));
                }

                fn convert_one(&mut self, chan: u8) -> u16 {
                    self.ensure_oneshot();
                    self.set_chan_smps(chan, SampleTime::default());
                    self.select_single_chan(chan);

                    self.rb.cr.modify(|_, w| w.adstart().set_bit());
                    while self.rb.isr.read().eos().bit_is_clear() {}
                    return self.rb.dr.read().regular_data().bits();
                }

                fn ensure_oneshot(&mut self) {
                    match self.operation_mode {
                        Some(mode) =>
                        {
                            if mode != OperationMode::OneShot {
                                self.setup_oneshot();
                            }
                        },
                        _ => self.setup_oneshot(),
                    };
                }

                fn select_single_chan(&self, chan: u8) {
                    self.rb.sqr1.modify(|_, w|
                        // NOTE(unsafe): set the ADC_INx
                        unsafe { w.sq1().bits(chan) }
                    );
                }

                // Note: only allowed when ADSTART = 0
                fn set_chan_smps(&self, chan: u8, smp: SampleTime) {
                    match chan {
                        1 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp1().bits(smp.bitcode())}),
                        2 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp2().bits(smp.bitcode())}),
                        3 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp3().bits(smp.bitcode())}),
                        4 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp4().bits(smp.bitcode())}),
                        5 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp5().bits(smp.bitcode())}),
                        6 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp6().bits(smp.bitcode())}),
                        7 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp7().bits(smp.bitcode())}),
                        8 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp8().bits(smp.bitcode())}),
                        9 => self.rb.smpr1.modify(|_, w|
                            unsafe {w.smp9().bits(smp.bitcode())}),
                        11 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp10().bits(smp.bitcode())}),
                        12 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp12().bits(smp.bitcode())}),
                        13 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp13().bits(smp.bitcode())}),
                        14 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp14().bits(smp.bitcode())}),
                        15 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp15().bits(smp.bitcode())}),
                        16 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp16().bits(smp.bitcode())}),
                        17 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp17().bits(smp.bitcode())}),
                        18 => self.rb.smpr2.modify(|_, w|
                            unsafe {w.smp18().bits(smp.bitcode())}),
                        _ => unreachable!(),
                    };
                }

            }

            impl<WORD, PIN> OneShot<$ADC, WORD, PIN> for Adc<$ADC>
            where
                WORD: From<u16>,
                PIN: Channel<$ADC, ID = u8>,
                {
                    type Error = ();

                    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
                        let res = self.convert_one(PIN::channel());
                        return Ok(res.into());
                    }
                }
        )+
    }
}

macro_rules! adc12_hal {
    ($(
            $ADC:ident: ($init:ident),
    )+) => {
        $(
            impl Adc<$ADC> {
                fn enable_clock(&self, ahb: &mut AHB, adc_common: &mut ADC1_2) {
                    ahb.enr().modify(|_, w| w.adc12en().enabled());
                    unsafe {
                        adc_common.ccr.modify(|_, w| w
                            .ckmode().bits(self.prescale.bitcode())
                        );
                    }
                }
            }
        )+
    adc_hal! {
        $ADC: ($init, ADC1_2)
    }
}

macro_rules! adc34_hal {
    ($(
            $ADC:ident: ($init:ident),
    )+) => {
        $(
            impl Adc<$ADC> {
                fn enable_clock(&self, ahb: &mut AHB, adc_common: &mut ADC3_4) {
                    ahb.enr().modify(|_, w| w.adc34en().enabled());
                    unsafe {
                        adc_common.ccr.modify(|_, w| w
                            .ckmode().bits(self.prescale.bitcode())
                        );
                    }
                }
            }
        )+
    adc_hal! {
        $ADC: ($init, ADC3_4)
    }
}

#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
adc12_hal! {
    ADC1: (adc1),
    ADC2: (adc2),
}
#[cfg(any(
    feature = "stm32f303xb",
    feature = "stm32f303xc",
    feature = "stm32f303xd",
    feature = "stm32f303xe",
))]
adc34_hal! {
    ADC3: (adc3),
    ADC4: (adc4),
}
