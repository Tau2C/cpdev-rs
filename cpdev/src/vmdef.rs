pub type WmBool = bool;
pub type WmByte = u8;
pub type WmWord = u16;
pub type WmDword = u32;
pub type WmInt = i16;
pub type WmSint = i8;
pub type WmDint = i32;

#[cfg(feature = "longs")]
pub type WmLword = u64;
#[cfg(feature = "longs")]
pub type WmLint = i64;
#[cfg(not(feature = "longs"))]
pub type WmLword = WmDword;

#[cfg(feature = "reals")]
pub type WmReal = f32;
#[cfg(feature = "long_reals")]
pub type WmLreal = f64;

pub type WmAddress16 = WmWord;
pub type WmAddress32 = WmDword;

#[cfg(feature = "32_addressing")]
pub type WmAddress = WmAddress32;
#[cfg(not(feature = "32_addressing"))]
pub type WmAddress = WmAddress16;

// aliasy
pub type WmUsint = WmByte;
pub type WmUint = WmWord;
pub type WmUdint = WmDword;
pub type WmUlint = WmLword;
pub type WmTime = WmDword;

#[cfg(feature = "datatime_suport")]
#[derive(Debug, Clone, Copy)]
pub struct WmDateT {
    day: WmByte,
    month: WmByte,
    year: WmWord,
}

#[cfg(feature = "datatime_suport")]
pub union WM_DATE {
    date: WmDateT,
    dw_date: WmDword,
}

#[cfg(feature = "datatime_suport")]
#[derive(Clone, Copy)]
pub struct WmTodT {
    csec: WmByte,
    sec: WmByte,
    min: WmByte,
    hour: WmByte,
}

#[cfg(feature = "datatime_suport")]
pub union WmTimeOfDay {
    tod: WmTodT,
    dw_tod: WmDword,
}

#[cfg(feature = "datatime_suport")]
pub type WmTod = WmTimeOfDay;

#[cfg(feature = "datatime_suport")]
pub struct WmDateAndTime {
    tod: WmTimeOfDay,
    date: WM_DATE,
}

// #[cfg(all(feature = "strings", feature = "widechar"))]
// pub type WmCharacter = wchar;
#[cfg(all(feature = "strings", not(feature = "widechar")))]
pub type WmCharacter = char;

#[cfg(feature = "strings")]
pub struct WmString {
    length: WmByte,          // actual string length
    chars_size: WmByte,      // size of buffer chars
    padding: WmWord,         // reserved
    chars: *mut WmCharacter, // string characters
}

pub const VMACH_VERSION: u16 = 0x2900;

// Grupy funkcji

pub const FG_ADD: u8 = 0x01;
pub const FG_SUB: u8 = 0x02;
pub const FG_MUL: u8 = 0x03;
pub const FG_DIV_MOD: u8 = 0x04;
pub const FG_NEG_NOT: u8 = 0x05;
pub const FG_EXPT_ABS_DOW: u8 = 0x06;
pub const FG_SYSMEM: u8 = 0x07;
pub const FG_AND: u8 = 0x08;
pub const FG_OR: u8 = 0x09;
pub const FG_XOR: u8 = 0x0A;
pub const FG_SHI_ROT: u8 = 0x0B;
pub const FG_SEL: u8 = 0x0C;
pub const FG_LIMIT: u8 = 0x0D;
pub const FG_MAX: u8 = 0x0E;
pub const FG_MIN: u8 = 0x0F;
pub const FG_GT: u8 = 0x10;
pub const FG_GE: u8 = 0x11;
pub const FG_EQ: u8 = 0x12;
pub const FG_LE: u8 = 0x13;
pub const FG_LT: u8 = 0x14;
pub const FG_NE: u8 = 0x15;
pub const FG_MUX: u8 = 0x16;
pub const FG_STRING: u8 = 0x18;
pub const FG_CONV: u8 = 0x19;
pub const FG_SYSCTRL: u8 = 0x1C;
pub const FG_CUSTOMSPACE: u8 = 0x1D;
pub const FG_POINTERS: u8 = 0x1E;
pub const FG_HALT: u8 = 0xFF;

// Funkcje maszyny wirtualnej

pub const VMF_ADD_SINT: u16 = 0x01F1;
pub const VMF_ADD_INT: u16 = 0x01F2;
pub const VMF_ADD_DINT: u16 = 0x01F3;
pub const VMF_ADD_LINT: u16 = 0x01F4;
pub const VMF_ADD_BYTE: u16 = 0x01F5;
pub const VMF_ADD_WORD: u16 = 0x01F6;
pub const VMF_ADD_DWORD: u16 = 0x01F7;
pub const VMF_ADD_LWORD: u16 = 0x01F8;
pub const VMF_ADD_REAL: u16 = 0x01F9;
pub const VMF_ADD_LREAL: u16 = 0x01FA;
pub const VMF_ADD_TIME: u16 = 0x01FB;
pub const VMF_ADD_STRING: u16 = 0x01FF;
pub const VMF_ADD_WSTRING: u16 = 0x01FF;
pub const VMF_CONCAT_STRING: u16 = 0x01FF;
pub const VMF_CONCAT_WSTRING: u16 = 0x01FF;

pub const VMF_SUB_SINT_SINT: u16 = 0x0201;
pub const VMF_SUB_INT_INT: u16 = 0x0202;
pub const VMF_SUB_DINT_DINT: u16 = 0x0203;
pub const VMF_SUB_LINT_LINT: u16 = 0x0204;
pub const VMF_SUB_BYTE_BYTE: u16 = 0x0205;
pub const VMF_SUB_WORD_WORD: u16 = 0x0206;
pub const VMF_SUB_DWORD_DWORD: u16 = 0x0207;
pub const VMF_SUB_LWORD_LWORD: u16 = 0x0208;
pub const VMF_SUB_REAL_REAL: u16 = 0x0209;
pub const VMF_SUB_LREAL_LREAL: u16 = 0x020A;
pub const VMF_SUB_TIME_TIME: u16 = 0x020B;
pub const VMF_MUL_SINT: u16 = 0x03F1;
pub const VMF_MUL_INT: u16 = 0x03F2;
pub const VMF_MUL_DINT: u16 = 0x03F3;
pub const VMF_MUL_LINT: u16 = 0x03F4;
pub const VMF_MUL_BYTE: u16 = 0x03F5;
pub const VMF_MUL_WORD: u16 = 0x03F6;
pub const VMF_MUL_DWORD: u16 = 0x03F7;
pub const VMF_MUL_LWORD: u16 = 0x03F8;
pub const VMF_MUL_REAL: u16 = 0x03F9;
pub const VMF_MUL_LREAL: u16 = 0x03FA;
pub const VMF_DIV_SINT_SINT: u16 = 0x0401;
pub const VMF_DIV_INT_INT: u16 = 0x0402;
pub const VMF_DIV_DINT_DINT: u16 = 0x0403;
pub const VMF_DIV_LINT_LINT: u16 = 0x0404;
pub const VMF_DIV_BYTE_BYTE: u16 = 0x0405;
pub const VMF_DIV_WORD_WORD: u16 = 0x0406;
pub const VMF_DIV_DWORD_DWORD: u16 = 0x0407;
pub const VMF_DIV_LWORD_LWORD: u16 = 0x0408;
pub const VMF_DIV_REAL_REAL: u16 = 0x0409;
pub const VMF_DIV_LREAL_LREAL: u16 = 0x040A;
pub const VMF_MOD_SINT_SINT: u16 = 0x0411;
pub const VMF_MOD_INT_INT: u16 = 0x0412;
pub const VMF_MOD_DINT_DINT: u16 = 0x0413;
pub const VMF_MOD_LINT_LINT: u16 = 0x0414;
pub const VMF_MOD_BYTE_BYTE: u16 = 0x0415;
pub const VMF_MOD_WORD_WORD: u16 = 0x0416;
pub const VMF_MOD_DWORD_DWORD: u16 = 0x0417;
pub const VMF_MOD_LWORD_LWORD: u16 = 0x0418;
pub const VMF_NEG_SINT: u16 = 0x0501;
pub const VMF_NEG_INT: u16 = 0x0502;
pub const VMF_NEG_DINT: u16 = 0x0503;
pub const VMF_NEG_LINT: u16 = 0x0504;
pub const VMF_NEG_REAL: u16 = 0x0507;
pub const VMF_NEG_LREAL: u16 = 0x0509;
pub const VMF_NOT_BOOL: u16 = 0x0510;
pub const VMF_NOT_BYTE: u16 = 0x0511;
pub const VMF_NOT_WORD: u16 = 0x0512;
pub const VMF_NOT_DWORD: u16 = 0x0513;
pub const VMF_NOT_LWORD: u16 = 0x0514;
pub const VMF_EXPT_SINT_SINT: u16 = 0x0601;
pub const VMF_EXPT_INT_SINT: u16 = 0x0602;
pub const VMF_EXPT_DINT_SINT: u16 = 0x0603;
pub const VMF_EXPT_REAL_SINT: u16 = 0x0604;
pub const VMF_EXPT_REAL_INT: u16 = 0x0605;
pub const VMF_EXPT_REAL_DINT: u16 = 0x0606;
pub const VMF_EXPT_REAL_REAL: u16 = 0x0607;
pub const VMF_EXPT_LREAL_LREAL: u16 = 0x0608;
pub const VMF_ABS_SINT: u16 = 0x0611;
pub const VMF_ABS_INT: u16 = 0x0612;
pub const VMF_ABS_DINT: u16 = 0x0613;
pub const VMF_ABS_LINT: u16 = 0x0614;
pub const VMF_ABS_BYTE: u16 = 0x0615;
pub const VMF_ABS_WORD: u16 = 0x0616;
pub const VMF_ABS_DWORD: u16 = 0x0617;
pub const VMF_ABS_LWORD: u16 = 0x0618;
pub const VMF_ABS_REAL: u16 = 0x0619;
pub const VMF_ABS_LREAL: u16 = 0x061A;

pub const VMF_SQRT_REAL: u16 = 0x0620;
pub const VMF_SQRT_LREAL: u16 = 0x0621;
pub const VMF_LN_REAL: u16 = 0x0622;
pub const VMF_LN_LREAL: u16 = 0x0623;
pub const VMF_LOG_REAL: u16 = 0x0624;
pub const VMF_LOG_LREAL: u16 = 0x0625;
pub const VMF_EXP_REAL: u16 = 0x0626;
pub const VMF_EXP_LREAL: u16 = 0x0627;
pub const VMF_SIN_REAL: u16 = 0x0628;
pub const VMF_SIN_LREAL: u16 = 0x0629;
pub const VMF_COS_REAL: u16 = 0x062A;
pub const VMF_COS_LREAL: u16 = 0x062B;
pub const VMF_TAN_REAL: u16 = 0x062C;
pub const VMF_TAN_LREAL: u16 = 0x062D;
pub const VMF_ASIN_REAL: u16 = 0x062E;
pub const VMF_ASIN_LREAL: u16 = 0x062F;
pub const VMF_ACOS_REAL: u16 = 0x0630;
pub const VMF_ACOS_LREAL: u16 = 0x0631;
pub const VMF_ATAN_REAL: u16 = 0x0632;
pub const VMF_ATAN_LREAL: u16 = 0x0633;
pub const VMF_TRUNC_REAL: u16 = 0x0634;
pub const VMF_TRUNC_LREAL: u16 = 0x0635;
pub const VMF_ROUND_REAL: u16 = 0x0636;
pub const VMF_ROUND_LREAL: u16 = 0x0637;

pub const VMF_GET_DAYOFWEEK_DATE: u16 = 0x0640;
pub const VMF_GET_DAYOFWEEK_DATETIME: u16 = 0x0641;

pub const VMF_AND_BOOL: u16 = 0x08F0;
pub const VMF_AND_BYTE: u16 = 0x08F1;
pub const VMF_AND_WORD: u16 = 0x08F2;
pub const VMF_AND_DWORD: u16 = 0x08F3;
pub const VMF_AND_LWORD: u16 = 0x08F4;
pub const VMF_OR_BOOL: u16 = 0x09F0;
pub const VMF_OR_BYTE: u16 = 0x09F1;
pub const VMF_OR_WORD: u16 = 0x09F2;
pub const VMF_OR_DWORD: u16 = 0x09F3;
pub const VMF_OR_LWORD: u16 = 0x09F4;
pub const VMF_XOR_BOOL: u16 = 0x0AF0;
pub const VMF_XOR_BYTE: u16 = 0x0AF1;
pub const VMF_XOR_WORD: u16 = 0x0AF2;
pub const VMF_XOR_DWORD: u16 = 0x0AF4;
pub const VMF_XOR_LWORD: u16 = 0x0AF5;
pub const VMF_SHL_BYTE_INT: u16 = 0x0B01;
pub const VMF_SHL_WORD_INT: u16 = 0x0B02;
pub const VMF_SHL_DWORD_INT: u16 = 0x0B03;
pub const VMF_SHL_LWORD_INT: u16 = 0x0B04;
pub const VMF_SHR_BYTE_INT: u16 = 0x0B11;
pub const VMF_SHR_WORD_INT: u16 = 0x0B12;
pub const VMF_SHR_DWORD_INT: u16 = 0x0B13;
pub const VMF_SHR_LWORD_INT: u16 = 0x0B14;
pub const VMF_ROL_BYTE_INT: u16 = 0x0B21;
pub const VMF_ROL_WORD_INT: u16 = 0x0B22;
pub const VMF_ROL_DWORD_INT: u16 = 0x0B23;
pub const VMF_ROL_LWORD_INT: u16 = 0x0B24;
pub const VMF_ROR_BYTE_INT: u16 = 0x0B31;
pub const VMF_ROR_WORD_INT: u16 = 0x0B32;
pub const VMF_ROR_DWORD_INT: u16 = 0x0B33;
pub const VMF_ROR_LWORD_INT: u16 = 0x0B34;
pub const VMF_SEL_BOOL_BOOL_BOOL: u16 = 0x0C00;
pub const VMF_SEL_BOOL_SINT_SINT: u16 = 0x0C01;
pub const VMF_SEL_BOOL_INT_INT: u16 = 0x0C02;
pub const VMF_SEL_BOOL_DINT_DINT: u16 = 0x0C03;
pub const VMF_SEL_BOOL_LINT_LINT: u16 = 0x0C04;
pub const VMF_SEL_BOOL_BYTE_BYTE: u16 = 0x0C05;
pub const VMF_SEL_BOOL_WORD_WORD: u16 = 0x0C06;
pub const VMF_SEL_BOOL_DWORD_DWORD: u16 = 0x0C07;
pub const VMF_SEL_BOOL_LWORD_LWORD: u16 = 0x0C08;
pub const VMF_SEL_BOOL_REAL_REAL: u16 = 0x0C09;
pub const VMF_SEL_BOOL_LREAL_LREAL: u16 = 0x0C0A;
pub const VMF_SEL_BOOL_TIME_TIME: u16 = 0x0C0B;
pub const VMF_SEL_BOOL_DATE_DATE: u16 = 0x0C0C;
pub const VMF_SEL_BOOL_TOD_TOD: u16 = 0x0C0D;
pub const VMF_SEL_BOOL_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x0C0E;
pub const VMF_SEL_BOOL_STRING_STRING: u16 = 0x0C0F;
pub const VMF_SEL_BOOL_WSTRING_WSTRING: u16 = 0x0C0F;

pub const VMF_LIMIT_BOOL_BOOL_BOOL: u16 = 0x0D00;
pub const VMF_LIMIT_SINT_SINT_SINT: u16 = 0x0D01;
pub const VMF_LIMIT_INT_INT_INT: u16 = 0x0D02;
pub const VMF_LIMIT_DINT_DINT_DINT: u16 = 0x0D03;
pub const VMF_LIMIT_LINT_LINT_LINT: u16 = 0x0D04;
pub const VMF_LIMIT_BYTE_BYTE_BYTE: u16 = 0x0D05;
pub const VMF_LIMIT_WORD_WORD_WORD: u16 = 0x0D06;
pub const VMF_LIMIT_DWORD_DWORD_DWORD: u16 = 0x0D07;
pub const VMF_LIMIT_LWORD_LWORD_LWORD: u16 = 0x0D08;
pub const VMF_LIMIT_REAL_REAL_REAL: u16 = 0x0D09;
pub const VMF_LIMIT_LREAL_LREAL_LREAL: u16 = 0x0D0A;
pub const VMF_LIMIT_TIME_TIME_TIME: u16 = 0x0D0B;
pub const VMF_LIMIT_DATE_DATE_DATE: u16 = 0x0D0C;
pub const VMF_LIMIT_TIME_OF_DAY_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x0D0D;
pub const VMF_LIMIT_DATE_AND_TIME_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x0D0E;
pub const VMF_LIMIT_STRING_STRING_STRING: u16 = 0x0D0F;
pub const VMF_LIMIT_WSTRING_WSTRING_WSTRING: u16 = 0x0D0F;

pub const VMF_MAX_BOOL_BOOL: u16 = 0x0E00;
pub const VMF_MAX_SINT_SINT: u16 = 0x0E01;
pub const VMF_MAX_INT_INT: u16 = 0x0E02;
pub const VMF_MAX_DINT_DINT: u16 = 0x0E03;
pub const VMF_MAX_LINT_LINT: u16 = 0x0E04;
pub const VMF_MAX_BYTE_BYTE: u16 = 0x0E05;
pub const VMF_MAX_WORD_WORD: u16 = 0x0E06;
pub const VMF_MAX_DWORD_DWORD: u16 = 0x0E07;
pub const VMF_MAX_LWORD_LWORD: u16 = 0x0E08;
pub const VMF_MAX_REAL_REAL: u16 = 0x0E09;
pub const VMF_MAX_LREAL_LREAL: u16 = 0x0E0A;
pub const VMF_MAX_TIME_TIME: u16 = 0x0E0B;
pub const VMF_MAX_DATE_DATE: u16 = 0x0E0C;
pub const VMF_MAX_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x0E0D;
pub const VMF_MAX_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x0E0E;
pub const VMF_MAX_STRING_STRING: u16 = 0x0E0F;
pub const VMF_MAX_WSTRING_WSTRING: u16 = 0x0E0F;

pub const VMF_MIN_BOOL_BOOL: u16 = 0x0F00;
pub const VMF_MIN_SINT_SINT: u16 = 0x0F01;
pub const VMF_MIN_INT_INT: u16 = 0x0F02;
pub const VMF_MIN_DINT_DINT: u16 = 0x0F03;
pub const VMF_MIN_LINT_LINT: u16 = 0x0F04;
pub const VMF_MIN_BYTE_BYTE: u16 = 0x0F05;
pub const VMF_MIN_WORD_WORD: u16 = 0x0F06;
pub const VMF_MIN_DWORD_DWORD: u16 = 0x0F07;
pub const VMF_MIN_LWORD_LWORD: u16 = 0x0F08;
pub const VMF_MIN_REAL_REAL: u16 = 0x0F09;
pub const VMF_MIN_LREAL_LREAL: u16 = 0x0F0A;
pub const VMF_MIN_TIME_TIME: u16 = 0x0F0B;
pub const VMF_MIN_DATE_DATE: u16 = 0x0F0C;
pub const VMF_MIN_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x0F0D;
pub const VMF_MIN_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x0F0E;
pub const VMF_MIN_STRING_STRING: u16 = 0x0F0F;
pub const VMF_MIN_WSTRING_WSTRING: u16 = 0x0F0F;

pub const VMF_GT_BOOL_BOOL: u16 = 0x1000;
pub const VMF_GT_SINT_SINT: u16 = 0x1001;
pub const VMF_GT_INT_INT: u16 = 0x1002;
pub const VMF_GT_DINT_DINT: u16 = 0x1003;
pub const VMF_GT_LINT_LINT: u16 = 0x1004;
pub const VMF_GT_BYTE_BYTE: u16 = 0x1005;
pub const VMF_GT_WORD_WORD: u16 = 0x1006;
pub const VMF_GT_DWORD_DWORD: u16 = 0x1007;
pub const VMF_GT_LWORD_LWORD: u16 = 0x1008;
pub const VMF_GT_REAL_REAL: u16 = 0x1009;
pub const VMF_GT_LREAL_LREAL: u16 = 0x100A;
pub const VMF_GT_TIME_TIME: u16 = 0x100B;
pub const VMF_GT_DATE_DATE: u16 = 0x100C;
pub const VMF_GT_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x100D;
pub const VMF_GT_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x100E;
pub const VMF_GT_STRING_STRING: u16 = 0x100F;
pub const VMF_GT_WSTRING_WSTRING: u16 = 0x100F;

pub const VMF_GE_BOOL_BOOL: u16 = 0x1100;
pub const VMF_GE_SINT_SINT: u16 = 0x1101;
pub const VMF_GE_INT_INT: u16 = 0x1102;
pub const VMF_GE_DINT_DINT: u16 = 0x1103;
pub const VMF_GE_LINT_LINT: u16 = 0x1104;
pub const VMF_GE_BYTE_BYTE: u16 = 0x1105;
pub const VMF_GE_WORD_WORD: u16 = 0x1106;
pub const VMF_GE_DWORD_DWORD: u16 = 0x1107;
pub const VMF_GE_LWORD_LWORD: u16 = 0x1108;
pub const VMF_GE_REAL_REAL: u16 = 0x1109;
pub const VMF_GE_LREAL_LREAL: u16 = 0x110A;
pub const VMF_GE_TIME_TIME: u16 = 0x110B;
pub const VMF_GE_DATE_DATE: u16 = 0x110C;
pub const VMF_GE_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x110D;
pub const VMF_GE_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x110E;
pub const VMF_GE_STRING_STRING: u16 = 0x110F;
pub const VMF_GE_WSTRING_WSTRING: u16 = 0x110F;

pub const VMF_EQ_BOOL_BOOL: u16 = 0x1200;
pub const VMF_EQ_SINT_SINT: u16 = 0x1201;
pub const VMF_EQ_INT_INT: u16 = 0x1202;
pub const VMF_EQ_DINT_DINT: u16 = 0x1203;
pub const VMF_EQ_LINT_LINT: u16 = 0x1204;
pub const VMF_EQ_BYTE_BYTE: u16 = 0x1205;
pub const VMF_EQ_WORD_WORD: u16 = 0x1206;
pub const VMF_EQ_DWORD_DWORD: u16 = 0x1207;
pub const VMF_EQ_LWORD_LWORD: u16 = 0x1208;
pub const VMF_EQ_REAL_REAL: u16 = 0x1209;
pub const VMF_EQ_LREAL_LREAL: u16 = 0x120A;
pub const VMF_EQ_TIME_TIME: u16 = 0x120B;
pub const VMF_EQ_DATE_DATE: u16 = 0x120C;
pub const VMF_EQ_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x120D;
pub const VMF_EQ_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x120E;
pub const VMF_EQ_STRING_STRING: u16 = 0x120F;
pub const VMF_EQ_WSTRING_WSTRING: u16 = 0x120F;

pub const VMF_LE_BOOL_BOOL: u16 = 0x1300;
pub const VMF_LE_SINT_SINT: u16 = 0x1301;
pub const VMF_LE_INT_INT: u16 = 0x1302;
pub const VMF_LE_DINT_DINT: u16 = 0x1303;
pub const VMF_LE_LINT_LINT: u16 = 0x1304;
pub const VMF_LE_BYTE_BYTE: u16 = 0x1305;
pub const VMF_LE_WORD_WORD: u16 = 0x1306;
pub const VMF_LE_DWORD_DWORD: u16 = 0x1307;
pub const VMF_LE_LWORD_LWORD: u16 = 0x1308;
pub const VMF_LE_REAL_REAL: u16 = 0x1309;
pub const VMF_LE_LREAL_LREAL: u16 = 0x130A;
pub const VMF_LE_TIME_TIME: u16 = 0x130B;
pub const VMF_LE_DATE_DATE: u16 = 0x130C;
pub const VMF_LE_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x130D;
pub const VMF_LE_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x130E;
pub const VMF_LE_STRING_STRING: u16 = 0x130F;
pub const VMF_LE_WSTRING_WSTRING: u16 = 0x130F;

pub const VMF_LT_BOOL_BOOL: u16 = 0x1400;
pub const VMF_LT_SINT_SINT: u16 = 0x1401;
pub const VMF_LT_INT_INT: u16 = 0x1402;
pub const VMF_LT_DINT_DINT: u16 = 0x1403;
pub const VMF_LT_LINT_LINT: u16 = 0x1404;
pub const VMF_LT_BYTE_BYTE: u16 = 0x1405;
pub const VMF_LT_WORD_WORD: u16 = 0x1406;
pub const VMF_LT_DWORD_DWORD: u16 = 0x1407;
pub const VMF_LT_LWORD_LWORD: u16 = 0x1408;
pub const VMF_LT_REAL_REAL: u16 = 0x1409;
pub const VMF_LT_LREAL_LREAL: u16 = 0x140A;
pub const VMF_LT_TIME_TIME: u16 = 0x140B;
pub const VMF_LT_DATE_DATE: u16 = 0x140C;
pub const VMF_LT_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x140D;
pub const VMF_LT_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x140E;
pub const VMF_LT_STRING_STRING: u16 = 0x140F;
pub const VMF_LT_WSTRING_WSTRING: u16 = 0x140F;

pub const VMF_NE_BOOL_BOOL: u16 = 0x1500;
pub const VMF_NE_SINT_SINT: u16 = 0x1501;
pub const VMF_NE_INT_INT: u16 = 0x1502;
pub const VMF_NE_DINT_DINT: u16 = 0x1503;
pub const VMF_NE_LINT_LINT: u16 = 0x1504;
pub const VMF_NE_BYTE_BYTE: u16 = 0x1505;
pub const VMF_NE_WORD_WORD: u16 = 0x1506;
pub const VMF_NE_DWORD_DWORD: u16 = 0x1507;
pub const VMF_NE_LWORD_LWORD: u16 = 0x1508;
pub const VMF_NE_REAL_REAL: u16 = 0x1509;
pub const VMF_NE_LREAL_LREAL: u16 = 0x150A;
pub const VMF_NE_TIME_TIME: u16 = 0x150B;
pub const VMF_NE_DATE_DATE: u16 = 0x150C;
pub const VMF_NE_TIME_OF_DAY_TIME_OF_DAY: u16 = 0x150D;
pub const VMF_NE_DATE_AND_TIME_DATE_AND_TIME: u16 = 0x150E;
pub const VMF_NE_STRING_STRING: u16 = 0x150F;
pub const VMF_NE_WSTRING_WSTRING: u16 = 0x150F;

pub const VMF_MUX_INT_BOOL: u16 = 0x16F0;
pub const VMF_MUX_INT_SINT: u16 = 0x16F1;
pub const VMF_MUX_INT_INT: u16 = 0x16F2;
pub const VMF_MUX_INT_DINT: u16 = 0x16F3;
pub const VMF_MUX_INT_LINT: u16 = 0x16F4;
pub const VMF_MUX_INT_BYTE: u16 = 0x16F5;
pub const VMF_MUX_INT_WORD: u16 = 0x16F6;
pub const VMF_MUX_INT_DWORD: u16 = 0x16F7;
pub const VMF_MUX_INT_LWORD: u16 = 0x16F8;
pub const VMF_MUX_INT_REAL: u16 = 0x16F9;
pub const VMF_MUX_INT_LREAL: u16 = 0x16FA;
pub const VMF_MUX_INT_TIME: u16 = 0x16FB;
pub const VMF_MUX_INT_DATE: u16 = 0x16FC;
pub const VMF_MUX_INT_TIME_OF_DAY: u16 = 0x16FD;
pub const VMF_MUX_INT_DATE_AND_TIME: u16 = 0x16FE;
pub const VMF_MUX_INT_STRING: u16 = 0x16FF;
pub const VMF_MUX_INT_WSTRING: u16 = 0x16FF;

pub const VMF_STRASGN: u16 = 0x1800;
pub const VMF_LEN_STRING_INT: u16 = 0x1801;
pub const VMF_LEN_WSTRING_INT: u16 = 0x1801;

pub const VMF_LEFT_STRING_INT: u16 = 0x1802;
pub const VMF_LEFT_WSTRING_INT: u16 = 0x1802;

pub const VMF_RIGHT_STRING_INT: u16 = 0x1803;
pub const VMF_RIGHT_WSTRING_INT: u16 = 0x1803;

pub const VMF_MID_STRING_INT_INT: u16 = 0x1804;
pub const VMF_MID_WSTRING_INT_INT: u16 = 0x1804;

pub const VMF_INSERT_STRING_STRING_INT: u16 = 0x1805;
pub const VMF_INSERT_WSTRING_WSTRING_INT: u16 = 0x1805;

pub const VMF_DELETE_STRING_INT_INT: u16 = 0x1806;
pub const VMF_DELETE_WSTRING_INT_INT: u16 = 0x1806;

pub const VMF_REPLACE_STRING_STRING_INT_INT: u16 = 0x1807;
pub const VMF_REPLACE_WSTRING_WSTRING_INT_INT: u16 = 0x1807;

pub const VMF_FIND_INT_STRING_STRING: u16 = 0x1808;
pub const VMF_FIND_INT_WSTRING_WSTRING: u16 = 0x1808;

pub const VMF_STRING_TO_REAL: u16 = 0x1809;
pub const VMF_WSTRING_TO_REAL: u16 = 0x1809;

pub const VMF_STRING_TO_INT: u16 = 0x180A;
pub const VMF_WSTRING_TO_INT: u16 = 0x180A;

pub const VMF_INT_TO_STRING: u16 = 0x180B;
pub const VMF_INT_TO_WSTRING: u16 = 0x180B;

pub const VMF_REAL_TO_STRING: u16 = 0x180C;
pub const VMF_REAL_TO_WSTRING: u16 = 0x180C;

pub const VMF_STRING_TO_LREAL: u16 = 0x180D;
pub const VMF_WSTRING_TO_LREAL: u16 = 0x180D;

pub const VMF_STRING_TO_DINT: u16 = 0x180E;
pub const VMF_WSTRING_TO_DINT: u16 = 0x180E;

pub const VMF_DINT_TO_STRING: u16 = 0x180F;
pub const VMF_DINT_TO_WSTRING: u16 = 0x180F;

pub const VMF_LREAL_TO_STRING: u16 = 0x1810;
pub const VMF_LREAL_TO_WSTRING: u16 = 0x1810;

pub const VMF_STRING_TO_LINT: u16 = 0x1811;
pub const VMF_WSTRING_TO_LINT: u16 = 0x1811;

pub const VMF_LINT_TO_STRING: u16 = 0x1812;
pub const VMF_LINT_TO_WSTRING: u16 = 0x1812;

pub const VMF_CHAR_AT_STRING_INT: u16 = 0x1813;

pub const VMF_CHAR_SET_STRING_INT_BYTE: u16 = 0x1814;
pub const VMF_FINDREPLACE_STRING_STRING_INT_INT_STRING: u16 = 0x1815;
pub const VMF_FINDREPLACE_WSTRING_WSTRING_INT_INT_WSTRING: u16 = 0x1815;

pub const VMF_FINDEX_INT_STRING_STRING_INT_INT: u16 = 0x1816;
pub const VMF_FINDEX_INT_WSTRING_WSTRING_INT_INT: u16 = 0x1816;

pub const VMF_WCHAR_AT_WSTRING_INT: u16 = 0x1817;

pub const VMF_WCHAR_SET_WSTRING_INT_WORD: u16 = 0x1818;

pub const VMF_WSTRING_TO_STRING: u16 = 0x1819;
pub const VMF_STRING_TO_WSTRING: u16 = 0x181A;

pub const VMF_INT_TO_REAL: u16 = 0x1900;
pub const VMF_DINT_TO_REAL: u16 = 0x1901;
pub const VMF_TIME_TO_DINT: u16 = 0x1902;
pub const VMF_DINT_TO_TIME: u16 = 0x1903;
pub const VMF_TIME_TO_REAL: u16 = 0x1904;
pub const VMF_REAL_TO_TIME: u16 = 0x1905;
pub const VMF_BCD_TO_INT_BYTE: u16 = 0x1906;
pub const VMF_BCD_TO_INT_WORD: u16 = 0x1907;
pub const VMF_INT_TO_BYTE_BCD: u16 = 0x1908;
pub const VMF_INT_TO_WORD_BCD: u16 = 0x1909;
pub const VMF_REAL_TO_INT: u16 = 0x190A;
pub const VMF_INT_TO_BOOL: u16 = 0x190B;
pub const VMF_INT_TO_DINT: u16 = 0x190C;
pub const VMF_REAL_TO_LREAL: u16 = 0x190D;
pub const VMF_LREAL_TO_REAL: u16 = 0x190E;
pub const VMF_LINT_TO_LREAL: u16 = 0x190F;
pub const VMF_DWORD_TO_REAL: u16 = 0x1910;
pub const VMF_REAL_TO_DWORD: u16 = 0x1911;

pub const VMF_JMP: u16 = 0x1C00;
pub const VMF_JNZ: u16 = 0x1C01;
pub const VMF_JZ: u16 = 0x1C02;
pub const VMF_JR: u16 = 0x1C03;
pub const VMF_JRN: u16 = 0x1C04;
pub const VMF_JRZ: u16 = 0x1C05;

pub const VMF_CALF: u16 = 0x1C10;

pub const VMF_RETURN: u16 = 0x1C13;

pub const VMF_MCD: u16 = 0x1C15;
pub const VMF_CALB: u16 = 0x1C16;
pub const VMF_CUR_TIME: u16 = 0x1C17;
pub const VMF_RETFB: u16 = 0x1C18;

pub const VMF_TRML: u16 = 0x1C1D;
pub const VMF_CALP: u16 = 0x1C1E;
pub const VMF_MEMCP: u16 = 0x1C1F;

pub const VMF_READ_RTC: u16 = 0x1C20;
pub const VMF_WRITE_RTC: u16 = 0x1C21;

pub const VMF_RANDOML: u16 = 0x1C22;

pub const VMF_FPAT: u16 = 0x1C23;
pub const VMF_AURD: u16 = 0x1C24;
pub const VMF_AUWR: u16 = 0x1C25;

pub const VMF_AORD: u16 = 0x1C26;
pub const VMF_AOWR: u16 = 0x1C27;

pub const VMF_ATRD: u16 = 0x1C28;
pub const VMF_ATWR: u16 = 0x1C29;

pub const VMF_AFRD: u16 = 0x1C2A;
pub const VMF_AFWR: u16 = 0x1C2B;

pub const VMF_AQRD: u16 = 0x1C2C;
pub const VMF_AQWR: u16 = 0x1C2D;

pub const VMF_CEAC: u16 = 0x1C2E;

pub const VMF_GET_STATUS_WORD1: u16 = 0x1C2F;
pub const VMF_GET_TST_DATETIME: u16 = 0x1C30;

pub const VMF_TASK_CYCLE: u16 = 0x1C31;
pub const VMF_GET_VMACH_VERSION: u16 = 0x1C32;

pub const VMF_MCD2A: u16 = 0x1C33; // for FPGA
pub const VMF_GARD: u16 = 0x1C34;
pub const VMF_GAWR: u16 = 0x1C35;
pub const VMF_TASKSETUP: u16 = 0x1C36; // for FPGA

pub const VMF_MCD4A: u16 = 0x1C37;
pub const VMF_FPAT4A: u16 = 0x1C38;
pub const VMF_MEMCP4A: u16 = 0x1C39;

/* Pointer instructions */
pub const VMF_DPSTO: u16 = 0x1E00;
pub const VMF_DPRDL: u16 = 0x1E01;
pub const VMF_DPWRL: u16 = 0x1E02;
pub const VMF_CALBR: u16 = 0x1E03;
pub const VMF_JNUL: u16 = 0x1E04;
pub const VMF_MEMCPP: u16 = 0x1E05;

pub const VMF_DPRDL4A: u16 = 0x1E11;
pub const VMF_DPWRL4A: u16 = 0x1E12;
pub const VMF_HWFBC4A: u16 = 0x1E13;
pub const VMF_HWFBI4A: u16 = 0x1E14;

pub const VMF_MMV: u16 = 0x0700;

use core::ptr;

pub trait FromBeBytes: Sized {
    unsafe fn from_be(self) -> Self;
}

impl FromBeBytes for u16 {
    unsafe fn from_be(self) -> Self {
        Self::from_be(self)
    }
}

impl FromBeBytes for u32 {
    unsafe fn from_be(self) -> Self {
        Self::from_be(self)
    }
}

impl FromBeBytes for u64 {
    unsafe fn from_be(self) -> Self {
        Self::from_be(self)
    }
}

impl FromBeBytes for u8 {
    unsafe fn from_be(self) -> Self {
        Self::from_be(self)
    }
}
