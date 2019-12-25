#[doc = "Reader of register MTDR"]
pub type R = crate::R<u32, super::MTDR>;
#[doc = "Writer for register MTDR"]
pub type W = crate::W<u32, super::MTDR>;
#[doc = "Register MTDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Command Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_AW {
    #[doc = "0: Transmit DATA\\[7:0\\]"]
    CMD_0 = 0,
    #[doc = "1: Receive (DATA\\[7:0\\] + 1) bytes"]
    CMD_1 = 1,
    #[doc = "2: Generate STOP condition"]
    CMD_2 = 2,
    #[doc = "3: Receive and discard (DATA\\[7:0\\] + 1) bytes"]
    CMD_3 = 3,
    #[doc = "4: Generate (repeated) START and transmit address in DATA\\[7:0\\]"]
    CMD_4 = 4,
    #[doc = "5: Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    CMD_5 = 5,
    #[doc = "6: Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode"]
    CMD_6 = 6,
    #[doc = "7: Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode. This transfer expects a NACK to be returned."]
    CMD_7 = 7,
}
impl From<CMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transmit DATA\\[7:0\\]"]
    #[inline(always)]
    pub fn cmd_0(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_0)
    }
    #[doc = "Receive (DATA\\[7:0\\] + 1) bytes"]
    #[inline(always)]
    pub fn cmd_1(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_1)
    }
    #[doc = "Generate STOP condition"]
    #[inline(always)]
    pub fn cmd_2(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_2)
    }
    #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes"]
    #[inline(always)]
    pub fn cmd_3(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_3)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]"]
    #[inline(always)]
    pub fn cmd_4(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_4)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn cmd_5(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_5)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode"]
    #[inline(always)]
    pub fn cmd_6(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_6)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn cmd_7(self) -> &'a mut W {
        self.variant(CMD_AW::CMD_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:7 - Transmit Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 8:10 - Command Data"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
