#![doc = "Peripheral access API for STM32F40X microcontrollers(generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG_IRQ();
    fn PVD_IRQ();
    fn TAMP_STAMP_IRQ();
    fn RTC_WKUP_IRQ();
    fn RCC_IRQ();
    fn EXTI0_IRQ();
    fn EXTI1_IRQ();
    fn EXTI2_IRQ();
    fn EXTI3_IRQ();
    fn EXTI4_IRQ();
    fn DMA1_STREAM0_IRQ();
    fn DMA1_STREAM1_IRQ();
    fn DMA1_STREAM2_IRQ();
    fn DMA1_STREAM3_IRQ();
    fn DMA1_STREAM4_IRQ();
    fn DMA1_STREAM5_IRQ();
    fn DMA1_STREAM6_IRQ();
    fn ADC_IRQ();
    fn CAN1_TX_IRQ();
    fn CAN1_RX0_IRQ();
    fn CAN1_RX1_IRQ();
    fn CAN1_SCE_IRQ();
    fn EXTI9_5_IRQ();
    fn TIM1_BRK_TIM9_IRQ();
    fn TIM1_UP_TIM10_IRQ();
    fn TIM1_TRG_COM_TIM11_IRQ();
    fn TIM1_CC_IRQ();
    fn TIM2_IRQ();
    fn TIM3_IRQ();
    fn TIM4_IRQ();
    fn I2C1_EV_IRQ();
    fn I2C1_ER_IRQ();
    fn I2C2_EV_IRQ();
    fn I2C2_ER_IRQ();
    fn SPI1_IRQ();
    fn SPI2_IRQ();
    fn USART1_IRQ();
    fn USART2_IRQ();
    fn USART3_IRQ();
    fn EXTI15_10_IRQ();
    fn RTC_ALARM_IRQ();
    fn OTG_FS_WKUP_IRQ();
    fn TIM8_BRK_TIM12_IRQ();
    fn TIM8_UP_TIM13_IRQ();
    fn TIM8_TRG_COM_TIM14_IRQ();
    fn TIM8_CC_IRQ();
    fn DMA1_STREAM7_IRQ();
    fn FSMC_IRQ();
    fn SDIO_IRQ();
    fn TIM5_IRQ();
    fn SPI3_IRQ();
    fn UART4_IRQ();
    fn UART5_IRQ();
    fn TIM6_DAC_IRQ();
    fn TIM7_IRQ();
    fn DMA2_STREAM0_IRQ();
    fn DMA2_STREAM1_IRQ();
    fn DMA2_STREAM2_IRQ();
    fn DMA2_STREAM3_IRQ();
    fn DMA2_STREAM4_IRQ();
    fn ETH_IRQ();
    fn ETH_WKUP_IRQ();
    fn CAN2_TX_IRQ();
    fn CAN2_RX0_IRQ();
    fn CAN2_RX1_IRQ();
    fn CAN2_SCE_IRQ();
    fn OTG_FS_IRQ();
    fn DMA2_STREAM5_IRQ();
    fn DMA2_STREAM6_IRQ();
    fn DMA2_STREAM7_IRQ();
    fn USART6_IRQ();
    fn I2C3_EV_IRQ();
    fn I2C3_ER_IRQ();
    fn OTG_HS_EP1_OUT_IRQ();
    fn OTG_HS_EP1_IN_IRQ();
    fn OTG_HS_WKUP_IRQ();
    fn OTG_HS_IRQ();
    fn DCMI_IRQ();
    fn FPU();
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
pub static __INTERRUPTS: [Vector; 82] = [
    Vector { _handler: WWDG_IRQ },
    Vector { _handler: PVD_IRQ },
    Vector { _handler: TAMP_STAMP_IRQ },
    Vector { _handler: RTC_WKUP_IRQ },
    Vector { _reserved: 0 },
    Vector { _handler: RCC_IRQ },
    Vector { _handler: EXTI0_IRQ },
    Vector { _handler: EXTI1_IRQ },
    Vector { _handler: EXTI2_IRQ },
    Vector { _handler: EXTI3_IRQ },
    Vector { _handler: EXTI4_IRQ },
    Vector { _handler: DMA1_STREAM0_IRQ },
    Vector { _handler: DMA1_STREAM1_IRQ },
    Vector { _handler: DMA1_STREAM2_IRQ },
    Vector { _handler: DMA1_STREAM3_IRQ },
    Vector { _handler: DMA1_STREAM4_IRQ },
    Vector { _handler: DMA1_STREAM5_IRQ },
    Vector { _handler: DMA1_STREAM6_IRQ },
    Vector { _handler: ADC_IRQ },
    Vector { _handler: CAN1_TX_IRQ },
    Vector { _handler: CAN1_RX0_IRQ },
    Vector { _handler: CAN1_RX1_IRQ },
    Vector { _handler: CAN1_SCE_IRQ },
    Vector { _handler: EXTI9_5_IRQ },
    Vector { _handler: TIM1_BRK_TIM9_IRQ },
    Vector { _handler: TIM1_UP_TIM10_IRQ },
    Vector { _handler: TIM1_TRG_COM_TIM11_IRQ },
    Vector { _handler: TIM1_CC_IRQ },
    Vector { _handler: TIM2_IRQ },
    Vector { _handler: TIM3_IRQ },
    Vector { _handler: TIM4_IRQ },
    Vector { _handler: I2C1_EV_IRQ },
    Vector { _handler: I2C1_ER_IRQ },
    Vector { _handler: I2C2_EV_IRQ },
    Vector { _handler: I2C2_ER_IRQ },
    Vector { _handler: SPI1_IRQ },
    Vector { _handler: SPI2_IRQ },
    Vector { _handler: USART1_IRQ },
    Vector { _handler: USART2_IRQ },
    Vector { _handler: USART3_IRQ },
    Vector { _handler: EXTI15_10_IRQ },
    Vector { _handler: RTC_ALARM_IRQ },
    Vector { _handler: OTG_FS_WKUP_IRQ },
    Vector { _handler: TIM8_BRK_TIM12_IRQ },
    Vector { _handler: TIM8_UP_TIM13_IRQ },
    Vector { _handler: TIM8_TRG_COM_TIM14_IRQ },
    Vector { _handler: TIM8_CC_IRQ },
    Vector { _handler: DMA1_STREAM7_IRQ },
    Vector { _handler: FSMC_IRQ },
    Vector { _handler: SDIO_IRQ },
    Vector { _handler: TIM5_IRQ },
    Vector { _handler: SPI3_IRQ },
    Vector { _handler: UART4_IRQ },
    Vector { _handler: UART5_IRQ },
    Vector { _handler: TIM6_DAC_IRQ },
    Vector { _handler: TIM7_IRQ },
    Vector { _handler: DMA2_STREAM0_IRQ },
    Vector { _handler: DMA2_STREAM1_IRQ },
    Vector { _handler: DMA2_STREAM2_IRQ },
    Vector { _handler: DMA2_STREAM3_IRQ },
    Vector { _handler: DMA2_STREAM4_IRQ },
    Vector { _handler: ETH_IRQ },
    Vector { _handler: ETH_WKUP_IRQ },
    Vector { _handler: CAN2_TX_IRQ },
    Vector { _handler: CAN2_RX0_IRQ },
    Vector { _handler: CAN2_RX1_IRQ },
    Vector { _handler: CAN2_SCE_IRQ },
    Vector { _handler: OTG_FS_IRQ },
    Vector { _handler: DMA2_STREAM5_IRQ },
    Vector { _handler: DMA2_STREAM6_IRQ },
    Vector { _handler: DMA2_STREAM7_IRQ },
    Vector { _handler: USART6_IRQ },
    Vector { _handler: I2C3_EV_IRQ },
    Vector { _handler: I2C3_ER_IRQ },
    Vector { _handler: OTG_HS_EP1_OUT_IRQ },
    Vector { _handler: OTG_HS_EP1_IN_IRQ },
    Vector { _handler: OTG_HS_WKUP_IRQ },
    Vector { _handler: OTG_HS_IRQ },
    Vector { _handler: DCMI_IRQ },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FPU },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler(a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r");"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG_IRQ,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD_IRQ,
    #[doc = "2 - Tamper and TimeStamp interrupts through the EXTI line"]
    TAMP_STAMP_IRQ,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
    RTC_WKUP_IRQ,
    #[doc = "5 - RCC global interrupt"]
    RCC_IRQ,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0_IRQ,
    #[doc = "7 - EXTI Line1 interrupt"]
    EXTI1_IRQ,
    #[doc = "8 - EXTI Line2 interrupt"]
    EXTI2_IRQ,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3_IRQ,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4_IRQ,
    #[doc = "11 - DMA1 Stream0 global interrupt"]
    DMA1_STREAM0_IRQ,
    #[doc = "12 - DMA1 Stream1 global interrupt"]
    DMA1_STREAM1_IRQ,
    #[doc = "13 - DMA1 Stream2 global interrupt"]
    DMA1_STREAM2_IRQ,
    #[doc = "14 - DMA1 Stream3 global interrupt"]
    DMA1_STREAM3_IRQ,
    #[doc = "15 - DMA1 Stream4 global interrupt"]
    DMA1_STREAM4_IRQ,
    #[doc = "16 - DMA1 Stream5 global interrupt"]
    DMA1_STREAM5_IRQ,
    #[doc = "17 - DMA1 Stream6 global interrupt"]
    DMA1_STREAM6_IRQ,
    #[doc = "18 - ADC3 global interrupts"]
    ADC_IRQ,
    #[doc = "19 - CAN1 TX interrupts"]
    CAN1_TX_IRQ,
    #[doc = "20 - CAN1 RX0 interrupts"]
    CAN1_RX0_IRQ,
    #[doc = "21 - CAN1 RX1 interrupts"]
    CAN1_RX1_IRQ,
    #[doc = "22 - CAN1 SCE interrupt"]
    CAN1_SCE_IRQ,
    #[doc = "23 - EXTI Line[9:5] interrupts"]
    EXTI9_5_IRQ,
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
    TIM1_BRK_TIM9_IRQ,
    #[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"]
    TIM1_UP_TIM10_IRQ,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    TIM1_TRG_COM_TIM11_IRQ,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC_IRQ,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2_IRQ,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3_IRQ,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4_IRQ,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV_IRQ,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER_IRQ,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV_IRQ,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER_IRQ,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1_IRQ,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2_IRQ,
    #[doc = "37 - USART1 global interrupt"]
    USART1_IRQ,
    #[doc = "38 - USART2 global interrupt"]
    USART2_IRQ,
    #[doc = "39 - USART3 global interrupt"]
    USART3_IRQ,
    #[doc = "40 - EXTI Line[15:10] interrupts"]
    EXTI15_10_IRQ,
    #[doc = "41 - RTC Alarms(A and B) through EXTI line interrupt"]
    RTC_ALARM_IRQ,
    #[doc = "42 - USB On-The-Go FS Wakeup through EXTI line interrupt"]
    OTG_FS_WKUP_IRQ,
    #[doc = "43 - TIM8 Break interrupt and TIM12 global interrupt"]
    TIM8_BRK_TIM12_IRQ,
    #[doc = "44 - TIM8 Update interrupt and TIM13 global interrupt"]
    TIM8_UP_TIM13_IRQ,
    #[doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt"]
    TIM8_TRG_COM_TIM14_IRQ,
    #[doc = "46 - TIM8 Capture Compare interrupt"]
    TIM8_CC_IRQ,
    #[doc = "47 - DMA1 Stream7 global interrupt"]
    DMA1_STREAM7_IRQ,
    #[doc = "48 - FSMC global interrupt"]
    FSMC_IRQ,
    #[doc = "49 - SDIO global interrupt"]
    SDIO_IRQ,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5_IRQ,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3_IRQ,
    #[doc = "52 - UART4 global interrupt"]
    UART4_IRQ,
    #[doc = "53 - UART5 global interrupt"]
    UART5_IRQ,
    #[doc = "54 - TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt"]
    TIM6_DAC_IRQ,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7_IRQ,
    #[doc = "56 - DMA2 Stream0 global interrupt"]
    DMA2_STREAM0_IRQ,
    #[doc = "57 - DMA2 Stream1 global interrupt"]
    DMA2_STREAM1_IRQ,
    #[doc = "58 - DMA2 Stream2 global interrupt"]
    DMA2_STREAM2_IRQ,
    #[doc = "59 - DMA2 Stream3 global interrupt"]
    DMA2_STREAM3_IRQ,
    #[doc = "60 - DMA2 Stream4 global interrupt"]
    DMA2_STREAM4_IRQ,
    #[doc = "61 - Ethernet global interrupt"]
    ETH_IRQ,
    #[doc = "62 - Ethernet Wakeup through EXTI line interrupt"]
    ETH_WKUP_IRQ,
    #[doc = "63 - CAN2 TX interrupts"]
    CAN2_TX_IRQ,
    #[doc = "64 - CAN2 RX0 interrupts"]
    CAN2_RX0_IRQ,
    #[doc = "65 - CAN2 RX1 interrupts"]
    CAN2_RX1_IRQ,
    #[doc = "66 - CAN2 SCE interrupt"]
    CAN2_SCE_IRQ,
    #[doc = "67 - USB On The Go FS global interrupt"]
    OTG_FS_IRQ,
    #[doc = "68 - DMA2 Stream5 global interrupt"]
    DMA2_STREAM5_IRQ,
    #[doc = "69 - DMA2 Stream6 global interrupt"]
    DMA2_STREAM6_IRQ,
    #[doc = "70 - DMA2 Stream7 global interrupt"]
    DMA2_STREAM7_IRQ,
    #[doc = "71 - USART6 global interrupt"]
    USART6_IRQ,
    #[doc = "72 - I2C3 event interrupt"]
    I2C3_EV_IRQ,
    #[doc = "73 - I2C3 error interrupt"]
    I2C3_ER_IRQ,
    #[doc = "74 - USB On The Go HS End Point 1 Out global interrupt"]
    OTG_HS_EP1_OUT_IRQ,
    #[doc = "75 - USB On The Go HS End Point 1 In global interrupt"]
    OTG_HS_EP1_IN_IRQ,
    #[doc = "76 - USB On The Go HS Wakeup through EXTI interrupt"]
    OTG_HS_WKUP_IRQ,
    #[doc = "77 - USB On The Go HS global interrupt"]
    OTG_HS_IRQ,
    #[doc = "78 - DCMI global interrupt"]
    DCMI_IRQ,
    #[doc = "81 - FPU interrupt"]
    FPU,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG_IRQ => 0,
            Interrupt::PVD_IRQ => 1,
            Interrupt::TAMP_STAMP_IRQ => 2,
            Interrupt::RTC_WKUP_IRQ => 3,
            Interrupt::RCC_IRQ => 5,
            Interrupt::EXTI0_IRQ => 6,
            Interrupt::EXTI1_IRQ => 7,
            Interrupt::EXTI2_IRQ => 8,
            Interrupt::EXTI3_IRQ => 9,
            Interrupt::EXTI4_IRQ => 10,
            Interrupt::DMA1_STREAM0_IRQ => 11,
            Interrupt::DMA1_STREAM1_IRQ => 12,
            Interrupt::DMA1_STREAM2_IRQ => 13,
            Interrupt::DMA1_STREAM3_IRQ => 14,
            Interrupt::DMA1_STREAM4_IRQ => 15,
            Interrupt::DMA1_STREAM5_IRQ => 16,
            Interrupt::DMA1_STREAM6_IRQ => 17,
            Interrupt::ADC_IRQ => 18,
            Interrupt::CAN1_TX_IRQ => 19,
            Interrupt::CAN1_RX0_IRQ => 20,
            Interrupt::CAN1_RX1_IRQ => 21,
            Interrupt::CAN1_SCE_IRQ => 22,
            Interrupt::EXTI9_5_IRQ => 23,
            Interrupt::TIM1_BRK_TIM9_IRQ => 24,
            Interrupt::TIM1_UP_TIM10_IRQ => 25,
            Interrupt::TIM1_TRG_COM_TIM11_IRQ => 26,
            Interrupt::TIM1_CC_IRQ => 27,
            Interrupt::TIM2_IRQ => 28,
            Interrupt::TIM3_IRQ => 29,
            Interrupt::TIM4_IRQ => 30,
            Interrupt::I2C1_EV_IRQ => 31,
            Interrupt::I2C1_ER_IRQ => 32,
            Interrupt::I2C2_EV_IRQ => 33,
            Interrupt::I2C2_ER_IRQ => 34,
            Interrupt::SPI1_IRQ => 35,
            Interrupt::SPI2_IRQ => 36,
            Interrupt::USART1_IRQ => 37,
            Interrupt::USART2_IRQ => 38,
            Interrupt::USART3_IRQ => 39,
            Interrupt::EXTI15_10_IRQ => 40,
            Interrupt::RTC_ALARM_IRQ => 41,
            Interrupt::OTG_FS_WKUP_IRQ => 42,
            Interrupt::TIM8_BRK_TIM12_IRQ => 43,
            Interrupt::TIM8_UP_TIM13_IRQ => 44,
            Interrupt::TIM8_TRG_COM_TIM14_IRQ => 45,
            Interrupt::TIM8_CC_IRQ => 46,
            Interrupt::DMA1_STREAM7_IRQ => 47,
            Interrupt::FSMC_IRQ => 48,
            Interrupt::SDIO_IRQ => 49,
            Interrupt::TIM5_IRQ => 50,
            Interrupt::SPI3_IRQ => 51,
            Interrupt::UART4_IRQ => 52,
            Interrupt::UART5_IRQ => 53,
            Interrupt::TIM6_DAC_IRQ => 54,
            Interrupt::TIM7_IRQ => 55,
            Interrupt::DMA2_STREAM0_IRQ => 56,
            Interrupt::DMA2_STREAM1_IRQ => 57,
            Interrupt::DMA2_STREAM2_IRQ => 58,
            Interrupt::DMA2_STREAM3_IRQ => 59,
            Interrupt::DMA2_STREAM4_IRQ => 60,
            Interrupt::ETH_IRQ => 61,
            Interrupt::ETH_WKUP_IRQ => 62,
            Interrupt::CAN2_TX_IRQ => 63,
            Interrupt::CAN2_RX0_IRQ => 64,
            Interrupt::CAN2_RX1_IRQ => 65,
            Interrupt::CAN2_SCE_IRQ => 66,
            Interrupt::OTG_FS_IRQ => 67,
            Interrupt::DMA2_STREAM5_IRQ => 68,
            Interrupt::DMA2_STREAM6_IRQ => 69,
            Interrupt::DMA2_STREAM7_IRQ => 70,
            Interrupt::USART6_IRQ => 71,
            Interrupt::I2C3_EV_IRQ => 72,
            Interrupt::I2C3_ER_IRQ => 73,
            Interrupt::OTG_HS_EP1_OUT_IRQ => 74,
            Interrupt::OTG_HS_EP1_IN_IRQ => 75,
            Interrupt::OTG_HS_WKUP_IRQ => 76,
            Interrupt::OTG_HS_IRQ => 77,
            Interrupt::DCMI_IRQ => 78,
            Interrupt::FPU => 81,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "Random number generator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1342572544 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random number generator"]
pub mod rng;
#[doc = "Digital camera interface"]
pub struct DCMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCMI {}
impl DCMI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dcmi::RegisterBlock {
        1342504960 as *const _
    }
}
impl Deref for DCMI {
    type Target = dcmi::RegisterBlock;
    fn deref(&self) -> &dcmi::RegisterBlock {
        unsafe { &*DCMI::ptr() }
    }
}
#[doc = "Digital camera interface"]
pub mod dcmi;
#[doc = "Flexible static memory controller"]
pub struct FSMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FSMC {}
impl FSMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fsmc::RegisterBlock {
        2684354560 as *const _
    }
}
impl Deref for FSMC {
    type Target = fsmc::RegisterBlock;
    fn deref(&self) -> &fsmc::RegisterBlock {
        unsafe { &*FSMC::ptr() }
    }
}
#[doc = "Flexible static memory controller"]
pub mod fsmc;
#[doc = "Debug support"]
pub struct DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBG {}
impl DBG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbg::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBG {
    type Target = dbg::RegisterBlock;
    fn deref(&self) -> &dbg::RegisterBlock {
        unsafe { &*DBG::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbg;
#[doc = "DMA controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2::RegisterBlock {
        1073898496 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma2;
#[doc = "DMA1"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma2::RegisterBlock {
        1073897472 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma2::RegisterBlock;
    fn deref(&self) -> &dma2::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073887232 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "General-purpose I/Os"]
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioi::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpioi::RegisterBlock;
    fn deref(&self) -> &gpioi::RegisterBlock {
        unsafe { &*GPIOI::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioi;
#[doc = "General-purpose I/Os"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioh::RegisterBlock {
        1073880064 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioh::RegisterBlock;
    fn deref(&self) -> &gpioh::RegisterBlock {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioh;
#[doc = "General-purpose I/Os"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiog::RegisterBlock {
        1073879040 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpiog::RegisterBlock;
    fn deref(&self) -> &gpiog::RegisterBlock {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiog;
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiof::RegisterBlock {
        1073878016 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpiof::RegisterBlock;
    fn deref(&self) -> &gpiof::RegisterBlock {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiof;
#[doc = "General-purpose I/Os"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioe::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    fn deref(&self) -> &gpioe::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioe;
#[doc = "General-purpose I/Os"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiod::RegisterBlock {
        1073875968 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    fn deref(&self) -> &gpiod::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiod;
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioc::RegisterBlock {
        1073874944 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &gpioc::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioc;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiob::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &gpiob::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpiob;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "System configuration controller"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const syscfg::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    fn deref(&self) -> &syscfg::RegisterBlock {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "System configuration controller"]
pub mod syscfg;
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073756160 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "SPI3"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073757184 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "I2S2ext"]
pub struct I2S2EXT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S2EXT {}
impl I2S2EXT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073755136 as *const _
    }
}
impl Deref for I2S2EXT {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*I2S2EXT::ptr() }
    }
}
#[doc = "I2S3ext"]
pub struct I2S3EXT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S3EXT {}
impl I2S3EXT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for I2S3EXT {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*I2S3EXT::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdio::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    fn deref(&self) -> &sdio::RegisterBlock {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdio;
#[doc = "Analog-to-digital converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-digital converter"]
pub mod adc1;
#[doc = "ADC2"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073815808 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "ADC3"]
pub struct ADC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC3 {}
impl ADC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073816064 as *const _
    }
}
impl Deref for ADC3 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073812480 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart6;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart6::RegisterBlock {
        1073760256 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart6::RegisterBlock;
    fn deref(&self) -> &usart6::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1073771520 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-analog converter"]
pub mod dac;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Inter-integrated circuit"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073765376 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c3;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073764352 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c3::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &i2c3::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "Real-time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time clock"]
pub mod rtc;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1073761280 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod uart4;
#[doc = "UART5"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for UART5 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "Common ADC registers"]
pub struct C_ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for C_ADC {}
impl C_ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const c_adc::RegisterBlock {
        1073816320 as *const _
    }
}
impl Deref for C_ADC {
    type Target = c_adc::RegisterBlock;
    fn deref(&self) -> &c_adc::RegisterBlock {
        unsafe { &*C_ADC::ptr() }
    }
}
#[doc = "Common ADC registers"]
pub mod c_adc;
#[doc = "Advanced-timers"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced-timers"]
pub mod tim1;
#[doc = "TIM8"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "General purpose timers"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim2;
#[doc = "General purpose timers"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim3::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim3;
#[doc = "TIM4"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim3::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim3::RegisterBlock;
    fn deref(&self) -> &tim3::RegisterBlock {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim5::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim5::RegisterBlock;
    fn deref(&self) -> &tim5::RegisterBlock {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim5;
#[doc = "General purpose timers"]
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM9 {}
impl TIM9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for TIM9 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM9::ptr() }
    }
}
#[doc = "General purpose timers"]
pub mod tim9;
#[doc = "TIM12"]
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM12 {}
impl TIM12 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073747968 as *const _
    }
}
impl Deref for TIM12 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM12::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM10 {}
impl TIM10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073824768 as *const _
    }
}
impl Deref for TIM10 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM10::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim10;
#[doc = "TIM13"]
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM13 {}
impl TIM13 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073748992 as *const _
    }
}
impl Deref for TIM13 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM13::ptr() }
    }
}
#[doc = "TIM14"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM14::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub struct TIM11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM11 {}
impl TIM11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim11::RegisterBlock {
        1073825792 as *const _
    }
}
impl Deref for TIM11 {
    type Target = tim11::RegisterBlock;
    fn deref(&self) -> &tim11::RegisterBlock {
        unsafe { &*TIM11::ptr() }
    }
}
#[doc = "General-purpose-timers"]
pub mod tim11;
#[doc = "Basic timers"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic timers"]
pub mod tim6;
#[doc = "TIM7"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "Ethernet: media access control(MAC)"]
pub struct ETHERNET_MAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MAC {}
impl ETHERNET_MAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_mac::RegisterBlock {
        1073905664 as *const _
    }
}
impl Deref for ETHERNET_MAC {
    type Target = ethernet_mac::RegisterBlock;
    fn deref(&self) -> &ethernet_mac::RegisterBlock {
        unsafe { &*ETHERNET_MAC::ptr() }
    }
}
#[doc = "Ethernet: media access control(MAC)"]
pub mod ethernet_mac;
#[doc = "Ethernet: MAC management counters"]
pub struct ETHERNET_MMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_MMC {}
impl ETHERNET_MMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_mmc::RegisterBlock {
        1073905920 as *const _
    }
}
impl Deref for ETHERNET_MMC {
    type Target = ethernet_mmc::RegisterBlock;
    fn deref(&self) -> &ethernet_mmc::RegisterBlock {
        unsafe { &*ETHERNET_MMC::ptr() }
    }
}
#[doc = "Ethernet: MAC management counters"]
pub mod ethernet_mmc;
#[doc = "Ethernet: Precision time protocol"]
pub struct ETHERNET_PTP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_PTP {}
impl ETHERNET_PTP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_ptp::RegisterBlock {
        1073907456 as *const _
    }
}
impl Deref for ETHERNET_PTP {
    type Target = ethernet_ptp::RegisterBlock;
    fn deref(&self) -> &ethernet_ptp::RegisterBlock {
        unsafe { &*ETHERNET_PTP::ptr() }
    }
}
#[doc = "Ethernet: Precision time protocol"]
pub mod ethernet_ptp;
#[doc = "Ethernet: DMA controller operation"]
pub struct ETHERNET_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETHERNET_DMA {}
impl ETHERNET_DMA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ethernet_dma::RegisterBlock {
        1073909760 as *const _
    }
}
impl Deref for ETHERNET_DMA {
    type Target = ethernet_dma::RegisterBlock;
    fn deref(&self) -> &ethernet_dma::RegisterBlock {
        unsafe { &*ETHERNET_DMA::ptr() }
    }
}
#[doc = "Ethernet: DMA controller operation"]
pub mod ethernet_dma;
#[doc = "Cryptographic processor"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod crc;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_GLOBAL {}
impl OTG_FS_GLOBAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_global::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for OTG_FS_GLOBAL {
    type Target = otg_fs_global::RegisterBlock;
    fn deref(&self) -> &otg_fs_global::RegisterBlock {
        unsafe { &*OTG_FS_GLOBAL::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_global;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_HOST {}
impl OTG_FS_HOST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_host::RegisterBlock {
        1342178304 as *const _
    }
}
impl Deref for OTG_FS_HOST {
    type Target = otg_fs_host::RegisterBlock;
    fn deref(&self) -> &otg_fs_host::RegisterBlock {
        unsafe { &*OTG_FS_HOST::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_host;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_DEVICE {}
impl OTG_FS_DEVICE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_device::RegisterBlock {
        1342179328 as *const _
    }
}
impl Deref for OTG_FS_DEVICE {
    type Target = otg_fs_device::RegisterBlock;
    fn deref(&self) -> &otg_fs_device::RegisterBlock {
        unsafe { &*OTG_FS_DEVICE::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_device;
#[doc = "USB on the go full speed"]
pub struct OTG_FS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_FS_PWRCLK {}
impl OTG_FS_PWRCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_fs_pwrclk::RegisterBlock {
        1342180864 as *const _
    }
}
impl Deref for OTG_FS_PWRCLK {
    type Target = otg_fs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_fs_pwrclk::RegisterBlock {
        unsafe { &*OTG_FS_PWRCLK::ptr() }
    }
}
#[doc = "USB on the go full speed"]
pub mod otg_fs_pwrclk;
#[doc = "Controller area network"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1073767424 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Controller area network"]
pub mod can1;
#[doc = "CAN2"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1073768448 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "FLASH"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073888256 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "FLASH"]
pub mod flash;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073822720 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_GLOBAL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_GLOBAL {}
impl OTG_HS_GLOBAL {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_global::RegisterBlock {
        1074003968 as *const _
    }
}
impl Deref for OTG_HS_GLOBAL {
    type Target = otg_hs_global::RegisterBlock;
    fn deref(&self) -> &otg_hs_global::RegisterBlock {
        unsafe { &*OTG_HS_GLOBAL::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_global;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_HOST {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_HOST {}
impl OTG_HS_HOST {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_host::RegisterBlock {
        1074004992 as *const _
    }
}
impl Deref for OTG_HS_HOST {
    type Target = otg_hs_host::RegisterBlock;
    fn deref(&self) -> &otg_hs_host::RegisterBlock {
        unsafe { &*OTG_HS_HOST::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_host;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_DEVICE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_DEVICE {}
impl OTG_HS_DEVICE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_device::RegisterBlock {
        1074006016 as *const _
    }
}
impl Deref for OTG_HS_DEVICE {
    type Target = otg_hs_device::RegisterBlock;
    fn deref(&self) -> &otg_hs_device::RegisterBlock {
        unsafe { &*OTG_HS_DEVICE::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_device;
#[doc = "USB on the go high speed"]
pub struct OTG_HS_PWRCLK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OTG_HS_PWRCLK {}
impl OTG_HS_PWRCLK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const otg_hs_pwrclk::RegisterBlock {
        1074007552 as *const _
    }
}
impl Deref for OTG_HS_PWRCLK {
    type Target = otg_hs_pwrclk::RegisterBlock;
    fn deref(&self) -> &otg_hs_pwrclk::RegisterBlock {
        unsafe { &*OTG_HS_PWRCLK::ptr() }
    }
}
#[doc = "USB on the go high speed"]
pub mod otg_hs_pwrclk;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "DCMI"]
    pub DCMI: DCMI,
    #[doc = "FSMC"]
    pub FSMC: FSMC,
    #[doc = "DBG"]
    pub DBG: DBG,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "GPIOI"]
    pub GPIOI: GPIOI,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "I2S2EXT"]
    pub I2S2EXT: I2S2EXT,
    #[doc = "I2S3EXT"]
    pub I2S3EXT: I2S3EXT,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "ADC3"]
    pub ADC3: ADC3,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "C_ADC"]
    pub C_ADC: C_ADC,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM9"]
    pub TIM9: TIM9,
    #[doc = "TIM12"]
    pub TIM12: TIM12,
    #[doc = "TIM10"]
    pub TIM10: TIM10,
    #[doc = "TIM13"]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "TIM11"]
    pub TIM11: TIM11,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "ETHERNET_MAC"]
    pub ETHERNET_MAC: ETHERNET_MAC,
    #[doc = "ETHERNET_MMC"]
    pub ETHERNET_MMC: ETHERNET_MMC,
    #[doc = "ETHERNET_PTP"]
    pub ETHERNET_PTP: ETHERNET_PTP,
    #[doc = "ETHERNET_DMA"]
    pub ETHERNET_DMA: ETHERNET_DMA,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "OTG_FS_GLOBAL"]
    pub OTG_FS_GLOBAL: OTG_FS_GLOBAL,
    #[doc = "OTG_FS_HOST"]
    pub OTG_FS_HOST: OTG_FS_HOST,
    #[doc = "OTG_FS_DEVICE"]
    pub OTG_FS_DEVICE: OTG_FS_DEVICE,
    #[doc = "OTG_FS_PWRCLK"]
    pub OTG_FS_PWRCLK: OTG_FS_PWRCLK,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "OTG_HS_GLOBAL"]
    pub OTG_HS_GLOBAL: OTG_HS_GLOBAL,
    #[doc = "OTG_HS_HOST"]
    pub OTG_HS_HOST: OTG_HS_HOST,
    #[doc = "OTG_HS_DEVICE"]
    pub OTG_HS_DEVICE: OTG_HS_DEVICE,
    #[doc = "OTG_HS_PWRCLK"]
    pub OTG_HS_PWRCLK: OTG_HS_PWRCLK,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| if unsafe { DEVICE_PERIPHERALS } { None } else { Some(unsafe { Peripherals::steal() }) })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            RNG: RNG { _marker: PhantomData },
            DCMI: DCMI { _marker: PhantomData },
            FSMC: FSMC { _marker: PhantomData },
            DBG: DBG { _marker: PhantomData },
            DMA2: DMA2 { _marker: PhantomData },
            DMA1: DMA1 { _marker: PhantomData },
            RCC: RCC { _marker: PhantomData },
            GPIOI: GPIOI { _marker: PhantomData },
            GPIOH: GPIOH { _marker: PhantomData },
            GPIOG: GPIOG { _marker: PhantomData },
            GPIOF: GPIOF { _marker: PhantomData },
            GPIOE: GPIOE { _marker: PhantomData },
            GPIOD: GPIOD { _marker: PhantomData },
            GPIOC: GPIOC { _marker: PhantomData },
            GPIOB: GPIOB { _marker: PhantomData },
            GPIOA: GPIOA { _marker: PhantomData },
            SYSCFG: SYSCFG { _marker: PhantomData },
            SPI1: SPI1 { _marker: PhantomData },
            SPI2: SPI2 { _marker: PhantomData },
            SPI3: SPI3 { _marker: PhantomData },
            I2S2EXT: I2S2EXT { _marker: PhantomData },
            I2S3EXT: I2S3EXT { _marker: PhantomData },
            SDIO: SDIO { _marker: PhantomData },
            ADC1: ADC1 { _marker: PhantomData },
            ADC2: ADC2 { _marker: PhantomData },
            ADC3: ADC3 { _marker: PhantomData },
            USART6: USART6 { _marker: PhantomData },
            USART1: USART1 { _marker: PhantomData },
            USART2: USART2 { _marker: PhantomData },
            USART3: USART3 { _marker: PhantomData },
            DAC: DAC { _marker: PhantomData },
            PWR: PWR { _marker: PhantomData },
            I2C3: I2C3 { _marker: PhantomData },
            I2C2: I2C2 { _marker: PhantomData },
            I2C1: I2C1 { _marker: PhantomData },
            IWDG: IWDG { _marker: PhantomData },
            WWDG: WWDG { _marker: PhantomData },
            RTC: RTC { _marker: PhantomData },
            UART4: UART4 { _marker: PhantomData },
            UART5: UART5 { _marker: PhantomData },
            C_ADC: C_ADC { _marker: PhantomData },
            TIM1: TIM1 { _marker: PhantomData },
            TIM8: TIM8 { _marker: PhantomData },
            TIM2: TIM2 { _marker: PhantomData },
            TIM3: TIM3 { _marker: PhantomData },
            TIM4: TIM4 { _marker: PhantomData },
            TIM5: TIM5 { _marker: PhantomData },
            TIM9: TIM9 { _marker: PhantomData },
            TIM12: TIM12 { _marker: PhantomData },
            TIM10: TIM10 { _marker: PhantomData },
            TIM13: TIM13 { _marker: PhantomData },
            TIM14: TIM14 { _marker: PhantomData },
            TIM11: TIM11 { _marker: PhantomData },
            TIM6: TIM6 { _marker: PhantomData },
            TIM7: TIM7 { _marker: PhantomData },
            ETHERNET_MAC: ETHERNET_MAC { _marker: PhantomData },
            ETHERNET_MMC: ETHERNET_MMC { _marker: PhantomData },
            ETHERNET_PTP: ETHERNET_PTP { _marker: PhantomData },
            ETHERNET_DMA: ETHERNET_DMA { _marker: PhantomData },
            CRC: CRC { _marker: PhantomData },
            OTG_FS_GLOBAL: OTG_FS_GLOBAL { _marker: PhantomData },
            OTG_FS_HOST: OTG_FS_HOST { _marker: PhantomData },
            OTG_FS_DEVICE: OTG_FS_DEVICE { _marker: PhantomData },
            OTG_FS_PWRCLK: OTG_FS_PWRCLK { _marker: PhantomData },
            CAN1: CAN1 { _marker: PhantomData },
            CAN2: CAN2 { _marker: PhantomData },
            FLASH: FLASH { _marker: PhantomData },
            EXTI: EXTI { _marker: PhantomData },
            OTG_HS_GLOBAL: OTG_HS_GLOBAL { _marker: PhantomData },
            OTG_HS_HOST: OTG_HS_HOST { _marker: PhantomData },
            OTG_HS_DEVICE: OTG_HS_DEVICE { _marker: PhantomData },
            OTG_HS_PWRCLK: OTG_HS_PWRCLK { _marker: PhantomData },
        }
    }
}
