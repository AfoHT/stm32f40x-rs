#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = r" Value of the field"]
pub struct PLLI2SRDYR {
    bits: bool,
}
impl PLLI2SRDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PLLI2SON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLI2SONR {
    #[doc = "Dsabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PLLI2SONR {
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
            PLLI2SONR::DISABLED => false,
            PLLI2SONR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLI2SONR {
        match value {
            false => PLLI2SONR::DISABLED,
            true => PLLI2SONR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PLLI2SONR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PLLI2SONR::ENABLED
    }
}
#[doc = "Possible values of the field `PLLRDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLRDYR {
    #[doc = "PLL Unlocked"]
    UNLOCKED,
    #[doc = "PLL Locked"]
    LOCKED,
}
impl PLLRDYR {
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
            PLLRDYR::UNLOCKED => false,
            PLLRDYR::LOCKED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLRDYR {
        match value {
            false => PLLRDYR::UNLOCKED,
            true => PLLRDYR::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDYR::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDYR::LOCKED
    }
}
#[doc = "Possible values of the field `PLLON`"]
pub type PLLONR = PLLI2SONR;
#[doc = "Possible values of the field `CSSON`"]
pub type CSSONR = PLLI2SONR;
#[doc = "Possible values of the field `HSEBYP`"]
pub type HSEBYPR = PLLI2SONR;
#[doc = "Possible values of the field `HSERDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDYR {
    #[doc = "HSE Not Ready"]
    NOTREADY,
    #[doc = "HSE Ready"]
    READY,
}
impl HSERDYR {
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
            HSERDYR::NOTREADY => false,
            HSERDYR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSERDYR {
        match value {
            false => HSERDYR::NOTREADY,
            true => HSERDYR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTREADY`"]
    #[inline]
    pub fn is_notready(&self) -> bool {
        *self == HSERDYR::NOTREADY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == HSERDYR::READY
    }
}
#[doc = "Possible values of the field `HSEON`"]
pub type HSEONR = PLLI2SONR;
#[doc = r" Value of the field"]
pub struct HSICALR {
    bits: u8,
}
impl HSICALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSITRIMR {
    bits: u8,
}
impl HSITRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSIRDYR {
    bits: bool,
}
impl HSIRDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `HSION`"]
pub type HSIONR = PLLI2SONR;
#[doc = "Values that can be written to the field `PLLI2SON`"]
pub enum PLLI2SONW {
    #[doc = "Dsabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PLLI2SONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLI2SONW::DISABLED => false,
            PLLI2SONW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLI2SONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLI2SONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLI2SONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::ENABLED)
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
#[doc = "Values that can be written to the field `PLLON`"]
pub type PLLONW = PLLI2SONW;
#[doc = r" Proxy"]
pub struct _PLLONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::ENABLED)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CSSON`"]
pub type CSSONW = PLLI2SONW;
#[doc = r" Proxy"]
pub struct _CSSONW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::ENABLED)
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
#[doc = "Values that can be written to the field `HSEBYP`"]
pub type HSEBYPW = PLLI2SONW;
#[doc = r" Proxy"]
pub struct _HSEBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSEBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::ENABLED)
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
#[doc = "Values that can be written to the field `HSERDY`"]
pub enum HSERDYW {
    #[doc = "HSE Not Ready"]
    NOTREADY,
    #[doc = "HSE Ready"]
    READY,
}
impl HSERDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSERDYW::NOTREADY => false,
            HSERDYW::READY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSERDYW<'a> {
    w: &'a mut W,
}
impl<'a> _HSERDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSERDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSE Not Ready"]
    #[inline]
    pub fn notready(self) -> &'a mut W {
        self.variant(HSERDYW::NOTREADY)
    }
    #[doc = "HSE Ready"]
    #[inline]
    pub fn ready(self) -> &'a mut W {
        self.variant(HSERDYW::READY)
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
#[doc = "Values that can be written to the field `HSEON`"]
pub type HSEONW = PLLI2SONW;
#[doc = r" Proxy"]
pub struct _HSEONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSEONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _HSITRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _HSITRIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HSION`"]
pub type HSIONW = PLLI2SONW;
#[doc = r" Proxy"]
pub struct _HSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLI2SONW::ENABLED)
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
    #[doc = "Bit 27 - PLLI2S clock ready flag"]
    #[inline]
    pub fn plli2srdy(&self) -> PLLI2SRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLI2SRDYR { bits }
    }
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline]
    pub fn plli2son(&self) -> PLLI2SONR {
        PLLI2SONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Main PLL(PLL) clock ready flag"]
    #[inline]
    pub fn pllrdy(&self) -> PLLRDYR {
        PLLRDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Main PLL(PLL) enable"]
    #[inline]
    pub fn pllon(&self) -> PLLONR {
        PLLONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline]
    pub fn csson(&self) -> CSSONR {
        CSSONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline]
    pub fn hsebyp(&self) -> HSEBYPR {
        HSEBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline]
    pub fn hserdy(&self) -> HSERDYR {
        HSERDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline]
    pub fn hseon(&self) -> HSEONR {
        HSEONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Internal high-speed clock calibration"]
    #[inline]
    pub fn hsical(&self) -> HSICALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSICALR { bits }
    }
    #[doc = "Bits 3:7 - Internal high-speed clock trimming"]
    #[inline]
    pub fn hsitrim(&self) -> HSITRIMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSITRIMR { bits }
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline]
    pub fn hsirdy(&self) -> HSIRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSIRDYR { bits }
    }
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline]
    pub fn hsion(&self) -> HSIONR {
        HSIONR::_from({
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
        W { bits: 131 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 26 - PLLI2S enable"]
    #[inline]
    pub fn plli2son(&mut self) -> _PLLI2SONW {
        _PLLI2SONW { w: self }
    }
    #[doc = "Bit 24 - Main PLL(PLL) enable"]
    #[inline]
    pub fn pllon(&mut self) -> _PLLONW {
        _PLLONW { w: self }
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline]
    pub fn csson(&mut self) -> _CSSONW {
        _CSSONW { w: self }
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline]
    pub fn hsebyp(&mut self) -> _HSEBYPW {
        _HSEBYPW { w: self }
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline]
    pub fn hserdy(&mut self) -> _HSERDYW {
        _HSERDYW { w: self }
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline]
    pub fn hseon(&mut self) -> _HSEONW {
        _HSEONW { w: self }
    }
    #[doc = "Bits 3:7 - Internal high-speed clock trimming"]
    #[inline]
    pub fn hsitrim(&mut self) -> _HSITRIMW {
        _HSITRIMW { w: self }
    }
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline]
    pub fn hsion(&mut self) -> _HSIONW {
        _HSIONW { w: self }
    }
}
