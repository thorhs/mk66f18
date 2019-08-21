#![doc = "Peripheral access API for MK66F18 microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
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
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA0_DMA16();
    fn DMA1_DMA17();
    fn DMA2_DMA18();
    fn DMA3_DMA19();
    fn DMA4_DMA20();
    fn DMA5_DMA21();
    fn DMA6_DMA22();
    fn DMA7_DMA23();
    fn DMA8_DMA24();
    fn DMA9_DMA25();
    fn DMA10_DMA26();
    fn DMA11_DMA27();
    fn DMA12_DMA28();
    fn DMA13_DMA29();
    fn DMA14_DMA30();
    fn DMA15_DMA31();
    fn DMA_ERROR();
    fn MCM();
    fn FTFE();
    fn READ_COLLISION();
    fn LVD_LVW();
    fn LLWU();
    fn WDOG_EWM();
    fn RNG();
    fn I2C0();
    fn I2C1();
    fn SPI0();
    fn SPI1();
    fn I2S0_TX();
    fn I2S0_RX();
    fn UART0_RX_TX();
    fn UART0_ERR();
    fn UART1_RX_TX();
    fn UART1_ERR();
    fn UART2_RX_TX();
    fn UART2_ERR();
    fn UART3_RX_TX();
    fn UART3_ERR();
    fn ADC0();
    fn CMP0();
    fn CMP1();
    fn FTM0();
    fn FTM1();
    fn FTM2();
    fn CMT();
    fn RTC();
    fn RTC_SECONDS();
    fn PIT0();
    fn PIT1();
    fn PIT2();
    fn PIT3();
    fn PDB0();
    fn USB0();
    fn USBDCD();
    fn DAC0();
    fn LPTMR0();
    fn PORTA();
    fn PORTB();
    fn PORTC();
    fn PORTD();
    fn PORTE();
    fn SPI2();
    fn UART4_RX_TX();
    fn UART4_ERR();
    fn CMP2();
    fn FTM3();
    fn DAC1();
    fn ADC1();
    fn I2C2();
    fn CAN0_ORED_MESSAGE_BUFFER();
    fn CAN0_BUS_OFF();
    fn CAN0_ERROR();
    fn CAN0_TX_WARNING();
    fn CAN0_RX_WARNING();
    fn CAN0_WAKE_UP();
    fn SDHC();
    fn ENET_1588_TIMER();
    fn ENET_TRANSMIT();
    fn ENET_RECEIVE();
    fn ENET_ERROR();
    fn LPUART0();
    fn TSI0();
    fn TPM1();
    fn TPM2();
    fn USBHSDCD();
    fn I2C3();
    fn CMP3();
    fn USBHS();
    fn CAN1_ORED_MESSAGE_BUFFER();
    fn CAN1_BUS_OFF();
    fn CAN1_ERROR();
    fn CAN1_TX_WARNING();
    fn CAN1_RX_WARNING();
    fn CAN1_WAKE_UP();
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
pub static __INTERRUPTS: [Vector; 100] = [
    Vector {
        _handler: DMA0_DMA16,
    },
    Vector {
        _handler: DMA1_DMA17,
    },
    Vector {
        _handler: DMA2_DMA18,
    },
    Vector {
        _handler: DMA3_DMA19,
    },
    Vector {
        _handler: DMA4_DMA20,
    },
    Vector {
        _handler: DMA5_DMA21,
    },
    Vector {
        _handler: DMA6_DMA22,
    },
    Vector {
        _handler: DMA7_DMA23,
    },
    Vector {
        _handler: DMA8_DMA24,
    },
    Vector {
        _handler: DMA9_DMA25,
    },
    Vector {
        _handler: DMA10_DMA26,
    },
    Vector {
        _handler: DMA11_DMA27,
    },
    Vector {
        _handler: DMA12_DMA28,
    },
    Vector {
        _handler: DMA13_DMA29,
    },
    Vector {
        _handler: DMA14_DMA30,
    },
    Vector {
        _handler: DMA15_DMA31,
    },
    Vector {
        _handler: DMA_ERROR,
    },
    Vector { _handler: MCM },
    Vector { _handler: FTFE },
    Vector {
        _handler: READ_COLLISION,
    },
    Vector { _handler: LVD_LVW },
    Vector { _handler: LLWU },
    Vector { _handler: WDOG_EWM },
    Vector { _handler: RNG },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _handler: I2S0_TX },
    Vector { _handler: I2S0_RX },
    Vector { _reserved: 0 },
    Vector {
        _handler: UART0_RX_TX,
    },
    Vector {
        _handler: UART0_ERR,
    },
    Vector {
        _handler: UART1_RX_TX,
    },
    Vector {
        _handler: UART1_ERR,
    },
    Vector {
        _handler: UART2_RX_TX,
    },
    Vector {
        _handler: UART2_ERR,
    },
    Vector {
        _handler: UART3_RX_TX,
    },
    Vector {
        _handler: UART3_ERR,
    },
    Vector { _handler: ADC0 },
    Vector { _handler: CMP0 },
    Vector { _handler: CMP1 },
    Vector { _handler: FTM0 },
    Vector { _handler: FTM1 },
    Vector { _handler: FTM2 },
    Vector { _handler: CMT },
    Vector { _handler: RTC },
    Vector {
        _handler: RTC_SECONDS,
    },
    Vector { _handler: PIT0 },
    Vector { _handler: PIT1 },
    Vector { _handler: PIT2 },
    Vector { _handler: PIT3 },
    Vector { _handler: PDB0 },
    Vector { _handler: USB0 },
    Vector { _handler: USBDCD },
    Vector { _reserved: 0 },
    Vector { _handler: DAC0 },
    Vector { _reserved: 0 },
    Vector { _handler: LPTMR0 },
    Vector { _handler: PORTA },
    Vector { _handler: PORTB },
    Vector { _handler: PORTC },
    Vector { _handler: PORTD },
    Vector { _handler: PORTE },
    Vector { _reserved: 0 },
    Vector { _handler: SPI2 },
    Vector {
        _handler: UART4_RX_TX,
    },
    Vector {
        _handler: UART4_ERR,
    },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: CMP2 },
    Vector { _handler: FTM3 },
    Vector { _handler: DAC1 },
    Vector { _handler: ADC1 },
    Vector { _handler: I2C2 },
    Vector {
        _handler: CAN0_ORED_MESSAGE_BUFFER,
    },
    Vector {
        _handler: CAN0_BUS_OFF,
    },
    Vector {
        _handler: CAN0_ERROR,
    },
    Vector {
        _handler: CAN0_TX_WARNING,
    },
    Vector {
        _handler: CAN0_RX_WARNING,
    },
    Vector {
        _handler: CAN0_WAKE_UP,
    },
    Vector { _handler: SDHC },
    Vector {
        _handler: ENET_1588_TIMER,
    },
    Vector {
        _handler: ENET_TRANSMIT,
    },
    Vector {
        _handler: ENET_RECEIVE,
    },
    Vector {
        _handler: ENET_ERROR,
    },
    Vector { _handler: LPUART0 },
    Vector { _handler: TSI0 },
    Vector { _handler: TPM1 },
    Vector { _handler: TPM2 },
    Vector { _handler: USBHSDCD },
    Vector { _handler: I2C3 },
    Vector { _handler: CMP3 },
    Vector { _handler: USBHS },
    Vector {
        _handler: CAN1_ORED_MESSAGE_BUFFER,
    },
    Vector {
        _handler: CAN1_BUS_OFF,
    },
    Vector {
        _handler: CAN1_ERROR,
    },
    Vector {
        _handler: CAN1_TX_WARNING,
    },
    Vector {
        _handler: CAN1_RX_WARNING,
    },
    Vector {
        _handler: CAN1_WAKE_UP,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - DMA0_DMA16"]
    DMA0_DMA16,
    #[doc = "1 - DMA1_DMA17"]
    DMA1_DMA17,
    #[doc = "2 - DMA2_DMA18"]
    DMA2_DMA18,
    #[doc = "3 - DMA3_DMA19"]
    DMA3_DMA19,
    #[doc = "4 - DMA4_DMA20"]
    DMA4_DMA20,
    #[doc = "5 - DMA5_DMA21"]
    DMA5_DMA21,
    #[doc = "6 - DMA6_DMA22"]
    DMA6_DMA22,
    #[doc = "7 - DMA7_DMA23"]
    DMA7_DMA23,
    #[doc = "8 - DMA8_DMA24"]
    DMA8_DMA24,
    #[doc = "9 - DMA9_DMA25"]
    DMA9_DMA25,
    #[doc = "10 - DMA10_DMA26"]
    DMA10_DMA26,
    #[doc = "11 - DMA11_DMA27"]
    DMA11_DMA27,
    #[doc = "12 - DMA12_DMA28"]
    DMA12_DMA28,
    #[doc = "13 - DMA13_DMA29"]
    DMA13_DMA29,
    #[doc = "14 - DMA14_DMA30"]
    DMA14_DMA30,
    #[doc = "15 - DMA15_DMA31"]
    DMA15_DMA31,
    #[doc = "16 - DMA_Error"]
    DMA_ERROR,
    #[doc = "17 - MCM"]
    MCM,
    #[doc = "18 - FTFE"]
    FTFE,
    #[doc = "19 - Read_Collision"]
    READ_COLLISION,
    #[doc = "20 - LVD_LVW"]
    LVD_LVW,
    #[doc = "21 - LLWU"]
    LLWU,
    #[doc = "22 - WDOG_EWM"]
    WDOG_EWM,
    #[doc = "23 - RNG"]
    RNG,
    #[doc = "24 - I2C0"]
    I2C0,
    #[doc = "25 - I2C1"]
    I2C1,
    #[doc = "26 - SPI0"]
    SPI0,
    #[doc = "27 - SPI1"]
    SPI1,
    #[doc = "28 - I2S0_Tx"]
    I2S0_TX,
    #[doc = "29 - I2S0_Rx"]
    I2S0_RX,
    #[doc = "31 - UART0_RX_TX"]
    UART0_RX_TX,
    #[doc = "32 - UART0_ERR"]
    UART0_ERR,
    #[doc = "33 - UART1_RX_TX"]
    UART1_RX_TX,
    #[doc = "34 - UART1_ERR"]
    UART1_ERR,
    #[doc = "35 - UART2_RX_TX"]
    UART2_RX_TX,
    #[doc = "36 - UART2_ERR"]
    UART2_ERR,
    #[doc = "37 - UART3_RX_TX"]
    UART3_RX_TX,
    #[doc = "38 - UART3_ERR"]
    UART3_ERR,
    #[doc = "39 - ADC0"]
    ADC0,
    #[doc = "40 - CMP0"]
    CMP0,
    #[doc = "41 - CMP1"]
    CMP1,
    #[doc = "42 - FTM0"]
    FTM0,
    #[doc = "43 - FTM1"]
    FTM1,
    #[doc = "44 - FTM2"]
    FTM2,
    #[doc = "45 - CMT"]
    CMT,
    #[doc = "46 - RTC"]
    RTC,
    #[doc = "47 - RTC_Seconds"]
    RTC_SECONDS,
    #[doc = "48 - PIT0"]
    PIT0,
    #[doc = "49 - PIT1"]
    PIT1,
    #[doc = "50 - PIT2"]
    PIT2,
    #[doc = "51 - PIT3"]
    PIT3,
    #[doc = "52 - PDB0"]
    PDB0,
    #[doc = "53 - USB0"]
    USB0,
    #[doc = "54 - USBDCD"]
    USBDCD,
    #[doc = "56 - DAC0"]
    DAC0,
    #[doc = "58 - LPTMR0"]
    LPTMR0,
    #[doc = "59 - PORTA"]
    PORTA,
    #[doc = "60 - PORTB"]
    PORTB,
    #[doc = "61 - PORTC"]
    PORTC,
    #[doc = "62 - PORTD"]
    PORTD,
    #[doc = "63 - PORTE"]
    PORTE,
    #[doc = "65 - SPI2"]
    SPI2,
    #[doc = "66 - UART4_RX_TX"]
    UART4_RX_TX,
    #[doc = "67 - UART4_ERR"]
    UART4_ERR,
    #[doc = "70 - CMP2"]
    CMP2,
    #[doc = "71 - FTM3"]
    FTM3,
    #[doc = "72 - DAC1"]
    DAC1,
    #[doc = "73 - ADC1"]
    ADC1,
    #[doc = "74 - I2C2"]
    I2C2,
    #[doc = "75 - CAN0_ORed_Message_buffer"]
    CAN0_ORED_MESSAGE_BUFFER,
    #[doc = "76 - CAN0_Bus_Off"]
    CAN0_BUS_OFF,
    #[doc = "77 - CAN0_Error"]
    CAN0_ERROR,
    #[doc = "78 - CAN0_Tx_Warning"]
    CAN0_TX_WARNING,
    #[doc = "79 - CAN0_Rx_Warning"]
    CAN0_RX_WARNING,
    #[doc = "80 - CAN0_Wake_Up"]
    CAN0_WAKE_UP,
    #[doc = "81 - SDHC"]
    SDHC,
    #[doc = "82 - ENET_1588_Timer"]
    ENET_1588_TIMER,
    #[doc = "83 - ENET_Transmit"]
    ENET_TRANSMIT,
    #[doc = "84 - ENET_Receive"]
    ENET_RECEIVE,
    #[doc = "85 - ENET_Error"]
    ENET_ERROR,
    #[doc = "86 - LPUART0"]
    LPUART0,
    #[doc = "87 - TSI0"]
    TSI0,
    #[doc = "88 - TPM1"]
    TPM1,
    #[doc = "89 - TPM2"]
    TPM2,
    #[doc = "90 - USBHSDCD"]
    USBHSDCD,
    #[doc = "91 - I2C3"]
    I2C3,
    #[doc = "92 - CMP3"]
    CMP3,
    #[doc = "93 - USBHS"]
    USBHS,
    #[doc = "94 - CAN1_ORed_Message_buffer"]
    CAN1_ORED_MESSAGE_BUFFER,
    #[doc = "95 - CAN1_Bus_Off"]
    CAN1_BUS_OFF,
    #[doc = "96 - CAN1_Error"]
    CAN1_ERROR,
    #[doc = "97 - CAN1_Tx_Warning"]
    CAN1_TX_WARNING,
    #[doc = "98 - CAN1_Rx_Warning"]
    CAN1_RX_WARNING,
    #[doc = "99 - CAN1_Wake_Up"]
    CAN1_WAKE_UP,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::DMA0_DMA16 => 0,
            Interrupt::DMA1_DMA17 => 1,
            Interrupt::DMA2_DMA18 => 2,
            Interrupt::DMA3_DMA19 => 3,
            Interrupt::DMA4_DMA20 => 4,
            Interrupt::DMA5_DMA21 => 5,
            Interrupt::DMA6_DMA22 => 6,
            Interrupt::DMA7_DMA23 => 7,
            Interrupt::DMA8_DMA24 => 8,
            Interrupt::DMA9_DMA25 => 9,
            Interrupt::DMA10_DMA26 => 10,
            Interrupt::DMA11_DMA27 => 11,
            Interrupt::DMA12_DMA28 => 12,
            Interrupt::DMA13_DMA29 => 13,
            Interrupt::DMA14_DMA30 => 14,
            Interrupt::DMA15_DMA31 => 15,
            Interrupt::DMA_ERROR => 16,
            Interrupt::MCM => 17,
            Interrupt::FTFE => 18,
            Interrupt::READ_COLLISION => 19,
            Interrupt::LVD_LVW => 20,
            Interrupt::LLWU => 21,
            Interrupt::WDOG_EWM => 22,
            Interrupt::RNG => 23,
            Interrupt::I2C0 => 24,
            Interrupt::I2C1 => 25,
            Interrupt::SPI0 => 26,
            Interrupt::SPI1 => 27,
            Interrupt::I2S0_TX => 28,
            Interrupt::I2S0_RX => 29,
            Interrupt::UART0_RX_TX => 31,
            Interrupt::UART0_ERR => 32,
            Interrupt::UART1_RX_TX => 33,
            Interrupt::UART1_ERR => 34,
            Interrupt::UART2_RX_TX => 35,
            Interrupt::UART2_ERR => 36,
            Interrupt::UART3_RX_TX => 37,
            Interrupt::UART3_ERR => 38,
            Interrupt::ADC0 => 39,
            Interrupt::CMP0 => 40,
            Interrupt::CMP1 => 41,
            Interrupt::FTM0 => 42,
            Interrupt::FTM1 => 43,
            Interrupt::FTM2 => 44,
            Interrupt::CMT => 45,
            Interrupt::RTC => 46,
            Interrupt::RTC_SECONDS => 47,
            Interrupt::PIT0 => 48,
            Interrupt::PIT1 => 49,
            Interrupt::PIT2 => 50,
            Interrupt::PIT3 => 51,
            Interrupt::PDB0 => 52,
            Interrupt::USB0 => 53,
            Interrupt::USBDCD => 54,
            Interrupt::DAC0 => 56,
            Interrupt::LPTMR0 => 58,
            Interrupt::PORTA => 59,
            Interrupt::PORTB => 60,
            Interrupt::PORTC => 61,
            Interrupt::PORTD => 62,
            Interrupt::PORTE => 63,
            Interrupt::SPI2 => 65,
            Interrupt::UART4_RX_TX => 66,
            Interrupt::UART4_ERR => 67,
            Interrupt::CMP2 => 70,
            Interrupt::FTM3 => 71,
            Interrupt::DAC1 => 72,
            Interrupt::ADC1 => 73,
            Interrupt::I2C2 => 74,
            Interrupt::CAN0_ORED_MESSAGE_BUFFER => 75,
            Interrupt::CAN0_BUS_OFF => 76,
            Interrupt::CAN0_ERROR => 77,
            Interrupt::CAN0_TX_WARNING => 78,
            Interrupt::CAN0_RX_WARNING => 79,
            Interrupt::CAN0_WAKE_UP => 80,
            Interrupt::SDHC => 81,
            Interrupt::ENET_1588_TIMER => 82,
            Interrupt::ENET_TRANSMIT => 83,
            Interrupt::ENET_RECEIVE => 84,
            Interrupt::ENET_ERROR => 85,
            Interrupt::LPUART0 => 86,
            Interrupt::TSI0 => 87,
            Interrupt::TPM1 => 88,
            Interrupt::TPM2 => 89,
            Interrupt::USBHSDCD => 90,
            Interrupt::I2C3 => 91,
            Interrupt::CMP3 => 92,
            Interrupt::USBHS => 93,
            Interrupt::CAN1_ORED_MESSAGE_BUFFER => 94,
            Interrupt::CAN1_BUS_OFF => 95,
            Interrupt::CAN1_ERROR => 96,
            Interrupt::CAN1_TX_WARNING => 97,
            Interrupt::CAN1_RX_WARNING => 98,
            Interrupt::CAN1_WAKE_UP => 99,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Flash configuration field"]
pub struct FTFE_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFE_FLASHCONFIG {}
impl FTFE_FLASHCONFIG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftfe_flash_config::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for FTFE_FLASHCONFIG {
    type Target = ftfe_flash_config::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTFE_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash configuration field"]
pub mod ftfe_flash_config;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPS0 {}
impl AIPS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aips0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for AIPS0 {
    type Target = aips0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AIPS0::ptr() }
    }
}
#[doc = "AIPS-Lite Bridge"]
pub mod aips0;
#[doc = "AIPS-Lite Bridge"]
pub struct AIPS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIPS1 {}
impl AIPS1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aips1::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for AIPS1 {
    type Target = aips1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AIPS1::ptr() }
    }
}
#[doc = "AIPS-Lite Bridge"]
pub mod aips1;
#[doc = "Crossbar switch"]
pub struct AXBS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXBS {}
impl AXBS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const axbs::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for AXBS {
    type Target = axbs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AXBS::ptr() }
    }
}
#[doc = "Crossbar switch"]
pub mod axbs;
#[doc = "Enhanced direct memory access controller"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "Enhanced direct memory access controller"]
pub mod dma;
#[doc = "FlexBus external bus interface"]
pub struct FB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FB {}
impl FB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fb::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for FB {
    type Target = fb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FB::ptr() }
    }
}
#[doc = "FlexBus external bus interface"]
pub mod fb;
#[doc = "Memory protection unit"]
pub struct SYSMPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSMPU {}
impl SYSMPU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysmpu::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for SYSMPU {
    type Target = sysmpu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSMPU::ptr() }
    }
}
#[doc = "Memory protection unit"]
pub mod sysmpu;
#[doc = "Synchronous DRAM Controller"]
pub struct SDRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDRAM {}
impl SDRAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdram::RegisterBlock {
        0x4000_f000 as *const _
    }
}
impl Deref for SDRAM {
    type Target = sdram::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDRAM::ptr() }
    }
}
#[doc = "Synchronous DRAM Controller"]
pub mod sdram;
#[doc = "Flash Memory Controller-greg"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x4001_f000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "Flash Memory Controller-greg"]
pub mod fmc;
#[doc = "Flash Memory Interface"]
pub struct FTFE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFE {}
impl FTFE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftfe::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for FTFE {
    type Target = ftfe::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTFE::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfe;
#[doc = "DMA channel multiplexor"]
pub struct DMAMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX {}
impl DMAMUX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux::RegisterBlock {
        0x4002_1000 as *const _
    }
}
impl Deref for DMAMUX {
    type Target = dmamux::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX::ptr() }
    }
}
#[doc = "DMA channel multiplexor"]
pub mod dmamux;
#[doc = "Flex Controller Area Network module"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can0::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Flex Controller Area Network module"]
pub mod can0;
#[doc = "Flex Controller Area Network module"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x400a_4000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Flex Controller Area Network module"]
pub mod can1;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4002_d000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi1;
#[doc = "Serial Peripheral Interface"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi2::RegisterBlock {
        0x400a_c000 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi2;
#[doc = "Inter-IC Sound / Synchronous Audio Interface"]
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S0 {}
impl I2S0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s0::RegisterBlock {
        0x4002_f000 as *const _
    }
}
impl Deref for I2S0 {
    type Target = i2s0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S0::ptr() }
    }
}
#[doc = "Inter-IC Sound / Synchronous Audio Interface"]
pub mod i2s0;
#[doc = "Cyclic Redundancy Check"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc::RegisterBlock {
        0x4003_2000 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "Cyclic Redundancy Check"]
pub mod crc;
#[doc = "USB Device Charger Detection module"]
pub struct USBDCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBDCD {}
impl USBDCD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbdcd::RegisterBlock {
        0x4003_5000 as *const _
    }
}
impl Deref for USBDCD {
    type Target = usbdcd::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBDCD::ptr() }
    }
}
#[doc = "USB Device Charger Detection module"]
pub mod usbdcd;
#[doc = "Programmable Delay Block"]
pub struct PDB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDB0 {}
impl PDB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdb0::RegisterBlock {
        0x4003_6000 as *const _
    }
}
impl Deref for PDB0 {
    type Target = pdb0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDB0::ptr() }
    }
}
#[doc = "Programmable Delay Block"]
pub mod pdb0;
#[doc = "Periodic Interrupt Timer"]
pub struct PIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIT {}
impl PIT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pit::RegisterBlock {
        0x4003_7000 as *const _
    }
}
impl Deref for PIT {
    type Target = pit::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIT::ptr() }
    }
}
#[doc = "Periodic Interrupt Timer"]
pub mod pit;
#[doc = "FlexTimer Module"]
pub struct FTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM0 {}
impl FTM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftm0::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for FTM0 {
    type Target = ftm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTM0::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm0;
#[doc = "FlexTimer Module"]
pub struct FTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM1 {}
impl FTM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftm1::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for FTM1 {
    type Target = ftm1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTM1::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm1;
#[doc = "FlexTimer Module"]
pub struct FTM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM2 {}
impl FTM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftm2::RegisterBlock {
        0x4003_a000 as *const _
    }
}
impl Deref for FTM2 {
    type Target = ftm2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTM2::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm2;
#[doc = "FlexTimer Module"]
pub struct FTM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTM3 {}
impl FTM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftm3::RegisterBlock {
        0x400b_9000 as *const _
    }
}
impl Deref for FTM3 {
    type Target = ftm3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTM3::ptr() }
    }
}
#[doc = "FlexTimer Module"]
pub mod ftm3;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4003_b000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x400b_b000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc1;
#[doc = "Secure Real Time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4003_d000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Secure Real Time Clock"]
pub mod rtc;
#[doc = "VBAT register file"]
pub struct RFVBAT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFVBAT {}
impl RFVBAT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfvbat::RegisterBlock {
        0x4003_e000 as *const _
    }
}
impl Deref for RFVBAT {
    type Target = rfvbat::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFVBAT::ptr() }
    }
}
#[doc = "VBAT register file"]
pub mod rfvbat;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptmr0::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "System register file"]
pub struct RFSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RFSYS {}
impl RFSYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rfsys::RegisterBlock {
        0x4004_1000 as *const _
    }
}
impl Deref for RFSYS {
    type Target = rfsys::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RFSYS::ptr() }
    }
}
#[doc = "System register file"]
pub mod rfsys;
#[doc = "Touch sense input"]
pub struct TSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TSI0 {}
impl TSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tsi0::RegisterBlock {
        0x4004_5000 as *const _
    }
}
impl Deref for TSI0 {
    type Target = tsi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TSI0::ptr() }
    }
}
#[doc = "Touch sense input"]
pub mod tsi0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sim::RegisterBlock {
        0x4004_7000 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porta::RegisterBlock {
        0x4004_9000 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portb::RegisterBlock {
        0x4004_a000 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Pin Control and Interrupts"]
pub struct PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTC {}
impl PORTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portc::RegisterBlock {
        0x4004_b000 as *const _
    }
}
impl Deref for PORTC {
    type Target = portc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTC::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portc;
#[doc = "Pin Control and Interrupts"]
pub struct PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTD {}
impl PORTD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portd::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for PORTD {
    type Target = portd::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTD::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portd;
#[doc = "Pin Control and Interrupts"]
pub struct PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTE {}
impl PORTE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porte::RegisterBlock {
        0x4004_d000 as *const _
    }
}
impl Deref for PORTE {
    type Target = porte::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTE::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porte;
#[doc = "Generation 2008 Watchdog Timer"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog::RegisterBlock {
        0x4005_2000 as *const _
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDOG::ptr() }
    }
}
#[doc = "Generation 2008 Watchdog Timer"]
pub mod wdog;
#[doc = "External Watchdog Monitor"]
pub struct EWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EWM {}
impl EWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ewm::RegisterBlock {
        0x4006_1000 as *const _
    }
}
impl Deref for EWM {
    type Target = ewm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EWM::ptr() }
    }
}
#[doc = "External Watchdog Monitor"]
pub mod ewm;
#[doc = "Carrier Modulator Transmitter"]
pub struct CMT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMT {}
impl CMT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmt::RegisterBlock {
        0x4006_2000 as *const _
    }
}
impl Deref for CMT {
    type Target = cmt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMT::ptr() }
    }
}
#[doc = "Carrier Modulator Transmitter"]
pub mod cmt;
#[doc = "Multipurpose Clock Generator module"]
pub struct MCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCG {}
impl MCG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcg::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for MCG {
    type Target = mcg::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCG::ptr() }
    }
}
#[doc = "Multipurpose Clock Generator module"]
pub mod mcg;
#[doc = "Oscillator"]
pub struct OSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC {}
impl OSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const osc::RegisterBlock {
        0x4006_5000 as *const _
    }
}
impl Deref for OSC {
    type Target = osc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSC::ptr() }
    }
}
#[doc = "Oscillator"]
pub mod osc;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4006_6000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4006_7000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c1;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c2::RegisterBlock {
        0x400e_6000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c2;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c3::RegisterBlock {
        0x400e_7000 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c3;
#[doc = "Serial Communication Interface"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4006_a000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart0;
#[doc = "Serial Communication Interface"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4006_b000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart1;
#[doc = "Serial Communication Interface"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart2::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart2;
#[doc = "Serial Communication Interface"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart3::RegisterBlock {
        0x4006_d000 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart3;
#[doc = "Serial Communication Interface"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart4::RegisterBlock {
        0x400e_a000 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Serial Communication Interface"]
pub mod uart4;
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub struct USB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB0 {}
impl USB0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb0::RegisterBlock {
        0x4007_2000 as *const _
    }
}
impl Deref for USB0 {
    type Target = usb0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB0::ptr() }
    }
}
#[doc = "Universal Serial Bus, OTG Capable Controller"]
pub mod usb0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp0::RegisterBlock {
        0x4007_3000 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP1 {}
impl CMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp1::RegisterBlock {
        0x4007_3008 as *const _
    }
}
impl Deref for CMP1 {
    type Target = cmp1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP1::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp1;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP2 {}
impl CMP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp2::RegisterBlock {
        0x4007_3010 as *const _
    }
}
impl Deref for CMP2 {
    type Target = cmp2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP2::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp2;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP3 {}
impl CMP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp3::RegisterBlock {
        0x4007_3018 as *const _
    }
}
impl Deref for CMP3 {
    type Target = cmp3::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP3::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp3;
#[doc = "Voltage Reference"]
pub struct VREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vref::RegisterBlock {
        0x4007_4000 as *const _
    }
}
impl Deref for VREF {
    type Target = vref::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREF::ptr() }
    }
}
#[doc = "Voltage Reference"]
pub mod vref;
#[doc = "Low leakage wakeup unit"]
pub struct LLWU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LLWU {}
impl LLWU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const llwu::RegisterBlock {
        0x4007_c000 as *const _
    }
}
impl Deref for LLWU {
    type Target = llwu::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LLWU::ptr() }
    }
}
#[doc = "Low leakage wakeup unit"]
pub mod llwu;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x4007_d000 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smc::RegisterBlock {
        0x4007_e000 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcm::RegisterBlock {
        0x4007_f000 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "Random Number Generator Accelerator"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        0x400a_0000 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random Number Generator Accelerator"]
pub mod rng;
#[doc = "USB HS/FS/LS OTG Controller"]
pub struct USBHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHS {}
impl USBHS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhs::RegisterBlock {
        0x400a_1000 as *const _
    }
}
impl Deref for USBHS {
    type Target = usbhs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHS::ptr() }
    }
}
#[doc = "USB HS/FS/LS OTG Controller"]
pub mod usbhs;
#[doc = "USBPHY Register Reference Index"]
pub struct USBPHY {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBPHY {}
impl USBPHY {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbphy::RegisterBlock {
        0x400a_2000 as *const _
    }
}
impl Deref for USBPHY {
    type Target = usbphy::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBPHY::ptr() }
    }
}
#[doc = "USBPHY Register Reference Index"]
pub mod usbphy;
#[doc = "USB Device Charger Detection module"]
pub struct USBHSDCD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHSDCD {}
impl USBHSDCD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhsdcd::RegisterBlock {
        0x400a_3000 as *const _
    }
}
impl Deref for USBHSDCD {
    type Target = usbhsdcd::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHSDCD::ptr() }
    }
}
#[doc = "USB Device Charger Detection module"]
pub mod usbhsdcd;
#[doc = "Secured Digital Host Controller"]
pub struct SDHC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDHC {}
impl SDHC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdhc::RegisterBlock {
        0x400b_1000 as *const _
    }
}
impl Deref for SDHC {
    type Target = sdhc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDHC::ptr() }
    }
}
#[doc = "Secured Digital Host Controller"]
pub mod sdhc;
#[doc = "Ethernet MAC-NET Core"]
pub struct ENET {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ENET {}
impl ENET {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const enet::RegisterBlock {
        0x400c_0000 as *const _
    }
}
impl Deref for ENET {
    type Target = enet::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ENET::ptr() }
    }
}
#[doc = "Ethernet MAC-NET Core"]
pub mod enet;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct LPUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPUART0 {}
impl LPUART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lpuart0::RegisterBlock {
        0x400c_4000 as *const _
    }
}
impl Deref for LPUART0 {
    type Target = lpuart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPUART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod lpuart0;
#[doc = "Timer/PWM Module"]
pub struct TPM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM1 {}
impl TPM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tpm1::RegisterBlock {
        0x400c_9000 as *const _
    }
}
impl Deref for TPM1 {
    type Target = tpm1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TPM1::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm1;
#[doc = "Timer/PWM Module"]
pub struct TPM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM2 {}
impl TPM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tpm2::RegisterBlock {
        0x400c_a000 as *const _
    }
}
impl Deref for TPM2 {
    type Target = tpm2::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TPM2::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm2;
#[doc = "12-Bit Digital-to-Analog Converter"]
pub struct DAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC0 {}
impl DAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac0::RegisterBlock {
        0x400c_c000 as *const _
    }
}
impl Deref for DAC0 {
    type Target = dac0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC0::ptr() }
    }
}
#[doc = "12-Bit Digital-to-Analog Converter"]
pub mod dac0;
#[doc = "12-Bit Digital-to-Analog Converter"]
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x400c_d000 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac1::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC1::ptr() }
    }
}
#[doc = "12-Bit Digital-to-Analog Converter"]
pub mod dac1;
#[doc = "General Purpose Input/Output"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x400f_f000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioa;
#[doc = "General Purpose Input/Output"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x400f_f040 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiob;
#[doc = "General Purpose Input/Output"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x400f_f080 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioc;
#[doc = "General Purpose Input/Output"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x400f_f0c0 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiod;
#[doc = "General Purpose Input/Output"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioe::RegisterBlock {
        0x400f_f100 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioe;
#[doc = "System Control Block"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Block"]
pub mod system_control;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcm::RegisterBlock {
        0xe008_0000 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "Memory Mapped Cryptographic Acceleration Unit (MMCAU)"]
pub struct CAU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAU {}
impl CAU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cau::RegisterBlock {
        0xe008_1000 as *const _
    }
}
impl Deref for CAU {
    type Target = cau::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAU::ptr() }
    }
}
#[doc = "Memory Mapped Cryptographic Acceleration Unit (MMCAU)"]
pub mod cau;
#[doc = "Local Memory Controller"]
pub struct LMEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LMEM {}
impl LMEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lmem::RegisterBlock {
        0xe008_2000 as *const _
    }
}
impl Deref for LMEM {
    type Target = lmem::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LMEM::ptr() }
    }
}
#[doc = "Local Memory Controller"]
pub mod lmem;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTFE_FLASHCONFIG"]
    pub FTFE_FLASHCONFIG: FTFE_FLASHCONFIG,
    #[doc = "AIPS0"]
    pub AIPS0: AIPS0,
    #[doc = "AIPS1"]
    pub AIPS1: AIPS1,
    #[doc = "AXBS"]
    pub AXBS: AXBS,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "FB"]
    pub FB: FB,
    #[doc = "SYSMPU"]
    pub SYSMPU: SYSMPU,
    #[doc = "SDRAM"]
    pub SDRAM: SDRAM,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "FTFE"]
    pub FTFE: FTFE,
    #[doc = "DMAMUX"]
    pub DMAMUX: DMAMUX,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "I2S0"]
    pub I2S0: I2S0,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "USBDCD"]
    pub USBDCD: USBDCD,
    #[doc = "PDB0"]
    pub PDB0: PDB0,
    #[doc = "PIT"]
    pub PIT: PIT,
    #[doc = "FTM0"]
    pub FTM0: FTM0,
    #[doc = "FTM1"]
    pub FTM1: FTM1,
    #[doc = "FTM2"]
    pub FTM2: FTM2,
    #[doc = "FTM3"]
    pub FTM3: FTM3,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "RFVBAT"]
    pub RFVBAT: RFVBAT,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "RFSYS"]
    pub RFSYS: RFSYS,
    #[doc = "TSI0"]
    pub TSI0: TSI0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "PORTC"]
    pub PORTC: PORTC,
    #[doc = "PORTD"]
    pub PORTD: PORTD,
    #[doc = "PORTE"]
    pub PORTE: PORTE,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
    #[doc = "EWM"]
    pub EWM: EWM,
    #[doc = "CMT"]
    pub CMT: CMT,
    #[doc = "MCG"]
    pub MCG: MCG,
    #[doc = "OSC"]
    pub OSC: OSC,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "USB0"]
    pub USB0: USB0,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "CMP1"]
    pub CMP1: CMP1,
    #[doc = "CMP2"]
    pub CMP2: CMP2,
    #[doc = "CMP3"]
    pub CMP3: CMP3,
    #[doc = "VREF"]
    pub VREF: VREF,
    #[doc = "LLWU"]
    pub LLWU: LLWU,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "USBHS"]
    pub USBHS: USBHS,
    #[doc = "USBPHY"]
    pub USBPHY: USBPHY,
    #[doc = "USBHSDCD"]
    pub USBHSDCD: USBHSDCD,
    #[doc = "SDHC"]
    pub SDHC: SDHC,
    #[doc = "ENET"]
    pub ENET: ENET,
    #[doc = "LPUART0"]
    pub LPUART0: LPUART0,
    #[doc = "TPM1"]
    pub TPM1: TPM1,
    #[doc = "TPM2"]
    pub TPM2: TPM2,
    #[doc = "DAC0"]
    pub DAC0: DAC0,
    #[doc = "DAC1"]
    pub DAC1: DAC1,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "CAU"]
    pub CAU: CAU,
    #[doc = "LMEM"]
    pub LMEM: LMEM,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
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
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            FTFE_FLASHCONFIG: FTFE_FLASHCONFIG {
                _marker: PhantomData,
            },
            AIPS0: AIPS0 {
                _marker: PhantomData,
            },
            AIPS1: AIPS1 {
                _marker: PhantomData,
            },
            AXBS: AXBS {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            FB: FB {
                _marker: PhantomData,
            },
            SYSMPU: SYSMPU {
                _marker: PhantomData,
            },
            SDRAM: SDRAM {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            FTFE: FTFE {
                _marker: PhantomData,
            },
            DMAMUX: DMAMUX {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            I2S0: I2S0 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            USBDCD: USBDCD {
                _marker: PhantomData,
            },
            PDB0: PDB0 {
                _marker: PhantomData,
            },
            PIT: PIT {
                _marker: PhantomData,
            },
            FTM0: FTM0 {
                _marker: PhantomData,
            },
            FTM1: FTM1 {
                _marker: PhantomData,
            },
            FTM2: FTM2 {
                _marker: PhantomData,
            },
            FTM3: FTM3 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            RFVBAT: RFVBAT {
                _marker: PhantomData,
            },
            LPTMR0: LPTMR0 {
                _marker: PhantomData,
            },
            RFSYS: RFSYS {
                _marker: PhantomData,
            },
            TSI0: TSI0 {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
            PORTC: PORTC {
                _marker: PhantomData,
            },
            PORTD: PORTD {
                _marker: PhantomData,
            },
            PORTE: PORTE {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
            EWM: EWM {
                _marker: PhantomData,
            },
            CMT: CMT {
                _marker: PhantomData,
            },
            MCG: MCG {
                _marker: PhantomData,
            },
            OSC: OSC {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            USB0: USB0 {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            CMP1: CMP1 {
                _marker: PhantomData,
            },
            CMP2: CMP2 {
                _marker: PhantomData,
            },
            CMP3: CMP3 {
                _marker: PhantomData,
            },
            VREF: VREF {
                _marker: PhantomData,
            },
            LLWU: LLWU {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            RCM: RCM {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            USBHS: USBHS {
                _marker: PhantomData,
            },
            USBPHY: USBPHY {
                _marker: PhantomData,
            },
            USBHSDCD: USBHSDCD {
                _marker: PhantomData,
            },
            SDHC: SDHC {
                _marker: PhantomData,
            },
            ENET: ENET {
                _marker: PhantomData,
            },
            LPUART0: LPUART0 {
                _marker: PhantomData,
            },
            TPM1: TPM1 {
                _marker: PhantomData,
            },
            TPM2: TPM2 {
                _marker: PhantomData,
            },
            DAC0: DAC0 {
                _marker: PhantomData,
            },
            DAC1: DAC1 {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            CAU: CAU {
                _marker: PhantomData,
            },
            LMEM: LMEM {
                _marker: PhantomData,
            },
        }
    }
}
