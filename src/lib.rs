#![doc = "Peripheral access API for NRF52810 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UARTE0_UART0();
    fn TWIM0_TWIS0_TWI0();
    fn SPIM0_SPIS0_SPI0();
    fn GPIOTE();
    fn SAADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn TEMP();
    fn RNG();
    fn ECB();
    fn CCM_AAR();
    fn WDT();
    fn RTC1();
    fn QDEC();
    fn COMP();
    fn SWI0_EGU0();
    fn SWI1_EGU1();
    fn SWI2();
    fn SWI3();
    fn SWI4();
    fn SWI5();
    fn PWM0();
    fn PDM();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 30] = [
    Vector {
        _handler: POWER_CLOCK,
    },
    Vector { _handler: RADIO },
    Vector {
        _handler: UARTE0_UART0,
    },
    Vector {
        _handler: TWIM0_TWIS0_TWI0,
    },
    Vector {
        _handler: SPIM0_SPIS0_SPI0,
    },
    Vector { _reserved: 0 },
    Vector { _handler: GPIOTE },
    Vector { _handler: SAADC },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: RTC0 },
    Vector { _handler: TEMP },
    Vector { _handler: RNG },
    Vector { _handler: ECB },
    Vector { _handler: CCM_AAR },
    Vector { _handler: WDT },
    Vector { _handler: RTC1 },
    Vector { _handler: QDEC },
    Vector { _handler: COMP },
    Vector {
        _handler: SWI0_EGU0,
    },
    Vector {
        _handler: SWI1_EGU1,
    },
    Vector { _handler: SWI2 },
    Vector { _handler: SWI3 },
    Vector { _handler: SWI4 },
    Vector { _handler: SWI5 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PWM0 },
    Vector { _handler: PDM },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK,
    #[doc = "1 - RADIO"]
    RADIO,
    #[doc = "2 - UARTE0_UART0"]
    UARTE0_UART0,
    #[doc = "3 - TWIM0_TWIS0_TWI0"]
    TWIM0_TWIS0_TWI0,
    #[doc = "4 - SPIM0_SPIS0_SPI0"]
    SPIM0_SPIS0_SPI0,
    #[doc = "6 - GPIOTE"]
    GPIOTE,
    #[doc = "7 - SAADC"]
    SAADC,
    #[doc = "8 - TIMER0"]
    TIMER0,
    #[doc = "9 - TIMER1"]
    TIMER1,
    #[doc = "10 - TIMER2"]
    TIMER2,
    #[doc = "11 - RTC0"]
    RTC0,
    #[doc = "12 - TEMP"]
    TEMP,
    #[doc = "13 - RNG"]
    RNG,
    #[doc = "14 - ECB"]
    ECB,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR,
    #[doc = "16 - WDT"]
    WDT,
    #[doc = "17 - RTC1"]
    RTC1,
    #[doc = "18 - QDEC"]
    QDEC,
    #[doc = "19 - COMP"]
    COMP,
    #[doc = "20 - SWI0_EGU0"]
    SWI0_EGU0,
    #[doc = "21 - SWI1_EGU1"]
    SWI1_EGU1,
    #[doc = "22 - SWI2"]
    SWI2,
    #[doc = "23 - SWI3"]
    SWI3,
    #[doc = "24 - SWI4"]
    SWI4,
    #[doc = "25 - SWI5"]
    SWI5,
    #[doc = "28 - PWM0"]
    PWM0,
    #[doc = "29 - PDM"]
    PDM,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::POWER_CLOCK => 0,
            Interrupt::RADIO => 1,
            Interrupt::UARTE0_UART0 => 2,
            Interrupt::TWIM0_TWIS0_TWI0 => 3,
            Interrupt::SPIM0_SPIS0_SPI0 => 4,
            Interrupt::GPIOTE => 6,
            Interrupt::SAADC => 7,
            Interrupt::TIMER0 => 8,
            Interrupt::TIMER1 => 9,
            Interrupt::TIMER2 => 10,
            Interrupt::RTC0 => 11,
            Interrupt::TEMP => 12,
            Interrupt::RNG => 13,
            Interrupt::ECB => 14,
            Interrupt::CCM_AAR => 15,
            Interrupt::WDT => 16,
            Interrupt::RTC1 => 17,
            Interrupt::QDEC => 18,
            Interrupt::COMP => 19,
            Interrupt::SWI0_EGU0 => 20,
            Interrupt::SWI1_EGU1 => 21,
            Interrupt::SWI2 => 22,
            Interrupt::SWI3 => 23,
            Interrupt::SWI4 => 24,
            Interrupt::SWI5 => 25,
            Interrupt::PWM0 => 28,
            Interrupt::PDM => 29,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Factory information configuration registers"]
pub struct FICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR {}
impl FICR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ficr::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for FICR {
    type Target = ficr::RegisterBlock;
    fn deref(&self) -> &ficr::RegisterBlock {
        unsafe { &*FICR::ptr() }
    }
}
#[doc = "Factory information configuration registers"]
pub mod ficr;
#[doc = "User information configuration registers"]
pub struct UICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR {}
impl UICR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uicr::RegisterBlock {
        268439552 as *const _
    }
}
impl Deref for UICR {
    type Target = uicr::RegisterBlock;
    fn deref(&self) -> &uicr::RegisterBlock {
        unsafe { &*UICR::ptr() }
    }
}
#[doc = "User information configuration registers"]
pub mod uicr;
#[doc = "Block Protect"]
pub struct BPROT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BPROT {}
impl BPROT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bprot::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for BPROT {
    type Target = bprot::RegisterBlock;
    fn deref(&self) -> &bprot::RegisterBlock {
        unsafe { &*BPROT::ptr() }
    }
}
#[doc = "Block Protect"]
pub mod bprot;
#[doc = "Clock control"]
pub struct CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK {}
impl CLOCK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clock::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for CLOCK {
    type Target = clock::RegisterBlock;
    fn deref(&self) -> &clock::RegisterBlock {
        unsafe { &*CLOCK::ptr() }
    }
}
#[doc = "Clock control"]
pub mod clock;
#[doc = "Power control"]
pub struct POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER {}
impl POWER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const power::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for POWER {
    type Target = power::RegisterBlock;
    fn deref(&self) -> &power::RegisterBlock {
        unsafe { &*POWER::ptr() }
    }
}
#[doc = "Power control"]
pub mod power;
#[doc = "GPIO Port"]
pub struct P0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for P0 {}
impl P0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const p0::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for P0 {
    type Target = p0::RegisterBlock;
    fn deref(&self) -> &p0::RegisterBlock {
        unsafe { &*P0::ptr() }
    }
}
#[doc = "GPIO Port"]
pub mod p0;
#[doc = "2.4 GHz Radio"]
pub struct RADIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RADIO {}
impl RADIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const radio::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for RADIO {
    type Target = radio::RegisterBlock;
    fn deref(&self) -> &radio::RegisterBlock {
        unsafe { &*RADIO::ptr() }
    }
}
#[doc = "2.4 GHz Radio"]
pub mod radio;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart0;
#[doc = "UART with EasyDMA"]
pub struct UARTE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UARTE0 {}
impl UARTE0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uarte0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for UARTE0 {
    type Target = uarte0::RegisterBlock;
    fn deref(&self) -> &uarte0::RegisterBlock {
        unsafe { &*UARTE0::ptr() }
    }
}
#[doc = "UART with EasyDMA"]
pub mod uarte0;
#[doc = "I2C compatible Two-Wire Interface"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &twi0::RegisterBlock {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Interface"]
pub mod twi0;
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub struct TWIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIM0 {}
impl TWIM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twim0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for TWIM0 {
    type Target = twim0::RegisterBlock;
    fn deref(&self) -> &twim0::RegisterBlock {
        unsafe { &*TWIM0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Master Interface with EasyDMA"]
pub mod twim0;
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub struct TWIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIS0 {}
impl TWIS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twis0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for TWIS0 {
    type Target = twis0::RegisterBlock;
    fn deref(&self) -> &twis0::RegisterBlock {
        unsafe { &*TWIS0::ptr() }
    }
}
#[doc = "I2C compatible Two-Wire Slave Interface with EasyDMA"]
pub mod twis0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub struct SPIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0 {}
impl SPIM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIM0 {
    type Target = spim0::RegisterBlock;
    fn deref(&self) -> &spim0::RegisterBlock {
        unsafe { &*SPIM0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface Master with EasyDMA"]
pub mod spim0;
#[doc = "SPI Slave"]
pub struct SPIS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS0 {}
impl SPIS0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIS0 {
    type Target = spis0::RegisterBlock;
    fn deref(&self) -> &spis0::RegisterBlock {
        unsafe { &*SPIS0::ptr() }
    }
}
#[doc = "SPI Slave"]
pub mod spis0;
#[doc = "GPIO Tasks and Events"]
pub struct GPIOTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE {}
impl GPIOTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiote::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for GPIOTE {
    type Target = gpiote::RegisterBlock;
    fn deref(&self) -> &gpiote::RegisterBlock {
        unsafe { &*GPIOTE::ptr() }
    }
}
#[doc = "GPIO Tasks and Events"]
pub mod gpiote;
#[doc = "Analog to Digital Converter"]
pub struct SAADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAADC {}
impl SAADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const saadc::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for SAADC {
    type Target = saadc::RegisterBlock;
    fn deref(&self) -> &saadc::RegisterBlock {
        unsafe { &*SAADC::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod saadc;
#[doc = "Timer/Counter 0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer/Counter 0"]
pub mod timer0;
#[doc = "Timer/Counter 1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Timer/Counter 2"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Real time counter 0"]
pub struct RTC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0 {}
impl RTC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for RTC0 {
    type Target = rtc0::RegisterBlock;
    fn deref(&self) -> &rtc0::RegisterBlock {
        unsafe { &*RTC0::ptr() }
    }
}
#[doc = "Real time counter 0"]
pub mod rtc0;
#[doc = "Temperature Sensor"]
pub struct TEMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMP {}
impl TEMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const temp::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for TEMP {
    type Target = temp::RegisterBlock;
    fn deref(&self) -> &temp::RegisterBlock {
        unsafe { &*TEMP::ptr() }
    }
}
#[doc = "Temperature Sensor"]
pub mod temp;
#[doc = "Random Number Generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1073795072 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random Number Generator"]
pub mod rng;
#[doc = "AES ECB Mode Encryption"]
pub struct ECB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECB {}
impl ECB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ecb::RegisterBlock {
        1073799168 as *const _
    }
}
impl Deref for ECB {
    type Target = ecb::RegisterBlock;
    fn deref(&self) -> &ecb::RegisterBlock {
        unsafe { &*ECB::ptr() }
    }
}
#[doc = "AES ECB Mode Encryption"]
pub mod ecb;
#[doc = "Accelerated Address Resolver"]
pub struct AAR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AAR {}
impl AAR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aar::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for AAR {
    type Target = aar::RegisterBlock;
    fn deref(&self) -> &aar::RegisterBlock {
        unsafe { &*AAR::ptr() }
    }
}
#[doc = "Accelerated Address Resolver"]
pub mod aar;
#[doc = "AES CCM Mode Encryption"]
pub struct CCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM {}
impl CCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccm::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for CCM {
    type Target = ccm::RegisterBlock;
    fn deref(&self) -> &ccm::RegisterBlock {
        unsafe { &*CCM::ptr() }
    }
}
#[doc = "AES CCM Mode Encryption"]
pub mod ccm;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real time counter 1"]
pub struct RTC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1 {}
impl RTC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for RTC1 {
    type Target = rtc0::RegisterBlock;
    fn deref(&self) -> &rtc0::RegisterBlock {
        unsafe { &*RTC1::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub struct QDEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QDEC {}
impl QDEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qdec::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for QDEC {
    type Target = qdec::RegisterBlock;
    fn deref(&self) -> &qdec::RegisterBlock {
        unsafe { &*QDEC::ptr() }
    }
}
#[doc = "Quadrature Decoder"]
pub mod qdec;
#[doc = "Comparator"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const comp::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    fn deref(&self) -> &comp::RegisterBlock {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Comparator"]
pub mod comp;
#[doc = "Event Generator Unit 0"]
pub struct EGU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU0 {}
impl EGU0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for EGU0 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU0::ptr() }
    }
}
#[doc = "Event Generator Unit 0"]
pub mod egu0;
#[doc = "Software interrupt 0"]
pub struct SWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI0 {}
impl SWI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for SWI0 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI0::ptr() }
    }
}
#[doc = "Software interrupt 0"]
pub mod swi0;
#[doc = "Event Generator Unit 1"]
pub struct EGU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EGU1 {}
impl EGU1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const egu0::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for EGU1 {
    type Target = egu0::RegisterBlock;
    fn deref(&self) -> &egu0::RegisterBlock {
        unsafe { &*EGU1::ptr() }
    }
}
#[doc = "Software interrupt 1"]
pub struct SWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI1 {}
impl SWI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for SWI1 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI1::ptr() }
    }
}
#[doc = "Software interrupt 2"]
pub struct SWI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI2 {}
impl SWI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073831936 as *const _
    }
}
impl Deref for SWI2 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI2::ptr() }
    }
}
#[doc = "Software interrupt 3"]
pub struct SWI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI3 {}
impl SWI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073836032 as *const _
    }
}
impl Deref for SWI3 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI3::ptr() }
    }
}
#[doc = "Software interrupt 4"]
pub struct SWI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI4 {}
impl SWI4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for SWI4 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI4::ptr() }
    }
}
#[doc = "Software interrupt 5"]
pub struct SWI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI5 {}
impl SWI5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi0::RegisterBlock {
        1073844224 as *const _
    }
}
impl Deref for SWI5 {
    type Target = swi0::RegisterBlock;
    fn deref(&self) -> &swi0::RegisterBlock {
        unsafe { &*SWI5::ptr() }
    }
}
#[doc = "Pulse width modulation unit"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &pwm0::RegisterBlock {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "Pulse width modulation unit"]
pub mod pwm0;
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
pub struct PDM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDM {}
impl PDM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pdm::RegisterBlock {
        1073860608 as *const _
    }
}
impl Deref for PDM {
    type Target = pdm::RegisterBlock;
    fn deref(&self) -> &pdm::RegisterBlock {
        unsafe { &*PDM::ptr() }
    }
}
#[doc = "Pulse Density Modulation (Digital Microphone) Interface"]
pub mod pdm;
#[doc = "Non-volatile memory controller"]
pub struct NVMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC {}
impl NVMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmc::RegisterBlock {
        1073864704 as *const _
    }
}
impl Deref for NVMC {
    type Target = nvmc::RegisterBlock;
    fn deref(&self) -> &nvmc::RegisterBlock {
        unsafe { &*NVMC::ptr() }
    }
}
#[doc = "Non-volatile memory controller"]
pub mod nvmc;
#[doc = "Programmable Peripheral Interconnect"]
pub struct PPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPI {}
impl PPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ppi::RegisterBlock {
        1073868800 as *const _
    }
}
impl Deref for PPI {
    type Target = ppi::RegisterBlock;
    fn deref(&self) -> &ppi::RegisterBlock {
        unsafe { &*PPI::ptr() }
    }
}
#[doc = "Programmable Peripheral Interconnect"]
pub mod ppi;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FICR"]
    pub FICR: FICR,
    #[doc = "UICR"]
    pub UICR: UICR,
    #[doc = "BPROT"]
    pub BPROT: BPROT,
    #[doc = "CLOCK"]
    pub CLOCK: CLOCK,
    #[doc = "POWER"]
    pub POWER: POWER,
    #[doc = "P0"]
    pub P0: P0,
    #[doc = "RADIO"]
    pub RADIO: RADIO,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UARTE0"]
    pub UARTE0: UARTE0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWIM0"]
    pub TWIM0: TWIM0,
    #[doc = "TWIS0"]
    pub TWIS0: TWIS0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPIM0"]
    pub SPIM0: SPIM0,
    #[doc = "SPIS0"]
    pub SPIS0: SPIS0,
    #[doc = "GPIOTE"]
    pub GPIOTE: GPIOTE,
    #[doc = "SAADC"]
    pub SAADC: SAADC,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "RTC0"]
    pub RTC0: RTC0,
    #[doc = "TEMP"]
    pub TEMP: TEMP,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "ECB"]
    pub ECB: ECB,
    #[doc = "AAR"]
    pub AAR: AAR,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC1"]
    pub RTC1: RTC1,
    #[doc = "QDEC"]
    pub QDEC: QDEC,
    #[doc = "COMP"]
    pub COMP: COMP,
    #[doc = "EGU0"]
    pub EGU0: EGU0,
    #[doc = "SWI0"]
    pub SWI0: SWI0,
    #[doc = "EGU1"]
    pub EGU1: EGU1,
    #[doc = "SWI1"]
    pub SWI1: SWI1,
    #[doc = "SWI2"]
    pub SWI2: SWI2,
    #[doc = "SWI3"]
    pub SWI3: SWI3,
    #[doc = "SWI4"]
    pub SWI4: SWI4,
    #[doc = "SWI5"]
    pub SWI5: SWI5,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "PDM"]
    pub PDM: PDM,
    #[doc = "NVMC"]
    pub NVMC: NVMC,
    #[doc = "PPI"]
    pub PPI: PPI,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FICR: FICR {
                _marker: PhantomData,
            },
            UICR: UICR {
                _marker: PhantomData,
            },
            BPROT: BPROT {
                _marker: PhantomData,
            },
            CLOCK: CLOCK {
                _marker: PhantomData,
            },
            POWER: POWER {
                _marker: PhantomData,
            },
            P0: P0 {
                _marker: PhantomData,
            },
            RADIO: RADIO {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UARTE0: UARTE0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            TWIM0: TWIM0 {
                _marker: PhantomData,
            },
            TWIS0: TWIS0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPIM0: SPIM0 {
                _marker: PhantomData,
            },
            SPIS0: SPIS0 {
                _marker: PhantomData,
            },
            GPIOTE: GPIOTE {
                _marker: PhantomData,
            },
            SAADC: SAADC {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            RTC0: RTC0 {
                _marker: PhantomData,
            },
            TEMP: TEMP {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            ECB: ECB {
                _marker: PhantomData,
            },
            AAR: AAR {
                _marker: PhantomData,
            },
            CCM: CCM {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC1: RTC1 {
                _marker: PhantomData,
            },
            QDEC: QDEC {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            EGU0: EGU0 {
                _marker: PhantomData,
            },
            SWI0: SWI0 {
                _marker: PhantomData,
            },
            EGU1: EGU1 {
                _marker: PhantomData,
            },
            SWI1: SWI1 {
                _marker: PhantomData,
            },
            SWI2: SWI2 {
                _marker: PhantomData,
            },
            SWI3: SWI3 {
                _marker: PhantomData,
            },
            SWI4: SWI4 {
                _marker: PhantomData,
            },
            SWI5: SWI5 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            PDM: PDM {
                _marker: PhantomData,
            },
            NVMC: NVMC {
                _marker: PhantomData,
            },
            PPI: PPI {
                _marker: PhantomData,
            },
        }
    }
}