#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `AFRL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL7R {
    #[doc = "undocumented"]
    TIM1_CH1N,
    #[doc = "undocumented"]
    TIM3_CH2,
    #[doc = "undocumented"]
    TIM8_CH1N,
    #[doc = "undocumented"]
    SPI1_MOSI,
    #[doc = "undocumented"]
    TIM14_CH1,
    #[doc = "undocumented"]
    ETH_MII_RX_DV_ETH_RMMI_CRS_DV,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL7R::TIM1_CH1N => 1,
            AFRL7R::TIM3_CH2 => 2,
            AFRL7R::TIM8_CH1N => 3,
            AFRL7R::SPI1_MOSI => 5,
            AFRL7R::TIM14_CH1 => 9,
            AFRL7R::ETH_MII_RX_DV_ETH_RMMI_CRS_DV => 11,
            AFRL7R::EVENTOUT => 15,
            AFRL7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL7R {
        match value {
            1 => AFRL7R::TIM1_CH1N,
            2 => AFRL7R::TIM3_CH2,
            3 => AFRL7R::TIM8_CH1N,
            5 => AFRL7R::SPI1_MOSI,
            9 => AFRL7R::TIM14_CH1,
            11 => AFRL7R::ETH_MII_RX_DV_ETH_RMMI_CRS_DV,
            15 => AFRL7R::EVENTOUT,
            i => AFRL7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1_CH1N`"]
    #[inline]
    pub fn is_tim1_ch1n(&self) -> bool {
        *self == AFRL7R::TIM1_CH1N
    }
    #[doc = "Checks if the value of the field is `TIM3_CH2`"]
    #[inline]
    pub fn is_tim3_ch2(&self) -> bool {
        *self == AFRL7R::TIM3_CH2
    }
    #[doc = "Checks if the value of the field is `TIM8_CH1N`"]
    #[inline]
    pub fn is_tim8_ch1n(&self) -> bool {
        *self == AFRL7R::TIM8_CH1N
    }
    #[doc = "Checks if the value of the field is `SPI1_MOSI`"]
    #[inline]
    pub fn is_spi1_mosi(&self) -> bool {
        *self == AFRL7R::SPI1_MOSI
    }
    #[doc = "Checks if the value of the field is `TIM14_CH1`"]
    #[inline]
    pub fn is_tim14_ch1(&self) -> bool {
        *self == AFRL7R::TIM14_CH1
    }
    #[doc = "Checks if the value of the field is `ETH_MII_RX_DV_ETH_RMMI_CRS_DV`"]
    #[inline]
    pub fn is_eth_mii_rx_dv_eth_rmmi_crs_dv(&self) -> bool {
        *self == AFRL7R::ETH_MII_RX_DV_ETH_RMMI_CRS_DV
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL7R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL6R {
    #[doc = "undocumented"]
    TIM1_BKIN,
    #[doc = "undocumented"]
    TIM3_CH1,
    #[doc = "undocumented"]
    TIM8_BKIN,
    #[doc = "undocumented"]
    SPI1_MISO,
    #[doc = "undocumented"]
    TIM13_CH1,
    #[doc = "undocumented"]
    DCMI_PIXCK,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL6R::TIM1_BKIN => 1,
            AFRL6R::TIM3_CH1 => 2,
            AFRL6R::TIM8_BKIN => 3,
            AFRL6R::SPI1_MISO => 5,
            AFRL6R::TIM13_CH1 => 9,
            AFRL6R::DCMI_PIXCK => 13,
            AFRL6R::EVENTOUT => 15,
            AFRL6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL6R {
        match value {
            1 => AFRL6R::TIM1_BKIN,
            2 => AFRL6R::TIM3_CH1,
            3 => AFRL6R::TIM8_BKIN,
            5 => AFRL6R::SPI1_MISO,
            9 => AFRL6R::TIM13_CH1,
            13 => AFRL6R::DCMI_PIXCK,
            15 => AFRL6R::EVENTOUT,
            i => AFRL6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1_BKIN`"]
    #[inline]
    pub fn is_tim1_bkin(&self) -> bool {
        *self == AFRL6R::TIM1_BKIN
    }
    #[doc = "Checks if the value of the field is `TIM3_CH1`"]
    #[inline]
    pub fn is_tim3_ch1(&self) -> bool {
        *self == AFRL6R::TIM3_CH1
    }
    #[doc = "Checks if the value of the field is `TIM8_BKIN`"]
    #[inline]
    pub fn is_tim8_bkin(&self) -> bool {
        *self == AFRL6R::TIM8_BKIN
    }
    #[doc = "Checks if the value of the field is `SPI1_MISO`"]
    #[inline]
    pub fn is_spi1_miso(&self) -> bool {
        *self == AFRL6R::SPI1_MISO
    }
    #[doc = "Checks if the value of the field is `TIM13_CH1`"]
    #[inline]
    pub fn is_tim13_ch1(&self) -> bool {
        *self == AFRL6R::TIM13_CH1
    }
    #[doc = "Checks if the value of the field is `DCMI_PIXCK`"]
    #[inline]
    pub fn is_dcmi_pixck(&self) -> bool {
        *self == AFRL6R::DCMI_PIXCK
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL6R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL5R {
    #[doc = "undocumented"]
    TIM2_CH1_ETR,
    #[doc = "undocumented"]
    TIM8_CH1N,
    #[doc = "undocumented"]
    SPI1_SCK,
    #[doc = "undocumented"]
    OTG_HS_ULPI_CK,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL5R::TIM2_CH1_ETR => 1,
            AFRL5R::TIM8_CH1N => 3,
            AFRL5R::SPI1_SCK => 5,
            AFRL5R::OTG_HS_ULPI_CK => 10,
            AFRL5R::EVENTOUT => 15,
            AFRL5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL5R {
        match value {
            1 => AFRL5R::TIM2_CH1_ETR,
            3 => AFRL5R::TIM8_CH1N,
            5 => AFRL5R::SPI1_SCK,
            10 => AFRL5R::OTG_HS_ULPI_CK,
            15 => AFRL5R::EVENTOUT,
            i => AFRL5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM2_CH1_ETR`"]
    #[inline]
    pub fn is_tim2_ch1_etr(&self) -> bool {
        *self == AFRL5R::TIM2_CH1_ETR
    }
    #[doc = "Checks if the value of the field is `TIM8_CH1N`"]
    #[inline]
    pub fn is_tim8_ch1n(&self) -> bool {
        *self == AFRL5R::TIM8_CH1N
    }
    #[doc = "Checks if the value of the field is `SPI1_SCK`"]
    #[inline]
    pub fn is_spi1_sck(&self) -> bool {
        *self == AFRL5R::SPI1_SCK
    }
    #[doc = "Checks if the value of the field is `OTG_HS_ULPI_CK`"]
    #[inline]
    pub fn is_otg_hs_ulpi_ck(&self) -> bool {
        *self == AFRL5R::OTG_HS_ULPI_CK
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL5R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL4R {
    #[doc = "undocumented"]
    SPI1_NSS,
    #[doc = "undocumented"]
    SPI3_NSS_I2S3_WS,
    #[doc = "undocumented"]
    USART2_CK,
    #[doc = "undocumented"]
    OTG_HS_SOF,
    #[doc = "undocumented"]
    DCMI_HSYNC,
    #[doc = "EVENTOUT"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL4R::SPI1_NSS => 5,
            AFRL4R::SPI3_NSS_I2S3_WS => 6,
            AFRL4R::USART2_CK => 7,
            AFRL4R::OTG_HS_SOF => 12,
            AFRL4R::DCMI_HSYNC => 13,
            AFRL4R::EVENTOUT => 15,
            AFRL4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL4R {
        match value {
            5 => AFRL4R::SPI1_NSS,
            6 => AFRL4R::SPI3_NSS_I2S3_WS,
            7 => AFRL4R::USART2_CK,
            12 => AFRL4R::OTG_HS_SOF,
            13 => AFRL4R::DCMI_HSYNC,
            15 => AFRL4R::EVENTOUT,
            i => AFRL4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_NSS`"]
    #[inline]
    pub fn is_spi1_nss(&self) -> bool {
        *self == AFRL4R::SPI1_NSS
    }
    #[doc = "Checks if the value of the field is `SPI3_NSS_I2S3_WS`"]
    #[inline]
    pub fn is_spi3_nss_i2s3_ws(&self) -> bool {
        *self == AFRL4R::SPI3_NSS_I2S3_WS
    }
    #[doc = "Checks if the value of the field is `USART2_CK`"]
    #[inline]
    pub fn is_usart2_ck(&self) -> bool {
        *self == AFRL4R::USART2_CK
    }
    #[doc = "Checks if the value of the field is `OTG_HS_SOF`"]
    #[inline]
    pub fn is_otg_hs_sof(&self) -> bool {
        *self == AFRL4R::OTG_HS_SOF
    }
    #[doc = "Checks if the value of the field is `DCMI_HSYNC`"]
    #[inline]
    pub fn is_dcmi_hsync(&self) -> bool {
        *self == AFRL4R::DCMI_HSYNC
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL4R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL3R {
    #[doc = "undocumented"]
    TIM2_CH4,
    #[doc = "undocumented"]
    TIM5_CH5,
    #[doc = "undocumented"]
    TIM9_CH2,
    #[doc = "undocumented"]
    USART2_RX,
    #[doc = "undocumented"]
    OTG_HS_ULPI_D0,
    #[doc = "undocumented"]
    ETH_MII_COL,
    #[doc = "EVENTOUT"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL3R::TIM2_CH4 => 1,
            AFRL3R::TIM5_CH5 => 2,
            AFRL3R::TIM9_CH2 => 3,
            AFRL3R::USART2_RX => 7,
            AFRL3R::OTG_HS_ULPI_D0 => 10,
            AFRL3R::ETH_MII_COL => 11,
            AFRL3R::EVENTOUT => 15,
            AFRL3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL3R {
        match value {
            1 => AFRL3R::TIM2_CH4,
            2 => AFRL3R::TIM5_CH5,
            3 => AFRL3R::TIM9_CH2,
            7 => AFRL3R::USART2_RX,
            10 => AFRL3R::OTG_HS_ULPI_D0,
            11 => AFRL3R::ETH_MII_COL,
            15 => AFRL3R::EVENTOUT,
            i => AFRL3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM2_CH4`"]
    #[inline]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == AFRL3R::TIM2_CH4
    }
    #[doc = "Checks if the value of the field is `TIM5_CH5`"]
    #[inline]
    pub fn is_tim5_ch5(&self) -> bool {
        *self == AFRL3R::TIM5_CH5
    }
    #[doc = "Checks if the value of the field is `TIM9_CH2`"]
    #[inline]
    pub fn is_tim9_ch2(&self) -> bool {
        *self == AFRL3R::TIM9_CH2
    }
    #[doc = "Checks if the value of the field is `USART2_RX`"]
    #[inline]
    pub fn is_usart2_rx(&self) -> bool {
        *self == AFRL3R::USART2_RX
    }
    #[doc = "Checks if the value of the field is `OTG_HS_ULPI_D0`"]
    #[inline]
    pub fn is_otg_hs_ulpi_d0(&self) -> bool {
        *self == AFRL3R::OTG_HS_ULPI_D0
    }
    #[doc = "Checks if the value of the field is `ETH_MII_COL`"]
    #[inline]
    pub fn is_eth_mii_col(&self) -> bool {
        *self == AFRL3R::ETH_MII_COL
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL3R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL2R {
    #[doc = "undocumented"]
    TIM2_CH3,
    #[doc = "undocumented"]
    TIM5_CH3,
    #[doc = "undocumented"]
    TIM9_CH1,
    #[doc = "undocumented"]
    USART2_TX,
    #[doc = "undocumented"]
    ETH_MDIO,
    #[doc = "EVENTOUT"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL2R::TIM2_CH3 => 1,
            AFRL2R::TIM5_CH3 => 2,
            AFRL2R::TIM9_CH1 => 3,
            AFRL2R::USART2_TX => 7,
            AFRL2R::ETH_MDIO => 11,
            AFRL2R::EVENTOUT => 15,
            AFRL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL2R {
        match value {
            1 => AFRL2R::TIM2_CH3,
            2 => AFRL2R::TIM5_CH3,
            3 => AFRL2R::TIM9_CH1,
            7 => AFRL2R::USART2_TX,
            11 => AFRL2R::ETH_MDIO,
            15 => AFRL2R::EVENTOUT,
            i => AFRL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM2_CH3`"]
    #[inline]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == AFRL2R::TIM2_CH3
    }
    #[doc = "Checks if the value of the field is `TIM5_CH3`"]
    #[inline]
    pub fn is_tim5_ch3(&self) -> bool {
        *self == AFRL2R::TIM5_CH3
    }
    #[doc = "Checks if the value of the field is `TIM9_CH1`"]
    #[inline]
    pub fn is_tim9_ch1(&self) -> bool {
        *self == AFRL2R::TIM9_CH1
    }
    #[doc = "Checks if the value of the field is `USART2_TX`"]
    #[inline]
    pub fn is_usart2_tx(&self) -> bool {
        *self == AFRL2R::USART2_TX
    }
    #[doc = "Checks if the value of the field is `ETH_MDIO`"]
    #[inline]
    pub fn is_eth_mdio(&self) -> bool {
        *self == AFRL2R::ETH_MDIO
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL2R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL1R {
    #[doc = "undocumented"]
    TIM2_CH2,
    #[doc = "undocumented"]
    TIM5_CH2,
    #[doc = "undocumented"]
    USART2_RTS,
    #[doc = "undocumented"]
    UART4_RX,
    #[doc = "undocumented"]
    ETH_MII_RX_CLK_ETH_RMII_REF_CLK,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL1R::TIM2_CH2 => 1,
            AFRL1R::TIM5_CH2 => 2,
            AFRL1R::USART2_RTS => 7,
            AFRL1R::UART4_RX => 8,
            AFRL1R::ETH_MII_RX_CLK_ETH_RMII_REF_CLK => 11,
            AFRL1R::EVENTOUT => 15,
            AFRL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL1R {
        match value {
            1 => AFRL1R::TIM2_CH2,
            2 => AFRL1R::TIM5_CH2,
            7 => AFRL1R::USART2_RTS,
            8 => AFRL1R::UART4_RX,
            11 => AFRL1R::ETH_MII_RX_CLK_ETH_RMII_REF_CLK,
            15 => AFRL1R::EVENTOUT,
            i => AFRL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM2_CH2`"]
    #[inline]
    pub fn is_tim2_ch2(&self) -> bool {
        *self == AFRL1R::TIM2_CH2
    }
    #[doc = "Checks if the value of the field is `TIM5_CH2`"]
    #[inline]
    pub fn is_tim5_ch2(&self) -> bool {
        *self == AFRL1R::TIM5_CH2
    }
    #[doc = "Checks if the value of the field is `USART2_RTS`"]
    #[inline]
    pub fn is_usart2_rts(&self) -> bool {
        *self == AFRL1R::USART2_RTS
    }
    #[doc = "Checks if the value of the field is `UART4_RX`"]
    #[inline]
    pub fn is_uart4_rx(&self) -> bool {
        *self == AFRL1R::UART4_RX
    }
    #[doc = "Checks if the value of the field is `ETH_MII_RX_CLK_ETH_RMII_REF_CLK`"]
    #[inline]
    pub fn is_eth_mii_rx_clk_eth_rmii_ref_clk(&self) -> bool {
        *self == AFRL1R::ETH_MII_RX_CLK_ETH_RMII_REF_CLK
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL1R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRL0R {
    #[doc = "undocumented"]
    TIM2_CH1_ETR,
    #[doc = "undocumented"]
    TIM5_CH1,
    #[doc = "undocumented"]
    TIM8_ETR,
    #[doc = "undocumented"]
    USART2_CTS,
    #[doc = "undocumented"]
    UART4_TX,
    #[doc = "undocumented"]
    ETH_MII_CRS,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRL0R::TIM2_CH1_ETR => 1,
            AFRL0R::TIM5_CH1 => 2,
            AFRL0R::TIM8_ETR => 3,
            AFRL0R::USART2_CTS => 7,
            AFRL0R::UART4_TX => 8,
            AFRL0R::ETH_MII_CRS => 11,
            AFRL0R::EVENTOUT => 15,
            AFRL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRL0R {
        match value {
            1 => AFRL0R::TIM2_CH1_ETR,
            2 => AFRL0R::TIM5_CH1,
            3 => AFRL0R::TIM8_ETR,
            7 => AFRL0R::USART2_CTS,
            8 => AFRL0R::UART4_TX,
            11 => AFRL0R::ETH_MII_CRS,
            15 => AFRL0R::EVENTOUT,
            i => AFRL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM2_CH1_ETR`"]
    #[inline]
    pub fn is_tim2_ch1_etr(&self) -> bool {
        *self == AFRL0R::TIM2_CH1_ETR
    }
    #[doc = "Checks if the value of the field is `TIM5_CH1`"]
    #[inline]
    pub fn is_tim5_ch1(&self) -> bool {
        *self == AFRL0R::TIM5_CH1
    }
    #[doc = "Checks if the value of the field is `TIM8_ETR`"]
    #[inline]
    pub fn is_tim8_etr(&self) -> bool {
        *self == AFRL0R::TIM8_ETR
    }
    #[doc = "Checks if the value of the field is `USART2_CTS`"]
    #[inline]
    pub fn is_usart2_cts(&self) -> bool {
        *self == AFRL0R::USART2_CTS
    }
    #[doc = "Checks if the value of the field is `UART4_TX`"]
    #[inline]
    pub fn is_uart4_tx(&self) -> bool {
        *self == AFRL0R::UART4_TX
    }
    #[doc = "Checks if the value of the field is `ETH_MII_CRS`"]
    #[inline]
    pub fn is_eth_mii_crs(&self) -> bool {
        *self == AFRL0R::ETH_MII_CRS
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRL0R::EVENTOUT
    }
}
#[doc = "Values that can be written to the field `AFRL7`"]
pub enum AFRL7W {
    #[doc = "`1`"]
    TIM1_CH1N,
    #[doc = "`10`"]
    TIM3_CH2,
    #[doc = "`11`"]
    TIM8_CH1N,
    #[doc = "`101`"]
    SPI1_MOSI,
    #[doc = "`1001`"]
    TIM14_CH1,
    #[doc = "`1011`"]
    ETH_MII_RX_DV_ETH_RMMI_CRS_DV,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL7W::TIM1_CH1N => 1,
            AFRL7W::TIM3_CH2 => 2,
            AFRL7W::TIM8_CH1N => 3,
            AFRL7W::SPI1_MOSI => 5,
            AFRL7W::TIM14_CH1 => 9,
            AFRL7W::ETH_MII_RX_DV_ETH_RMMI_CRS_DV => 11,
            AFRL7W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL7W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim1_ch1n(self) -> &'a mut W {
        self.variant(AFRL7W::TIM1_CH1N)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn tim3_ch2(self) -> &'a mut W {
        self.variant(AFRL7W::TIM3_CH2)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn tim8_ch1n(self) -> &'a mut W {
        self.variant(AFRL7W::TIM8_CH1N)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn spi1_mosi(self) -> &'a mut W {
        self.variant(AFRL7W::SPI1_MOSI)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn tim14_ch1(self) -> &'a mut W {
        self.variant(AFRL7W::TIM14_CH1)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn eth_mii_rx_dv_eth_rmmi_crs_dv(self) -> &'a mut W {
        self.variant(AFRL7W::ETH_MII_RX_DV_ETH_RMMI_CRS_DV)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL7W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL6`"]
pub enum AFRL6W {
    #[doc = "`1`"]
    TIM1_BKIN,
    #[doc = "`10`"]
    TIM3_CH1,
    #[doc = "`11`"]
    TIM8_BKIN,
    #[doc = "`101`"]
    SPI1_MISO,
    #[doc = "`1001`"]
    TIM13_CH1,
    #[doc = "`1101`"]
    DCMI_PIXCK,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL6W::TIM1_BKIN => 1,
            AFRL6W::TIM3_CH1 => 2,
            AFRL6W::TIM8_BKIN => 3,
            AFRL6W::SPI1_MISO => 5,
            AFRL6W::TIM13_CH1 => 9,
            AFRL6W::DCMI_PIXCK => 13,
            AFRL6W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL6W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim1_bkin(self) -> &'a mut W {
        self.variant(AFRL6W::TIM1_BKIN)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn tim3_ch1(self) -> &'a mut W {
        self.variant(AFRL6W::TIM3_CH1)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn tim8_bkin(self) -> &'a mut W {
        self.variant(AFRL6W::TIM8_BKIN)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn spi1_miso(self) -> &'a mut W {
        self.variant(AFRL6W::SPI1_MISO)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn tim13_ch1(self) -> &'a mut W {
        self.variant(AFRL6W::TIM13_CH1)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn dcmi_pixck(self) -> &'a mut W {
        self.variant(AFRL6W::DCMI_PIXCK)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL6W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL5`"]
pub enum AFRL5W {
    #[doc = "`1`"]
    TIM2_CH1_ETR,
    #[doc = "`11`"]
    TIM8_CH1N,
    #[doc = "`101`"]
    SPI1_SCK,
    #[doc = "`1010`"]
    OTG_HS_ULPI_CK,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL5W::TIM2_CH1_ETR => 1,
            AFRL5W::TIM8_CH1N => 3,
            AFRL5W::SPI1_SCK => 5,
            AFRL5W::OTG_HS_ULPI_CK => 10,
            AFRL5W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL5W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim2_ch1_etr(self) -> &'a mut W {
        self.variant(AFRL5W::TIM2_CH1_ETR)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn tim8_ch1n(self) -> &'a mut W {
        self.variant(AFRL5W::TIM8_CH1N)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn spi1_sck(self) -> &'a mut W {
        self.variant(AFRL5W::SPI1_SCK)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn otg_hs_ulpi_ck(self) -> &'a mut W {
        self.variant(AFRL5W::OTG_HS_ULPI_CK)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL5W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL4`"]
pub enum AFRL4W {
    #[doc = "`101`"]
    SPI1_NSS,
    #[doc = "`110`"]
    SPI3_NSS_I2S3_WS,
    #[doc = "`111`"]
    USART2_CK,
    #[doc = "`1100`"]
    OTG_HS_SOF,
    #[doc = "`1101`"]
    DCMI_HSYNC,
    #[doc = "EVENTOUT"]
    EVENTOUT,
}
impl AFRL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL4W::SPI1_NSS => 5,
            AFRL4W::SPI3_NSS_I2S3_WS => 6,
            AFRL4W::USART2_CK => 7,
            AFRL4W::OTG_HS_SOF => 12,
            AFRL4W::DCMI_HSYNC => 13,
            AFRL4W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL4W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`101`"]
    #[inline]
    pub fn spi1_nss(self) -> &'a mut W {
        self.variant(AFRL4W::SPI1_NSS)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn spi3_nss_i2s3_ws(self) -> &'a mut W {
        self.variant(AFRL4W::SPI3_NSS_I2S3_WS)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart2_ck(self) -> &'a mut W {
        self.variant(AFRL4W::USART2_CK)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn otg_hs_sof(self) -> &'a mut W {
        self.variant(AFRL4W::OTG_HS_SOF)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn dcmi_hsync(self) -> &'a mut W {
        self.variant(AFRL4W::DCMI_HSYNC)
    }
    #[doc = "EVENTOUT"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL4W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL3`"]
pub enum AFRL3W {
    #[doc = "`1`"]
    TIM2_CH4,
    #[doc = "`10`"]
    TIM5_CH5,
    #[doc = "`11`"]
    TIM9_CH2,
    #[doc = "`111`"]
    USART2_RX,
    #[doc = "`1010`"]
    OTG_HS_ULPI_D0,
    #[doc = "`1011`"]
    ETH_MII_COL,
    #[doc = "EVENTOUT"]
    EVENTOUT,
}
impl AFRL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL3W::TIM2_CH4 => 1,
            AFRL3W::TIM5_CH5 => 2,
            AFRL3W::TIM9_CH2 => 3,
            AFRL3W::USART2_RX => 7,
            AFRL3W::OTG_HS_ULPI_D0 => 10,
            AFRL3W::ETH_MII_COL => 11,
            AFRL3W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL3W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(AFRL3W::TIM2_CH4)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn tim5_ch5(self) -> &'a mut W {
        self.variant(AFRL3W::TIM5_CH5)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn tim9_ch2(self) -> &'a mut W {
        self.variant(AFRL3W::TIM9_CH2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart2_rx(self) -> &'a mut W {
        self.variant(AFRL3W::USART2_RX)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn otg_hs_ulpi_d0(self) -> &'a mut W {
        self.variant(AFRL3W::OTG_HS_ULPI_D0)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn eth_mii_col(self) -> &'a mut W {
        self.variant(AFRL3W::ETH_MII_COL)
    }
    #[doc = "EVENTOUT"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL3W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL2`"]
pub enum AFRL2W {
    #[doc = "`1`"]
    TIM2_CH3,
    #[doc = "`10`"]
    TIM5_CH3,
    #[doc = "`11`"]
    TIM9_CH1,
    #[doc = "`111`"]
    USART2_TX,
    #[doc = "`1011`"]
    ETH_MDIO,
    #[doc = "EVENTOUT"]
    EVENTOUT,
}
impl AFRL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL2W::TIM2_CH3 => 1,
            AFRL2W::TIM5_CH3 => 2,
            AFRL2W::TIM9_CH1 => 3,
            AFRL2W::USART2_TX => 7,
            AFRL2W::ETH_MDIO => 11,
            AFRL2W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL2W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(AFRL2W::TIM2_CH3)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn tim5_ch3(self) -> &'a mut W {
        self.variant(AFRL2W::TIM5_CH3)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn tim9_ch1(self) -> &'a mut W {
        self.variant(AFRL2W::TIM9_CH1)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart2_tx(self) -> &'a mut W {
        self.variant(AFRL2W::USART2_TX)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn eth_mdio(self) -> &'a mut W {
        self.variant(AFRL2W::ETH_MDIO)
    }
    #[doc = "EVENTOUT"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL2W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL1`"]
pub enum AFRL1W {
    #[doc = "`1`"]
    TIM2_CH2,
    #[doc = "`10`"]
    TIM5_CH2,
    #[doc = "`111`"]
    USART2_RTS,
    #[doc = "`1000`"]
    UART4_RX,
    #[doc = "`1011`"]
    ETH_MII_RX_CLK_ETH_RMII_REF_CLK,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL1W::TIM2_CH2 => 1,
            AFRL1W::TIM5_CH2 => 2,
            AFRL1W::USART2_RTS => 7,
            AFRL1W::UART4_RX => 8,
            AFRL1W::ETH_MII_RX_CLK_ETH_RMII_REF_CLK => 11,
            AFRL1W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL1W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim2_ch2(self) -> &'a mut W {
        self.variant(AFRL1W::TIM2_CH2)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn tim5_ch2(self) -> &'a mut W {
        self.variant(AFRL1W::TIM5_CH2)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart2_rts(self) -> &'a mut W {
        self.variant(AFRL1W::USART2_RTS)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn uart4_rx(self) -> &'a mut W {
        self.variant(AFRL1W::UART4_RX)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn eth_mii_rx_clk_eth_rmii_ref_clk(self) -> &'a mut W {
        self.variant(AFRL1W::ETH_MII_RX_CLK_ETH_RMII_REF_CLK)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL1W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AFRL0`"]
pub enum AFRL0W {
    #[doc = "`1`"]
    TIM2_CH1_ETR,
    #[doc = "`10`"]
    TIM5_CH1,
    #[doc = "`11`"]
    TIM8_ETR,
    #[doc = "`111`"]
    USART2_CTS,
    #[doc = "`1000`"]
    UART4_TX,
    #[doc = "`1011`"]
    ETH_MII_CRS,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRL0W::TIM2_CH1_ETR => 1,
            AFRL0W::TIM5_CH1 => 2,
            AFRL0W::TIM8_ETR => 3,
            AFRL0W::USART2_CTS => 7,
            AFRL0W::UART4_TX => 8,
            AFRL0W::ETH_MII_CRS => 11,
            AFRL0W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRL0W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim2_ch1_etr(self) -> &'a mut W {
        self.variant(AFRL0W::TIM2_CH1_ETR)
    }
    #[doc = "`10`"]
    #[inline]
    pub fn tim5_ch1(self) -> &'a mut W {
        self.variant(AFRL0W::TIM5_CH1)
    }
    #[doc = "`11`"]
    #[inline]
    pub fn tim8_etr(self) -> &'a mut W {
        self.variant(AFRL0W::TIM8_ETR)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart2_cts(self) -> &'a mut W {
        self.variant(AFRL0W::USART2_CTS)
    }
    #[doc = "`1000`"]
    #[inline]
    pub fn uart4_tx(self) -> &'a mut W {
        self.variant(AFRL0W::UART4_TX)
    }
    #[doc = "`1011`"]
    #[inline]
    pub fn eth_mii_crs(self) -> &'a mut W {
        self.variant(AFRL0W::ETH_MII_CRS)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRL0W::EVENTOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl7(&self) -> AFRL7R {
        AFRL7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl6(&self) -> AFRL6R {
        AFRL6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl5(&self) -> AFRL5R {
        AFRL5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl4(&self) -> AFRL4R {
        AFRL4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl3(&self) -> AFRL3R {
        AFRL3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl2(&self) -> AFRL2R {
        AFRL2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl1(&self) -> AFRL1R {
        AFRL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl0(&self) -> AFRL0R {
        AFRL0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl7(&mut self) -> _AFRL7W {
        _AFRL7W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl6(&mut self) -> _AFRL6W {
        _AFRL6W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl5(&mut self) -> _AFRL5W {
        _AFRL5W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl4(&mut self) -> _AFRL4W {
        _AFRL4W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl3(&mut self) -> _AFRL3W {
        _AFRL3W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl2(&mut self) -> _AFRL2W {
        _AFRL2W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl1(&mut self) -> _AFRL1W {
        _AFRL1W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y(y = 0..7)"]
    #[inline]
    pub fn afrl0(&mut self) -> _AFRL0W {
        _AFRL0W { w: self }
    }
}
