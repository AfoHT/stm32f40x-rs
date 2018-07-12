#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::APB2ENR {
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
#[doc = "Possible values of the field `TIM11EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM11ENR {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl TIM11ENR {
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
            TIM11ENR::DISABLED => false,
            TIM11ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIM11ENR {
        match value {
            false => TIM11ENR::DISABLED,
            true => TIM11ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TIM11ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TIM11ENR::ENABLED
    }
}
#[doc = "Possible values of the field `TIM10EN`"]
pub type TIM10ENR = TIM11ENR;
#[doc = "Possible values of the field `TIM9EN`"]
pub type TIM9ENR = TIM11ENR;
#[doc = "Possible values of the field `SYSCFGEN`"]
pub type SYSCFGENR = TIM11ENR;
#[doc = "Possible values of the field `SPI1EN`"]
pub type SPI1ENR = TIM11ENR;
#[doc = "Possible values of the field `SDIOEN`"]
pub type SDIOENR = TIM11ENR;
#[doc = "Possible values of the field `ADC3EN`"]
pub type ADC3ENR = TIM11ENR;
#[doc = "Possible values of the field `ADC2EN`"]
pub type ADC2ENR = TIM11ENR;
#[doc = "Possible values of the field `ADC1EN`"]
pub type ADC1ENR = TIM11ENR;
#[doc = "Possible values of the field `USART6EN`"]
pub type USART6ENR = TIM11ENR;
#[doc = "Possible values of the field `USART1EN`"]
pub type USART1ENR = TIM11ENR;
#[doc = "Possible values of the field `TIM8EN`"]
pub type TIM8ENR = TIM11ENR;
#[doc = "Possible values of the field `TIM1EN`"]
pub type TIM1ENR = TIM11ENR;
#[doc = "Values that can be written to the field `TIM11EN`"]
pub enum TIM11ENW {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl TIM11ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIM11ENW::DISABLED => false,
            TIM11ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIM11ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM11ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM11ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM10EN`"]
pub type TIM10ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _TIM10ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM10ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM10ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM9EN`"]
pub type TIM9ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _TIM9ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM9ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM9ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SYSCFGEN`"]
pub type SYSCFGENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _SYSCFGENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCFGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSCFGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `SPI1EN`"]
pub type SPI1ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _SPI1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SDIOEN`"]
pub type SDIOENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _SDIOENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDIOENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDIOENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `ADC3EN`"]
pub type ADC3ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _ADC3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC3ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC3ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC2EN`"]
pub type ADC2ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _ADC2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC1EN`"]
pub type ADC1ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _ADC1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART6EN`"]
pub type USART6ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _USART6ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART6ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART6ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `USART1EN`"]
pub type USART1ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _USART1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM8EN`"]
pub type TIM8ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _TIM8ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM8ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM8ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
#[doc = "Values that can be written to the field `TIM1EN`"]
pub type TIM1ENW = TIM11ENW;
#[doc = r" Proxy"]
pub struct _TIM1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIM1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIM1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11ENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11ENW::ENABLED)
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
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline]
    pub fn tim11en(&self) -> TIM11ENR {
        TIM11ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline]
    pub fn tim10en(&self) -> TIM10ENR {
        TIM10ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline]
    pub fn tim9en(&self) -> TIM9ENR {
        TIM9ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline]
    pub fn syscfgen(&self) -> SYSCFGENR {
        SYSCFGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline]
    pub fn spi1en(&self) -> SPI1ENR {
        SPI1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline]
    pub fn sdioen(&self) -> SDIOENR {
        SDIOENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline]
    pub fn adc3en(&self) -> ADC3ENR {
        ADC3ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline]
    pub fn adc2en(&self) -> ADC2ENR {
        ADC2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline]
    pub fn adc1en(&self) -> ADC1ENR {
        ADC1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline]
    pub fn usart6en(&self) -> USART6ENR {
        USART6ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline]
    pub fn usart1en(&self) -> USART1ENR {
        USART1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline]
    pub fn tim8en(&self) -> TIM8ENR {
        TIM8ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline]
    pub fn tim1en(&self) -> TIM1ENR {
        TIM1ENR::_from({
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
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline]
    pub fn tim11en(&mut self) -> _TIM11ENW {
        _TIM11ENW { w: self }
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline]
    pub fn tim10en(&mut self) -> _TIM10ENW {
        _TIM10ENW { w: self }
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline]
    pub fn tim9en(&mut self) -> _TIM9ENW {
        _TIM9ENW { w: self }
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline]
    pub fn syscfgen(&mut self) -> _SYSCFGENW {
        _SYSCFGENW { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline]
    pub fn spi1en(&mut self) -> _SPI1ENW {
        _SPI1ENW { w: self }
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline]
    pub fn sdioen(&mut self) -> _SDIOENW {
        _SDIOENW { w: self }
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline]
    pub fn adc3en(&mut self) -> _ADC3ENW {
        _ADC3ENW { w: self }
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline]
    pub fn adc2en(&mut self) -> _ADC2ENW {
        _ADC2ENW { w: self }
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline]
    pub fn adc1en(&mut self) -> _ADC1ENW {
        _ADC1ENW { w: self }
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline]
    pub fn usart6en(&mut self) -> _USART6ENW {
        _USART6ENW { w: self }
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline]
    pub fn usart1en(&mut self) -> _USART1ENW {
        _USART1ENW { w: self }
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline]
    pub fn tim8en(&mut self) -> _TIM8ENW {
        _TIM8ENW { w: self }
    }
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline]
    pub fn tim1en(&mut self) -> _TIM1ENW {
        _TIM1ENW { w: self }
    }
}
