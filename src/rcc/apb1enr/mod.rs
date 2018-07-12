#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB1ENR {
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
#[doc = "Possible values of the field `DACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACENR {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl DACENR {
    #[doc = r" Returns `true` if the bit is clear(0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set(1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DACENR::DISABLED => false,
            DACENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACENR {
        match value {
            false => DACENR::DISABLED,
            true => DACENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DACENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DACENR::ENABLED
    }
}
#[doc = "Possible values of the field `PWREN`"]
pub type PWRENR = DACENR;
#[doc = "Possible values of the field `CAN2EN`"]
pub type CAN2ENR = DACENR;
#[doc = "Possible values of the field `CAN1EN`"]
pub type CAN1ENR = DACENR;
#[doc = "Possible values of the field `I2C3EN`"]
pub type I2C3ENR = DACENR;
#[doc = "Possible values of the field `I2C2EN`"]
pub type I2C2ENR = DACENR;
#[doc = "Possible values of the field `I2C1EN`"]
pub type I2C1ENR = DACENR;
#[doc = "Possible values of the field `UART5EN`"]
pub type UART5ENR = DACENR;
#[doc = "Possible values of the field `UART4EN`"]
pub type UART4ENR = DACENR;
#[doc = "Possible values of the field `USART3EN`"]
pub type USART3ENR = DACENR;
#[doc = "Possible values of the field `USART2EN`"]
pub type USART2ENR = DACENR;
#[doc = "Possible values of the field `SPI3EN`"]
pub type SPI3ENR = DACENR;
#[doc = "Possible values of the field `SPI2EN`"]
pub type SPI2ENR = DACENR;
#[doc = "Possible values of the field `WWDGEN`"]
pub type WWDGENR = DACENR;
#[doc = "Possible values of the field `TIM14EN`"]
pub type TIM14ENR = DACENR;
#[doc = "Possible values of the field `TIM13EN`"]
pub type TIM13ENR = DACENR;
#[doc = "Possible values of the field `TIM12EN`"]
pub type TIM12ENR = DACENR;
#[doc = "Possible values of the field `TIM7EN`"]
pub type TIM7ENR = DACENR;
#[doc = "Possible values of the field `TIM6EN`"]
pub type TIM6ENR = DACENR;
#[doc = "Possible values of the field `TIM5EN`"]
pub type TIM5ENR = DACENR;
#[doc = "Possible values of the field `TIM4EN`"]
pub type TIM4ENR = DACENR;
#[doc = "Possible values of the field `TIM3EN`"]
pub type TIM3ENR = DACENR;
#[doc = "Possible values of the field `TIM2EN`"]
pub type TIM2ENR = DACENR;
#[doc = "Values that can be written to the field `DACEN`"]
pub enum DACENW {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl DACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACENW::DISABLED => false,
            DACENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWREN`"]
pub type PWRENW = DACENW;
#[doc = r" Proxy"]
pub struct _PWRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAN2EN`"]
pub type CAN2ENW = DACENW;
#[doc = r" Proxy"]
pub struct _CAN2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAN1EN`"]
pub type CAN1ENW = DACENW;
#[doc = r" Proxy"]
pub struct _CAN1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAN1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C3EN`"]
pub type I2C3ENW = DACENW;
#[doc = r" Proxy"]
pub struct _I2C3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C2EN`"]
pub type I2C2ENW = DACENW;
#[doc = r" Proxy"]
pub struct _I2C2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `I2C1EN`"]
pub type I2C1ENW = DACENW;
#[doc = r" Proxy"]
pub struct _I2C1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART5EN`"]
pub type UART5ENW = DACENW;
#[doc = r" Proxy"]
pub struct _UART5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART5ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UART4EN`"]
pub type UART4ENW = DACENW;
#[doc = r" Proxy"]
pub struct _UART4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART3EN`"]
pub type USART3ENW = DACENW;
#[doc = r" Proxy"]
pub struct _USART3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USART2EN`"]
pub type USART2ENW = DACENW;
#[doc = r" Proxy"]
pub struct _USART2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI3EN`"]
pub type SPI3ENW = DACENW;
#[doc = r" Proxy"]
pub struct _SPI3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPI2EN`"]
pub type SPI2ENW = DACENW;
#[doc = r" Proxy"]
pub struct _SPI2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WWDGEN`"]
pub type WWDGENW = DACENW;
#[doc = r" Proxy"]
pub struct _WWDGENW<'a> {
    w: &'a mut W,
}
impl<'a> _WWDGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WWDGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM14EN`"]
pub type TIM14ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM14ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM14ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM14ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM13EN`"]
pub type TIM13ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM13ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM13ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM13ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM12EN`"]
pub type TIM12ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM12ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM12ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM12ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM7EN`"]
pub type TIM7ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM7ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM7ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM7ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM6EN`"]
pub type TIM6ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM5EN`"]
pub type TIM5ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM5ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM5ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM5ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM4EN`"]
pub type TIM4ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM4ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM4ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM4ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM3EN`"]
pub type TIM3ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIM2EN`"]
pub type TIM2ENW = DACENW;
#[doc = r" Proxy"]
pub struct _TIM2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DACENW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline]
    pub fn dacen(&self) -> DACENR {
        DACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline]
    pub fn pwren(&self) -> PWRENR {
        PWRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - CAN 2 clock enable"]
    #[inline]
    pub fn can2en(&self) -> CAN2ENR {
        CAN2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - CAN 1 clock enable"]
    #[inline]
    pub fn can1en(&self) -> CAN1ENR {
        CAN1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline]
    pub fn i2c3en(&self) -> I2C3ENR {
        I2C3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline]
    pub fn i2c2en(&self) -> I2C2ENR {
        I2C2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline]
    pub fn i2c1en(&self) -> I2C1ENR {
        I2C1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline]
    pub fn uart5en(&self) -> UART5ENR {
        UART5ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline]
    pub fn uart4en(&self) -> UART4ENR {
        UART4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline]
    pub fn usart3en(&self) -> USART3ENR {
        USART3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline]
    pub fn usart2en(&self) -> USART2ENR {
        USART2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline]
    pub fn spi3en(&self) -> SPI3ENR {
        SPI3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline]
    pub fn spi2en(&self) -> SPI2ENR {
        SPI2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline]
    pub fn wwdgen(&self) -> WWDGENR {
        WWDGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - TIM14 clock enable"]
    #[inline]
    pub fn tim14en(&self) -> TIM14ENR {
        TIM14ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - TIM13 clock enable"]
    #[inline]
    pub fn tim13en(&self) -> TIM13ENR {
        TIM13ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - TIM12 clock enable"]
    #[inline]
    pub fn tim12en(&self) -> TIM12ENR {
        TIM12ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline]
    pub fn tim7en(&self) -> TIM7ENR {
        TIM7ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline]
    pub fn tim6en(&self) -> TIM6ENR {
        TIM6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline]
    pub fn tim5en(&self) -> TIM5ENR {
        TIM5ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline]
    pub fn tim4en(&self) -> TIM4ENR {
        TIM4ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline]
    pub fn tim3en(&self) -> TIM3ENR {
        TIM3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline]
    pub fn tim2en(&self) -> TIM2ENR {
        TIM2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline]
    pub fn dacen(&mut self) -> _DACENW {
        _DACENW { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline]
    pub fn pwren(&mut self) -> _PWRENW {
        _PWRENW { w: self }
    }
    #[doc = "Bit 26 - CAN 2 clock enable"]
    #[inline]
    pub fn can2en(&mut self) -> _CAN2ENW {
        _CAN2ENW { w: self }
    }
    #[doc = "Bit 25 - CAN 1 clock enable"]
    #[inline]
    pub fn can1en(&mut self) -> _CAN1ENW {
        _CAN1ENW { w: self }
    }
    #[doc = "Bit 23 - I2C3 clock enable"]
    #[inline]
    pub fn i2c3en(&mut self) -> _I2C3ENW {
        _I2C3ENW { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline]
    pub fn i2c2en(&mut self) -> _I2C2ENW {
        _I2C2ENW { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline]
    pub fn i2c1en(&mut self) -> _I2C1ENW {
        _I2C1ENW { w: self }
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline]
    pub fn uart5en(&mut self) -> _UART5ENW {
        _UART5ENW { w: self }
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline]
    pub fn uart4en(&mut self) -> _UART4ENW {
        _UART4ENW { w: self }
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline]
    pub fn usart3en(&mut self) -> _USART3ENW {
        _USART3ENW { w: self }
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline]
    pub fn usart2en(&mut self) -> _USART2ENW {
        _USART2ENW { w: self }
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline]
    pub fn spi3en(&mut self) -> _SPI3ENW {
        _SPI3ENW { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline]
    pub fn spi2en(&mut self) -> _SPI2ENW {
        _SPI2ENW { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline]
    pub fn wwdgen(&mut self) -> _WWDGENW {
        _WWDGENW { w: self }
    }
    #[doc = "Bit 8 - TIM14 clock enable"]
    #[inline]
    pub fn tim14en(&mut self) -> _TIM14ENW {
        _TIM14ENW { w: self }
    }
    #[doc = "Bit 7 - TIM13 clock enable"]
    #[inline]
    pub fn tim13en(&mut self) -> _TIM13ENW {
        _TIM13ENW { w: self }
    }
    #[doc = "Bit 6 - TIM12 clock enable"]
    #[inline]
    pub fn tim12en(&mut self) -> _TIM12ENW {
        _TIM12ENW { w: self }
    }
    #[doc = "Bit 5 - TIM7 clock enable"]
    #[inline]
    pub fn tim7en(&mut self) -> _TIM7ENW {
        _TIM7ENW { w: self }
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline]
    pub fn tim6en(&mut self) -> _TIM6ENW {
        _TIM6ENW { w: self }
    }
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline]
    pub fn tim5en(&mut self) -> _TIM5ENW {
        _TIM5ENW { w: self }
    }
    #[doc = "Bit 2 - TIM4 clock enable"]
    #[inline]
    pub fn tim4en(&mut self) -> _TIM4ENW {
        _TIM4ENW { w: self }
    }
    #[doc = "Bit 1 - TIM3 clock enable"]
    #[inline]
    pub fn tim3en(&mut self) -> _TIM3ENW {
        _TIM3ENW { w: self }
    }
    #[doc = "Bit 0 - TIM2 clock enable"]
    #[inline]
    pub fn tim2en(&mut self) -> _TIM2ENW {
        _TIM2ENW { w: self }
    }
}
