//! CCFG - Customer Configuration
//!
//! For details see p. 710 in the cc2650 technical reference manual.
//!
//! Currently setup to use the default settings.

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub CCFG_EXT_LF_CLK : u32,
    pub CCFG_MODE_CONF_1 : u32,
    pub CCFG_SIZE_AND_DIS_FLAGS : u32,
    pub CCFG_MODE_CONF : u32,
    pub CCFG_VOLT_LOAD_0 : u32,
    pub CCFG_VOLT_LOAD_1 : u32,
    pub CCFG_RTC_OFFSET : u32,
    pub CCFG_FREQ_OFFSET : u32,
    pub CCFG_IEEE_MAC_0 : u32,
    pub CCFG_IEEE_MAC_1 : u32,
    pub CCFG_IEEE_BLE_0 : u32,
    pub CCFG_IEEE_BLE_1 : u32,
    pub CCFG_BL_CONFIG : u32,
    pub CCFG_ERASE_CONF : u32,
    pub CCFG_CCFG_TI_OPTIONS : u32,
    pub CCFG_CCFG_TAP_DAP_0 : u32,
    pub CCFG_CCFG_TAP_DAP_1 : u32,
    pub CCFG_IMAGE_VALID_CONF : u32,
    pub CCFG_CCFG_PROT_31_0 : u32,
    pub CCFG_CCFG_PROT_63_32 : u32,
    pub CCFG_CCFG_PROT_95_64 : u32,
    pub CCFG_CCFG_PROT_127_96 : u32,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[no_mangle]
#[link_section = ".ccfg"]
pub static CCFG_CONF
: Struct1
= Struct1 {
    CCFG_EXT_LF_CLK: (0x1u32 << 24i32 | !0xff000000u32) & (0x800000u32 << 0i32 | !0xffffffi32 as (u32)),
    CCFG_MODE_CONF_1: (0x8u32 << 20i32 | !0xf00000i32 as (u32)) & (0x0u32 << 19i32 | !0x80000i32 as (u32)) & (0x4u32 << 16i32 | !0x70000i32 as (u32)) & (0x0u32 << 12i32 | !0xf000i32 as (u32)) & (0x0u32 << 8i32 | !0xf00i32 as (u32)) & (0x10u32 << 0i32 | !0xffi32 as (u32)),
    CCFG_SIZE_AND_DIS_FLAGS: (0x58u32 << 16i32 | !0xffff0000u32) & ((0xfff0i32 >> 4i32) as (u32) << 4i32 | !0xfff0i32 as (u32)) & (0x1u32 << 3i32 | !0x8i32 as (u32)) & (0x1u32 << 2i32 | !0x4i32 as (u32)) & (0x0u32 << 1i32 | !0x2i32 as (u32)) & (0x1u32 << 0i32 | !0x1i32 as (u32)),
    CCFG_MODE_CONF: (0xfu32 << 28i32 | !0xf0000000u32) & (0x0u32 << 27i32 | !0x8000000i32 as (u32)) & (0x0u32 << 26i32 | !0x4000000i32 as (u32)) & (0x1u32 << 25i32 | !0x2000000i32 as (u32)) & (0x1u32 << 24i32 | !0x1000000i32 as (u32)) & (0x2u32 << 22i32 | !0xc00000i32 as (u32)) & (0x1u32 << 21i32 | !0x200000i32 as (u32)) & (0x1u32 << 20i32 | !0x100000i32 as (u32)) & (0x2u32 << 18i32 | !0xc0000i32 as (u32)) & (0x1u32 << 17i32 | !0x20000i32 as (u32)) & (0x1u32 << 16i32 | !0x10000i32 as (u32)) & (0xffu32 << 8i32 | !0xff00i32 as (u32)) & (0x3au32 << 0i32 | !0xffi32 as (u32)),
    CCFG_VOLT_LOAD_0: (0xffu32 << 24i32 | !0xff000000u32) & (0xffu32 << 16i32 | !0xff0000i32 as (u32)) & (0xffu32 << 8i32 | !0xff00i32 as (u32)) & (0xffu32 << 0i32 | !0xffi32 as (u32)),
    CCFG_VOLT_LOAD_1: (0xffu32 << 24i32 | !0xff000000u32) & (0xffu32 << 16i32 | !0xff0000i32 as (u32)) & (0xffu32 << 8i32 | !0xff00i32 as (u32)) & (0xffu32 << 0i32 | !0xffi32 as (u32)),
    CCFG_RTC_OFFSET: (0xffffu32 << 16i32 | !0xffff0000u32) & (0xffu32 << 8i32 | !0xff00i32 as (u32)) & (0xffu32 << 0i32 | !0xffi32 as (u32)),
    CCFG_FREQ_OFFSET: (0xffffu32 << 16i32 | !0xffff0000u32) & (0xffu32 << 8i32 | !0xff00i32 as (u32)) & (0xffu32 << 0i32 | !0xffi32 as (u32)),
    CCFG_IEEE_MAC_0: 0xffffffffu32,
    CCFG_IEEE_MAC_1: 0xffffffffu32,
    CCFG_IEEE_BLE_0: 0xffffffffu32,
    CCFG_IEEE_BLE_1: 0xffffffffu32,
    CCFG_BL_CONFIG: (0x0u32 << 24i32 | !0xff000000u32) & (0x1u32 << 16i32 | !0x10000i32 as (u32)) & (0xffu32 << 8i32 | !0xff00i32 as (u32)) & (0xffu32 << 0i32 | !0xffi32 as (u32)),
    CCFG_ERASE_CONF: (0x1u32 << 8i32 | !0x100i32 as (u32)) & (0x1u32 << 0i32 | !0x1i32 as (u32)),
    CCFG_CCFG_TI_OPTIONS: 0x0u32 << 0i32 | !0xffi32 as (u32),
    CCFG_CCFG_TAP_DAP_0: (0xc5u32 << 16i32 | !0xff0000i32 as (u32)) & (0xc5u32 << 8i32 | !0xff00i32 as (u32)) & (0x0u32 << 0i32 | !0xffi32 as (u32)),
    CCFG_CCFG_TAP_DAP_1: (0x0u32 << 16i32 | !0xff0000i32 as (u32)) & (0x0u32 << 8i32 | !0xff00i32 as (u32)) & (0x0u32 << 0i32 | !0xffi32 as (u32)),
    CCFG_IMAGE_VALID_CONF: 0x0u32,
    CCFG_CCFG_PROT_31_0: 0xffffffffu32,
    CCFG_CCFG_PROT_63_32: 0xffffffffu32,
    CCFG_CCFG_PROT_95_64: 0xffffffffu32,
    CCFG_CCFG_PROT_127_96: 0xffffffffu32
};


/*
pub static CCFG_CONF: [u32; 22] = [
    0x01800000,
    0xFF820010,
    0x0058FFFD,
    0xF3BBFF3A, //0xF3BFFF3A,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0x00FFFFFF,
    0xFFFFFFFF,
    0xFFFFFF00,
    0xFFC5C5C5,
    0xFFC5C5C5,
    0x00000000, // Set image as valid
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
];*/
