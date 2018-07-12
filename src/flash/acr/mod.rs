#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACR {
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
pub struct LATENCYR {
    bits: u8,
}
impl LATENCYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PRFTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRFTENR {
    #[doc = "Dsabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PRFTENR {
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
            PRFTENR::DISABLED => false,
            PRFTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRFTENR {
        match value {
            false => PRFTENR::DISABLED,
            true => PRFTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTENR::ENABLED
    }
}
#[doc = "Possible values of the field `ICEN`"]
pub type ICENR = PRFTENR;
#[doc = "Possible values of the field `DCEN`"]
pub type DCENR = PRFTENR;
#[doc = "Possible values of the field `DCRST`"]
pub type DCRSTR = PRFTENR;
#[doc = r" Proxy"]
pub struct _LATENCYW<'a> {
    w: &'a mut W,
}
impl<'a> _LATENCYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRFTEN`"]
pub enum PRFTENW {
    #[doc = "Dsabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl PRFTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRFTENW::DISABLED => false,
            PRFTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRFTENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRFTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRFTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTENW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTENW::ENABLED)
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
#[doc = "Values that can be written to the field `ICEN`"]
pub type ICENW = PRFTENW;
#[doc = r" Proxy"]
pub struct _ICENW<'a> {
    w: &'a mut W,
}
impl<'a> _ICENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTENW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTENW::ENABLED)
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
#[doc = "Values that can be written to the field `DCEN`"]
pub type DCENW = PRFTENW;
#[doc = r" Proxy"]
pub struct _DCENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTENW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTENW::ENABLED)
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
#[doc = "Values that can be written to the field `ICRST`"]
pub type ICRSTW = PRFTENW;
#[doc = r" Proxy"]
pub struct _ICRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ICRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTENW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTENW::ENABLED)
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
#[doc = "Values that can be written to the field `DCRST`"]
pub type DCRSTW = PRFTENW;
#[doc = r" Proxy"]
pub struct _DCRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _DCRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Dsabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTENW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTENW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Latency"]
    #[inline]
    pub fn latency(&self) -> LATENCYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LATENCYR { bits }
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline]
    pub fn prften(&self) -> PRFTENR {
        PRFTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline]
    pub fn icen(&self) -> ICENR {
        ICENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline]
    pub fn dcen(&self) -> DCENR {
        DCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline]
    pub fn dcrst(&self) -> DCRSTR {
        DCRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:2 - Latency"]
    #[inline]
    pub fn latency(&mut self) -> _LATENCYW {
        _LATENCYW { w: self }
    }
    #[doc = "Bit 8 - Prefetch enable"]
    #[inline]
    pub fn prften(&mut self) -> _PRFTENW {
        _PRFTENW { w: self }
    }
    #[doc = "Bit 9 - Instruction cache enable"]
    #[inline]
    pub fn icen(&mut self) -> _ICENW {
        _ICENW { w: self }
    }
    #[doc = "Bit 10 - Data cache enable"]
    #[inline]
    pub fn dcen(&mut self) -> _DCENW {
        _DCENW { w: self }
    }
    #[doc = "Bit 11 - Instruction cache reset"]
    #[inline]
    pub fn icrst(&mut self) -> _ICRSTW {
        _ICRSTW { w: self }
    }
    #[doc = "Bit 12 - Data cache reset"]
    #[inline]
    pub fn dcrst(&mut self) -> _DCRSTW {
        _DCRSTW { w: self }
    }
}
