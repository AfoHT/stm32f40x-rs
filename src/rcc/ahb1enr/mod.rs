#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB1ENR {
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
#[doc = "Possible values of the field `OTGHSULPIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGHSULPIENR {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl OTGHSULPIENR {
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
            OTGHSULPIENR::DISABLED => false,
            OTGHSULPIENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGHSULPIENR {
        match value {
            false => OTGHSULPIENR::DISABLED,
            true => OTGHSULPIENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OTGHSULPIENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OTGHSULPIENR::ENABLED
    }
}
#[doc = "Possible values of the field `OTGHSEN`"]
pub type OTGHSENR = OTGHSULPIENR;
#[doc = "Possible values of the field `ETHMACPTPEN`"]
pub type ETHMACPTPENR = OTGHSULPIENR;
#[doc = "Possible values of the field `ETHMACRXEN`"]
pub type ETHMACRXENR = OTGHSULPIENR;
#[doc = "Possible values of the field `ETHMACTXEN`"]
pub type ETHMACTXENR = OTGHSULPIENR;
#[doc = "Possible values of the field `ETHMACEN`"]
pub type ETHMACENR = OTGHSULPIENR;
#[doc = "Possible values of the field `DMA2EN`"]
pub type DMA2ENR = OTGHSULPIENR;
#[doc = "Possible values of the field `DMA1EN`"]
pub type DMA1ENR = OTGHSULPIENR;
#[doc = "Possible values of the field `BKPSRAMEN`"]
pub type BKPSRAMENR = OTGHSULPIENR;
#[doc = "Possible values of the field `CRCEN`"]
pub type CRCENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOIEN`"]
pub type GPIOIENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOHEN`"]
pub type GPIOHENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOGEN`"]
pub type GPIOGENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOFEN`"]
pub type GPIOFENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOEEN`"]
pub type GPIOEENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIODEN`"]
pub type GPIODENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOCEN`"]
pub type GPIOCENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOBEN`"]
pub type GPIOBENR = OTGHSULPIENR;
#[doc = "Possible values of the field `GPIOAEN`"]
pub type GPIOAENR = OTGHSULPIENR;
#[doc = "Values that can be written to the field `OTGHSULPIEN`"]
pub enum OTGHSULPIENW {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl OTGHSULPIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGHSULPIENW::DISABLED => false,
            OTGHSULPIENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGHSULPIENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGHSULPIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGHSULPIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OTGHSEN`"]
pub type OTGHSENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _OTGHSENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGHSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGHSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `ETHMACPTPEN`"]
pub type ETHMACPTPENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _ETHMACPTPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACPTPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACPTPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `ETHMACRXEN`"]
pub type ETHMACRXENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _ETHMACRXENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACRXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACRXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ETHMACTXEN`"]
pub type ETHMACTXENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _ETHMACTXENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACTXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACTXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `ETHMACEN`"]
pub type ETHMACENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _ETHMACENW<'a> {
    w: &'a mut W,
}
impl<'a> _ETHMACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETHMACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `DMA2EN`"]
pub type DMA2ENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _DMA2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `DMA1EN`"]
pub type DMA1ENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _DMA1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMA1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `BKPSRAMEN`"]
pub type BKPSRAMENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _BKPSRAMENW<'a> {
    w: &'a mut W,
}
impl<'a> _BKPSRAMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BKPSRAMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `CRCEN`"]
pub type CRCENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _CRCENW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOIEN`"]
pub type GPIOIENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOIENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOHEN`"]
pub type GPIOHENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOHENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOGEN`"]
pub type GPIOGENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOGENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOFEN`"]
pub type GPIOFENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOFENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOEEN`"]
pub type GPIOEENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOEENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIODEN`"]
pub type GPIODENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIODENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIODENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIODENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOCEN`"]
pub type GPIOCENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOCENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOBEN`"]
pub type GPIOBENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOBENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOBENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
#[doc = "Values that can be written to the field `GPIOAEN`"]
pub type GPIOAENW = OTGHSULPIENW;
#[doc = r" Proxy"]
pub struct _GPIOAENW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIOAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIOAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGHSULPIENW::ENABLED)
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
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline]
    pub fn otghsulpien(&self) -> OTGHSULPIENR {
        OTGHSULPIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline]
    pub fn otghsen(&self) -> OTGHSENR {
        OTGHSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline]
    pub fn ethmacptpen(&self) -> ETHMACPTPENR {
        ETHMACPTPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Ethernet Reception clock enable"]
    #[inline]
    pub fn ethmacrxen(&self) -> ETHMACRXENR {
        ETHMACRXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Ethernet Transmission clock enable"]
    #[inline]
    pub fn ethmactxen(&self) -> ETHMACTXENR {
        ETHMACTXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable"]
    #[inline]
    pub fn ethmacen(&self) -> ETHMACENR {
        ETHMACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline]
    pub fn dma2en(&self) -> DMA2ENR {
        DMA2ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline]
    pub fn dma1en(&self) -> DMA1ENR {
        DMA1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline]
    pub fn bkpsramen(&self) -> BKPSRAMENR {
        BKPSRAMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline]
    pub fn crcen(&self) -> CRCENR {
        CRCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline]
    pub fn gpioien(&self) -> GPIOIENR {
        GPIOIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline]
    pub fn gpiohen(&self) -> GPIOHENR {
        GPIOHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline]
    pub fn gpiogen(&self) -> GPIOGENR {
        GPIOGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline]
    pub fn gpiofen(&self) -> GPIOFENR {
        GPIOFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline]
    pub fn gpioeen(&self) -> GPIOEENR {
        GPIOEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline]
    pub fn gpioden(&self) -> GPIODENR {
        GPIODENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline]
    pub fn gpiocen(&self) -> GPIOCENR {
        GPIOCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline]
    pub fn gpioben(&self) -> GPIOBENR {
        GPIOBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline]
    pub fn gpioaen(&self) -> GPIOAENR {
        GPIOAENR::_from({
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
        W { bits: 1048576 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 30 - USB OTG HSULPI clock enable"]
    #[inline]
    pub fn otghsulpien(&mut self) -> _OTGHSULPIENW {
        _OTGHSULPIENW { w: self }
    }
    #[doc = "Bit 29 - USB OTG HS clock enable"]
    #[inline]
    pub fn otghsen(&mut self) -> _OTGHSENW {
        _OTGHSENW { w: self }
    }
    #[doc = "Bit 28 - Ethernet PTP clock enable"]
    #[inline]
    pub fn ethmacptpen(&mut self) -> _ETHMACPTPENW {
        _ETHMACPTPENW { w: self }
    }
    #[doc = "Bit 27 - Ethernet Reception clock enable"]
    #[inline]
    pub fn ethmacrxen(&mut self) -> _ETHMACRXENW {
        _ETHMACRXENW { w: self }
    }
    #[doc = "Bit 26 - Ethernet Transmission clock enable"]
    #[inline]
    pub fn ethmactxen(&mut self) -> _ETHMACTXENW {
        _ETHMACTXENW { w: self }
    }
    #[doc = "Bit 25 - Ethernet MAC clock enable"]
    #[inline]
    pub fn ethmacen(&mut self) -> _ETHMACENW {
        _ETHMACENW { w: self }
    }
    #[doc = "Bit 22 - DMA2 clock enable"]
    #[inline]
    pub fn dma2en(&mut self) -> _DMA2ENW {
        _DMA2ENW { w: self }
    }
    #[doc = "Bit 21 - DMA1 clock enable"]
    #[inline]
    pub fn dma1en(&mut self) -> _DMA1ENW {
        _DMA1ENW { w: self }
    }
    #[doc = "Bit 18 - Backup SRAM interface clock enable"]
    #[inline]
    pub fn bkpsramen(&mut self) -> _BKPSRAMENW {
        _BKPSRAMENW { w: self }
    }
    #[doc = "Bit 12 - CRC clock enable"]
    #[inline]
    pub fn crcen(&mut self) -> _CRCENW {
        _CRCENW { w: self }
    }
    #[doc = "Bit 8 - IO port I clock enable"]
    #[inline]
    pub fn gpioien(&mut self) -> _GPIOIENW {
        _GPIOIENW { w: self }
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline]
    pub fn gpiohen(&mut self) -> _GPIOHENW {
        _GPIOHENW { w: self }
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline]
    pub fn gpiogen(&mut self) -> _GPIOGENW {
        _GPIOGENW { w: self }
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline]
    pub fn gpiofen(&mut self) -> _GPIOFENW {
        _GPIOFENW { w: self }
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline]
    pub fn gpioeen(&mut self) -> _GPIOEENW {
        _GPIOEENW { w: self }
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline]
    pub fn gpioden(&mut self) -> _GPIODENW {
        _GPIODENW { w: self }
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline]
    pub fn gpiocen(&mut self) -> _GPIOCENW {
        _GPIOCENW { w: self }
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline]
    pub fn gpioben(&mut self) -> _GPIOBENW {
        _GPIOBENW { w: self }
    }
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline]
    pub fn gpioaen(&mut self) -> _GPIOAENW {
        _GPIOAENW { w: self }
    }
}
