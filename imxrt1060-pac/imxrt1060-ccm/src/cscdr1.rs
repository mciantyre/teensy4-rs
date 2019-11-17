#[doc = "Reader of register CSCDR1"]
pub type R = crate::R<u32, super::CSCDR1>;
#[doc = "Writer for register CSCDR1"]
pub type W = crate::W<u32, super::CSCDR1>;
#[doc = "Register CSCDR1 `reset()`'s with value 0x0649_0b00"]
impl crate::ResetValue for super::CSCDR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0649_0b00
    }
}
#[doc = "Divider for uart clock podf.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_CLK_PODF_A {
    #[doc = "0: Divide by 1"]
    DIVIDE_1,
    #[doc = "1: Divide by 2"]
    DIVIDE_2,
    #[doc = "2: Divide by 3"]
    DIVIDE_3,
    #[doc = "3: Divide by 4"]
    DIVIDE_4,
    #[doc = "4: Divide by 5"]
    DIVIDE_5,
    #[doc = "5: Divide by 6"]
    DIVIDE_6,
    #[doc = "6: Divide by 7"]
    DIVIDE_7,
    #[doc = "7: Divide by 8"]
    DIVIDE_8,
    #[doc = "8: Divide by 9"]
    DIVIDE_9,
    #[doc = "9: Divide by 10"]
    DIVIDE_10,
    #[doc = "10: Divide by 11"]
    DIVIDE_11,
    #[doc = "11: Divide by 12"]
    DIVIDE_12,
    #[doc = "12: Divide by 13"]
    DIVIDE_13,
    #[doc = "13: Divide by 14"]
    DIVIDE_14,
    #[doc = "14: Divide by 15"]
    DIVIDE_15,
    #[doc = "15: Divide by 16"]
    DIVIDE_16,
    #[doc = "16: Divide by 17"]
    DIVIDE_17,
    #[doc = "17: Divide by 18"]
    DIVIDE_18,
    #[doc = "18: Divide by 19"]
    DIVIDE_19,
    #[doc = "19: Divide by 20"]
    DIVIDE_20,
    #[doc = "20: Divide by 21"]
    DIVIDE_21,
    #[doc = "21: Divide by 22"]
    DIVIDE_22,
    #[doc = "22: Divide by 23"]
    DIVIDE_23,
    #[doc = "23: Divide by 24"]
    DIVIDE_24,
    #[doc = "24: Divide by 25"]
    DIVIDE_25,
    #[doc = "25: Divide by 26"]
    DIVIDE_26,
    #[doc = "26: Divide by 27"]
    DIVIDE_27,
    #[doc = "27: Divide by 28"]
    DIVIDE_28,
    #[doc = "28: Divide by 29"]
    DIVIDE_29,
    #[doc = "29: Divide by 30"]
    DIVIDE_30,
    #[doc = "30: Divide by 31"]
    DIVIDE_31,
    #[doc = "31: Divide by 32"]
    DIVIDE_32,
    #[doc = "32: Divide by 33"]
    DIVIDE_33,
    #[doc = "33: Divide by 34"]
    DIVIDE_34,
    #[doc = "34: Divide by 35"]
    DIVIDE_35,
    #[doc = "35: Divide by 36"]
    DIVIDE_36,
    #[doc = "36: Divide by 37"]
    DIVIDE_37,
    #[doc = "37: Divide by 38"]
    DIVIDE_38,
    #[doc = "38: Divide by 39"]
    DIVIDE_39,
    #[doc = "39: Divide by 40"]
    DIVIDE_40,
    #[doc = "40: Divide by 41"]
    DIVIDE_41,
    #[doc = "41: Divide by 42"]
    DIVIDE_42,
    #[doc = "42: Divide by 43"]
    DIVIDE_43,
    #[doc = "43: Divide by 44"]
    DIVIDE_44,
    #[doc = "44: Divide by 45"]
    DIVIDE_45,
    #[doc = "45: Divide by 46"]
    DIVIDE_46,
    #[doc = "46: Divide by 47"]
    DIVIDE_47,
    #[doc = "47: Divide by 48"]
    DIVIDE_48,
    #[doc = "48: Divide by 49"]
    DIVIDE_49,
    #[doc = "49: Divide by 50"]
    DIVIDE_50,
    #[doc = "50: Divide by 51"]
    DIVIDE_51,
    #[doc = "51: Divide by 52"]
    DIVIDE_52,
    #[doc = "52: Divide by 53"]
    DIVIDE_53,
    #[doc = "53: Divide by 54"]
    DIVIDE_54,
    #[doc = "54: Divide by 55"]
    DIVIDE_55,
    #[doc = "55: Divide by 56"]
    DIVIDE_56,
    #[doc = "56: Divide by 57"]
    DIVIDE_57,
    #[doc = "57: Divide by 58"]
    DIVIDE_58,
    #[doc = "58: Divide by 59"]
    DIVIDE_59,
    #[doc = "59: Divide by 60"]
    DIVIDE_60,
    #[doc = "60: Divide by 61"]
    DIVIDE_61,
    #[doc = "61: Divide by 62"]
    DIVIDE_62,
    #[doc = "62: Divide by 63"]
    DIVIDE_63,
    #[doc = "63: Divide by 64"]
    DIVIDE_64,
}
impl From<UART_CLK_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: UART_CLK_PODF_A) -> Self {
        match variant {
            UART_CLK_PODF_A::DIVIDE_1 => 0,
            UART_CLK_PODF_A::DIVIDE_2 => 1,
            UART_CLK_PODF_A::DIVIDE_3 => 2,
            UART_CLK_PODF_A::DIVIDE_4 => 3,
            UART_CLK_PODF_A::DIVIDE_5 => 4,
            UART_CLK_PODF_A::DIVIDE_6 => 5,
            UART_CLK_PODF_A::DIVIDE_7 => 6,
            UART_CLK_PODF_A::DIVIDE_8 => 7,
            UART_CLK_PODF_A::DIVIDE_9 => 8,
            UART_CLK_PODF_A::DIVIDE_10 => 9,
            UART_CLK_PODF_A::DIVIDE_11 => 10,
            UART_CLK_PODF_A::DIVIDE_12 => 11,
            UART_CLK_PODF_A::DIVIDE_13 => 12,
            UART_CLK_PODF_A::DIVIDE_14 => 13,
            UART_CLK_PODF_A::DIVIDE_15 => 14,
            UART_CLK_PODF_A::DIVIDE_16 => 15,
            UART_CLK_PODF_A::DIVIDE_17 => 16,
            UART_CLK_PODF_A::DIVIDE_18 => 17,
            UART_CLK_PODF_A::DIVIDE_19 => 18,
            UART_CLK_PODF_A::DIVIDE_20 => 19,
            UART_CLK_PODF_A::DIVIDE_21 => 20,
            UART_CLK_PODF_A::DIVIDE_22 => 21,
            UART_CLK_PODF_A::DIVIDE_23 => 22,
            UART_CLK_PODF_A::DIVIDE_24 => 23,
            UART_CLK_PODF_A::DIVIDE_25 => 24,
            UART_CLK_PODF_A::DIVIDE_26 => 25,
            UART_CLK_PODF_A::DIVIDE_27 => 26,
            UART_CLK_PODF_A::DIVIDE_28 => 27,
            UART_CLK_PODF_A::DIVIDE_29 => 28,
            UART_CLK_PODF_A::DIVIDE_30 => 29,
            UART_CLK_PODF_A::DIVIDE_31 => 30,
            UART_CLK_PODF_A::DIVIDE_32 => 31,
            UART_CLK_PODF_A::DIVIDE_33 => 32,
            UART_CLK_PODF_A::DIVIDE_34 => 33,
            UART_CLK_PODF_A::DIVIDE_35 => 34,
            UART_CLK_PODF_A::DIVIDE_36 => 35,
            UART_CLK_PODF_A::DIVIDE_37 => 36,
            UART_CLK_PODF_A::DIVIDE_38 => 37,
            UART_CLK_PODF_A::DIVIDE_39 => 38,
            UART_CLK_PODF_A::DIVIDE_40 => 39,
            UART_CLK_PODF_A::DIVIDE_41 => 40,
            UART_CLK_PODF_A::DIVIDE_42 => 41,
            UART_CLK_PODF_A::DIVIDE_43 => 42,
            UART_CLK_PODF_A::DIVIDE_44 => 43,
            UART_CLK_PODF_A::DIVIDE_45 => 44,
            UART_CLK_PODF_A::DIVIDE_46 => 45,
            UART_CLK_PODF_A::DIVIDE_47 => 46,
            UART_CLK_PODF_A::DIVIDE_48 => 47,
            UART_CLK_PODF_A::DIVIDE_49 => 48,
            UART_CLK_PODF_A::DIVIDE_50 => 49,
            UART_CLK_PODF_A::DIVIDE_51 => 50,
            UART_CLK_PODF_A::DIVIDE_52 => 51,
            UART_CLK_PODF_A::DIVIDE_53 => 52,
            UART_CLK_PODF_A::DIVIDE_54 => 53,
            UART_CLK_PODF_A::DIVIDE_55 => 54,
            UART_CLK_PODF_A::DIVIDE_56 => 55,
            UART_CLK_PODF_A::DIVIDE_57 => 56,
            UART_CLK_PODF_A::DIVIDE_58 => 57,
            UART_CLK_PODF_A::DIVIDE_59 => 58,
            UART_CLK_PODF_A::DIVIDE_60 => 59,
            UART_CLK_PODF_A::DIVIDE_61 => 60,
            UART_CLK_PODF_A::DIVIDE_62 => 61,
            UART_CLK_PODF_A::DIVIDE_63 => 62,
            UART_CLK_PODF_A::DIVIDE_64 => 63,
        }
    }
}
#[doc = "Reader of field `UART_CLK_PODF`"]
pub type UART_CLK_PODF_R = crate::R<u8, UART_CLK_PODF_A>;
impl UART_CLK_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_CLK_PODF_A {
        match self.bits {
            0 => UART_CLK_PODF_A::DIVIDE_1,
            1 => UART_CLK_PODF_A::DIVIDE_2,
            2 => UART_CLK_PODF_A::DIVIDE_3,
            3 => UART_CLK_PODF_A::DIVIDE_4,
            4 => UART_CLK_PODF_A::DIVIDE_5,
            5 => UART_CLK_PODF_A::DIVIDE_6,
            6 => UART_CLK_PODF_A::DIVIDE_7,
            7 => UART_CLK_PODF_A::DIVIDE_8,
            8 => UART_CLK_PODF_A::DIVIDE_9,
            9 => UART_CLK_PODF_A::DIVIDE_10,
            10 => UART_CLK_PODF_A::DIVIDE_11,
            11 => UART_CLK_PODF_A::DIVIDE_12,
            12 => UART_CLK_PODF_A::DIVIDE_13,
            13 => UART_CLK_PODF_A::DIVIDE_14,
            14 => UART_CLK_PODF_A::DIVIDE_15,
            15 => UART_CLK_PODF_A::DIVIDE_16,
            16 => UART_CLK_PODF_A::DIVIDE_17,
            17 => UART_CLK_PODF_A::DIVIDE_18,
            18 => UART_CLK_PODF_A::DIVIDE_19,
            19 => UART_CLK_PODF_A::DIVIDE_20,
            20 => UART_CLK_PODF_A::DIVIDE_21,
            21 => UART_CLK_PODF_A::DIVIDE_22,
            22 => UART_CLK_PODF_A::DIVIDE_23,
            23 => UART_CLK_PODF_A::DIVIDE_24,
            24 => UART_CLK_PODF_A::DIVIDE_25,
            25 => UART_CLK_PODF_A::DIVIDE_26,
            26 => UART_CLK_PODF_A::DIVIDE_27,
            27 => UART_CLK_PODF_A::DIVIDE_28,
            28 => UART_CLK_PODF_A::DIVIDE_29,
            29 => UART_CLK_PODF_A::DIVIDE_30,
            30 => UART_CLK_PODF_A::DIVIDE_31,
            31 => UART_CLK_PODF_A::DIVIDE_32,
            32 => UART_CLK_PODF_A::DIVIDE_33,
            33 => UART_CLK_PODF_A::DIVIDE_34,
            34 => UART_CLK_PODF_A::DIVIDE_35,
            35 => UART_CLK_PODF_A::DIVIDE_36,
            36 => UART_CLK_PODF_A::DIVIDE_37,
            37 => UART_CLK_PODF_A::DIVIDE_38,
            38 => UART_CLK_PODF_A::DIVIDE_39,
            39 => UART_CLK_PODF_A::DIVIDE_40,
            40 => UART_CLK_PODF_A::DIVIDE_41,
            41 => UART_CLK_PODF_A::DIVIDE_42,
            42 => UART_CLK_PODF_A::DIVIDE_43,
            43 => UART_CLK_PODF_A::DIVIDE_44,
            44 => UART_CLK_PODF_A::DIVIDE_45,
            45 => UART_CLK_PODF_A::DIVIDE_46,
            46 => UART_CLK_PODF_A::DIVIDE_47,
            47 => UART_CLK_PODF_A::DIVIDE_48,
            48 => UART_CLK_PODF_A::DIVIDE_49,
            49 => UART_CLK_PODF_A::DIVIDE_50,
            50 => UART_CLK_PODF_A::DIVIDE_51,
            51 => UART_CLK_PODF_A::DIVIDE_52,
            52 => UART_CLK_PODF_A::DIVIDE_53,
            53 => UART_CLK_PODF_A::DIVIDE_54,
            54 => UART_CLK_PODF_A::DIVIDE_55,
            55 => UART_CLK_PODF_A::DIVIDE_56,
            56 => UART_CLK_PODF_A::DIVIDE_57,
            57 => UART_CLK_PODF_A::DIVIDE_58,
            58 => UART_CLK_PODF_A::DIVIDE_59,
            59 => UART_CLK_PODF_A::DIVIDE_60,
            60 => UART_CLK_PODF_A::DIVIDE_61,
            61 => UART_CLK_PODF_A::DIVIDE_62,
            62 => UART_CLK_PODF_A::DIVIDE_63,
            63 => UART_CLK_PODF_A::DIVIDE_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_8
    }
    #[doc = "Checks if the value of the field is `DIVIDE_9`"]
    #[inline(always)]
    pub fn is_divide_9(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_9
    }
    #[doc = "Checks if the value of the field is `DIVIDE_10`"]
    #[inline(always)]
    pub fn is_divide_10(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_10
    }
    #[doc = "Checks if the value of the field is `DIVIDE_11`"]
    #[inline(always)]
    pub fn is_divide_11(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_11
    }
    #[doc = "Checks if the value of the field is `DIVIDE_12`"]
    #[inline(always)]
    pub fn is_divide_12(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_12
    }
    #[doc = "Checks if the value of the field is `DIVIDE_13`"]
    #[inline(always)]
    pub fn is_divide_13(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_13
    }
    #[doc = "Checks if the value of the field is `DIVIDE_14`"]
    #[inline(always)]
    pub fn is_divide_14(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_14
    }
    #[doc = "Checks if the value of the field is `DIVIDE_15`"]
    #[inline(always)]
    pub fn is_divide_15(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_15
    }
    #[doc = "Checks if the value of the field is `DIVIDE_16`"]
    #[inline(always)]
    pub fn is_divide_16(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_16
    }
    #[doc = "Checks if the value of the field is `DIVIDE_17`"]
    #[inline(always)]
    pub fn is_divide_17(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_17
    }
    #[doc = "Checks if the value of the field is `DIVIDE_18`"]
    #[inline(always)]
    pub fn is_divide_18(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_18
    }
    #[doc = "Checks if the value of the field is `DIVIDE_19`"]
    #[inline(always)]
    pub fn is_divide_19(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_19
    }
    #[doc = "Checks if the value of the field is `DIVIDE_20`"]
    #[inline(always)]
    pub fn is_divide_20(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_20
    }
    #[doc = "Checks if the value of the field is `DIVIDE_21`"]
    #[inline(always)]
    pub fn is_divide_21(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_21
    }
    #[doc = "Checks if the value of the field is `DIVIDE_22`"]
    #[inline(always)]
    pub fn is_divide_22(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_22
    }
    #[doc = "Checks if the value of the field is `DIVIDE_23`"]
    #[inline(always)]
    pub fn is_divide_23(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_23
    }
    #[doc = "Checks if the value of the field is `DIVIDE_24`"]
    #[inline(always)]
    pub fn is_divide_24(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_24
    }
    #[doc = "Checks if the value of the field is `DIVIDE_25`"]
    #[inline(always)]
    pub fn is_divide_25(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_25
    }
    #[doc = "Checks if the value of the field is `DIVIDE_26`"]
    #[inline(always)]
    pub fn is_divide_26(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_26
    }
    #[doc = "Checks if the value of the field is `DIVIDE_27`"]
    #[inline(always)]
    pub fn is_divide_27(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_27
    }
    #[doc = "Checks if the value of the field is `DIVIDE_28`"]
    #[inline(always)]
    pub fn is_divide_28(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_28
    }
    #[doc = "Checks if the value of the field is `DIVIDE_29`"]
    #[inline(always)]
    pub fn is_divide_29(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_29
    }
    #[doc = "Checks if the value of the field is `DIVIDE_30`"]
    #[inline(always)]
    pub fn is_divide_30(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_30
    }
    #[doc = "Checks if the value of the field is `DIVIDE_31`"]
    #[inline(always)]
    pub fn is_divide_31(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_31
    }
    #[doc = "Checks if the value of the field is `DIVIDE_32`"]
    #[inline(always)]
    pub fn is_divide_32(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_32
    }
    #[doc = "Checks if the value of the field is `DIVIDE_33`"]
    #[inline(always)]
    pub fn is_divide_33(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_33
    }
    #[doc = "Checks if the value of the field is `DIVIDE_34`"]
    #[inline(always)]
    pub fn is_divide_34(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_34
    }
    #[doc = "Checks if the value of the field is `DIVIDE_35`"]
    #[inline(always)]
    pub fn is_divide_35(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_35
    }
    #[doc = "Checks if the value of the field is `DIVIDE_36`"]
    #[inline(always)]
    pub fn is_divide_36(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_36
    }
    #[doc = "Checks if the value of the field is `DIVIDE_37`"]
    #[inline(always)]
    pub fn is_divide_37(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_37
    }
    #[doc = "Checks if the value of the field is `DIVIDE_38`"]
    #[inline(always)]
    pub fn is_divide_38(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_38
    }
    #[doc = "Checks if the value of the field is `DIVIDE_39`"]
    #[inline(always)]
    pub fn is_divide_39(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_39
    }
    #[doc = "Checks if the value of the field is `DIVIDE_40`"]
    #[inline(always)]
    pub fn is_divide_40(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_40
    }
    #[doc = "Checks if the value of the field is `DIVIDE_41`"]
    #[inline(always)]
    pub fn is_divide_41(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_41
    }
    #[doc = "Checks if the value of the field is `DIVIDE_42`"]
    #[inline(always)]
    pub fn is_divide_42(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_42
    }
    #[doc = "Checks if the value of the field is `DIVIDE_43`"]
    #[inline(always)]
    pub fn is_divide_43(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_43
    }
    #[doc = "Checks if the value of the field is `DIVIDE_44`"]
    #[inline(always)]
    pub fn is_divide_44(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_44
    }
    #[doc = "Checks if the value of the field is `DIVIDE_45`"]
    #[inline(always)]
    pub fn is_divide_45(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_45
    }
    #[doc = "Checks if the value of the field is `DIVIDE_46`"]
    #[inline(always)]
    pub fn is_divide_46(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_46
    }
    #[doc = "Checks if the value of the field is `DIVIDE_47`"]
    #[inline(always)]
    pub fn is_divide_47(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_47
    }
    #[doc = "Checks if the value of the field is `DIVIDE_48`"]
    #[inline(always)]
    pub fn is_divide_48(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_48
    }
    #[doc = "Checks if the value of the field is `DIVIDE_49`"]
    #[inline(always)]
    pub fn is_divide_49(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_49
    }
    #[doc = "Checks if the value of the field is `DIVIDE_50`"]
    #[inline(always)]
    pub fn is_divide_50(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_50
    }
    #[doc = "Checks if the value of the field is `DIVIDE_51`"]
    #[inline(always)]
    pub fn is_divide_51(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_51
    }
    #[doc = "Checks if the value of the field is `DIVIDE_52`"]
    #[inline(always)]
    pub fn is_divide_52(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_52
    }
    #[doc = "Checks if the value of the field is `DIVIDE_53`"]
    #[inline(always)]
    pub fn is_divide_53(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_53
    }
    #[doc = "Checks if the value of the field is `DIVIDE_54`"]
    #[inline(always)]
    pub fn is_divide_54(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_54
    }
    #[doc = "Checks if the value of the field is `DIVIDE_55`"]
    #[inline(always)]
    pub fn is_divide_55(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_55
    }
    #[doc = "Checks if the value of the field is `DIVIDE_56`"]
    #[inline(always)]
    pub fn is_divide_56(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_56
    }
    #[doc = "Checks if the value of the field is `DIVIDE_57`"]
    #[inline(always)]
    pub fn is_divide_57(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_57
    }
    #[doc = "Checks if the value of the field is `DIVIDE_58`"]
    #[inline(always)]
    pub fn is_divide_58(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_58
    }
    #[doc = "Checks if the value of the field is `DIVIDE_59`"]
    #[inline(always)]
    pub fn is_divide_59(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_59
    }
    #[doc = "Checks if the value of the field is `DIVIDE_60`"]
    #[inline(always)]
    pub fn is_divide_60(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_60
    }
    #[doc = "Checks if the value of the field is `DIVIDE_61`"]
    #[inline(always)]
    pub fn is_divide_61(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_61
    }
    #[doc = "Checks if the value of the field is `DIVIDE_62`"]
    #[inline(always)]
    pub fn is_divide_62(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_62
    }
    #[doc = "Checks if the value of the field is `DIVIDE_63`"]
    #[inline(always)]
    pub fn is_divide_63(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_63
    }
    #[doc = "Checks if the value of the field is `DIVIDE_64`"]
    #[inline(always)]
    pub fn is_divide_64(&self) -> bool {
        *self == UART_CLK_PODF_A::DIVIDE_64
    }
}
#[doc = "Write proxy for field `UART_CLK_PODF`"]
pub struct UART_CLK_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_CLK_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_8)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn divide_9(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_9)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn divide_10(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_10)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn divide_11(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_11)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn divide_12(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_12)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn divide_13(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_13)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn divide_14(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_14)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn divide_15(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_15)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn divide_16(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_16)
    }
    #[doc = "Divide by 17"]
    #[inline(always)]
    pub fn divide_17(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_17)
    }
    #[doc = "Divide by 18"]
    #[inline(always)]
    pub fn divide_18(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_18)
    }
    #[doc = "Divide by 19"]
    #[inline(always)]
    pub fn divide_19(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_19)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn divide_20(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_20)
    }
    #[doc = "Divide by 21"]
    #[inline(always)]
    pub fn divide_21(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_21)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn divide_22(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_22)
    }
    #[doc = "Divide by 23"]
    #[inline(always)]
    pub fn divide_23(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_23)
    }
    #[doc = "Divide by 24"]
    #[inline(always)]
    pub fn divide_24(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_24)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn divide_25(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_25)
    }
    #[doc = "Divide by 26"]
    #[inline(always)]
    pub fn divide_26(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_26)
    }
    #[doc = "Divide by 27"]
    #[inline(always)]
    pub fn divide_27(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_27)
    }
    #[doc = "Divide by 28"]
    #[inline(always)]
    pub fn divide_28(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_28)
    }
    #[doc = "Divide by 29"]
    #[inline(always)]
    pub fn divide_29(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_29)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn divide_30(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_30)
    }
    #[doc = "Divide by 31"]
    #[inline(always)]
    pub fn divide_31(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_31)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn divide_32(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_32)
    }
    #[doc = "Divide by 33"]
    #[inline(always)]
    pub fn divide_33(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_33)
    }
    #[doc = "Divide by 34"]
    #[inline(always)]
    pub fn divide_34(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_34)
    }
    #[doc = "Divide by 35"]
    #[inline(always)]
    pub fn divide_35(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_35)
    }
    #[doc = "Divide by 36"]
    #[inline(always)]
    pub fn divide_36(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_36)
    }
    #[doc = "Divide by 37"]
    #[inline(always)]
    pub fn divide_37(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_37)
    }
    #[doc = "Divide by 38"]
    #[inline(always)]
    pub fn divide_38(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_38)
    }
    #[doc = "Divide by 39"]
    #[inline(always)]
    pub fn divide_39(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_39)
    }
    #[doc = "Divide by 40"]
    #[inline(always)]
    pub fn divide_40(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_40)
    }
    #[doc = "Divide by 41"]
    #[inline(always)]
    pub fn divide_41(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_41)
    }
    #[doc = "Divide by 42"]
    #[inline(always)]
    pub fn divide_42(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_42)
    }
    #[doc = "Divide by 43"]
    #[inline(always)]
    pub fn divide_43(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_43)
    }
    #[doc = "Divide by 44"]
    #[inline(always)]
    pub fn divide_44(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_44)
    }
    #[doc = "Divide by 45"]
    #[inline(always)]
    pub fn divide_45(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_45)
    }
    #[doc = "Divide by 46"]
    #[inline(always)]
    pub fn divide_46(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_46)
    }
    #[doc = "Divide by 47"]
    #[inline(always)]
    pub fn divide_47(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_47)
    }
    #[doc = "Divide by 48"]
    #[inline(always)]
    pub fn divide_48(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_48)
    }
    #[doc = "Divide by 49"]
    #[inline(always)]
    pub fn divide_49(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_49)
    }
    #[doc = "Divide by 50"]
    #[inline(always)]
    pub fn divide_50(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_50)
    }
    #[doc = "Divide by 51"]
    #[inline(always)]
    pub fn divide_51(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_51)
    }
    #[doc = "Divide by 52"]
    #[inline(always)]
    pub fn divide_52(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_52)
    }
    #[doc = "Divide by 53"]
    #[inline(always)]
    pub fn divide_53(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_53)
    }
    #[doc = "Divide by 54"]
    #[inline(always)]
    pub fn divide_54(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_54)
    }
    #[doc = "Divide by 55"]
    #[inline(always)]
    pub fn divide_55(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_55)
    }
    #[doc = "Divide by 56"]
    #[inline(always)]
    pub fn divide_56(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_56)
    }
    #[doc = "Divide by 57"]
    #[inline(always)]
    pub fn divide_57(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_57)
    }
    #[doc = "Divide by 58"]
    #[inline(always)]
    pub fn divide_58(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_58)
    }
    #[doc = "Divide by 59"]
    #[inline(always)]
    pub fn divide_59(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_59)
    }
    #[doc = "Divide by 60"]
    #[inline(always)]
    pub fn divide_60(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_60)
    }
    #[doc = "Divide by 61"]
    #[inline(always)]
    pub fn divide_61(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_61)
    }
    #[doc = "Divide by 62"]
    #[inline(always)]
    pub fn divide_62(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_62)
    }
    #[doc = "Divide by 63"]
    #[inline(always)]
    pub fn divide_63(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_63)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn divide_64(self) -> &'a mut W {
        self.variant(UART_CLK_PODF_A::DIVIDE_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Selector for the UART clock multiplexor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_CLK_SEL_A {
    #[doc = "0: derive clock from pll3_80m"]
    UART_CLK_SEL_0,
    #[doc = "1: derive clock from osc_clk"]
    UART_CLK_SEL_1,
}
impl From<UART_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: UART_CLK_SEL_A) -> Self {
        match variant {
            UART_CLK_SEL_A::UART_CLK_SEL_0 => false,
            UART_CLK_SEL_A::UART_CLK_SEL_1 => true,
        }
    }
}
#[doc = "Reader of field `UART_CLK_SEL`"]
pub type UART_CLK_SEL_R = crate::R<bool, UART_CLK_SEL_A>;
impl UART_CLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART_CLK_SEL_A {
        match self.bits {
            false => UART_CLK_SEL_A::UART_CLK_SEL_0,
            true => UART_CLK_SEL_A::UART_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_uart_clk_sel_0(&self) -> bool {
        *self == UART_CLK_SEL_A::UART_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `UART_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_uart_clk_sel_1(&self) -> bool {
        *self == UART_CLK_SEL_A::UART_CLK_SEL_1
    }
}
#[doc = "Write proxy for field `UART_CLK_SEL`"]
pub struct UART_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_CLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "derive clock from pll3_80m"]
    #[inline(always)]
    pub fn uart_clk_sel_0(self) -> &'a mut W {
        self.variant(UART_CLK_SEL_A::UART_CLK_SEL_0)
    }
    #[doc = "derive clock from osc_clk"]
    #[inline(always)]
    pub fn uart_clk_sel_1(self) -> &'a mut W {
        self.variant(UART_CLK_SEL_A::UART_CLK_SEL_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Divider for usdhc1 clock podf. Divider should be updated when output clock is gated.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC1_PODF_A {
    #[doc = "0: divide by 1"]
    USDHC1_PODF_0,
    #[doc = "1: divide by 2"]
    USDHC1_PODF_1,
    #[doc = "2: divide by 3"]
    USDHC1_PODF_2,
    #[doc = "3: divide by 4"]
    USDHC1_PODF_3,
    #[doc = "4: divide by 5"]
    USDHC1_PODF_4,
    #[doc = "5: divide by 6"]
    USDHC1_PODF_5,
    #[doc = "6: divide by 7"]
    USDHC1_PODF_6,
    #[doc = "7: divide by 8"]
    USDHC1_PODF_7,
}
impl From<USDHC1_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: USDHC1_PODF_A) -> Self {
        match variant {
            USDHC1_PODF_A::USDHC1_PODF_0 => 0,
            USDHC1_PODF_A::USDHC1_PODF_1 => 1,
            USDHC1_PODF_A::USDHC1_PODF_2 => 2,
            USDHC1_PODF_A::USDHC1_PODF_3 => 3,
            USDHC1_PODF_A::USDHC1_PODF_4 => 4,
            USDHC1_PODF_A::USDHC1_PODF_5 => 5,
            USDHC1_PODF_A::USDHC1_PODF_6 => 6,
            USDHC1_PODF_A::USDHC1_PODF_7 => 7,
        }
    }
}
#[doc = "Reader of field `USDHC1_PODF`"]
pub type USDHC1_PODF_R = crate::R<u8, USDHC1_PODF_A>;
impl USDHC1_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC1_PODF_A {
        match self.bits {
            0 => USDHC1_PODF_A::USDHC1_PODF_0,
            1 => USDHC1_PODF_A::USDHC1_PODF_1,
            2 => USDHC1_PODF_A::USDHC1_PODF_2,
            3 => USDHC1_PODF_A::USDHC1_PODF_3,
            4 => USDHC1_PODF_A::USDHC1_PODF_4,
            5 => USDHC1_PODF_A::USDHC1_PODF_5,
            6 => USDHC1_PODF_A::USDHC1_PODF_6,
            7 => USDHC1_PODF_A::USDHC1_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_0`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_0(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_0
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_1`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_1(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_1
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_2`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_2(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_2
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_3`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_3(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_3
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_4`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_4(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_4
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_5`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_5(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_5
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_6`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_6(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_6
    }
    #[doc = "Checks if the value of the field is `USDHC1_PODF_7`"]
    #[inline(always)]
    pub fn is_usdhc1_podf_7(&self) -> bool {
        *self == USDHC1_PODF_A::USDHC1_PODF_7
    }
}
#[doc = "Write proxy for field `USDHC1_PODF`"]
pub struct USDHC1_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> USDHC1_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USDHC1_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn usdhc1_podf_0(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn usdhc1_podf_1(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn usdhc1_podf_2(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn usdhc1_podf_3(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn usdhc1_podf_4(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn usdhc1_podf_5(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn usdhc1_podf_6(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn usdhc1_podf_7(self) -> &'a mut W {
        self.variant(USDHC1_PODF_A::USDHC1_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Divider for usdhc2 clock. Divider should be updated when output clock is gated.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USDHC2_PODF_A {
    #[doc = "0: divide by 1"]
    USDHC2_PODF_0,
    #[doc = "1: divide by 2"]
    USDHC2_PODF_1,
    #[doc = "2: divide by 3"]
    USDHC2_PODF_2,
    #[doc = "3: divide by 4"]
    USDHC2_PODF_3,
    #[doc = "4: divide by 5"]
    USDHC2_PODF_4,
    #[doc = "5: divide by 6"]
    USDHC2_PODF_5,
    #[doc = "6: divide by 7"]
    USDHC2_PODF_6,
    #[doc = "7: divide by 8"]
    USDHC2_PODF_7,
}
impl From<USDHC2_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: USDHC2_PODF_A) -> Self {
        match variant {
            USDHC2_PODF_A::USDHC2_PODF_0 => 0,
            USDHC2_PODF_A::USDHC2_PODF_1 => 1,
            USDHC2_PODF_A::USDHC2_PODF_2 => 2,
            USDHC2_PODF_A::USDHC2_PODF_3 => 3,
            USDHC2_PODF_A::USDHC2_PODF_4 => 4,
            USDHC2_PODF_A::USDHC2_PODF_5 => 5,
            USDHC2_PODF_A::USDHC2_PODF_6 => 6,
            USDHC2_PODF_A::USDHC2_PODF_7 => 7,
        }
    }
}
#[doc = "Reader of field `USDHC2_PODF`"]
pub type USDHC2_PODF_R = crate::R<u8, USDHC2_PODF_A>;
impl USDHC2_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USDHC2_PODF_A {
        match self.bits {
            0 => USDHC2_PODF_A::USDHC2_PODF_0,
            1 => USDHC2_PODF_A::USDHC2_PODF_1,
            2 => USDHC2_PODF_A::USDHC2_PODF_2,
            3 => USDHC2_PODF_A::USDHC2_PODF_3,
            4 => USDHC2_PODF_A::USDHC2_PODF_4,
            5 => USDHC2_PODF_A::USDHC2_PODF_5,
            6 => USDHC2_PODF_A::USDHC2_PODF_6,
            7 => USDHC2_PODF_A::USDHC2_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_0`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_0(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_0
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_1`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_1(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_1
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_2`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_2(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_2
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_3`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_3(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_3
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_4`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_4(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_4
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_5`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_5(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_5
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_6`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_6(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_6
    }
    #[doc = "Checks if the value of the field is `USDHC2_PODF_7`"]
    #[inline(always)]
    pub fn is_usdhc2_podf_7(&self) -> bool {
        *self == USDHC2_PODF_A::USDHC2_PODF_7
    }
}
#[doc = "Write proxy for field `USDHC2_PODF`"]
pub struct USDHC2_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> USDHC2_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USDHC2_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn usdhc2_podf_0(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn usdhc2_podf_1(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn usdhc2_podf_2(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn usdhc2_podf_3(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn usdhc2_podf_4(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn usdhc2_podf_5(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn usdhc2_podf_6(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn usdhc2_podf_7(self) -> &'a mut W {
        self.variant(USDHC2_PODF_A::USDHC2_PODF_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Divider for trace clock. Divider should be updated when output clock is gated.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACE_PODF_A {
    #[doc = "0: divide by 1"]
    TRACE_PODF_0,
    #[doc = "1: divide by 2"]
    TRACE_PODF_1,
    #[doc = "2: divide by 3"]
    TRACE_PODF_2,
    #[doc = "3: divide by 4"]
    TRACE_PODF_3,
}
impl From<TRACE_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACE_PODF_A) -> Self {
        match variant {
            TRACE_PODF_A::TRACE_PODF_0 => 0,
            TRACE_PODF_A::TRACE_PODF_1 => 1,
            TRACE_PODF_A::TRACE_PODF_2 => 2,
            TRACE_PODF_A::TRACE_PODF_3 => 3,
        }
    }
}
#[doc = "Reader of field `TRACE_PODF`"]
pub type TRACE_PODF_R = crate::R<u8, TRACE_PODF_A>;
impl TRACE_PODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACE_PODF_A {
        match self.bits {
            0 => TRACE_PODF_A::TRACE_PODF_0,
            1 => TRACE_PODF_A::TRACE_PODF_1,
            2 => TRACE_PODF_A::TRACE_PODF_2,
            3 => TRACE_PODF_A::TRACE_PODF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_0`"]
    #[inline(always)]
    pub fn is_trace_podf_0(&self) -> bool {
        *self == TRACE_PODF_A::TRACE_PODF_0
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_1`"]
    #[inline(always)]
    pub fn is_trace_podf_1(&self) -> bool {
        *self == TRACE_PODF_A::TRACE_PODF_1
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_2`"]
    #[inline(always)]
    pub fn is_trace_podf_2(&self) -> bool {
        *self == TRACE_PODF_A::TRACE_PODF_2
    }
    #[doc = "Checks if the value of the field is `TRACE_PODF_3`"]
    #[inline(always)]
    pub fn is_trace_podf_3(&self) -> bool {
        *self == TRACE_PODF_A::TRACE_PODF_3
    }
}
#[doc = "Write proxy for field `TRACE_PODF`"]
pub struct TRACE_PODF_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACE_PODF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACE_PODF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn trace_podf_0(self) -> &'a mut W {
        self.variant(TRACE_PODF_A::TRACE_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn trace_podf_1(self) -> &'a mut W {
        self.variant(TRACE_PODF_A::TRACE_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn trace_podf_2(self) -> &'a mut W {
        self.variant(TRACE_PODF_A::TRACE_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn trace_podf_3(self) -> &'a mut W {
        self.variant(TRACE_PODF_A::TRACE_PODF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Divider for uart clock podf."]
    #[inline(always)]
    pub fn uart_clk_podf(&self) -> UART_CLK_PODF_R {
        UART_CLK_PODF_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Selector for the UART clock multiplexor"]
    #[inline(always)]
    pub fn uart_clk_sel(&self) -> UART_CLK_SEL_R {
        UART_CLK_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Divider for usdhc1 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn usdhc1_podf(&self) -> USDHC1_PODF_R {
        USDHC1_PODF_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Divider for usdhc2 clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn usdhc2_podf(&self) -> USDHC2_PODF_R {
        USDHC2_PODF_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 25:26 - Divider for trace clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn trace_podf(&self) -> TRACE_PODF_R {
        TRACE_PODF_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divider for uart clock podf."]
    #[inline(always)]
    pub fn uart_clk_podf(&mut self) -> UART_CLK_PODF_W {
        UART_CLK_PODF_W { w: self }
    }
    #[doc = "Bit 6 - Selector for the UART clock multiplexor"]
    #[inline(always)]
    pub fn uart_clk_sel(&mut self) -> UART_CLK_SEL_W {
        UART_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Divider for usdhc1 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn usdhc1_podf(&mut self) -> USDHC1_PODF_W {
        USDHC1_PODF_W { w: self }
    }
    #[doc = "Bits 16:18 - Divider for usdhc2 clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn usdhc2_podf(&mut self) -> USDHC2_PODF_W {
        USDHC2_PODF_W { w: self }
    }
    #[doc = "Bits 25:26 - Divider for trace clock. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn trace_podf(&mut self) -> TRACE_PODF_W {
        TRACE_PODF_W { w: self }
    }
}
