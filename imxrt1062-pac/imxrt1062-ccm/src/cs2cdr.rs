#[doc = "Reader of register CS2CDR"]
pub type R = crate::R<u32, super::CS2CDR>;
#[doc = "Writer for register CS2CDR"]
pub type W = crate::W<u32, super::CS2CDR>;
#[doc = "Register CS2CDR `reset()`'s with value 0x0073_36c1"]
impl crate::ResetValue for super::CS2CDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0073_36c1
    }
}
#[doc = "Divider for sai2 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI2_CLK_PODF_A {
    #[doc = "0: Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "1: Divide by 2"]
    DIVIDE_2 = 1,
    #[doc = "2: Divide by 3"]
    DIVIDE_3 = 2,
    #[doc = "3: Divide by 4"]
    DIVIDE_4 = 3,
    #[doc = "4: Divide by 5"]
    DIVIDE_5 = 4,
    #[doc = "5: Divide by 6"]
    DIVIDE_6 = 5,
    #[doc = "6: Divide by 7"]
    DIVIDE_7 = 6,
    #[doc = "7: Divide by 8"]
    DIVIDE_8 = 7,
    #[doc = "8: Divide by 9"]
    DIVIDE_9 = 8,
    #[doc = "9: Divide by 10"]
    DIVIDE_10 = 9,
    #[doc = "10: Divide by 11"]
    DIVIDE_11 = 10,
    #[doc = "11: Divide by 12"]
    DIVIDE_12 = 11,
    #[doc = "12: Divide by 13"]
    DIVIDE_13 = 12,
    #[doc = "13: Divide by 14"]
    DIVIDE_14 = 13,
    #[doc = "14: Divide by 15"]
    DIVIDE_15 = 14,
    #[doc = "15: Divide by 16"]
    DIVIDE_16 = 15,
    #[doc = "16: Divide by 17"]
    DIVIDE_17 = 16,
    #[doc = "17: Divide by 18"]
    DIVIDE_18 = 17,
    #[doc = "18: Divide by 19"]
    DIVIDE_19 = 18,
    #[doc = "19: Divide by 20"]
    DIVIDE_20 = 19,
    #[doc = "20: Divide by 21"]
    DIVIDE_21 = 20,
    #[doc = "21: Divide by 22"]
    DIVIDE_22 = 21,
    #[doc = "22: Divide by 23"]
    DIVIDE_23 = 22,
    #[doc = "23: Divide by 24"]
    DIVIDE_24 = 23,
    #[doc = "24: Divide by 25"]
    DIVIDE_25 = 24,
    #[doc = "25: Divide by 26"]
    DIVIDE_26 = 25,
    #[doc = "26: Divide by 27"]
    DIVIDE_27 = 26,
    #[doc = "27: Divide by 28"]
    DIVIDE_28 = 27,
    #[doc = "28: Divide by 29"]
    DIVIDE_29 = 28,
    #[doc = "29: Divide by 30"]
    DIVIDE_30 = 29,
    #[doc = "30: Divide by 31"]
    DIVIDE_31 = 30,
    #[doc = "31: Divide by 32"]
    DIVIDE_32 = 31,
    #[doc = "32: Divide by 33"]
    DIVIDE_33 = 32,
    #[doc = "33: Divide by 34"]
    DIVIDE_34 = 33,
    #[doc = "34: Divide by 35"]
    DIVIDE_35 = 34,
    #[doc = "35: Divide by 36"]
    DIVIDE_36 = 35,
    #[doc = "36: Divide by 37"]
    DIVIDE_37 = 36,
    #[doc = "37: Divide by 38"]
    DIVIDE_38 = 37,
    #[doc = "38: Divide by 39"]
    DIVIDE_39 = 38,
    #[doc = "39: Divide by 40"]
    DIVIDE_40 = 39,
    #[doc = "40: Divide by 41"]
    DIVIDE_41 = 40,
    #[doc = "41: Divide by 42"]
    DIVIDE_42 = 41,
    #[doc = "42: Divide by 43"]
    DIVIDE_43 = 42,
    #[doc = "43: Divide by 44"]
    DIVIDE_44 = 43,
    #[doc = "44: Divide by 45"]
    DIVIDE_45 = 44,
    #[doc = "45: Divide by 46"]
    DIVIDE_46 = 45,
    #[doc = "46: Divide by 47"]
    DIVIDE_47 = 46,
    #[doc = "47: Divide by 48"]
    DIVIDE_48 = 47,
    #[doc = "48: Divide by 49"]
    DIVIDE_49 = 48,
    #[doc = "49: Divide by 50"]
    DIVIDE_50 = 49,
    #[doc = "50: Divide by 51"]
    DIVIDE_51 = 50,
    #[doc = "51: Divide by 52"]
    DIVIDE_52 = 51,
    #[doc = "52: Divide by 53"]
    DIVIDE_53 = 52,
    #[doc = "53: Divide by 54"]
    DIVIDE_54 = 53,
    #[doc = "54: Divide by 55"]
    DIVIDE_55 = 54,
    #[doc = "55: Divide by 56"]
    DIVIDE_56 = 55,
    #[doc = "56: Divide by 57"]
    DIVIDE_57 = 56,
    #[doc = "57: Divide by 58"]
    DIVIDE_58 = 57,
    #[doc = "58: Divide by 59"]
    DIVIDE_59 = 58,
    #[doc = "59: Divide by 60"]
    DIVIDE_60 = 59,
    #[doc = "60: Divide by 61"]
    DIVIDE_61 = 60,
    #[doc = "61: Divide by 62"]
    DIVIDE_62 = 61,
    #[doc = "62: Divide by 63"]
    DIVIDE_63 = 62,
    #[doc = "63: Divide by 64"]
    DIVIDE_64 = 63,
}
impl From<SAI2_CLK_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2_CLK_PODF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI2_CLK_PODF`"]
pub type SAI2_CLK_PODF_R = crate::R<u8, SAI2_CLK_PODF_A>;
impl SAI2_CLK_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_CLK_PODF_A {
        match self.bits {
            0 => SAI2_CLK_PODF_A::DIVIDE_1,
            1 => SAI2_CLK_PODF_A::DIVIDE_2,
            2 => SAI2_CLK_PODF_A::DIVIDE_3,
            3 => SAI2_CLK_PODF_A::DIVIDE_4,
            4 => SAI2_CLK_PODF_A::DIVIDE_5,
            5 => SAI2_CLK_PODF_A::DIVIDE_6,
            6 => SAI2_CLK_PODF_A::DIVIDE_7,
            7 => SAI2_CLK_PODF_A::DIVIDE_8,
            8 => SAI2_CLK_PODF_A::DIVIDE_9,
            9 => SAI2_CLK_PODF_A::DIVIDE_10,
            10 => SAI2_CLK_PODF_A::DIVIDE_11,
            11 => SAI2_CLK_PODF_A::DIVIDE_12,
            12 => SAI2_CLK_PODF_A::DIVIDE_13,
            13 => SAI2_CLK_PODF_A::DIVIDE_14,
            14 => SAI2_CLK_PODF_A::DIVIDE_15,
            15 => SAI2_CLK_PODF_A::DIVIDE_16,
            16 => SAI2_CLK_PODF_A::DIVIDE_17,
            17 => SAI2_CLK_PODF_A::DIVIDE_18,
            18 => SAI2_CLK_PODF_A::DIVIDE_19,
            19 => SAI2_CLK_PODF_A::DIVIDE_20,
            20 => SAI2_CLK_PODF_A::DIVIDE_21,
            21 => SAI2_CLK_PODF_A::DIVIDE_22,
            22 => SAI2_CLK_PODF_A::DIVIDE_23,
            23 => SAI2_CLK_PODF_A::DIVIDE_24,
            24 => SAI2_CLK_PODF_A::DIVIDE_25,
            25 => SAI2_CLK_PODF_A::DIVIDE_26,
            26 => SAI2_CLK_PODF_A::DIVIDE_27,
            27 => SAI2_CLK_PODF_A::DIVIDE_28,
            28 => SAI2_CLK_PODF_A::DIVIDE_29,
            29 => SAI2_CLK_PODF_A::DIVIDE_30,
            30 => SAI2_CLK_PODF_A::DIVIDE_31,
            31 => SAI2_CLK_PODF_A::DIVIDE_32,
            32 => SAI2_CLK_PODF_A::DIVIDE_33,
            33 => SAI2_CLK_PODF_A::DIVIDE_34,
            34 => SAI2_CLK_PODF_A::DIVIDE_35,
            35 => SAI2_CLK_PODF_A::DIVIDE_36,
            36 => SAI2_CLK_PODF_A::DIVIDE_37,
            37 => SAI2_CLK_PODF_A::DIVIDE_38,
            38 => SAI2_CLK_PODF_A::DIVIDE_39,
            39 => SAI2_CLK_PODF_A::DIVIDE_40,
            40 => SAI2_CLK_PODF_A::DIVIDE_41,
            41 => SAI2_CLK_PODF_A::DIVIDE_42,
            42 => SAI2_CLK_PODF_A::DIVIDE_43,
            43 => SAI2_CLK_PODF_A::DIVIDE_44,
            44 => SAI2_CLK_PODF_A::DIVIDE_45,
            45 => SAI2_CLK_PODF_A::DIVIDE_46,
            46 => SAI2_CLK_PODF_A::DIVIDE_47,
            47 => SAI2_CLK_PODF_A::DIVIDE_48,
            48 => SAI2_CLK_PODF_A::DIVIDE_49,
            49 => SAI2_CLK_PODF_A::DIVIDE_50,
            50 => SAI2_CLK_PODF_A::DIVIDE_51,
            51 => SAI2_CLK_PODF_A::DIVIDE_52,
            52 => SAI2_CLK_PODF_A::DIVIDE_53,
            53 => SAI2_CLK_PODF_A::DIVIDE_54,
            54 => SAI2_CLK_PODF_A::DIVIDE_55,
            55 => SAI2_CLK_PODF_A::DIVIDE_56,
            56 => SAI2_CLK_PODF_A::DIVIDE_57,
            57 => SAI2_CLK_PODF_A::DIVIDE_58,
            58 => SAI2_CLK_PODF_A::DIVIDE_59,
            59 => SAI2_CLK_PODF_A::DIVIDE_60,
            60 => SAI2_CLK_PODF_A::DIVIDE_61,
            61 => SAI2_CLK_PODF_A::DIVIDE_62,
            62 => SAI2_CLK_PODF_A::DIVIDE_63,
            63 => SAI2_CLK_PODF_A::DIVIDE_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_8
    }
    #[doc = "Checks if the value of the field is `DIVIDE_9`"]
    #[inline(always)]
    pub fn is_divide_9(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_9
    }
    #[doc = "Checks if the value of the field is `DIVIDE_10`"]
    #[inline(always)]
    pub fn is_divide_10(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_10
    }
    #[doc = "Checks if the value of the field is `DIVIDE_11`"]
    #[inline(always)]
    pub fn is_divide_11(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_11
    }
    #[doc = "Checks if the value of the field is `DIVIDE_12`"]
    #[inline(always)]
    pub fn is_divide_12(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_12
    }
    #[doc = "Checks if the value of the field is `DIVIDE_13`"]
    #[inline(always)]
    pub fn is_divide_13(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_13
    }
    #[doc = "Checks if the value of the field is `DIVIDE_14`"]
    #[inline(always)]
    pub fn is_divide_14(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_14
    }
    #[doc = "Checks if the value of the field is `DIVIDE_15`"]
    #[inline(always)]
    pub fn is_divide_15(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_15
    }
    #[doc = "Checks if the value of the field is `DIVIDE_16`"]
    #[inline(always)]
    pub fn is_divide_16(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_16
    }
    #[doc = "Checks if the value of the field is `DIVIDE_17`"]
    #[inline(always)]
    pub fn is_divide_17(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_17
    }
    #[doc = "Checks if the value of the field is `DIVIDE_18`"]
    #[inline(always)]
    pub fn is_divide_18(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_18
    }
    #[doc = "Checks if the value of the field is `DIVIDE_19`"]
    #[inline(always)]
    pub fn is_divide_19(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_19
    }
    #[doc = "Checks if the value of the field is `DIVIDE_20`"]
    #[inline(always)]
    pub fn is_divide_20(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_20
    }
    #[doc = "Checks if the value of the field is `DIVIDE_21`"]
    #[inline(always)]
    pub fn is_divide_21(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_21
    }
    #[doc = "Checks if the value of the field is `DIVIDE_22`"]
    #[inline(always)]
    pub fn is_divide_22(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_22
    }
    #[doc = "Checks if the value of the field is `DIVIDE_23`"]
    #[inline(always)]
    pub fn is_divide_23(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_23
    }
    #[doc = "Checks if the value of the field is `DIVIDE_24`"]
    #[inline(always)]
    pub fn is_divide_24(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_24
    }
    #[doc = "Checks if the value of the field is `DIVIDE_25`"]
    #[inline(always)]
    pub fn is_divide_25(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_25
    }
    #[doc = "Checks if the value of the field is `DIVIDE_26`"]
    #[inline(always)]
    pub fn is_divide_26(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_26
    }
    #[doc = "Checks if the value of the field is `DIVIDE_27`"]
    #[inline(always)]
    pub fn is_divide_27(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_27
    }
    #[doc = "Checks if the value of the field is `DIVIDE_28`"]
    #[inline(always)]
    pub fn is_divide_28(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_28
    }
    #[doc = "Checks if the value of the field is `DIVIDE_29`"]
    #[inline(always)]
    pub fn is_divide_29(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_29
    }
    #[doc = "Checks if the value of the field is `DIVIDE_30`"]
    #[inline(always)]
    pub fn is_divide_30(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_30
    }
    #[doc = "Checks if the value of the field is `DIVIDE_31`"]
    #[inline(always)]
    pub fn is_divide_31(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_31
    }
    #[doc = "Checks if the value of the field is `DIVIDE_32`"]
    #[inline(always)]
    pub fn is_divide_32(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_32
    }
    #[doc = "Checks if the value of the field is `DIVIDE_33`"]
    #[inline(always)]
    pub fn is_divide_33(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_33
    }
    #[doc = "Checks if the value of the field is `DIVIDE_34`"]
    #[inline(always)]
    pub fn is_divide_34(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_34
    }
    #[doc = "Checks if the value of the field is `DIVIDE_35`"]
    #[inline(always)]
    pub fn is_divide_35(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_35
    }
    #[doc = "Checks if the value of the field is `DIVIDE_36`"]
    #[inline(always)]
    pub fn is_divide_36(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_36
    }
    #[doc = "Checks if the value of the field is `DIVIDE_37`"]
    #[inline(always)]
    pub fn is_divide_37(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_37
    }
    #[doc = "Checks if the value of the field is `DIVIDE_38`"]
    #[inline(always)]
    pub fn is_divide_38(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_38
    }
    #[doc = "Checks if the value of the field is `DIVIDE_39`"]
    #[inline(always)]
    pub fn is_divide_39(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_39
    }
    #[doc = "Checks if the value of the field is `DIVIDE_40`"]
    #[inline(always)]
    pub fn is_divide_40(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_40
    }
    #[doc = "Checks if the value of the field is `DIVIDE_41`"]
    #[inline(always)]
    pub fn is_divide_41(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_41
    }
    #[doc = "Checks if the value of the field is `DIVIDE_42`"]
    #[inline(always)]
    pub fn is_divide_42(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_42
    }
    #[doc = "Checks if the value of the field is `DIVIDE_43`"]
    #[inline(always)]
    pub fn is_divide_43(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_43
    }
    #[doc = "Checks if the value of the field is `DIVIDE_44`"]
    #[inline(always)]
    pub fn is_divide_44(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_44
    }
    #[doc = "Checks if the value of the field is `DIVIDE_45`"]
    #[inline(always)]
    pub fn is_divide_45(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_45
    }
    #[doc = "Checks if the value of the field is `DIVIDE_46`"]
    #[inline(always)]
    pub fn is_divide_46(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_46
    }
    #[doc = "Checks if the value of the field is `DIVIDE_47`"]
    #[inline(always)]
    pub fn is_divide_47(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_47
    }
    #[doc = "Checks if the value of the field is `DIVIDE_48`"]
    #[inline(always)]
    pub fn is_divide_48(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_48
    }
    #[doc = "Checks if the value of the field is `DIVIDE_49`"]
    #[inline(always)]
    pub fn is_divide_49(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_49
    }
    #[doc = "Checks if the value of the field is `DIVIDE_50`"]
    #[inline(always)]
    pub fn is_divide_50(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_50
    }
    #[doc = "Checks if the value of the field is `DIVIDE_51`"]
    #[inline(always)]
    pub fn is_divide_51(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_51
    }
    #[doc = "Checks if the value of the field is `DIVIDE_52`"]
    #[inline(always)]
    pub fn is_divide_52(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_52
    }
    #[doc = "Checks if the value of the field is `DIVIDE_53`"]
    #[inline(always)]
    pub fn is_divide_53(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_53
    }
    #[doc = "Checks if the value of the field is `DIVIDE_54`"]
    #[inline(always)]
    pub fn is_divide_54(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_54
    }
    #[doc = "Checks if the value of the field is `DIVIDE_55`"]
    #[inline(always)]
    pub fn is_divide_55(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_55
    }
    #[doc = "Checks if the value of the field is `DIVIDE_56`"]
    #[inline(always)]
    pub fn is_divide_56(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_56
    }
    #[doc = "Checks if the value of the field is `DIVIDE_57`"]
    #[inline(always)]
    pub fn is_divide_57(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_57
    }
    #[doc = "Checks if the value of the field is `DIVIDE_58`"]
    #[inline(always)]
    pub fn is_divide_58(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_58
    }
    #[doc = "Checks if the value of the field is `DIVIDE_59`"]
    #[inline(always)]
    pub fn is_divide_59(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_59
    }
    #[doc = "Checks if the value of the field is `DIVIDE_60`"]
    #[inline(always)]
    pub fn is_divide_60(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_60
    }
    #[doc = "Checks if the value of the field is `DIVIDE_61`"]
    #[inline(always)]
    pub fn is_divide_61(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_61
    }
    #[doc = "Checks if the value of the field is `DIVIDE_62`"]
    #[inline(always)]
    pub fn is_divide_62(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_62
    }
    #[doc = "Checks if the value of the field is `DIVIDE_63`"]
    #[inline(always)]
    pub fn is_divide_63(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_63
    }
    #[doc = "Checks if the value of the field is `DIVIDE_64`"]
    #[inline(always)]
    pub fn is_divide_64(&self) -> bool {
        *self == SAI2_CLK_PODF_A::DIVIDE_64
    }
}
#[doc = "Write proxy for field `SAI2_CLK_PODF`"]
pub struct SAI2_CLK_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2_CLK_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2_CLK_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn divide_9(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn divide_10(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn divide_11(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn divide_12(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn divide_13(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn divide_14(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn divide_15(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn divide_16(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_16)
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn divide_17(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_17)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn divide_18(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_18)
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn divide_19(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_19)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn divide_20(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_20)
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn divide_21(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_21)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn divide_22(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_22)
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn divide_23(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_23)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn divide_24(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_24)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn divide_25(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_25)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn divide_26(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_26)
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn divide_27(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_27)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn divide_28(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_28)
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn divide_29(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_29)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn divide_30(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_30)
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn divide_31(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_31)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn divide_32(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_32)
    }
    #[doc = "Divide by 33"]
    #[inline(always)]
    pub fn divide_33(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_33)
    }
    #[doc = "Divide by 34"]
    #[inline(always)]
    pub fn divide_34(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_34)
    }
    #[doc = "Divide by 35"]
    #[inline(always)]
    pub fn divide_35(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_35)
    }
    #[doc = "Divide by 36"]
    #[inline(always)]
    pub fn divide_36(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_36)
    }
    #[doc = "Divide by 37"]
    #[inline(always)]
    pub fn divide_37(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_37)
    }
    #[doc = "Divide by 38"]
    #[inline(always)]
    pub fn divide_38(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_38)
    }
    #[doc = "Divide by 39"]
    #[inline(always)]
    pub fn divide_39(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_39)
    }
    #[doc = "Divide by 40"]
    #[inline(always)]
    pub fn divide_40(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_40)
    }
    #[doc = "Divide by 41"]
    #[inline(always)]
    pub fn divide_41(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_41)
    }
    #[doc = "Divide by 42"]
    #[inline(always)]
    pub fn divide_42(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_42)
    }
    #[doc = "Divide by 43"]
    #[inline(always)]
    pub fn divide_43(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_43)
    }
    #[doc = "Divide by 44"]
    #[inline(always)]
    pub fn divide_44(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_44)
    }
    #[doc = "Divide by 45"]
    #[inline(always)]
    pub fn divide_45(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_45)
    }
    #[doc = "Divide by 46"]
    #[inline(always)]
    pub fn divide_46(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_46)
    }
    #[doc = "Divide by 47"]
    #[inline(always)]
    pub fn divide_47(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_47)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn divide_48(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_48)
    }
    #[doc = "Divide by 49"]
    #[inline(always)]
    pub fn divide_49(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_49)
    }
    #[doc = "Divide by 50"]
    #[inline(always)]
    pub fn divide_50(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_50)
    }
    #[doc = "Divide by 51"]
    #[inline(always)]
    pub fn divide_51(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_51)
    }
    #[doc = "Divide by 52"]
    #[inline(always)]
    pub fn divide_52(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_52)
    }
    #[doc = "Divide by 53"]
    #[inline(always)]
    pub fn divide_53(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_53)
    }
    #[doc = "Divide by 54"]
    #[inline(always)]
    pub fn divide_54(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_54)
    }
    #[doc = "Divide by 55"]
    #[inline(always)]
    pub fn divide_55(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_55)
    }
    #[doc = "Divide by 56"]
    #[inline(always)]
    pub fn divide_56(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_56)
    }
    #[doc = "Divide by 57"]
    #[inline(always)]
    pub fn divide_57(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_57)
    }
    #[doc = "Divide by 58"]
    #[inline(always)]
    pub fn divide_58(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_58)
    }
    #[doc = "Divide by 59"]
    #[inline(always)]
    pub fn divide_59(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_59)
    }
    #[doc = "Divide by 60"]
    #[inline(always)]
    pub fn divide_60(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_60)
    }
    #[doc = "Divide by 61"]
    #[inline(always)]
    pub fn divide_61(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_61)
    }
    #[doc = "Divide by 62"]
    #[inline(always)]
    pub fn divide_62(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_62)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn divide_63(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn divide_64(self) -> &'a mut W {
        self.variant(SAI2_CLK_PODF_A::DIVIDE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Divider for sai2 clock pred.Divider should be updated when output clock is gated.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI2_CLK_PRED_A {
    #[doc = "0: divide by 1"]
    SAI2_CLK_PRED_0 = 0,
    #[doc = "1: divide by 2"]
    SAI2_CLK_PRED_1 = 1,
    #[doc = "2: divide by 3"]
    SAI2_CLK_PRED_2 = 2,
    #[doc = "3: divide by 4"]
    SAI2_CLK_PRED_3 = 3,
    #[doc = "4: divide by 5"]
    SAI2_CLK_PRED_4 = 4,
    #[doc = "5: divide by 6"]
    SAI2_CLK_PRED_5 = 5,
    #[doc = "6: divide by 7"]
    SAI2_CLK_PRED_6 = 6,
    #[doc = "7: divide by 8"]
    SAI2_CLK_PRED_7 = 7,
}
impl From<SAI2_CLK_PRED_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2_CLK_PRED_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI2_CLK_PRED`"]
pub type SAI2_CLK_PRED_R = crate::R<u8, SAI2_CLK_PRED_A>;
impl SAI2_CLK_PRED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_CLK_PRED_A {
        match self.bits {
            0 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_0,
            1 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_1,
            2 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_2,
            3 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_3,
            4 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_4,
            5 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_5,
            6 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_6,
            7 => SAI2_CLK_PRED_A::SAI2_CLK_PRED_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_0`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_0(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_0
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_1`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_1(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_1
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_2`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_2(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_2
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_3`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_3(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_3
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_4`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_4(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_4
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_5`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_5(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_5
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_6`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_6(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_6
    }
    #[doc = "Checks if the value of the field is `SAI2_CLK_PRED_7`"]
    #[inline(always)]
    pub fn is_sai2_clk_pred_7(&self) -> bool {
        *self == SAI2_CLK_PRED_A::SAI2_CLK_PRED_7
    }
}
#[doc = "Write proxy for field `SAI2_CLK_PRED`"]
pub struct SAI2_CLK_PRED_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2_CLK_PRED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2_CLK_PRED_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn sai2_clk_pred_0(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn sai2_clk_pred_1(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn sai2_clk_pred_2(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn sai2_clk_pred_3(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn sai2_clk_pred_4(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn sai2_clk_pred_5(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn sai2_clk_pred_6(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn sai2_clk_pred_7(self) -> &'a mut W {
        self.variant(SAI2_CLK_PRED_A::SAI2_CLK_PRED_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Divider for sai2 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub fn sai2_clk_podf(&self) -> SAI2_CLK_PODF_R {
        SAI2_CLK_PODF_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - Divider for sai2 clock pred.Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn sai2_clk_pred(&self) -> SAI2_CLK_PRED_R {
        SAI2_CLK_PRED_R::new(((self.bits >> 6) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divider for sai2 clock podf. The input clock to this divider should be lower than 300Mhz, the predivider can be used to achieve this."]
    #[inline(always)]
    pub fn sai2_clk_podf(&mut self) -> SAI2_CLK_PODF_W {
        SAI2_CLK_PODF_W { w: self }
    }
    #[doc = "Bits 6:8 - Divider for sai2 clock pred.Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn sai2_clk_pred(&mut self) -> SAI2_CLK_PRED_W {
        SAI2_CLK_PRED_W { w: self }
    }
}
