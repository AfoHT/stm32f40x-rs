#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFRH {
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
#[doc = "Possible values of the field `AFRH15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH15R {
    #[doc = "undocumented"]
    JTDI,
    #[doc = "undocumented"]
    TIM2_CH1_TIM2_ETR,
    #[doc = "undocumented"]
    SPI1_NSS,
    #[doc = "undocumented"]
    SPI3_NSS_I2S3_WS,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH15R::JTDI => 0,
            AFRH15R::TIM2_CH1_TIM2_ETR => 1,
            AFRH15R::SPI1_NSS => 5,
            AFRH15R::SPI3_NSS_I2S3_WS => 6,
            AFRH15R::EVENTOUT => 15,
            AFRH15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH15R {
        match value {
            0 => AFRH15R::JTDI,
            1 => AFRH15R::TIM2_CH1_TIM2_ETR,
            5 => AFRH15R::SPI1_NSS,
            6 => AFRH15R::SPI3_NSS_I2S3_WS,
            15 => AFRH15R::EVENTOUT,
            i => AFRH15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `JTDI`"]
    #[inline]
    pub fn is_jtdi(&self) -> bool {
        *self == AFRH15R::JTDI
    }
    #[doc = "Checks if the value of the field is `TIM2_CH1_TIM2_ETR`"]
    #[inline]
    pub fn is_tim2_ch1_tim2_etr(&self) -> bool {
        *self == AFRH15R::TIM2_CH1_TIM2_ETR
    }
    #[doc = "Checks if the value of the field is `SPI1_NSS`"]
    #[inline]
    pub fn is_spi1_nss(&self) -> bool {
        *self == AFRH15R::SPI1_NSS
    }
    #[doc = "Checks if the value of the field is `SPI3_NSS_I2S3_WS`"]
    #[inline]
    pub fn is_spi3_nss_i2s3_ws(&self) -> bool {
        *self == AFRH15R::SPI3_NSS_I2S3_WS
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH15R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRH14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH14R {
    #[doc = "undocumented"]
    JTCK_SWDCLK,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH14R::JTCK_SWDCLK => 0,
            AFRH14R::EVENTOUT => 15,
            AFRH14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH14R {
        match value {
            0 => AFRH14R::JTCK_SWDCLK,
            15 => AFRH14R::EVENTOUT,
            i => AFRH14R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `JTCK_SWDCLK`"]
    #[inline]
    pub fn is_jtck_swdclk(&self) -> bool {
        *self == AFRH14R::JTCK_SWDCLK
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH14R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRH13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH13R {
    #[doc = "undocumented"]
    JTMS_SWDIO,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH13R::JTMS_SWDIO => 0,
            AFRH13R::EVENTOUT => 15,
            AFRH13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH13R {
        match value {
            0 => AFRH13R::JTMS_SWDIO,
            15 => AFRH13R::EVENTOUT,
            i => AFRH13R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `JTMS_SWDIO`"]
    #[inline]
    pub fn is_jtms_swdio(&self) -> bool {
        *self == AFRH13R::JTMS_SWDIO
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH13R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRH12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH12R {
    #[doc = "undocumented"]
    TIM1_ETR,
    #[doc = "undocumented"]
    USART1_RTS,
    #[doc = "undocumented"]
    CAN1_TX,
    #[doc = "undocumented"]
    OTG_FS_DP,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH12R::TIM1_ETR => 1,
            AFRH12R::USART1_RTS => 7,
            AFRH12R::CAN1_TX => 9,
            AFRH12R::OTG_FS_DP => 10,
            AFRH12R::EVENTOUT => 15,
            AFRH12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH12R {
        match value {
            1 => AFRH12R::TIM1_ETR,
            7 => AFRH12R::USART1_RTS,
            9 => AFRH12R::CAN1_TX,
            10 => AFRH12R::OTG_FS_DP,
            15 => AFRH12R::EVENTOUT,
            i => AFRH12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1_ETR`"]
    #[inline]
    pub fn is_tim1_etr(&self) -> bool {
        *self == AFRH12R::TIM1_ETR
    }
    #[doc = "Checks if the value of the field is `USART1_RTS`"]
    #[inline]
    pub fn is_usart1_rts(&self) -> bool {
        *self == AFRH12R::USART1_RTS
    }
    #[doc = "Checks if the value of the field is `CAN1_TX`"]
    #[inline]
    pub fn is_can1_tx(&self) -> bool {
        *self == AFRH12R::CAN1_TX
    }
    #[doc = "Checks if the value of the field is `OTG_FS_DP`"]
    #[inline]
    pub fn is_otg_fs_dp(&self) -> bool {
        *self == AFRH12R::OTG_FS_DP
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH12R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRH11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH11R {
    #[doc = "undocumented"]
    TIM1_CH4,
    #[doc = "undocumented"]
    USART1_CTS,
    #[doc = "undocumented"]
    CAN1_RX,
    #[doc = "undocumented"]
    OTG_FS_DM,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH11R::TIM1_CH4 => 1,
            AFRH11R::USART1_CTS => 7,
            AFRH11R::CAN1_RX => 9,
            AFRH11R::OTG_FS_DM => 10,
            AFRH11R::EVENTOUT => 15,
            AFRH11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH11R {
        match value {
            1 => AFRH11R::TIM1_CH4,
            7 => AFRH11R::USART1_CTS,
            9 => AFRH11R::CAN1_RX,
            10 => AFRH11R::OTG_FS_DM,
            15 => AFRH11R::EVENTOUT,
            i => AFRH11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1_CH4`"]
    #[inline]
    pub fn is_tim1_ch4(&self) -> bool {
        *self == AFRH11R::TIM1_CH4
    }
    #[doc = "Checks if the value of the field is `USART1_CTS`"]
    #[inline]
    pub fn is_usart1_cts(&self) -> bool {
        *self == AFRH11R::USART1_CTS
    }
    #[doc = "Checks if the value of the field is `CAN1_RX`"]
    #[inline]
    pub fn is_can1_rx(&self) -> bool {
        *self == AFRH11R::CAN1_RX
    }
    #[doc = "Checks if the value of the field is `OTG_FS_DM`"]
    #[inline]
    pub fn is_otg_fs_dm(&self) -> bool {
        *self == AFRH11R::OTG_FS_DM
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH11R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRH10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH10R {
    #[doc = "undocumented"]
    TIM1_CH3,
    #[doc = "undocumented"]
    USART1_RX,
    #[doc = "undocumented"]
    OTG_FS_ID,
    #[doc = "undocumented"]
    DCMI_D1,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH10R::TIM1_CH3 => 1,
            AFRH10R::USART1_RX => 7,
            AFRH10R::OTG_FS_ID => 10,
            AFRH10R::DCMI_D1 => 13,
            AFRH10R::EVENTOUT => 15,
            AFRH10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH10R {
        match value {
            1 => AFRH10R::TIM1_CH3,
            7 => AFRH10R::USART1_RX,
            10 => AFRH10R::OTG_FS_ID,
            13 => AFRH10R::DCMI_D1,
            15 => AFRH10R::EVENTOUT,
            i => AFRH10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1_CH3`"]
    #[inline]
    pub fn is_tim1_ch3(&self) -> bool {
        *self == AFRH10R::TIM1_CH3
    }
    #[doc = "Checks if the value of the field is `USART1_RX`"]
    #[inline]
    pub fn is_usart1_rx(&self) -> bool {
        *self == AFRH10R::USART1_RX
    }
    #[doc = "Checks if the value of the field is `OTG_FS_ID`"]
    #[inline]
    pub fn is_otg_fs_id(&self) -> bool {
        *self == AFRH10R::OTG_FS_ID
    }
    #[doc = "Checks if the value of the field is `DCMI_D1`"]
    #[inline]
    pub fn is_dcmi_d1(&self) -> bool {
        *self == AFRH10R::DCMI_D1
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH10R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRH9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH9R {
    #[doc = "undocumented"]
    TIM1_CH2,
    #[doc = "undocumented"]
    I2C3_SMBA,
    #[doc = "undocumented"]
    USART1_TX,
    #[doc = "undocumented"]
    DCMI_D0,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH9R::TIM1_CH2 => 1,
            AFRH9R::I2C3_SMBA => 4,
            AFRH9R::USART1_TX => 7,
            AFRH9R::DCMI_D0 => 13,
            AFRH9R::EVENTOUT => 15,
            AFRH9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH9R {
        match value {
            1 => AFRH9R::TIM1_CH2,
            4 => AFRH9R::I2C3_SMBA,
            7 => AFRH9R::USART1_TX,
            13 => AFRH9R::DCMI_D0,
            15 => AFRH9R::EVENTOUT,
            i => AFRH9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIM1_CH2`"]
    #[inline]
    pub fn is_tim1_ch2(&self) -> bool {
        *self == AFRH9R::TIM1_CH2
    }
    #[doc = "Checks if the value of the field is `I2C3_SMBA`"]
    #[inline]
    pub fn is_i2c3_smba(&self) -> bool {
        *self == AFRH9R::I2C3_SMBA
    }
    #[doc = "Checks if the value of the field is `USART1_TX`"]
    #[inline]
    pub fn is_usart1_tx(&self) -> bool {
        *self == AFRH9R::USART1_TX
    }
    #[doc = "Checks if the value of the field is `DCMI_D0`"]
    #[inline]
    pub fn is_dcmi_d0(&self) -> bool {
        *self == AFRH9R::DCMI_D0
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH9R::EVENTOUT
    }
}
#[doc = "Possible values of the field `AFRH8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFRH8R {
    #[doc = "undocumented"]
    MCO1,
    #[doc = "undocumented"]
    TIM1_CH1,
    #[doc = "undocumented"]
    I2C3_SCL,
    #[doc = "undocumented"]
    USART1_CK,
    #[doc = "undocumented"]
    OTG_FS_SOF,
    #[doc = "undocumented"]
    EVENTOUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFRH8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFRH8R::MCO1 => 0,
            AFRH8R::TIM1_CH1 => 1,
            AFRH8R::I2C3_SCL => 4,
            AFRH8R::USART1_CK => 7,
            AFRH8R::OTG_FS_SOF => 10,
            AFRH8R::EVENTOUT => 15,
            AFRH8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFRH8R {
        match value {
            0 => AFRH8R::MCO1,
            1 => AFRH8R::TIM1_CH1,
            4 => AFRH8R::I2C3_SCL,
            7 => AFRH8R::USART1_CK,
            10 => AFRH8R::OTG_FS_SOF,
            15 => AFRH8R::EVENTOUT,
            i => AFRH8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCO1`"]
    #[inline]
    pub fn is_mco1(&self) -> bool {
        *self == AFRH8R::MCO1
    }
    #[doc = "Checks if the value of the field is `TIM1_CH1`"]
    #[inline]
    pub fn is_tim1_ch1(&self) -> bool {
        *self == AFRH8R::TIM1_CH1
    }
    #[doc = "Checks if the value of the field is `I2C3_SCL`"]
    #[inline]
    pub fn is_i2c3_scl(&self) -> bool {
        *self == AFRH8R::I2C3_SCL
    }
    #[doc = "Checks if the value of the field is `USART1_CK`"]
    #[inline]
    pub fn is_usart1_ck(&self) -> bool {
        *self == AFRH8R::USART1_CK
    }
    #[doc = "Checks if the value of the field is `OTG_FS_SOF`"]
    #[inline]
    pub fn is_otg_fs_sof(&self) -> bool {
        *self == AFRH8R::OTG_FS_SOF
    }
    #[doc = "Checks if the value of the field is `EVENTOUT`"]
    #[inline]
    pub fn is_eventout(&self) -> bool {
        *self == AFRH8R::EVENTOUT
    }
}
#[doc = "Values that can be written to the field `AFRH15`"]
pub enum AFRH15W {
    #[doc = "`0`"]
    JTDI,
    #[doc = "`1`"]
    TIM2_CH1_TIM2_ETR,
    #[doc = "`101`"]
    SPI1_NSS,
    #[doc = "`110`"]
    SPI3_NSS_I2S3_WS,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH15W::JTDI => 0,
            AFRH15W::TIM2_CH1_TIM2_ETR => 1,
            AFRH15W::SPI1_NSS => 5,
            AFRH15W::SPI3_NSS_I2S3_WS => 6,
            AFRH15W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH15W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn jtdi(self) -> &'a mut W {
        self.variant(AFRH15W::JTDI)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim2_ch1_tim2_etr(self) -> &'a mut W {
        self.variant(AFRH15W::TIM2_CH1_TIM2_ETR)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn spi1_nss(self) -> &'a mut W {
        self.variant(AFRH15W::SPI1_NSS)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn spi3_nss_i2s3_ws(self) -> &'a mut W {
        self.variant(AFRH15W::SPI3_NSS_I2S3_WS)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH15W::EVENTOUT)
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
#[doc = "Values that can be written to the field `AFRH14`"]
pub enum AFRH14W {
    #[doc = "`0`"]
    JTCK_SWDCLK,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH14W::JTCK_SWDCLK => 0,
            AFRH14W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH14W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn jtck_swdclk(self) -> &'a mut W {
        self.variant(AFRH14W::JTCK_SWDCLK)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH14W::EVENTOUT)
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
#[doc = "Values that can be written to the field `AFRH13`"]
pub enum AFRH13W {
    #[doc = "`0`"]
    JTMS_SWDIO,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH13W::JTMS_SWDIO => 0,
            AFRH13W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH13W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn jtms_swdio(self) -> &'a mut W {
        self.variant(AFRH13W::JTMS_SWDIO)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH13W::EVENTOUT)
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
#[doc = "Values that can be written to the field `AFRH12`"]
pub enum AFRH12W {
    #[doc = "`1`"]
    TIM1_ETR,
    #[doc = "`111`"]
    USART1_RTS,
    #[doc = "`1001`"]
    CAN1_TX,
    #[doc = "`1010`"]
    OTG_FS_DP,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH12W::TIM1_ETR => 1,
            AFRH12W::USART1_RTS => 7,
            AFRH12W::CAN1_TX => 9,
            AFRH12W::OTG_FS_DP => 10,
            AFRH12W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH12W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim1_etr(self) -> &'a mut W {
        self.variant(AFRH12W::TIM1_ETR)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart1_rts(self) -> &'a mut W {
        self.variant(AFRH12W::USART1_RTS)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn can1_tx(self) -> &'a mut W {
        self.variant(AFRH12W::CAN1_TX)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn otg_fs_dp(self) -> &'a mut W {
        self.variant(AFRH12W::OTG_FS_DP)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH12W::EVENTOUT)
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
#[doc = "Values that can be written to the field `AFRH11`"]
pub enum AFRH11W {
    #[doc = "`1`"]
    TIM1_CH4,
    #[doc = "`111`"]
    USART1_CTS,
    #[doc = "`1001`"]
    CAN1_RX,
    #[doc = "`1010`"]
    OTG_FS_DM,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH11W::TIM1_CH4 => 1,
            AFRH11W::USART1_CTS => 7,
            AFRH11W::CAN1_RX => 9,
            AFRH11W::OTG_FS_DM => 10,
            AFRH11W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH11W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim1_ch4(self) -> &'a mut W {
        self.variant(AFRH11W::TIM1_CH4)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart1_cts(self) -> &'a mut W {
        self.variant(AFRH11W::USART1_CTS)
    }
    #[doc = "`1001`"]
    #[inline]
    pub fn can1_rx(self) -> &'a mut W {
        self.variant(AFRH11W::CAN1_RX)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn otg_fs_dm(self) -> &'a mut W {
        self.variant(AFRH11W::OTG_FS_DM)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH11W::EVENTOUT)
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
#[doc = "Values that can be written to the field `AFRH10`"]
pub enum AFRH10W {
    #[doc = "`1`"]
    TIM1_CH3,
    #[doc = "`111`"]
    USART1_RX,
    #[doc = "`1010`"]
    OTG_FS_ID,
    #[doc = "`1101`"]
    DCMI_D1,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH10W::TIM1_CH3 => 1,
            AFRH10W::USART1_RX => 7,
            AFRH10W::OTG_FS_ID => 10,
            AFRH10W::DCMI_D1 => 13,
            AFRH10W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH10W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim1_ch3(self) -> &'a mut W {
        self.variant(AFRH10W::TIM1_CH3)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart1_rx(self) -> &'a mut W {
        self.variant(AFRH10W::USART1_RX)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn otg_fs_id(self) -> &'a mut W {
        self.variant(AFRH10W::OTG_FS_ID)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn dcmi_d1(self) -> &'a mut W {
        self.variant(AFRH10W::DCMI_D1)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH10W::EVENTOUT)
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
#[doc = "Values that can be written to the field `AFRH9`"]
pub enum AFRH9W {
    #[doc = "`1`"]
    TIM1_CH2,
    #[doc = "`100`"]
    I2C3_SMBA,
    #[doc = "`111`"]
    USART1_TX,
    #[doc = "`1101`"]
    DCMI_D0,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH9W::TIM1_CH2 => 1,
            AFRH9W::I2C3_SMBA => 4,
            AFRH9W::USART1_TX => 7,
            AFRH9W::DCMI_D0 => 13,
            AFRH9W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH9W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim1_ch2(self) -> &'a mut W {
        self.variant(AFRH9W::TIM1_CH2)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn i2c3_smba(self) -> &'a mut W {
        self.variant(AFRH9W::I2C3_SMBA)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart1_tx(self) -> &'a mut W {
        self.variant(AFRH9W::USART1_TX)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn dcmi_d0(self) -> &'a mut W {
        self.variant(AFRH9W::DCMI_D0)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH9W::EVENTOUT)
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
#[doc = "Values that can be written to the field `AFRH8`"]
pub enum AFRH8W {
    #[doc = "`0`"]
    MCO1,
    #[doc = "`1`"]
    TIM1_CH1,
    #[doc = "`100`"]
    I2C3_SCL,
    #[doc = "`111`"]
    USART1_CK,
    #[doc = "`1010`"]
    OTG_FS_SOF,
    #[doc = "`1111`"]
    EVENTOUT,
}
impl AFRH8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AFRH8W::MCO1 => 0,
            AFRH8W::TIM1_CH1 => 1,
            AFRH8W::I2C3_SCL => 4,
            AFRH8W::USART1_CK => 7,
            AFRH8W::OTG_FS_SOF => 10,
            AFRH8W::EVENTOUT => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AFRH8W<'a> {
    w: &'a mut W,
}
impl<'a> _AFRH8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFRH8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`0`"]
    #[inline]
    pub fn mco1(self) -> &'a mut W {
        self.variant(AFRH8W::MCO1)
    }
    #[doc = "`1`"]
    #[inline]
    pub fn tim1_ch1(self) -> &'a mut W {
        self.variant(AFRH8W::TIM1_CH1)
    }
    #[doc = "`100`"]
    #[inline]
    pub fn i2c3_scl(self) -> &'a mut W {
        self.variant(AFRH8W::I2C3_SCL)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn usart1_ck(self) -> &'a mut W {
        self.variant(AFRH8W::USART1_CK)
    }
    #[doc = "`1010`"]
    #[inline]
    pub fn otg_fs_sof(self) -> &'a mut W {
        self.variant(AFRH8W::OTG_FS_SOF)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn eventout(self) -> &'a mut W {
        self.variant(AFRH8W::EVENTOUT)
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
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh15(&self) -> AFRH15R {
        AFRH15R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh14(&self) -> AFRH14R {
        AFRH14R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh13(&self) -> AFRH13R {
        AFRH13R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh12(&self) -> AFRH12R {
        AFRH12R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh11(&self) -> AFRH11R {
        AFRH11R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh10(&self) -> AFRH10R {
        AFRH10R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh9(&self) -> AFRH9R {
        AFRH9R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh8(&self) -> AFRH8R {
        AFRH8R::_from({
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
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh15(&mut self) -> _AFRH15W {
        _AFRH15W { w: self }
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh14(&mut self) -> _AFRH14W {
        _AFRH14W { w: self }
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh13(&mut self) -> _AFRH13W {
        _AFRH13W { w: self }
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh12(&mut self) -> _AFRH12W {
        _AFRH12W { w: self }
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh11(&mut self) -> _AFRH11W {
        _AFRH11W { w: self }
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh10(&mut self) -> _AFRH10W {
        _AFRH10W { w: self }
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh9(&mut self) -> _AFRH9W {
        _AFRH9W { w: self }
    }
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y(y = 8..15)"]
    #[inline]
    pub fn afrh8(&mut self) -> _AFRH8W {
        _AFRH8W { w: self }
    }
}
