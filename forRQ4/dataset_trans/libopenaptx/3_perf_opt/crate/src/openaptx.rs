use core::ffi::*;
pub use crate::src::ffi::*;
pub use crate::src::c_consts::*;
pub use crate::src::c_types::*;

pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_context {
    pub decode_sync_packets: size_t,
    pub decode_dropped: size_t,
    pub channels: [aptx_channel; 2],
    pub hd: uint8_t,
    pub sync_idx: uint8_t,
    pub encode_remaining: uint8_t,
    pub decode_skip_leading: uint8_t,
    pub decode_sync_buffer_len: uint8_t,
    pub decode_sync_buffer: [c_uchar; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_channel {
    pub codeword_history: int32_t,
    pub dither_parity: int32_t,
    pub dither: [int32_t; 4],
    pub qmf: aptx_QMF_analysis,
    pub quantize: [aptx_quantize; 4],
    pub invert_quantize: [aptx_invert_quantize; 4],
    pub prediction: [aptx_prediction; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_prediction {
    pub prev_sign: [int32_t; 2],
    pub s_weight: [int32_t; 2],
    pub d_weight: [int32_t; 24],
    pub pos: int32_t,
    pub reconstructed_differences: [int32_t; 48],
    pub previous_reconstructed_sample: int32_t,
    pub predicted_difference: int32_t,
    pub predicted_sample: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_invert_quantize {
    pub quantization_factor: int32_t,
    pub factor_select: int32_t,
    pub reconstructed_difference: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_quantize {
    pub quantized_sample: int32_t,
    pub quantized_sample_parity_change: int32_t,
    pub error: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_QMF_analysis {
    pub outer_filter_signal: [aptx_filter_signal; 2],
    pub inner_filter_signal: [[aptx_filter_signal; 2]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_filter_signal {
    pub buffer: [int32_t; 32],
    pub pos: uint8_t,
}
pub const NB_CHANNELS: channels = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aptx_tables {
    pub quantize_intervals: *const int32_t,
    pub invert_quantize_dither_factors: *const int32_t,
    pub quantize_dither_factors: *const int32_t,
    pub quantize_factor_select_offset: *const int16_t,
    pub tables_size: c_int,
    pub factor_max: int32_t,
    pub prediction_order: c_int,
}
pub const RIGHT: channels = 1;
pub const LEFT: channels = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub u: uint32_t,
    pub s: int32_t,
}
pub type channels = c_uint;
pub const NULL: *mut c_void = ::core::ptr::null_mut::<c_void>();
#[inline]
fn clip_intp2(mut a: int32_t, mut p: c_uint) -> int32_t { {
    if (a as uint32_t).wrapping_add((1 as c_int as uint32_t) << p)
        & !((2 as c_int as uint32_t) << p).wrapping_sub(1 as uint32_t)
        != 0
    {
        return a >> 31 as c_int ^ ((1 as int32_t) << p) - 1 as int32_t;
    } else {
        return a;
    };
} }
#[inline]
fn clip(mut a: int32_t, mut amin: int32_t, mut amax: int32_t) -> int32_t { {
    if a < amin {
        return amin;
    } else if a > amax {
        return amax;
    } else {
        return a;
    };
} }
#[inline]
fn sign_extend(mut val: int32_t, mut bits: c_uint) -> int32_t { unsafe {
    let shift: c_uint = (8 as usize)
        .wrapping_mul(::core::mem::size_of::<int32_t>() as usize)
        .wrapping_sub(bits as usize) as c_uint;
    let mut v: C2RustUnnamed = C2RustUnnamed { u: 0 };
    v.u = (val as uint32_t) << shift;
    return v.s >> shift;
} }
pub const NB_SUBBANDS: c_int = 4 as c_int;
pub const NB_FILTERS: c_int = 2 as c_int;
pub const FILTER_TAPS: c_int = 16 as c_int;
pub const LATENCY_SAMPLES: c_int = 90 as c_int;
static mut quantize_intervals_LF: [int32_t; 65] = [
    -(9948 as c_int),
    9948 as c_int,
    29860 as c_int,
    49808 as c_int,
    69822 as c_int,
    89926 as c_int,
    110144 as c_int,
    130502 as c_int,
    151026 as c_int,
    171738 as c_int,
    192666 as c_int,
    213832 as c_int,
    235264 as c_int,
    256982 as c_int,
    279014 as c_int,
    301384 as c_int,
    324118 as c_int,
    347244 as c_int,
    370790 as c_int,
    394782 as c_int,
    419250 as c_int,
    444226 as c_int,
    469742 as c_int,
    495832 as c_int,
    522536 as c_int,
    549890 as c_int,
    577936 as c_int,
    606720 as c_int,
    636290 as c_int,
    666700 as c_int,
    698006 as c_int,
    730270 as c_int,
    763562 as c_int,
    797958 as c_int,
    833538 as c_int,
    870398 as c_int,
    908640 as c_int,
    948376 as c_int,
    989740 as c_int,
    1032874 as c_int,
    1077948 as c_int,
    1125150 as c_int,
    1174700 as c_int,
    1226850 as c_int,
    1281900 as c_int,
    1340196 as c_int,
    1402156 as c_int,
    1468282 as c_int,
    1539182 as c_int,
    1615610 as c_int,
    1698514 as c_int,
    1789098 as c_int,
    1888944 as c_int,
    2000168 as c_int,
    2125700 as c_int,
    2269750 as c_int,
    2438670 as c_int,
    2642660 as c_int,
    2899462 as c_int,
    3243240 as c_int,
    3746078 as c_int,
    4535138 as c_int,
    5664098 as c_int,
    7102424 as c_int,
    8897462 as c_int,
];
static mut invert_quantize_dither_factors_LF: [int32_t; 65] = [
    9948 as c_int,
    9948 as c_int,
    9962 as c_int,
    9988 as c_int,
    10026 as c_int,
    10078 as c_int,
    10142 as c_int,
    10218 as c_int,
    10306 as c_int,
    10408 as c_int,
    10520 as c_int,
    10646 as c_int,
    10784 as c_int,
    10934 as c_int,
    11098 as c_int,
    11274 as c_int,
    11462 as c_int,
    11664 as c_int,
    11880 as c_int,
    12112 as c_int,
    12358 as c_int,
    12618 as c_int,
    12898 as c_int,
    13194 as c_int,
    13510 as c_int,
    13844 as c_int,
    14202 as c_int,
    14582 as c_int,
    14988 as c_int,
    15422 as c_int,
    15884 as c_int,
    16380 as c_int,
    16912 as c_int,
    17484 as c_int,
    18098 as c_int,
    18762 as c_int,
    19480 as c_int,
    20258 as c_int,
    21106 as c_int,
    22030 as c_int,
    23044 as c_int,
    24158 as c_int,
    25390 as c_int,
    26760 as c_int,
    28290 as c_int,
    30008 as c_int,
    31954 as c_int,
    34172 as c_int,
    36728 as c_int,
    39700 as c_int,
    43202 as c_int,
    47382 as c_int,
    52462 as c_int,
    58762 as c_int,
    66770 as c_int,
    77280 as c_int,
    91642 as c_int,
    112348 as c_int,
    144452 as c_int,
    199326 as c_int,
    303512 as c_int,
    485546 as c_int,
    643414 as c_int,
    794914 as c_int,
    1000124 as c_int,
];
static mut quantize_dither_factors_LF: [int32_t; 65] = [
    0 as c_int,
    4 as c_int,
    7 as c_int,
    10 as c_int,
    13 as c_int,
    16 as c_int,
    19 as c_int,
    22 as c_int,
    26 as c_int,
    28 as c_int,
    32 as c_int,
    35 as c_int,
    38 as c_int,
    41 as c_int,
    44 as c_int,
    47 as c_int,
    51 as c_int,
    54 as c_int,
    58 as c_int,
    62 as c_int,
    65 as c_int,
    70 as c_int,
    74 as c_int,
    79 as c_int,
    84 as c_int,
    90 as c_int,
    95 as c_int,
    102 as c_int,
    109 as c_int,
    116 as c_int,
    124 as c_int,
    133 as c_int,
    143 as c_int,
    154 as c_int,
    166 as c_int,
    180 as c_int,
    195 as c_int,
    212 as c_int,
    231 as c_int,
    254 as c_int,
    279 as c_int,
    308 as c_int,
    343 as c_int,
    383 as c_int,
    430 as c_int,
    487 as c_int,
    555 as c_int,
    639 as c_int,
    743 as c_int,
    876 as c_int,
    1045 as c_int,
    1270 as c_int,
    1575 as c_int,
    2002 as c_int,
    2628 as c_int,
    3591 as c_int,
    5177 as c_int,
    8026 as c_int,
    13719 as c_int,
    26047 as c_int,
    45509 as c_int,
    39467 as c_int,
    37875 as c_int,
    51303 as c_int,
    0 as c_int,
];
static mut quantize_factor_select_offset_LF: [int16_t; 65] = [
    0 as c_int as int16_t,
    -(21 as c_int) as int16_t,
    -(19 as c_int) as int16_t,
    -(17 as c_int) as int16_t,
    -(15 as c_int) as int16_t,
    -(12 as c_int) as int16_t,
    -(10 as c_int) as int16_t,
    -(8 as c_int) as int16_t,
    -(6 as c_int) as int16_t,
    -(4 as c_int) as int16_t,
    -(1 as c_int) as int16_t,
    1 as c_int as int16_t,
    3 as c_int as int16_t,
    6 as c_int as int16_t,
    8 as c_int as int16_t,
    10 as c_int as int16_t,
    13 as c_int as int16_t,
    15 as c_int as int16_t,
    18 as c_int as int16_t,
    20 as c_int as int16_t,
    23 as c_int as int16_t,
    26 as c_int as int16_t,
    29 as c_int as int16_t,
    31 as c_int as int16_t,
    34 as c_int as int16_t,
    37 as c_int as int16_t,
    40 as c_int as int16_t,
    43 as c_int as int16_t,
    47 as c_int as int16_t,
    50 as c_int as int16_t,
    53 as c_int as int16_t,
    57 as c_int as int16_t,
    60 as c_int as int16_t,
    64 as c_int as int16_t,
    68 as c_int as int16_t,
    72 as c_int as int16_t,
    76 as c_int as int16_t,
    80 as c_int as int16_t,
    85 as c_int as int16_t,
    89 as c_int as int16_t,
    94 as c_int as int16_t,
    99 as c_int as int16_t,
    105 as c_int as int16_t,
    110 as c_int as int16_t,
    116 as c_int as int16_t,
    123 as c_int as int16_t,
    129 as c_int as int16_t,
    136 as c_int as int16_t,
    144 as c_int as int16_t,
    152 as c_int as int16_t,
    161 as c_int as int16_t,
    171 as c_int as int16_t,
    182 as c_int as int16_t,
    194 as c_int as int16_t,
    207 as c_int as int16_t,
    223 as c_int as int16_t,
    241 as c_int as int16_t,
    263 as c_int as int16_t,
    291 as c_int as int16_t,
    328 as c_int as int16_t,
    382 as c_int as int16_t,
    467 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
];
static mut quantize_intervals_MLF: [int32_t; 9] = [
    -(89806 as c_int),
    89806 as c_int,
    278502 as c_int,
    494338 as c_int,
    759442 as c_int,
    1113112 as c_int,
    1652322 as c_int,
    2720256 as c_int,
    5190186 as c_int,
];
static mut invert_quantize_dither_factors_MLF: [int32_t; 9] = [
    89806 as c_int,
    89806 as c_int,
    98890 as c_int,
    116946 as c_int,
    148158 as c_int,
    205512 as c_int,
    333698 as c_int,
    734236 as c_int,
    1735696 as c_int,
];
static mut quantize_dither_factors_MLF: [int32_t; 9] = [
    0 as c_int,
    2271 as c_int,
    4514 as c_int,
    7803 as c_int,
    14339 as c_int,
    32047 as c_int,
    100135 as c_int,
    250365 as c_int,
    0 as c_int,
];
static mut quantize_factor_select_offset_MLF: [int16_t; 9] = [
    0 as c_int as int16_t,
    -(14 as c_int) as int16_t,
    6 as c_int as int16_t,
    29 as c_int as int16_t,
    58 as c_int as int16_t,
    96 as c_int as int16_t,
    154 as c_int as int16_t,
    270 as c_int as int16_t,
    521 as c_int as int16_t,
];
static mut quantize_intervals_MHF: [int32_t; 3] = [
    -(194080 as c_int),
    194080 as c_int,
    890562 as c_int,
];
static mut invert_quantize_dither_factors_MHF: [int32_t; 3] = [
    194080 as c_int,
    194080 as c_int,
    502402 as c_int,
];
static mut quantize_dither_factors_MHF: [int32_t; 3] = [
    0 as c_int,
    77081 as c_int,
    0 as c_int,
];
static mut quantize_factor_select_offset_MHF: [int16_t; 3] = [
    0 as c_int as int16_t,
    -(33 as c_int) as int16_t,
    136 as c_int as int16_t,
];
static mut quantize_intervals_HF: [int32_t; 5] = [
    -(163006 as c_int),
    163006 as c_int,
    542708 as c_int,
    1120554 as c_int,
    2669238 as c_int,
];
static mut invert_quantize_dither_factors_HF: [int32_t; 5] = [
    163006 as c_int,
    163006 as c_int,
    216698 as c_int,
    361148 as c_int,
    1187538 as c_int,
];
static mut quantize_dither_factors_HF: [int32_t; 5] = [
    0 as c_int,
    13423 as c_int,
    36113 as c_int,
    206598 as c_int,
    0 as c_int,
];
static mut quantize_factor_select_offset_HF: [int16_t; 5] = [
    0 as c_int as int16_t,
    -(8 as c_int) as int16_t,
    33 as c_int as int16_t,
    95 as c_int as int16_t,
    262 as c_int as int16_t,
];
static mut hd_quantize_intervals_LF: [int32_t; 257] = [
    -(2436 as c_int),
    2436 as c_int,
    7308 as c_int,
    12180 as c_int,
    17054 as c_int,
    21930 as c_int,
    26806 as c_int,
    31686 as c_int,
    36566 as c_int,
    41450 as c_int,
    46338 as c_int,
    51230 as c_int,
    56124 as c_int,
    61024 as c_int,
    65928 as c_int,
    70836 as c_int,
    75750 as c_int,
    80670 as c_int,
    85598 as c_int,
    90530 as c_int,
    95470 as c_int,
    100418 as c_int,
    105372 as c_int,
    110336 as c_int,
    115308 as c_int,
    120288 as c_int,
    125278 as c_int,
    130276 as c_int,
    135286 as c_int,
    140304 as c_int,
    145334 as c_int,
    150374 as c_int,
    155426 as c_int,
    160490 as c_int,
    165566 as c_int,
    170654 as c_int,
    175756 as c_int,
    180870 as c_int,
    185998 as c_int,
    191138 as c_int,
    196294 as c_int,
    201466 as c_int,
    206650 as c_int,
    211850 as c_int,
    217068 as c_int,
    222300 as c_int,
    227548 as c_int,
    232814 as c_int,
    238096 as c_int,
    243396 as c_int,
    248714 as c_int,
    254050 as c_int,
    259406 as c_int,
    264778 as c_int,
    270172 as c_int,
    275584 as c_int,
    281018 as c_int,
    286470 as c_int,
    291944 as c_int,
    297440 as c_int,
    302956 as c_int,
    308496 as c_int,
    314056 as c_int,
    319640 as c_int,
    325248 as c_int,
    330878 as c_int,
    336532 as c_int,
    342212 as c_int,
    347916 as c_int,
    353644 as c_int,
    359398 as c_int,
    365178 as c_int,
    370986 as c_int,
    376820 as c_int,
    382680 as c_int,
    388568 as c_int,
    394486 as c_int,
    400430 as c_int,
    406404 as c_int,
    412408 as c_int,
    418442 as c_int,
    424506 as c_int,
    430600 as c_int,
    436726 as c_int,
    442884 as c_int,
    449074 as c_int,
    455298 as c_int,
    461554 as c_int,
    467844 as c_int,
    474168 as c_int,
    480528 as c_int,
    486922 as c_int,
    493354 as c_int,
    499820 as c_int,
    506324 as c_int,
    512866 as c_int,
    519446 as c_int,
    526064 as c_int,
    532722 as c_int,
    539420 as c_int,
    546160 as c_int,
    552940 as c_int,
    559760 as c_int,
    566624 as c_int,
    573532 as c_int,
    580482 as c_int,
    587478 as c_int,
    594520 as c_int,
    601606 as c_int,
    608740 as c_int,
    615920 as c_int,
    623148 as c_int,
    630426 as c_int,
    637754 as c_int,
    645132 as c_int,
    652560 as c_int,
    660042 as c_int,
    667576 as c_int,
    675164 as c_int,
    682808 as c_int,
    690506 as c_int,
    698262 as c_int,
    706074 as c_int,
    713946 as c_int,
    721876 as c_int,
    729868 as c_int,
    737920 as c_int,
    746036 as c_int,
    754216 as c_int,
    762460 as c_int,
    770770 as c_int,
    779148 as c_int,
    787594 as c_int,
    796108 as c_int,
    804694 as c_int,
    813354 as c_int,
    822086 as c_int,
    830892 as c_int,
    839774 as c_int,
    848736 as c_int,
    857776 as c_int,
    866896 as c_int,
    876100 as c_int,
    885386 as c_int,
    894758 as c_int,
    904218 as c_int,
    913766 as c_int,
    923406 as c_int,
    933138 as c_int,
    942964 as c_int,
    952886 as c_int,
    962908 as c_int,
    973030 as c_int,
    983254 as c_int,
    993582 as c_int,
    1004020 as c_int,
    1014566 as c_int,
    1025224 as c_int,
    1035996 as c_int,
    1046886 as c_int,
    1057894 as c_int,
    1069026 as c_int,
    1080284 as c_int,
    1091670 as c_int,
    1103186 as c_int,
    1114838 as c_int,
    1126628 as c_int,
    1138558 as c_int,
    1150634 as c_int,
    1162858 as c_int,
    1175236 as c_int,
    1187768 as c_int,
    1200462 as c_int,
    1213320 as c_int,
    1226346 as c_int,
    1239548 as c_int,
    1252928 as c_int,
    1266490 as c_int,
    1280242 as c_int,
    1294188 as c_int,
    1308334 as c_int,
    1322688 as c_int,
    1337252 as c_int,
    1352034 as c_int,
    1367044 as c_int,
    1382284 as c_int,
    1397766 as c_int,
    1413494 as c_int,
    1429478 as c_int,
    1445728 as c_int,
    1462252 as c_int,
    1479058 as c_int,
    1496158 as c_int,
    1513562 as c_int,
    1531280 as c_int,
    1549326 as c_int,
    1567710 as c_int,
    1586446 as c_int,
    1605550 as c_int,
    1625034 as c_int,
    1644914 as c_int,
    1665208 as c_int,
    1685932 as c_int,
    1707108 as c_int,
    1728754 as c_int,
    1750890 as c_int,
    1773542 as c_int,
    1796732 as c_int,
    1820488 as c_int,
    1844840 as c_int,
    1869816 as c_int,
    1895452 as c_int,
    1921780 as c_int,
    1948842 as c_int,
    1976680 as c_int,
    2005338 as c_int,
    2034868 as c_int,
    2065322 as c_int,
    2096766 as c_int,
    2129260 as c_int,
    2162880 as c_int,
    2197708 as c_int,
    2233832 as c_int,
    2271352 as c_int,
    2310384 as c_int,
    2351050 as c_int,
    2393498 as c_int,
    2437886 as c_int,
    2484404 as c_int,
    2533262 as c_int,
    2584710 as c_int,
    2639036 as c_int,
    2696578 as c_int,
    2757738 as c_int,
    2822998 as c_int,
    2892940 as c_int,
    2968278 as c_int,
    3049896 as c_int,
    3138912 as c_int,
    3236760 as c_int,
    3345312 as c_int,
    3467068 as c_int,
    3605434 as c_int,
    3765154 as c_int,
    3952904 as c_int,
    4177962 as c_int,
    4452178 as c_int,
    4787134 as c_int,
    5187290 as c_int,
    5647128 as c_int,
    6159120 as c_int,
    6720518 as c_int,
    7332904 as c_int,
    8000032 as c_int,
    8726664 as c_int,
    9518152 as c_int,
    10380372 as c_int,
];
static mut hd_invert_quantize_dither_factors_LF: [int32_t; 257] = [
    2436 as c_int,
    2436 as c_int,
    2436 as c_int,
    2436 as c_int,
    2438 as c_int,
    2438 as c_int,
    2438 as c_int,
    2440 as c_int,
    2442 as c_int,
    2442 as c_int,
    2444 as c_int,
    2446 as c_int,
    2448 as c_int,
    2450 as c_int,
    2454 as c_int,
    2456 as c_int,
    2458 as c_int,
    2462 as c_int,
    2464 as c_int,
    2468 as c_int,
    2472 as c_int,
    2476 as c_int,
    2480 as c_int,
    2484 as c_int,
    2488 as c_int,
    2492 as c_int,
    2498 as c_int,
    2502 as c_int,
    2506 as c_int,
    2512 as c_int,
    2518 as c_int,
    2524 as c_int,
    2528 as c_int,
    2534 as c_int,
    2540 as c_int,
    2548 as c_int,
    2554 as c_int,
    2560 as c_int,
    2568 as c_int,
    2574 as c_int,
    2582 as c_int,
    2588 as c_int,
    2596 as c_int,
    2604 as c_int,
    2612 as c_int,
    2620 as c_int,
    2628 as c_int,
    2636 as c_int,
    2646 as c_int,
    2654 as c_int,
    2664 as c_int,
    2672 as c_int,
    2682 as c_int,
    2692 as c_int,
    2702 as c_int,
    2712 as c_int,
    2722 as c_int,
    2732 as c_int,
    2742 as c_int,
    2752 as c_int,
    2764 as c_int,
    2774 as c_int,
    2786 as c_int,
    2798 as c_int,
    2810 as c_int,
    2822 as c_int,
    2834 as c_int,
    2846 as c_int,
    2858 as c_int,
    2870 as c_int,
    2884 as c_int,
    2896 as c_int,
    2910 as c_int,
    2924 as c_int,
    2938 as c_int,
    2952 as c_int,
    2966 as c_int,
    2980 as c_int,
    2994 as c_int,
    3010 as c_int,
    3024 as c_int,
    3040 as c_int,
    3056 as c_int,
    3070 as c_int,
    3086 as c_int,
    3104 as c_int,
    3120 as c_int,
    3136 as c_int,
    3154 as c_int,
    3170 as c_int,
    3188 as c_int,
    3206 as c_int,
    3224 as c_int,
    3242 as c_int,
    3262 as c_int,
    3280 as c_int,
    3300 as c_int,
    3320 as c_int,
    3338 as c_int,
    3360 as c_int,
    3380 as c_int,
    3400 as c_int,
    3422 as c_int,
    3442 as c_int,
    3464 as c_int,
    3486 as c_int,
    3508 as c_int,
    3532 as c_int,
    3554 as c_int,
    3578 as c_int,
    3602 as c_int,
    3626 as c_int,
    3652 as c_int,
    3676 as c_int,
    3702 as c_int,
    3728 as c_int,
    3754 as c_int,
    3780 as c_int,
    3808 as c_int,
    3836 as c_int,
    3864 as c_int,
    3892 as c_int,
    3920 as c_int,
    3950 as c_int,
    3980 as c_int,
    4010 as c_int,
    4042 as c_int,
    4074 as c_int,
    4106 as c_int,
    4138 as c_int,
    4172 as c_int,
    4206 as c_int,
    4240 as c_int,
    4276 as c_int,
    4312 as c_int,
    4348 as c_int,
    4384 as c_int,
    4422 as c_int,
    4460 as c_int,
    4500 as c_int,
    4540 as c_int,
    4580 as c_int,
    4622 as c_int,
    4664 as c_int,
    4708 as c_int,
    4752 as c_int,
    4796 as c_int,
    4842 as c_int,
    4890 as c_int,
    4938 as c_int,
    4986 as c_int,
    5036 as c_int,
    5086 as c_int,
    5138 as c_int,
    5192 as c_int,
    5246 as c_int,
    5300 as c_int,
    5358 as c_int,
    5416 as c_int,
    5474 as c_int,
    5534 as c_int,
    5596 as c_int,
    5660 as c_int,
    5726 as c_int,
    5792 as c_int,
    5860 as c_int,
    5930 as c_int,
    6002 as c_int,
    6074 as c_int,
    6150 as c_int,
    6226 as c_int,
    6306 as c_int,
    6388 as c_int,
    6470 as c_int,
    6556 as c_int,
    6644 as c_int,
    6736 as c_int,
    6828 as c_int,
    6924 as c_int,
    7022 as c_int,
    7124 as c_int,
    7228 as c_int,
    7336 as c_int,
    7448 as c_int,
    7562 as c_int,
    7680 as c_int,
    7802 as c_int,
    7928 as c_int,
    8058 as c_int,
    8192 as c_int,
    8332 as c_int,
    8476 as c_int,
    8624 as c_int,
    8780 as c_int,
    8940 as c_int,
    9106 as c_int,
    9278 as c_int,
    9458 as c_int,
    9644 as c_int,
    9840 as c_int,
    10042 as c_int,
    10252 as c_int,
    10472 as c_int,
    10702 as c_int,
    10942 as c_int,
    11194 as c_int,
    11458 as c_int,
    11734 as c_int,
    12024 as c_int,
    12328 as c_int,
    12648 as c_int,
    12986 as c_int,
    13342 as c_int,
    13720 as c_int,
    14118 as c_int,
    14540 as c_int,
    14990 as c_int,
    15466 as c_int,
    15976 as c_int,
    16520 as c_int,
    17102 as c_int,
    17726 as c_int,
    18398 as c_int,
    19124 as c_int,
    19908 as c_int,
    20760 as c_int,
    21688 as c_int,
    22702 as c_int,
    23816 as c_int,
    25044 as c_int,
    26404 as c_int,
    27922 as c_int,
    29622 as c_int,
    31540 as c_int,
    33720 as c_int,
    36222 as c_int,
    39116 as c_int,
    42502 as c_int,
    46514 as c_int,
    51334 as c_int,
    57218 as c_int,
    64536 as c_int,
    73830 as c_int,
    85890 as c_int,
    101860 as c_int,
    123198 as c_int,
    151020 as c_int,
    183936 as c_int,
    216220 as c_int,
    243618 as c_int,
    268374 as c_int,
    293022 as c_int,
    319362 as c_int,
    347768 as c_int,
    378864 as c_int,
    412626 as c_int,
    449596 as c_int,
];
static mut hd_quantize_dither_factors_LF: [int32_t; 256] = [
    0 as c_int,
    0 as c_int,
    0 as c_int,
    1 as c_int,
    0 as c_int,
    0 as c_int,
    1 as c_int,
    1 as c_int,
    0 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    1 as c_int,
    2 as c_int,
    1 as c_int,
    1 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    1 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    2 as c_int,
    3 as c_int,
    2 as c_int,
    3 as c_int,
    2 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    3 as c_int,
    4 as c_int,
    3 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    4 as c_int,
    5 as c_int,
    4 as c_int,
    4 as c_int,
    5 as c_int,
    4 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    5 as c_int,
    6 as c_int,
    5 as c_int,
    5 as c_int,
    6 as c_int,
    5 as c_int,
    6 as c_int,
    6 as c_int,
    6 as c_int,
    6 as c_int,
    6 as c_int,
    6 as c_int,
    6 as c_int,
    6 as c_int,
    7 as c_int,
    6 as c_int,
    7 as c_int,
    7 as c_int,
    7 as c_int,
    7 as c_int,
    7 as c_int,
    7 as c_int,
    7 as c_int,
    7 as c_int,
    7 as c_int,
    8 as c_int,
    8 as c_int,
    8 as c_int,
    8 as c_int,
    8 as c_int,
    8 as c_int,
    8 as c_int,
    9 as c_int,
    9 as c_int,
    9 as c_int,
    9 as c_int,
    9 as c_int,
    9 as c_int,
    9 as c_int,
    10 as c_int,
    10 as c_int,
    10 as c_int,
    10 as c_int,
    10 as c_int,
    11 as c_int,
    11 as c_int,
    11 as c_int,
    11 as c_int,
    11 as c_int,
    12 as c_int,
    12 as c_int,
    12 as c_int,
    12 as c_int,
    13 as c_int,
    13 as c_int,
    13 as c_int,
    14 as c_int,
    14 as c_int,
    14 as c_int,
    15 as c_int,
    15 as c_int,
    15 as c_int,
    15 as c_int,
    16 as c_int,
    16 as c_int,
    17 as c_int,
    17 as c_int,
    17 as c_int,
    18 as c_int,
    18 as c_int,
    18 as c_int,
    19 as c_int,
    19 as c_int,
    20 as c_int,
    21 as c_int,
    21 as c_int,
    22 as c_int,
    22 as c_int,
    23 as c_int,
    23 as c_int,
    24 as c_int,
    25 as c_int,
    26 as c_int,
    26 as c_int,
    27 as c_int,
    28 as c_int,
    29 as c_int,
    30 as c_int,
    31 as c_int,
    32 as c_int,
    33 as c_int,
    34 as c_int,
    35 as c_int,
    36 as c_int,
    37 as c_int,
    39 as c_int,
    40 as c_int,
    42 as c_int,
    43 as c_int,
    45 as c_int,
    47 as c_int,
    49 as c_int,
    51 as c_int,
    53 as c_int,
    55 as c_int,
    58 as c_int,
    60 as c_int,
    63 as c_int,
    66 as c_int,
    69 as c_int,
    73 as c_int,
    76 as c_int,
    80 as c_int,
    85 as c_int,
    89 as c_int,
    95 as c_int,
    100 as c_int,
    106 as c_int,
    113 as c_int,
    119 as c_int,
    128 as c_int,
    136 as c_int,
    146 as c_int,
    156 as c_int,
    168 as c_int,
    182 as c_int,
    196 as c_int,
    213 as c_int,
    232 as c_int,
    254 as c_int,
    279 as c_int,
    307 as c_int,
    340 as c_int,
    380 as c_int,
    425 as c_int,
    480 as c_int,
    545 as c_int,
    626 as c_int,
    724 as c_int,
    847 as c_int,
    1003 as c_int,
    1205 as c_int,
    1471 as c_int,
    1830 as c_int,
    2324 as c_int,
    3015 as c_int,
    3993 as c_int,
    5335 as c_int,
    6956 as c_int,
    8229 as c_int,
    8071 as c_int,
    6850 as c_int,
    6189 as c_int,
    6162 as c_int,
    6585 as c_int,
    7102 as c_int,
    7774 as c_int,
    8441 as c_int,
    9243 as c_int,
];
static mut hd_quantize_factor_select_offset_LF: [int16_t; 257] = [
    0 as c_int as int16_t,
    -(22 as c_int) as int16_t,
    -(21 as c_int) as int16_t,
    -(21 as c_int) as int16_t,
    -(20 as c_int) as int16_t,
    -(20 as c_int) as int16_t,
    -(19 as c_int) as int16_t,
    -(19 as c_int) as int16_t,
    -(18 as c_int) as int16_t,
    -(18 as c_int) as int16_t,
    -(17 as c_int) as int16_t,
    -(17 as c_int) as int16_t,
    -(16 as c_int) as int16_t,
    -(16 as c_int) as int16_t,
    -(15 as c_int) as int16_t,
    -(14 as c_int) as int16_t,
    -(14 as c_int) as int16_t,
    -(13 as c_int) as int16_t,
    -(13 as c_int) as int16_t,
    -(12 as c_int) as int16_t,
    -(12 as c_int) as int16_t,
    -(11 as c_int) as int16_t,
    -(11 as c_int) as int16_t,
    -(10 as c_int) as int16_t,
    -(10 as c_int) as int16_t,
    -(9 as c_int) as int16_t,
    -(9 as c_int) as int16_t,
    -(8 as c_int) as int16_t,
    -(7 as c_int) as int16_t,
    -(7 as c_int) as int16_t,
    -(6 as c_int) as int16_t,
    -(6 as c_int) as int16_t,
    -(5 as c_int) as int16_t,
    -(5 as c_int) as int16_t,
    -(4 as c_int) as int16_t,
    -(4 as c_int) as int16_t,
    -(3 as c_int) as int16_t,
    -(3 as c_int) as int16_t,
    -(2 as c_int) as int16_t,
    -(1 as c_int) as int16_t,
    -(1 as c_int) as int16_t,
    0 as c_int as int16_t,
    0 as c_int as int16_t,
    1 as c_int as int16_t,
    1 as c_int as int16_t,
    2 as c_int as int16_t,
    2 as c_int as int16_t,
    3 as c_int as int16_t,
    4 as c_int as int16_t,
    4 as c_int as int16_t,
    5 as c_int as int16_t,
    5 as c_int as int16_t,
    6 as c_int as int16_t,
    6 as c_int as int16_t,
    7 as c_int as int16_t,
    8 as c_int as int16_t,
    8 as c_int as int16_t,
    9 as c_int as int16_t,
    9 as c_int as int16_t,
    10 as c_int as int16_t,
    11 as c_int as int16_t,
    11 as c_int as int16_t,
    12 as c_int as int16_t,
    12 as c_int as int16_t,
    13 as c_int as int16_t,
    14 as c_int as int16_t,
    14 as c_int as int16_t,
    15 as c_int as int16_t,
    15 as c_int as int16_t,
    16 as c_int as int16_t,
    17 as c_int as int16_t,
    17 as c_int as int16_t,
    18 as c_int as int16_t,
    19 as c_int as int16_t,
    19 as c_int as int16_t,
    20 as c_int as int16_t,
    20 as c_int as int16_t,
    21 as c_int as int16_t,
    22 as c_int as int16_t,
    22 as c_int as int16_t,
    23 as c_int as int16_t,
    24 as c_int as int16_t,
    24 as c_int as int16_t,
    25 as c_int as int16_t,
    26 as c_int as int16_t,
    26 as c_int as int16_t,
    27 as c_int as int16_t,
    28 as c_int as int16_t,
    28 as c_int as int16_t,
    29 as c_int as int16_t,
    30 as c_int as int16_t,
    30 as c_int as int16_t,
    31 as c_int as int16_t,
    32 as c_int as int16_t,
    33 as c_int as int16_t,
    33 as c_int as int16_t,
    34 as c_int as int16_t,
    35 as c_int as int16_t,
    35 as c_int as int16_t,
    36 as c_int as int16_t,
    37 as c_int as int16_t,
    38 as c_int as int16_t,
    38 as c_int as int16_t,
    39 as c_int as int16_t,
    40 as c_int as int16_t,
    41 as c_int as int16_t,
    41 as c_int as int16_t,
    42 as c_int as int16_t,
    43 as c_int as int16_t,
    44 as c_int as int16_t,
    44 as c_int as int16_t,
    45 as c_int as int16_t,
    46 as c_int as int16_t,
    47 as c_int as int16_t,
    48 as c_int as int16_t,
    48 as c_int as int16_t,
    49 as c_int as int16_t,
    50 as c_int as int16_t,
    51 as c_int as int16_t,
    52 as c_int as int16_t,
    52 as c_int as int16_t,
    53 as c_int as int16_t,
    54 as c_int as int16_t,
    55 as c_int as int16_t,
    56 as c_int as int16_t,
    57 as c_int as int16_t,
    58 as c_int as int16_t,
    58 as c_int as int16_t,
    59 as c_int as int16_t,
    60 as c_int as int16_t,
    61 as c_int as int16_t,
    62 as c_int as int16_t,
    63 as c_int as int16_t,
    64 as c_int as int16_t,
    65 as c_int as int16_t,
    66 as c_int as int16_t,
    67 as c_int as int16_t,
    68 as c_int as int16_t,
    69 as c_int as int16_t,
    69 as c_int as int16_t,
    70 as c_int as int16_t,
    71 as c_int as int16_t,
    72 as c_int as int16_t,
    73 as c_int as int16_t,
    74 as c_int as int16_t,
    75 as c_int as int16_t,
    77 as c_int as int16_t,
    78 as c_int as int16_t,
    79 as c_int as int16_t,
    80 as c_int as int16_t,
    81 as c_int as int16_t,
    82 as c_int as int16_t,
    83 as c_int as int16_t,
    84 as c_int as int16_t,
    85 as c_int as int16_t,
    86 as c_int as int16_t,
    87 as c_int as int16_t,
    89 as c_int as int16_t,
    90 as c_int as int16_t,
    91 as c_int as int16_t,
    92 as c_int as int16_t,
    93 as c_int as int16_t,
    94 as c_int as int16_t,
    96 as c_int as int16_t,
    97 as c_int as int16_t,
    98 as c_int as int16_t,
    99 as c_int as int16_t,
    101 as c_int as int16_t,
    102 as c_int as int16_t,
    103 as c_int as int16_t,
    105 as c_int as int16_t,
    106 as c_int as int16_t,
    107 as c_int as int16_t,
    109 as c_int as int16_t,
    110 as c_int as int16_t,
    112 as c_int as int16_t,
    113 as c_int as int16_t,
    115 as c_int as int16_t,
    116 as c_int as int16_t,
    118 as c_int as int16_t,
    119 as c_int as int16_t,
    121 as c_int as int16_t,
    122 as c_int as int16_t,
    124 as c_int as int16_t,
    125 as c_int as int16_t,
    127 as c_int as int16_t,
    129 as c_int as int16_t,
    130 as c_int as int16_t,
    132 as c_int as int16_t,
    134 as c_int as int16_t,
    136 as c_int as int16_t,
    137 as c_int as int16_t,
    139 as c_int as int16_t,
    141 as c_int as int16_t,
    143 as c_int as int16_t,
    145 as c_int as int16_t,
    147 as c_int as int16_t,
    149 as c_int as int16_t,
    151 as c_int as int16_t,
    153 as c_int as int16_t,
    155 as c_int as int16_t,
    158 as c_int as int16_t,
    160 as c_int as int16_t,
    162 as c_int as int16_t,
    164 as c_int as int16_t,
    167 as c_int as int16_t,
    169 as c_int as int16_t,
    172 as c_int as int16_t,
    174 as c_int as int16_t,
    177 as c_int as int16_t,
    180 as c_int as int16_t,
    182 as c_int as int16_t,
    185 as c_int as int16_t,
    188 as c_int as int16_t,
    191 as c_int as int16_t,
    194 as c_int as int16_t,
    197 as c_int as int16_t,
    201 as c_int as int16_t,
    204 as c_int as int16_t,
    208 as c_int as int16_t,
    211 as c_int as int16_t,
    215 as c_int as int16_t,
    219 as c_int as int16_t,
    223 as c_int as int16_t,
    227 as c_int as int16_t,
    232 as c_int as int16_t,
    236 as c_int as int16_t,
    241 as c_int as int16_t,
    246 as c_int as int16_t,
    251 as c_int as int16_t,
    257 as c_int as int16_t,
    263 as c_int as int16_t,
    269 as c_int as int16_t,
    275 as c_int as int16_t,
    283 as c_int as int16_t,
    290 as c_int as int16_t,
    298 as c_int as int16_t,
    307 as c_int as int16_t,
    317 as c_int as int16_t,
    327 as c_int as int16_t,
    339 as c_int as int16_t,
    352 as c_int as int16_t,
    367 as c_int as int16_t,
    384 as c_int as int16_t,
    404 as c_int as int16_t,
    429 as c_int as int16_t,
    458 as c_int as int16_t,
    494 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
    522 as c_int as int16_t,
];
static mut hd_quantize_intervals_MLF: [int32_t; 33] = [
    -(21236 as c_int),
    21236 as c_int,
    63830 as c_int,
    106798 as c_int,
    150386 as c_int,
    194832 as c_int,
    240376 as c_int,
    287258 as c_int,
    335726 as c_int,
    386034 as c_int,
    438460 as c_int,
    493308 as c_int,
    550924 as c_int,
    611696 as c_int,
    676082 as c_int,
    744626 as c_int,
    817986 as c_int,
    896968 as c_int,
    982580 as c_int,
    1076118 as c_int,
    1179278 as c_int,
    1294344 as c_int,
    1424504 as c_int,
    1574386 as c_int,
    1751090 as c_int,
    1966260 as c_int,
    2240868 as c_int,
    2617662 as c_int,
    3196432 as c_int,
    4176450 as c_int,
    5658260 as c_int,
    7671068 as c_int,
    10380372 as c_int,
];
static mut hd_invert_quantize_dither_factors_MLF: [int32_t; 33] = [
    21236 as c_int,
    21236 as c_int,
    21360 as c_int,
    21608 as c_int,
    21978 as c_int,
    22468 as c_int,
    23076 as c_int,
    23806 as c_int,
    24660 as c_int,
    25648 as c_int,
    26778 as c_int,
    28070 as c_int,
    29544 as c_int,
    31228 as c_int,
    33158 as c_int,
    35386 as c_int,
    37974 as c_int,
    41008 as c_int,
    44606 as c_int,
    48934 as c_int,
    54226 as c_int,
    60840 as c_int,
    69320 as c_int,
    80564 as c_int,
    96140 as c_int,
    119032 as c_int,
    155576 as c_int,
    221218 as c_int,
    357552 as c_int,
    622468 as c_int,
    859344 as c_int,
    1153464 as c_int,
    1555840 as c_int,
];
static mut hd_quantize_dither_factors_MLF: [int32_t; 32] = [
    0 as c_int,
    31 as c_int,
    62 as c_int,
    93 as c_int,
    123 as c_int,
    152 as c_int,
    183 as c_int,
    214 as c_int,
    247 as c_int,
    283 as c_int,
    323 as c_int,
    369 as c_int,
    421 as c_int,
    483 as c_int,
    557 as c_int,
    647 as c_int,
    759 as c_int,
    900 as c_int,
    1082 as c_int,
    1323 as c_int,
    1654 as c_int,
    2120 as c_int,
    2811 as c_int,
    3894 as c_int,
    5723 as c_int,
    9136 as c_int,
    16411 as c_int,
    34084 as c_int,
    66229 as c_int,
    59219 as c_int,
    73530 as c_int,
    100594 as c_int,
];
static mut hd_quantize_factor_select_offset_MLF: [int16_t; 33] = [
    0 as c_int as int16_t,
    -(21 as c_int) as int16_t,
    -(16 as c_int) as int16_t,
    -(12 as c_int) as int16_t,
    -(7 as c_int) as int16_t,
    -(2 as c_int) as int16_t,
    3 as c_int as int16_t,
    8 as c_int as int16_t,
    13 as c_int as int16_t,
    19 as c_int as int16_t,
    24 as c_int as int16_t,
    30 as c_int as int16_t,
    36 as c_int as int16_t,
    43 as c_int as int16_t,
    50 as c_int as int16_t,
    57 as c_int as int16_t,
    65 as c_int as int16_t,
    74 as c_int as int16_t,
    83 as c_int as int16_t,
    93 as c_int as int16_t,
    104 as c_int as int16_t,
    117 as c_int as int16_t,
    131 as c_int as int16_t,
    147 as c_int as int16_t,
    166 as c_int as int16_t,
    189 as c_int as int16_t,
    219 as c_int as int16_t,
    259 as c_int as int16_t,
    322 as c_int as int16_t,
    427 as c_int as int16_t,
    521 as c_int as int16_t,
    521 as c_int as int16_t,
    521 as c_int as int16_t,
];
static mut hd_quantize_intervals_MHF: [int32_t; 9] = [
    -(95044 as c_int),
    95044 as c_int,
    295844 as c_int,
    528780 as c_int,
    821332 as c_int,
    1226438 as c_int,
    1890540 as c_int,
    3344850 as c_int,
    6450664 as c_int,
];
static mut hd_invert_quantize_dither_factors_MHF: [int32_t; 9] = [
    95044 as c_int,
    95044 as c_int,
    105754 as c_int,
    127180 as c_int,
    165372 as c_int,
    39736 as c_int,
    424366 as c_int,
    1029946 as c_int,
    2075866 as c_int,
];
static mut hd_quantize_dither_factors_MHF: [int32_t; 8] = [
    0 as c_int,
    2678 as c_int,
    5357 as c_int,
    9548 as c_int,
    -(31409 as c_int),
    96158 as c_int,
    151395 as c_int,
    261480 as c_int,
];
static mut hd_quantize_factor_select_offset_MHF: [int16_t; 9] = [
    0 as c_int as int16_t,
    -(17 as c_int) as int16_t,
    5 as c_int as int16_t,
    30 as c_int as int16_t,
    62 as c_int as int16_t,
    105 as c_int as int16_t,
    177 as c_int as int16_t,
    334 as c_int as int16_t,
    518 as c_int as int16_t,
];
static mut hd_quantize_intervals_HF: [int32_t; 17] = [
    -(45754 as c_int),
    45754 as c_int,
    138496 as c_int,
    234896 as c_int,
    337336 as c_int,
    448310 as c_int,
    570738 as c_int,
    708380 as c_int,
    866534 as c_int,
    1053262 as c_int,
    1281958 as c_int,
    1577438 as c_int,
    1993050 as c_int,
    2665984 as c_int,
    3900982 as c_int,
    5902844 as c_int,
    8897462 as c_int,
];
static mut hd_invert_quantize_dither_factors_HF: [int32_t; 17] = [
    45754 as c_int,
    45754 as c_int,
    46988 as c_int,
    49412 as c_int,
    53026 as c_int,
    57950 as c_int,
    64478 as c_int,
    73164 as c_int,
    84988 as c_int,
    101740 as c_int,
    126958 as c_int,
    168522 as c_int,
    247092 as c_int,
    425842 as c_int,
    809154 as c_int,
    1192708 as c_int,
    1801910 as c_int,
];
static mut hd_quantize_dither_factors_HF: [int32_t; 16] = [
    0 as c_int,
    309 as c_int,
    606 as c_int,
    904 as c_int,
    1231 as c_int,
    1632 as c_int,
    2172 as c_int,
    2956 as c_int,
    4188 as c_int,
    6305 as c_int,
    10391 as c_int,
    19643 as c_int,
    44688 as c_int,
    95828 as c_int,
    95889 as c_int,
    152301 as c_int,
];
static mut hd_quantize_factor_select_offset_HF: [int16_t; 17] = [
    0 as c_int as int16_t,
    -(18 as c_int) as int16_t,
    -(8 as c_int) as int16_t,
    2 as c_int as int16_t,
    13 as c_int as int16_t,
    25 as c_int as int16_t,
    38 as c_int as int16_t,
    53 as c_int as int16_t,
    70 as c_int as int16_t,
    90 as c_int as int16_t,
    115 as c_int as int16_t,
    147 as c_int as int16_t,
    192 as c_int as int16_t,
    264 as c_int as int16_t,
    398 as c_int as int16_t,
    521 as c_int as int16_t,
    521 as c_int as int16_t,
];
static mut all_tables: [[aptx_tables; 4]; 2] = [[aptx_tables {
    quantize_intervals: ::core::ptr::null::<int32_t>(),
    invert_quantize_dither_factors: ::core::ptr::null::<int32_t>(),
    quantize_dither_factors: ::core::ptr::null::<int32_t>(),
    quantize_factor_select_offset: ::core::ptr::null::<int16_t>(),
    tables_size: 0,
    factor_max: 0,
    prediction_order: 0,
}; 4]; 2];
static mut quantization_factors: [int16_t; 32] = [
    2048 as c_int as int16_t,
    2093 as c_int as int16_t,
    2139 as c_int as int16_t,
    2186 as c_int as int16_t,
    2233 as c_int as int16_t,
    2282 as c_int as int16_t,
    2332 as c_int as int16_t,
    2383 as c_int as int16_t,
    2435 as c_int as int16_t,
    2489 as c_int as int16_t,
    2543 as c_int as int16_t,
    2599 as c_int as int16_t,
    2656 as c_int as int16_t,
    2714 as c_int as int16_t,
    2774 as c_int as int16_t,
    2834 as c_int as int16_t,
    2896 as c_int as int16_t,
    2960 as c_int as int16_t,
    3025 as c_int as int16_t,
    3091 as c_int as int16_t,
    3158 as c_int as int16_t,
    3228 as c_int as int16_t,
    3298 as c_int as int16_t,
    3371 as c_int as int16_t,
    3444 as c_int as int16_t,
    3520 as c_int as int16_t,
    3597 as c_int as int16_t,
    3676 as c_int as int16_t,
    3756 as c_int as int16_t,
    3838 as c_int as int16_t,
    3922 as c_int as int16_t,
    4008 as c_int as int16_t,
];
#[inline]
fn rshift32(mut value: int32_t, mut shift: c_uint) -> int32_t { {
    let rounding: int32_t =
        (1 as c_int as int32_t) << shift.wrapping_sub(1 as c_uint);
    let mask: int32_t = ((1 as c_int as int32_t)
        << shift.wrapping_add(1 as c_uint))
        - 1 as int32_t;
    return (value + rounding >> shift) - (value & mask == rounding) as c_int;
} }
#[inline]
fn rshift32_clip24(
    mut value: int32_t,
    mut shift: c_uint,
) -> int32_t { {
    return clip_intp2(rshift32(value, shift), 23 as c_uint);
} }
#[inline]
fn rshift64(mut value: int64_t, mut shift: c_uint) -> int64_t { {
    let rounding: int64_t =
        (1 as c_int as int64_t) << shift.wrapping_sub(1 as c_uint);
    let mask: int64_t = ((1 as c_int as int64_t)
        << shift.wrapping_add(1 as c_uint))
        - 1 as int64_t;
    return (value + rounding >> shift)
        - (value & mask == rounding) as c_int as int64_t;
} }
#[inline]
fn rshift64_clip24(
    mut value: int64_t,
    mut shift: c_uint,
) -> int32_t { {
    return clip_intp2(rshift64(value, shift) as int32_t, 23 as c_uint);
} }
#[inline]
unsafe fn aptx_update_codeword_history(mut channel: *mut aptx_channel) {
    let channel_view: &mut aptx_channel = unsafe { &mut *channel };
    let cw: int32_t = ((channel_view.quantize[0 as c_int as usize].quantized_sample
        & 3 as int32_t)
        << 0 as c_int)
        + ((channel_view.quantize[1 as c_int as usize].quantized_sample & 2 as int32_t)
            << 1 as c_int)
        + ((channel_view.quantize[2 as c_int as usize].quantized_sample & 1 as int32_t)
            << 3 as c_int);
    channel_view.codeword_history = (cw << 8 as c_int)
        + ((channel_view.codeword_history as uint32_t) << 4 as c_int) as int32_t;
}
unsafe fn aptx_generate_dither(mut channel: *mut aptx_channel) {
    let mut subband: c_uint = 0;
    let mut m: int64_t = 0;
    let mut d: int32_t = 0;
    aptx_update_codeword_history(channel);
    m = 5184443 as c_int as int64_t
        * ((*channel).codeword_history >> 7 as c_int) as int64_t;
    d = (m * 4 as int64_t + (m >> 22 as c_int)) as int32_t;
    subband = 0 as c_uint;
    while subband < NB_SUBBANDS as c_uint {
        (*channel).dither[subband as usize] = ((d as uint32_t)
            << (23 as c_uint)
                .wrapping_sub((5 as c_uint).wrapping_mul(subband)))
            as int32_t;
        subband = subband.wrapping_add(1);
    }
    (*channel).dither_parity = d >> 25 as c_int & 1 as int32_t;
}
static mut aptx_qmf_outer_coeffs: [[int32_t; 16]; 2] = [
    [
        730 as c_int,
        -(413 as c_int),
        -(9611 as c_int),
        43626 as c_int,
        -(121026 as c_int),
        269973 as c_int,
        -(585547 as c_int),
        2801966 as c_int,
        697128 as c_int,
        -(160481 as c_int),
        27611 as c_int,
        8478 as c_int,
        -(10043 as c_int),
        3511 as c_int,
        688 as c_int,
        -(897 as c_int),
    ],
    [
        -(897 as c_int),
        688 as c_int,
        3511 as c_int,
        -(10043 as c_int),
        8478 as c_int,
        27611 as c_int,
        -(160481 as c_int),
        697128 as c_int,
        2801966 as c_int,
        -(585547 as c_int),
        269973 as c_int,
        -(121026 as c_int),
        43626 as c_int,
        -(9611 as c_int),
        -(413 as c_int),
        730 as c_int,
    ],
];
static mut aptx_qmf_inner_coeffs: [[int32_t; 16]; 2] = [
    [
        1033 as c_int,
        -(584 as c_int),
        -(13592 as c_int),
        61697 as c_int,
        -(171156 as c_int),
        381799 as c_int,
        -(828088 as c_int),
        3962579 as c_int,
        985888 as c_int,
        -(226954 as c_int),
        39048 as c_int,
        11990 as c_int,
        -(14203 as c_int),
        4966 as c_int,
        973 as c_int,
        -(1268 as c_int),
    ],
    [
        -(1268 as c_int),
        973 as c_int,
        4966 as c_int,
        -(14203 as c_int),
        11990 as c_int,
        39048 as c_int,
        -(226954 as c_int),
        985888 as c_int,
        3962579 as c_int,
        -(828088 as c_int),
        381799 as c_int,
        -(171156 as c_int),
        61697 as c_int,
        -(13592 as c_int),
        -(584 as c_int),
        1033 as c_int,
    ],
];
#[inline]
unsafe fn aptx_qmf_filter_signal_push(
    mut signal: *mut aptx_filter_signal,
    mut sample: int32_t,
) {
    let signal_view: &mut aptx_filter_signal = unsafe { &mut *signal };
    signal_view.buffer[signal_view.pos as usize] = sample;
    signal_view.buffer[(signal_view.pos as c_int + FILTER_TAPS) as usize] = sample;
    signal_view.pos = (signal_view.pos as c_int + 1 as c_int
        & FILTER_TAPS - 1 as c_int) as uint8_t;
}
#[inline]
unsafe fn aptx_qmf_convolution(
    mut signal: *const aptx_filter_signal,
    mut coeffs: *const int32_t,
    mut shift: c_uint,
) -> int32_t {
    let signal_view: &aptx_filter_signal = unsafe { &*signal };
    let mut sig: *const int32_t = (&raw const signal_view.buffer as *const int32_t)
        .offset(signal_view.pos as isize) as *const int32_t;
    let mut e: int64_t = 0 as int64_t;
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < FILTER_TAPS as c_uint {
        e += *sig.offset(i as isize) as int64_t * *coeffs.offset(i as isize) as int64_t;
        i = i.wrapping_add(1);
    }
    return rshift64_clip24(e, shift);
}
#[inline]
unsafe fn aptx_qmf_polyphase_analysis(
    mut signal: *mut aptx_filter_signal,
    mut coeffs: *const [int32_t; 16],
    mut shift: c_uint,
    mut samples: *const int32_t,
    mut low_subband_output: *mut int32_t,
    mut high_subband_output: *mut int32_t,
) {
    let mut subbands: [int32_t; 2] = [0; 2];
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < NB_FILTERS as c_uint {
        aptx_qmf_filter_signal_push(
            signal.offset(i as isize) as *mut aptx_filter_signal,
            *samples.offset(
                ((NB_FILTERS - 1 as c_int) as c_uint).wrapping_sub(i)
                    as isize,
            ),
        );
        subbands[i as usize] = aptx_qmf_convolution(
            signal.offset(i as isize) as *mut aptx_filter_signal,
            &raw const *coeffs.offset(i as isize) as *const int32_t,
            shift,
        );
        i = i.wrapping_add(1);
    }
    *low_subband_output = clip_intp2(
        subbands[0 as c_int as usize] + subbands[1 as c_int as usize],
        23 as c_uint,
    );
    *high_subband_output = clip_intp2(
        subbands[0 as c_int as usize] - subbands[1 as c_int as usize],
        23 as c_uint,
    );
}
unsafe fn aptx_qmf_tree_analysis(
    mut qmf: *mut aptx_QMF_analysis,
    mut samples: *const int32_t,
    mut subband_samples: *mut int32_t,
) {
    let qmf_view: &mut aptx_QMF_analysis = unsafe { &mut *qmf };
    let mut intermediate_samples: [int32_t; 4] = [0; 4];
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < 2 as c_uint {
        aptx_qmf_polyphase_analysis(
            &raw mut qmf_view.outer_filter_signal as *mut aptx_filter_signal,
            &raw const aptx_qmf_outer_coeffs as *const [int32_t; 16],
            23 as c_uint,
            samples.offset((2 as c_uint).wrapping_mul(i) as isize) as *const int32_t,
            (&raw mut intermediate_samples as *mut int32_t)
                .offset((0 as c_uint).wrapping_add(i) as isize)
                as *mut int32_t,
            (&raw mut intermediate_samples as *mut int32_t)
                .offset((2 as c_uint).wrapping_add(i) as isize)
                as *mut int32_t,
        );
        i = i.wrapping_add(1);
    }
    i = 0 as c_uint;
    while i < 2 as c_uint {
        aptx_qmf_polyphase_analysis(
            &raw mut *(&raw mut qmf_view.inner_filter_signal as *mut [aptx_filter_signal; 2])
                .offset(i as isize) as *mut aptx_filter_signal,
            &raw const aptx_qmf_inner_coeffs as *const [int32_t; 16],
            23 as c_uint,
            (&raw mut intermediate_samples as *mut int32_t)
                .offset((2 as c_uint).wrapping_mul(i) as isize)
                as *mut int32_t as *const int32_t,
            subband_samples.offset(
                (2 as c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(0 as c_uint) as isize,
            ) as *mut int32_t,
            subband_samples.offset(
                (2 as c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as c_uint) as isize,
            ) as *mut int32_t,
        );
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe fn aptx_qmf_polyphase_synthesis(
    mut signal: *mut aptx_filter_signal,
    mut coeffs: *const [int32_t; 16],
    mut shift: c_uint,
    mut low_subband_input: int32_t,
    mut high_subband_input: int32_t,
    mut samples: *mut int32_t,
) {
    let mut subbands: [int32_t; 2] = [0; 2];
    let mut i: c_uint = 0;
    subbands[0 as c_int as usize] = low_subband_input + high_subband_input;
    subbands[1 as c_int as usize] = low_subband_input - high_subband_input;
    i = 0 as c_uint;
    while i < NB_FILTERS as c_uint {
        aptx_qmf_filter_signal_push(
            signal.offset(i as isize) as *mut aptx_filter_signal,
            subbands[(1 as c_uint).wrapping_sub(i) as usize],
        );
        *samples.offset(i as isize) = aptx_qmf_convolution(
            signal.offset(i as isize) as *mut aptx_filter_signal,
            &raw const *coeffs.offset(i as isize) as *const int32_t,
            shift,
        );
        i = i.wrapping_add(1);
    }
}
unsafe fn aptx_qmf_tree_synthesis(
    mut qmf: *mut aptx_QMF_analysis,
    mut subband_samples: *const int32_t,
    mut samples: *mut int32_t,
) {
    let qmf_view: &mut aptx_QMF_analysis = unsafe { &mut *qmf };
    let mut intermediate_samples: [int32_t; 4] = [0; 4];
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < 2 as c_uint {
        aptx_qmf_polyphase_synthesis(
            &raw mut *(&raw mut qmf_view.inner_filter_signal as *mut [aptx_filter_signal; 2])
                .offset(i as isize) as *mut aptx_filter_signal,
            &raw const aptx_qmf_inner_coeffs as *const [int32_t; 16],
            22 as c_uint,
            *subband_samples.offset(
                (2 as c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(0 as c_uint) as isize,
            ),
            *subband_samples.offset(
                (2 as c_uint)
                    .wrapping_mul(i)
                    .wrapping_add(1 as c_uint) as isize,
            ),
            (&raw mut intermediate_samples as *mut int32_t)
                .offset((2 as c_uint).wrapping_mul(i) as isize)
                as *mut int32_t,
        );
        i = i.wrapping_add(1);
    }
    i = 0 as c_uint;
    while i < 2 as c_uint {
        aptx_qmf_polyphase_synthesis(
            &raw mut qmf_view.outer_filter_signal as *mut aptx_filter_signal,
            &raw const aptx_qmf_outer_coeffs as *const [int32_t; 16],
            21 as c_uint,
            intermediate_samples[(0 as c_uint).wrapping_add(i) as usize],
            intermediate_samples[(2 as c_uint).wrapping_add(i) as usize],
            samples.offset((2 as c_uint).wrapping_mul(i) as isize) as *mut int32_t,
        );
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe fn aptx_bin_search(
    mut value: int32_t,
    mut factor: int32_t,
    mut intervals: *const int32_t,
    mut nb_intervals: c_int,
) -> int32_t {
    let mut idx: int32_t = 0 as int32_t;
    let mut i: c_int = 0;
    i = nb_intervals >> 1 as c_int;
    while i > 0 as c_int {
        if factor as int64_t * *intervals.offset((idx + i as int32_t) as isize) as int64_t
            <= (value as int64_t) << 24 as c_int
        {
            idx = (idx as c_int + i) as int32_t;
        }
        i >>= 1 as c_int;
    }
    return idx;
}
unsafe fn aptx_quantize_difference(
    mut quantize: *mut aptx_quantize,
    mut sample_difference: int32_t,
    mut dither: int32_t,
    mut quantization_factor: int32_t,
    mut tables: *const aptx_tables,
) {
    let quantize_view: &mut aptx_quantize = unsafe { &mut *quantize };
    let mut intervals: *const int32_t = (*tables).quantize_intervals;
    let mut quantized_sample: int32_t = 0;
    let mut dithered_sample: int32_t = 0;
    let mut parity_change: int32_t = 0;
    let mut d: int32_t = 0;
    let mut mean: int32_t = 0;
    let mut interval: int32_t = 0;
    let mut inv: int32_t = 0;
    let mut sample_difference_abs: int32_t = 0;
    let mut error: int64_t = 0;
    sample_difference_abs = sample_difference;
    if sample_difference_abs < 0 as int32_t {
        sample_difference_abs = -sample_difference_abs;
    }
    if sample_difference_abs
        > ((1 as c_int as int32_t) << 23 as c_int) - 1 as int32_t
    {
        sample_difference_abs =
            ((1 as c_int as int32_t) << 23 as c_int) - 1 as int32_t;
    }
    quantized_sample = aptx_bin_search(
        sample_difference_abs >> 4 as c_int,
        quantization_factor,
        intervals,
        (*tables).tables_size,
    );
    d = rshift32_clip24(
        (dither as int64_t * dither as int64_t >> 32 as c_int) as int32_t,
        7 as c_uint,
    ) - ((1 as c_int as int32_t) << 23 as c_int);
    d = rshift64(
        d as int64_t
            * *(*tables)
                .quantize_dither_factors
                .offset(quantized_sample as isize) as int64_t,
        23 as c_uint,
    ) as int32_t;
    intervals = intervals.offset(quantized_sample as isize);
    mean = (*intervals.offset(1 as c_int as isize)
        + *intervals.offset(0 as c_int as isize))
        / 2 as int32_t;
    interval = (*intervals.offset(1 as c_int as isize)
        - *intervals.offset(0 as c_int as isize))
        * (-((sample_difference < 0 as int32_t) as c_int) | 1 as int32_t);
    dithered_sample = rshift64_clip24(
        dither as int64_t * interval as int64_t
            + ((clip_intp2(mean + d, 23 as c_uint) as int64_t)
                << 32 as c_int),
        32 as c_uint,
    );
    error = ((sample_difference_abs as int64_t) << 20 as c_int)
        - dithered_sample as int64_t * quantization_factor as int64_t;
    quantize_view.error = rshift64(error, 23 as c_uint) as int32_t;
    if quantize_view.error < 0 as int32_t {
        quantize_view.error = -quantize_view.error;
    }
    parity_change = quantized_sample;
    if error < 0 as int64_t {
        quantized_sample -= 1;
    } else {
        parity_change -= 1;
    }
    inv = -((sample_difference < 0 as int32_t) as c_int) as int32_t;
    quantize_view.quantized_sample = quantized_sample ^ inv;
    quantize_view.quantized_sample_parity_change = parity_change ^ inv;
}
unsafe fn aptx_encode_channel(
    mut channel: *mut aptx_channel,
    mut samples: *const int32_t,
    mut hd: c_int,
) {
    let mut subband_samples: [int32_t; 4] = [0; 4];
    let mut diff: int32_t = 0;
    let mut subband: c_uint = 0;
    aptx_qmf_tree_analysis(
        &raw mut (*channel).qmf,
        samples,
        &raw mut subband_samples as *mut int32_t,
    );
    aptx_generate_dither(channel);
    subband = 0 as c_uint;
    while subband < NB_SUBBANDS as c_uint {
        diff = clip_intp2(
            subband_samples[subband as usize]
                - (*channel).prediction[subband as usize].predicted_sample,
            23 as c_uint,
        );
        aptx_quantize_difference(
            (&raw mut (*channel).quantize as *mut aptx_quantize).offset(subband as isize)
                as *mut aptx_quantize,
            diff,
            (*channel).dither[subband as usize],
            (*channel).invert_quantize[subband as usize].quantization_factor,
            (&raw const *(&raw const all_tables as *const [aptx_tables; 4]).offset(hd as isize)
                as *const aptx_tables)
                .offset(subband as isize) as *const aptx_tables,
        );
        subband = subband.wrapping_add(1);
    }
}
unsafe fn aptx_decode_channel(
    mut channel: *mut aptx_channel,
    mut samples: *mut int32_t,
) {
    let channel_view: &mut aptx_channel = unsafe { &mut *channel };
    let mut subband_samples: [int32_t; 4] = [0; 4];
    let mut subband: c_uint = 0;
    subband = 0 as c_uint;
    while subband < NB_SUBBANDS as c_uint {
        subband_samples[subband as usize] =
            channel_view.prediction[subband as usize].previous_reconstructed_sample;
        subband = subband.wrapping_add(1);
    }
    aptx_qmf_tree_synthesis(
        &raw mut channel_view.qmf,
        &raw mut subband_samples as *mut int32_t as *const int32_t,
        samples,
    );
}
unsafe fn aptx_invert_quantization(
    mut invert_quantize: *mut aptx_invert_quantize,
    mut quantized_sample: int32_t,
    mut dither: int32_t,
    mut tables: *const aptx_tables,
) {
    let invert_quantize_view: &mut aptx_invert_quantize = unsafe { &mut *invert_quantize };
    let mut qr: int32_t = 0;
    let mut idx: int32_t = 0;
    let mut shift: int32_t = 0;
    let mut factor_select: int32_t = 0;
    idx = (quantized_sample ^ -((quantized_sample < 0 as int32_t) as c_int))
        + 1 as int32_t;
    qr = *(*tables).quantize_intervals.offset(idx as isize) / 2 as int32_t;
    if quantized_sample < 0 as int32_t {
        qr = -qr;
    }
    qr = rshift64_clip24(
        qr as int64_t * ((1 as c_int as int64_t) << 32 as c_int)
            + dither as int64_t
                * *(*tables)
                    .invert_quantize_dither_factors
                    .offset(idx as isize) as int64_t,
        32 as c_uint,
    );
    invert_quantize_view.reconstructed_difference =
        (invert_quantize_view.quantization_factor as int64_t * qr as int64_t
            >> 19 as c_int) as int32_t;
    factor_select = 32620 as int32_t * invert_quantize_view.factor_select;
    factor_select = rshift32(
        factor_select
            + *(*tables).quantize_factor_select_offset.offset(idx as isize) as int32_t
                * ((1 as int32_t) << 15 as c_int),
        15 as c_uint,
    );
    invert_quantize_view.factor_select = clip(factor_select, 0 as int32_t, (*tables).factor_max);
    idx = (invert_quantize_view.factor_select & 0xff as int32_t) >> 3 as c_int;
    shift = (*tables).factor_max - invert_quantize_view.factor_select >> 8 as c_int;
    invert_quantize_view.quantization_factor =
        ((quantization_factors[idx as usize] as c_int) << 11 as c_int
            >> shift) as int32_t;
}
unsafe fn aptx_reconstructed_differences_update(
    mut prediction: *mut aptx_prediction,
    mut reconstructed_difference: int32_t,
    mut order: c_int,
) -> *mut int32_t {
    let prediction_view: &mut aptx_prediction = unsafe { &mut *prediction };
    let mut rd1: *mut int32_t = &raw mut prediction_view.reconstructed_differences as *mut int32_t;
    let mut rd2: *mut int32_t = rd1.offset(order as isize);
    let mut p: c_int = prediction_view.pos as c_int;
    *rd1.offset(p as isize) = *rd2.offset(p as isize);
    p = (p + 1 as c_int) % order;
    prediction_view.pos = p as int32_t;
    *rd2.offset(p as isize) = reconstructed_difference;
    return rd2.offset(p as isize) as *mut int32_t;
}
// Applied rules: [III④, C3, C1]
// Skipped rules: [III③: Hit [4] is a call to aptx_reconstructed_differences_update, not a visible libc/custom bytewise memory primitive in this function body; without inspecting and inlining that callee body, III③ cannot address a concrete in-function memory-op site here.]
unsafe fn aptx_prediction_filtering(
    mut prediction: *mut aptx_prediction,
    mut reconstructed_difference: int32_t,
    mut order: c_int,
) {
    let mut reconstructed_sample: int32_t = 0;
    let mut predictor: int32_t = 0;
    let mut srd0: int32_t = 0;
    let mut srd: int32_t = 0;
    let mut reconstructed_differences: *mut int32_t = ::core::ptr::null_mut::<int32_t>();
    let mut predicted_difference: int64_t = 0 as int64_t;
    let mut i: c_int = 0;

    let prediction_ref = &mut *prediction;
    let s_weight0 = prediction_ref.s_weight[0];
    let s_weight1 = prediction_ref.s_weight[1];
    let previous_reconstructed_sample = prediction_ref.previous_reconstructed_sample;
    let predicted_sample_in = prediction_ref.predicted_sample;

    reconstructed_sample = clip_intp2(
        reconstructed_difference + predicted_sample_in,
        23 as c_uint,
    );
    predictor = clip_intp2(
        ((s_weight0 as int64_t * previous_reconstructed_sample as int64_t
            + s_weight1 as int64_t * reconstructed_sample as int64_t
            >> 22 as c_int) as int32_t),
        23 as c_uint,
    );
    prediction_ref.previous_reconstructed_sample = reconstructed_sample;
    reconstructed_differences =
        aptx_reconstructed_differences_update(prediction, reconstructed_difference, order);
    srd0 = ((reconstructed_difference > 0 as int32_t) as c_int
        - (reconstructed_difference < 0 as int32_t) as c_int) as int32_t
        * ((1 as c_int as int32_t) << 23 as c_int);

    let order_usize = order as usize;
    let d_weight = &mut prediction_ref.d_weight[..order_usize];
    // SAFETY: `aptx_reconstructed_differences_update` returns a cursor into the
    // `prediction_ref.reconstructed_differences` ring buffer. This loop accesses
    // `reconstructed_differences[-i - 1]` and `reconstructed_differences[-i]`
    // for `i in 0..order`, so the required readable region is exactly the
    // `order + 1` elements ending at `reconstructed_differences`. The pointer is
    // non-null by the callee contract, and this shared slice does not alias the
    // distinct mutable borrow of `d_weight`.
    let reconstructed_differences_view = unsafe {
        core::slice::from_raw_parts(
            reconstructed_differences.sub(order_usize),
            order_usize + 1,
        )
    };

    i = 0 as c_int;
    while i < order {
        let idx = i as usize;
        // SAFETY: `idx < order_usize` by the loop bound `i < order`, and
        // `d_weight` was sliced to length `order_usize`.
        let d_weight_i = unsafe { d_weight.get_unchecked_mut(idx) };
        let rd_prev = reconstructed_differences_view[idx];
        let rd_cur = reconstructed_differences_view[idx + 1];

        srd = rd_prev >> 31 as c_int | 1 as int32_t;
        *d_weight_i -= rshift32(*d_weight_i - srd * srd0, 8 as c_uint);
        predicted_difference += rd_cur as int64_t * *d_weight_i as int64_t;
        i += 1;
    }
    prediction_ref.predicted_difference = clip_intp2(
        (predicted_difference >> 22 as c_int) as int32_t,
        23 as c_uint,
    );
    prediction_ref.predicted_sample = clip_intp2(
        predictor + prediction_ref.predicted_difference,
        23 as c_uint,
    );
}
unsafe fn aptx_process_subband(
    mut invert_quantize: *mut aptx_invert_quantize,
    mut prediction: *mut aptx_prediction,
    mut quantized_sample: int32_t,
    mut dither: int32_t,
    mut tables: *const aptx_tables,
) {
    let tables_view: &aptx_tables = unsafe { &*tables };
    let mut sign: int32_t = 0;
    let mut same_sign: [int32_t; 2] = [0; 2];
    let mut weight: [int32_t; 2] = [0; 2];
    let mut sw1: int32_t = 0;
    let mut range: int32_t = 0;
    aptx_invert_quantization(invert_quantize, quantized_sample, dither, tables);
    sign = (((*invert_quantize).reconstructed_difference > -(*prediction).predicted_difference)
        as c_int
        - ((*invert_quantize).reconstructed_difference < -(*prediction).predicted_difference)
            as c_int) as int32_t;
    same_sign[0 as c_int as usize] =
        sign * (*prediction).prev_sign[0 as c_int as usize];
    same_sign[1 as c_int as usize] =
        sign * (*prediction).prev_sign[1 as c_int as usize];
    (*prediction).prev_sign[0 as c_int as usize] =
        (*prediction).prev_sign[1 as c_int as usize];
    (*prediction).prev_sign[1 as c_int as usize] = sign | 1 as int32_t;
    range = 0x100000 as c_int as int32_t;
    sw1 = rshift32(
        -same_sign[1 as c_int as usize]
            * (*prediction).s_weight[1 as c_int as usize],
        1 as c_uint,
    );
    sw1 = (clip(sw1, -range, range) & !(0xf as int32_t)) * 16 as int32_t;
    range = 0x300000 as c_int as int32_t;
    weight[0 as c_int as usize] = 254 as int32_t
        * (*prediction).s_weight[0 as c_int as usize]
        + 0x800000 as int32_t * same_sign[0 as c_int as usize]
        + sw1;
    (*prediction).s_weight[0 as c_int as usize] = clip(
        rshift32(
            weight[0 as c_int as usize],
            8 as c_uint,
        ),
        -range,
        range,
    );
    range = 0x3c0000 as int32_t - (*prediction).s_weight[0 as c_int as usize];
    weight[1 as c_int as usize] = 255 as int32_t
        * (*prediction).s_weight[1 as c_int as usize]
        + 0xc00000 as int32_t * same_sign[1 as c_int as usize];
    (*prediction).s_weight[1 as c_int as usize] = clip(
        rshift32(
            weight[1 as c_int as usize],
            8 as c_uint,
        ),
        -range,
        range,
    );
    aptx_prediction_filtering(
        prediction,
        (*invert_quantize).reconstructed_difference,
        tables_view.prediction_order,
    );
}
unsafe fn aptx_invert_quantize_and_prediction(
    mut channel: *mut aptx_channel,
    mut hd: c_int,
) {
    let channel_view: &mut aptx_channel = unsafe { &mut *channel };
    let mut subband: c_uint = 0;
    subband = 0 as c_uint;
    while subband < NB_SUBBANDS as c_uint {
        aptx_process_subband(
            (&raw mut channel_view.invert_quantize as *mut aptx_invert_quantize)
                .offset(subband as isize) as *mut aptx_invert_quantize,
            (&raw mut channel_view.prediction as *mut aptx_prediction).offset(subband as isize)
                as *mut aptx_prediction,
            channel_view.quantize[subband as usize].quantized_sample,
            channel_view.dither[subband as usize],
            (&raw const *(&raw const all_tables as *const [aptx_tables; 4]).offset(hd as isize)
                as *const aptx_tables)
                .offset(subband as isize) as *const aptx_tables,
        );
        subband = subband.wrapping_add(1);
    }
}
unsafe fn aptx_quantized_parity(mut channel: *const aptx_channel) -> int32_t {
    let channel_view: &aptx_channel = unsafe { &*channel };
    let mut parity: int32_t = channel_view.dither_parity;
    let mut subband: c_uint = 0;
    subband = 0 as c_uint;
    while subband < NB_SUBBANDS as c_uint {
        parity ^= channel_view.quantize[subband as usize].quantized_sample;
        subband = subband.wrapping_add(1);
    }
    return parity & 1 as int32_t;
}
unsafe fn aptx_check_parity(
    mut channels: *const aptx_channel,
    mut sync_idx: *mut uint8_t,
) -> c_int {
    let sync_idx_view: &mut uint8_t = unsafe { &mut *sync_idx };
    let parity: int32_t = aptx_quantized_parity(
        channels.offset(LEFT as c_int as isize) as *const aptx_channel,
    ) as int32_t
        ^ aptx_quantized_parity(
            channels.offset(RIGHT as c_int as isize) as *const aptx_channel
        ) as int32_t;
    let eighth: int32_t =
        (*sync_idx_view as c_int == 7 as c_int) as c_int;
    *sync_idx_view = (*sync_idx_view as c_int + 1 as c_int
        & 7 as c_int) as uint8_t;
    return parity as c_int ^ eighth as c_int;
}
unsafe fn aptx_insert_sync(mut channels: *mut aptx_channel, mut sync_idx: *mut uint8_t) {
    let mut i: c_uint = 0;
    let mut c: *mut aptx_channel = ::core::ptr::null_mut::<aptx_channel>();
    static mut map: [c_uint; 4] = [
        1 as c_int as c_uint,
        2 as c_int as c_uint,
        0 as c_int as c_uint,
        3 as c_int as c_uint,
    ];
    let mut min: *mut aptx_quantize = (&raw mut (*channels
        .offset((NB_CHANNELS as c_int - 1 as c_int) as isize))
    .quantize as *mut aptx_quantize)
        .offset(
            *(&raw const map as *const c_uint).offset(0 as c_int as isize)
                as isize,
        ) as *mut aptx_quantize;
    if aptx_check_parity(channels as *const aptx_channel, sync_idx) != 0 {
        c = channels.offset((NB_CHANNELS as c_int - 1 as c_int) as isize)
            as *mut aptx_channel;
        while c >= channels {
            i = 0 as c_uint;
            while i < NB_SUBBANDS as c_uint {
                if (*c).quantize[map[i as usize] as usize].error < (*min).error {
                    min = (&raw mut (*c).quantize as *mut aptx_quantize).offset(
                        *(&raw const map as *const c_uint).offset(i as isize) as isize,
                    ) as *mut aptx_quantize;
                }
                i = i.wrapping_add(1);
            }
            c = c.offset(-1);
        }
        (*min).quantized_sample = (*min).quantized_sample_parity_change;
    }
}
unsafe fn aptx_pack_codeword(mut channel: *const aptx_channel) -> uint16_t {
    let channel_view: &aptx_channel = unsafe { &*channel };
    let parity: int32_t = aptx_quantized_parity(channel) as int32_t;
    return ((channel_view.quantize[3 as c_int as usize].quantized_sample
        & 0x6 as int32_t
        | parity)
        << 13 as c_int
        | (channel_view.quantize[2 as c_int as usize].quantized_sample & 0x3 as int32_t)
            << 11 as c_int
        | (channel_view.quantize[1 as c_int as usize].quantized_sample & 0xf as int32_t)
            << 7 as c_int
        | (channel_view.quantize[0 as c_int as usize].quantized_sample
            & 0x7f as int32_t)
            << 0 as c_int) as uint16_t;
}
unsafe fn aptxhd_pack_codeword(mut channel: *const aptx_channel) -> uint32_t {
    let channel_view: &aptx_channel = unsafe { &*channel };
    let parity: int32_t = aptx_quantized_parity(channel) as int32_t;
    return ((channel_view.quantize[3 as c_int as usize].quantized_sample
        & 0x1e as int32_t
        | parity)
        << 19 as c_int
        | (channel_view.quantize[2 as c_int as usize].quantized_sample & 0xf as int32_t)
            << 15 as c_int
        | (channel_view.quantize[1 as c_int as usize].quantized_sample
            & 0x3f as int32_t)
            << 9 as c_int
        | (channel_view.quantize[0 as c_int as usize].quantized_sample
            & 0x1ff as int32_t)
            << 0 as c_int) as uint32_t;
}
unsafe fn aptx_unpack_codeword(mut channel: *mut aptx_channel, mut codeword: uint16_t) {
    (*channel).quantize[0 as c_int as usize].quantized_sample = sign_extend(
        codeword as int32_t >> 0 as c_int,
        7 as c_uint,
    );
    (*channel).quantize[1 as c_int as usize].quantized_sample = sign_extend(
        codeword as int32_t >> 7 as c_int,
        4 as c_uint,
    );
    (*channel).quantize[2 as c_int as usize].quantized_sample = sign_extend(
        codeword as int32_t >> 11 as c_int,
        2 as c_uint,
    );
    (*channel).quantize[3 as c_int as usize].quantized_sample = sign_extend(
        codeword as int32_t >> 13 as c_int,
        3 as c_uint,
    );
    (*channel).quantize[3 as c_int as usize].quantized_sample =
        (*channel).quantize[3 as c_int as usize].quantized_sample & !(1 as int32_t)
            | aptx_quantized_parity(channel);
}
unsafe fn aptxhd_unpack_codeword(
    mut channel: *mut aptx_channel,
    mut codeword: uint32_t,
) {
    (*channel).quantize[0 as c_int as usize].quantized_sample = sign_extend(
        (codeword >> 0 as c_int) as int32_t,
        9 as c_uint,
    );
    (*channel).quantize[1 as c_int as usize].quantized_sample = sign_extend(
        (codeword >> 9 as c_int) as int32_t,
        6 as c_uint,
    );
    (*channel).quantize[2 as c_int as usize].quantized_sample = sign_extend(
        (codeword >> 15 as c_int) as int32_t,
        4 as c_uint,
    );
    (*channel).quantize[3 as c_int as usize].quantized_sample = sign_extend(
        (codeword >> 19 as c_int) as int32_t,
        5 as c_uint,
    );
    (*channel).quantize[3 as c_int as usize].quantized_sample =
        (*channel).quantize[3 as c_int as usize].quantized_sample & !(1 as int32_t)
            | aptx_quantized_parity(channel);
}
unsafe fn aptx_encode_samples(
    mut ctx: *mut aptx_context,
    mut samples: *mut [int32_t; 4],
    mut output: *mut uint8_t,
) {
    let ctx_view: &mut aptx_context = unsafe { &mut *ctx };
    let mut channel: c_uint = 0;
    channel = 0 as c_uint;
    while channel < NB_CHANNELS as c_int as c_uint {
        aptx_encode_channel(
            (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                as *mut aptx_channel,
            &raw mut *samples.offset(channel as isize) as *mut int32_t as *const int32_t,
            ctx_view.hd as c_int,
        );
        channel = channel.wrapping_add(1);
    }
    aptx_insert_sync(
        &raw mut ctx_view.channels as *mut aptx_channel,
        &raw mut ctx_view.sync_idx,
    );
    channel = 0 as c_uint;
    while channel < NB_CHANNELS as c_int as c_uint {
        aptx_invert_quantize_and_prediction(
            (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                as *mut aptx_channel,
            ctx_view.hd as c_int,
        );
        if ctx_view.hd != 0 {
            let mut codeword: uint32_t = aptxhd_pack_codeword(
                (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                    as *mut aptx_channel,
            );
            *output.offset(
                (3 as c_uint)
                    .wrapping_mul(channel)
                    .wrapping_add(0 as c_uint) as isize,
            ) = (codeword >> 16 as c_int & 0xff as uint32_t) as uint8_t;
            *output.offset(
                (3 as c_uint)
                    .wrapping_mul(channel)
                    .wrapping_add(1 as c_uint) as isize,
            ) = (codeword >> 8 as c_int & 0xff as uint32_t) as uint8_t;
            *output.offset(
                (3 as c_uint)
                    .wrapping_mul(channel)
                    .wrapping_add(2 as c_uint) as isize,
            ) = (codeword >> 0 as c_int & 0xff as uint32_t) as uint8_t;
        } else {
            let mut codeword_0: uint16_t = aptx_pack_codeword(
                (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                    as *mut aptx_channel,
            );
            *output.offset(
                (2 as c_uint)
                    .wrapping_mul(channel)
                    .wrapping_add(0 as c_uint) as isize,
            ) = (codeword_0 as c_int >> 8 as c_int
                & 0xff as c_int) as uint8_t;
            *output.offset(
                (2 as c_uint)
                    .wrapping_mul(channel)
                    .wrapping_add(1 as c_uint) as isize,
            ) = (codeword_0 as c_int >> 0 as c_int
                & 0xff as c_int) as uint8_t;
        }
        channel = channel.wrapping_add(1);
    }
}
unsafe fn aptx_decode_samples(
    mut ctx: *mut aptx_context,
    mut input: *const uint8_t,
    mut samples: *mut [int32_t; 4],
) -> c_int {
    let ctx_view: &mut aptx_context = unsafe { &mut *ctx };
    let mut channel: c_uint = 0;
    let mut ret: c_int = 0;
    channel = 0 as c_uint;
    while channel < NB_CHANNELS as c_int as c_uint {
        aptx_generate_dither(
            (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                as *mut aptx_channel,
        );
        if ctx_view.hd != 0 {
            aptxhd_unpack_codeword(
                (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                    as *mut aptx_channel,
                (*input.offset(
                    (3 as c_uint)
                        .wrapping_mul(channel)
                        .wrapping_add(0 as c_uint) as isize,
                ) as uint32_t)
                    << 16 as c_int
                    | (*input.offset(
                        (3 as c_uint)
                            .wrapping_mul(channel)
                            .wrapping_add(1 as c_uint)
                            as isize,
                    ) as uint32_t)
                        << 8 as c_int
                    | (*input.offset(
                        (3 as c_uint)
                            .wrapping_mul(channel)
                            .wrapping_add(2 as c_uint)
                            as isize,
                    ) as uint32_t)
                        << 0 as c_int,
            );
        } else {
            aptx_unpack_codeword(
                (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                    as *mut aptx_channel,
                ((*input.offset(
                    (2 as c_uint)
                        .wrapping_mul(channel)
                        .wrapping_add(0 as c_uint) as isize,
                ) as uint16_t as c_int)
                    << 8 as c_int
                    | (*input.offset(
                        (2 as c_uint)
                            .wrapping_mul(channel)
                            .wrapping_add(1 as c_uint)
                            as isize,
                    ) as uint16_t as c_int)
                        << 0 as c_int) as uint16_t,
            );
        }
        aptx_invert_quantize_and_prediction(
            (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                as *mut aptx_channel,
            ctx_view.hd as c_int,
        );
        channel = channel.wrapping_add(1);
    }
    ret = aptx_check_parity(
        &raw mut ctx_view.channels as *mut aptx_channel as *const aptx_channel,
        &raw mut ctx_view.sync_idx,
    );
    channel = 0 as c_uint;
    while channel < NB_CHANNELS as c_int as c_uint {
        aptx_decode_channel(
            (&raw mut ctx_view.channels as *mut aptx_channel).offset(channel as isize)
                as *mut aptx_channel,
            &raw mut *samples.offset(channel as isize) as *mut int32_t,
        );
        channel = channel.wrapping_add(1);
    }
    return ret;
}
unsafe fn aptx_reset_decode_sync(mut ctx: *mut aptx_context) {
    let decode_dropped: size_t = (*ctx).decode_dropped;
    let decode_sync_packets: size_t = (*ctx).decode_sync_packets;
    let decode_sync_buffer_len: uint8_t = (*ctx).decode_sync_buffer_len;
    let mut decode_sync_buffer: [c_uchar; 6] = [0; 6];
    let mut i: c_uint = 0;
    i = 0 as c_uint;
    while i < 6 as c_uint {
        decode_sync_buffer[i as usize] = (*ctx).decode_sync_buffer[i as usize];
        i = i.wrapping_add(1);
    }
    aptx_reset(ctx);
    i = 0 as c_uint;
    while i < 6 as c_uint {
        (*ctx).decode_sync_buffer[i as usize] = decode_sync_buffer[i as usize];
        i = i.wrapping_add(1);
    }
    (*ctx).decode_sync_buffer_len = decode_sync_buffer_len;
    (*ctx).decode_sync_packets = decode_sync_packets;
    (*ctx).decode_dropped = decode_dropped;
}
#[no_mangle]
pub static mut aptx_major: c_int = OPENAPTX_MAJOR;
#[no_mangle]
pub static mut aptx_minor: c_int = OPENAPTX_MINOR;
#[no_mangle]
pub static mut aptx_patch: c_int = OPENAPTX_PATCH;
#[inline]
pub fn aptx_init(mut hd: c_int) -> *mut aptx_context { unsafe {
    let mut ctx: *mut aptx_context = ::core::ptr::null_mut::<aptx_context>();
    ctx = malloc(::core::mem::size_of::<aptx_context>() as size_t) as *mut aptx_context;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<aptx_context>();
    }
    (*ctx).hd = (if hd != 0 {
        1 as c_int
    } else {
        0 as c_int
    }) as uint8_t;
    aptx_reset(ctx);
    return ctx;
} }
#[inline]
pub unsafe fn aptx_reset(mut ctx: *mut aptx_context) {
    let hd: uint8_t = (*ctx).hd;
    let mut i: c_uint = 0;
    let mut chan: c_uint = 0;
    let mut subband: c_uint = 0;
    let mut channel: *mut aptx_channel = ::core::ptr::null_mut::<aptx_channel>();
    let mut prediction: *mut aptx_prediction = ::core::ptr::null_mut::<aptx_prediction>();
    i = 0 as c_uint;
    while (i as usize) < ::core::mem::size_of::<aptx_context>() as usize {
        *(ctx as *mut c_uchar).offset(i as isize) = 0 as c_uchar;
        i = i.wrapping_add(1);
    }
    (*ctx).hd = hd;
    (*ctx).decode_skip_leading =
        ((LATENCY_SAMPLES + 3 as c_int) / 4 as c_int) as uint8_t;
    (*ctx).encode_remaining =
        ((LATENCY_SAMPLES + 3 as c_int) / 4 as c_int) as uint8_t;
    chan = 0 as c_uint;
    while chan < NB_CHANNELS as c_int as c_uint {
        channel = (&raw mut (*ctx).channels as *mut aptx_channel).offset(chan as isize)
            as *mut aptx_channel;
        subband = 0 as c_uint;
        while subband < NB_SUBBANDS as c_uint {
            prediction = (&raw mut (*channel).prediction as *mut aptx_prediction)
                .offset(subband as isize) as *mut aptx_prediction;
            (*prediction).prev_sign[0 as c_int as usize] =
                1 as c_int as int32_t;
            (*prediction).prev_sign[1 as c_int as usize] =
                1 as c_int as int32_t;
            subband = subband.wrapping_add(1);
        }
        chan = chan.wrapping_add(1);
    }
}
#[inline]
pub unsafe fn aptx_finish(mut ctx: *mut aptx_context) {
    free(ctx as *mut c_void);
}
#[inline]
pub unsafe fn aptx_encode(
    mut ctx: *mut aptx_context,
    mut input: *const c_uchar,
    mut input_size: size_t,
    mut output: *mut c_uchar,
    mut output_size: size_t,
    mut written: *mut size_t,
) -> size_t {
    let ctx_view: &aptx_context = unsafe { &*ctx };
    let sample_size: size_t = (if ctx_view.hd as c_int != 0 {
        6 as c_int
    } else {
        4 as c_int
    }) as size_t;
    let mut samples: [[int32_t; 4]; 2] = [[0; 4]; 2];
    let mut sample: c_uint = 0;
    let mut channel: c_uint = 0;
    let mut ipos: size_t = 0;
    let mut opos: size_t = 0;
    ipos = 0 as size_t;
    opos = 0 as size_t;
    while ipos.wrapping_add(
        (3 as c_int * NB_CHANNELS as c_int * 4 as c_int)
            as size_t,
    ) <= input_size
        && opos.wrapping_add(sample_size) <= output_size
    {
        sample = 0 as c_uint;
        while sample < 4 as c_uint {
            channel = 0 as c_uint;
            while channel < NB_CHANNELS as c_int as c_uint {
                samples[channel as usize][sample as usize] =
                    ((*input.offset(ipos.wrapping_add(0 as size_t) as isize) as uint32_t)
                        << 0 as c_int
                        | (*input.offset(ipos.wrapping_add(1 as size_t) as isize) as uint32_t)
                            << 8 as c_int
                        | (*input.offset(ipos.wrapping_add(2 as size_t) as isize) as int8_t
                            as uint32_t)
                            << 16 as c_int) as int32_t;
                channel = channel.wrapping_add(1);
                ipos = ipos.wrapping_add(3 as size_t);
            }
            sample = sample.wrapping_add(1);
        }
        aptx_encode_samples(
            ctx,
            &raw mut samples as *mut [int32_t; 4],
            output.offset(opos as isize),
        );
        opos = opos.wrapping_add(sample_size);
    }
    *written = opos;
    return ipos;
}
#[inline]
pub unsafe fn aptx_encode_finish(
    mut ctx: *mut aptx_context,
    mut output: *mut c_uchar,
    mut output_size: size_t,
    mut written: *mut size_t,
) -> c_int {
    let sample_size: size_t = (if (*ctx).hd as c_int != 0 {
        6 as c_int
    } else {
        4 as c_int
    }) as size_t;
    let mut samples: [[int32_t; 4]; 2] = [[0 as c_int; 4], [0; 4]];
    let mut opos: size_t = 0;
    if (*ctx).encode_remaining as c_int == 0 as c_int {
        *written = 0 as size_t;
        return 1 as c_int;
    }
    opos = 0 as size_t;
    while (*ctx).encode_remaining as c_int > 0 as c_int
        && opos.wrapping_add(sample_size) <= output_size
    {
        aptx_encode_samples(
            ctx,
            &raw mut samples as *mut [int32_t; 4],
            output.offset(opos as isize),
        );
        (*ctx).encode_remaining = (*ctx).encode_remaining.wrapping_sub(1);
        opos = opos.wrapping_add(sample_size);
    }
    *written = opos;
    if (*ctx).encode_remaining as c_int > 0 as c_int {
        return 0 as c_int;
    }
    aptx_reset(ctx);
    return 1 as c_int;
}
#[inline]
pub unsafe fn aptx_decode(
    mut ctx: *mut aptx_context,
    mut input: *const c_uchar,
    mut input_size: size_t,
    mut output: *mut c_uchar,
    mut output_size: size_t,
    mut written: *mut size_t,
) -> size_t {
    let written_view: &mut size_t = unsafe { &mut *written };
    let sample_size: size_t = (if (*ctx).hd as c_int != 0 {
        6 as c_int
    } else {
        4 as c_int
    }) as size_t;
    let mut samples: [[int32_t; 4]; 2] = [[0; 4]; 2];
    let mut sample: c_uint = 0;
    let mut channel: c_uint = 0;
    let mut ipos: size_t = 0;
    let mut opos: size_t = 0;
    let mut current_block_11: u64;
    ipos = 0 as size_t;
    opos = 0 as size_t;
    while ipos.wrapping_add(sample_size) <= input_size
        && (opos.wrapping_add(
            (3 as c_int * NB_CHANNELS as c_int * 4 as c_int)
                as size_t,
        ) <= output_size
            || (*ctx).decode_skip_leading as c_int > 0 as c_int)
    {
        if aptx_decode_samples(
            ctx,
            input.offset(ipos as isize),
            &raw mut samples as *mut [int32_t; 4],
        ) != 0
        {
            break;
        }
        sample = 0 as c_uint;
        if (*ctx).decode_skip_leading as c_int > 0 as c_int {
            (*ctx).decode_skip_leading = (*ctx).decode_skip_leading.wrapping_sub(1);
            if (*ctx).decode_skip_leading as c_int > 0 as c_int {
                current_block_11 = 8258075665625361029;
            } else {
                sample = (LATENCY_SAMPLES % 4 as c_int) as c_uint;
                current_block_11 = 17216689946888361452;
            }
        } else {
            current_block_11 = 17216689946888361452;
        }
        match current_block_11 {
            17216689946888361452 => {
                while sample < 4 as c_uint {
                    channel = 0 as c_uint;
                    while channel < NB_CHANNELS as c_int as c_uint {
                        *output.offset(opos.wrapping_add(0 as size_t) as isize) =
                            (samples[channel as usize][sample as usize] as uint32_t
                                >> 0 as c_int
                                & 0xff as uint32_t) as uint8_t
                                as c_uchar;
                        *output.offset(opos.wrapping_add(1 as size_t) as isize) =
                            (samples[channel as usize][sample as usize] as uint32_t
                                >> 8 as c_int
                                & 0xff as uint32_t) as uint8_t
                                as c_uchar;
                        *output.offset(opos.wrapping_add(2 as size_t) as isize) =
                            (samples[channel as usize][sample as usize] as uint32_t
                                >> 16 as c_int
                                & 0xff as uint32_t) as uint8_t
                                as c_uchar;
                        channel = channel.wrapping_add(1);
                        opos = opos.wrapping_add(3 as size_t);
                    }
                    sample = sample.wrapping_add(1);
                }
            }
            _ => {}
        }
        ipos = ipos.wrapping_add(sample_size);
    }
    *written_view = opos;
    return ipos;
}
#[inline]
pub unsafe fn aptx_decode_sync(
    mut ctx: *mut aptx_context,
    mut input: *const c_uchar,
    mut input_size: size_t,
    mut output: *mut c_uchar,
    mut output_size: size_t,
    mut written: *mut size_t,
    mut synced: *mut c_int,
    mut dropped: *mut size_t,
) -> size_t {
    let sample_size: size_t = (if (*ctx).hd as c_int != 0 {
        6 as c_int
    } else {
        4 as c_int
    }) as size_t;
    let mut input_size_step: size_t = 0;
    let mut processed_step: size_t = 0;
    let mut written_step: size_t = 0;
    let mut ipos: size_t = 0 as size_t;
    let mut opos: size_t = 0 as size_t;
    let mut i: size_t = 0;
    *synced = 0 as c_int;
    *dropped = 0 as size_t;
    if (*ctx).decode_sync_buffer_len as c_int > 0 as c_int
        && sample_size
            .wrapping_sub(1 as size_t)
            .wrapping_sub((*ctx).decode_sync_buffer_len as size_t)
            <= input_size
    {
        while ((*ctx).decode_sync_buffer_len as size_t) < sample_size.wrapping_sub(1 as size_t) {
            let fresh0 = ipos;
            ipos = ipos.wrapping_add(1);
            let fresh1 = (*ctx).decode_sync_buffer_len;
            (*ctx).decode_sync_buffer_len = (*ctx).decode_sync_buffer_len.wrapping_add(1);
            (*ctx).decode_sync_buffer[fresh1 as usize] = *input.offset(fresh0 as isize);
        }
    }
    while (*ctx).decode_sync_buffer_len as size_t == sample_size.wrapping_sub(1 as size_t)
        && ipos < sample_size
        && ipos < input_size
        && (opos.wrapping_add(
            (3 as c_int * NB_CHANNELS as c_int * 4 as c_int)
                as size_t,
        ) <= output_size
            || (*ctx).decode_skip_leading as c_int > 0 as c_int
            || (*ctx).decode_dropped > 0 as size_t)
    {
        let fresh2 = ipos;
        ipos = ipos.wrapping_add(1);
        (*ctx).decode_sync_buffer[sample_size.wrapping_sub(1 as size_t) as usize] =
            *input.offset(fresh2 as isize);
        processed_step = aptx_decode(
            ctx,
            &raw mut (*ctx).decode_sync_buffer as *mut c_uchar,
            sample_size,
            output.offset(opos as isize),
            output_size.wrapping_sub(opos),
            &raw mut written_step,
        );
        opos = opos.wrapping_add(written_step);
        if (*ctx).decode_dropped > 0 as size_t && processed_step == sample_size {
            (*ctx).decode_dropped = (*ctx).decode_dropped.wrapping_add(processed_step);
            (*ctx).decode_sync_packets = (*ctx).decode_sync_packets.wrapping_add(1);
            if (*ctx).decode_sync_packets
                >= ((LATENCY_SAMPLES + 3 as c_int) / 4 as c_int) as size_t
            {
                *dropped = (*dropped).wrapping_add((*ctx).decode_dropped);
                (*ctx).decode_dropped = 0 as size_t;
                (*ctx).decode_sync_packets = 0 as size_t;
            }
        }
        if processed_step < sample_size {
            aptx_reset_decode_sync(ctx);
            *synced = 0 as c_int;
            (*ctx).decode_dropped = (*ctx).decode_dropped.wrapping_add(1);
            (*ctx).decode_sync_packets = 0 as size_t;
            i = 0 as size_t;
            while i < sample_size.wrapping_sub(1 as size_t) {
                (*ctx).decode_sync_buffer[i as usize] =
                    (*ctx).decode_sync_buffer[i.wrapping_add(1 as size_t) as usize];
                i = i.wrapping_add(1);
            }
        } else {
            if (*ctx).decode_dropped == 0 as size_t {
                *synced = 1 as c_int;
            }
            (*ctx).decode_sync_buffer_len = 0 as uint8_t;
        }
    }
    if (*ctx).decode_sync_buffer_len as size_t == sample_size.wrapping_sub(1 as size_t)
        && ipos == sample_size
    {
        ipos = 0 as size_t;
        (*ctx).decode_sync_buffer_len = 0 as uint8_t;
    }
    while ipos.wrapping_add(sample_size) <= input_size
        && (opos.wrapping_add(
            (3 as c_int * NB_CHANNELS as c_int * 4 as c_int)
                as size_t,
        ) <= output_size
            || (*ctx).decode_skip_leading as c_int > 0 as c_int
            || (*ctx).decode_dropped > 0 as size_t)
    {
        input_size_step = output_size
            .wrapping_sub(opos)
            .wrapping_div(3 as size_t)
            .wrapping_mul(NB_CHANNELS as c_int as size_t)
            .wrapping_mul(4 as size_t)
            .wrapping_add((*ctx).decode_skip_leading as size_t)
            .wrapping_mul(sample_size);
        if input_size_step
            > input_size
                .wrapping_sub(ipos)
                .wrapping_div(sample_size)
                .wrapping_mul(sample_size)
        {
            input_size_step = input_size
                .wrapping_sub(ipos)
                .wrapping_div(sample_size)
                .wrapping_mul(sample_size);
        }
        if input_size_step
            > (((LATENCY_SAMPLES + 3 as c_int) / 4 as c_int) as size_t)
                .wrapping_sub((*ctx).decode_sync_packets)
                .wrapping_mul(sample_size)
            && (*ctx).decode_dropped > 0 as size_t
        {
            input_size_step = (((LATENCY_SAMPLES + 3 as c_int)
                / 4 as c_int) as size_t)
                .wrapping_sub((*ctx).decode_sync_packets)
                .wrapping_mul(sample_size);
        }
        processed_step = aptx_decode(
            ctx,
            input.offset(ipos as isize),
            input_size_step,
            output.offset(opos as isize),
            output_size.wrapping_sub(opos),
            &raw mut written_step,
        );
        ipos = ipos.wrapping_add(processed_step);
        opos = opos.wrapping_add(written_step);
        if (*ctx).decode_dropped > 0 as size_t
            && processed_step.wrapping_div(sample_size) > 0 as size_t
        {
            (*ctx).decode_dropped = (*ctx).decode_dropped.wrapping_add(processed_step);
            (*ctx).decode_sync_packets = (*ctx)
                .decode_sync_packets
                .wrapping_add(processed_step.wrapping_div(sample_size));
            if (*ctx).decode_sync_packets
                >= ((LATENCY_SAMPLES + 3 as c_int) / 4 as c_int) as size_t
            {
                *dropped = (*dropped).wrapping_add((*ctx).decode_dropped);
                (*ctx).decode_dropped = 0 as size_t;
                (*ctx).decode_sync_packets = 0 as size_t;
            }
        }
        if processed_step < input_size_step {
            aptx_reset_decode_sync(ctx);
            *synced = 0 as c_int;
            ipos = ipos.wrapping_add(1);
            (*ctx).decode_dropped = (*ctx).decode_dropped.wrapping_add(1);
            (*ctx).decode_sync_packets = 0 as size_t;
        } else if (*ctx).decode_dropped == 0 as size_t {
            *synced = 1 as c_int;
        }
    }
    if ipos.wrapping_add(sample_size) > input_size {
        while ipos < input_size {
            let fresh3 = ipos;
            ipos = ipos.wrapping_add(1);
            let fresh4 = (*ctx).decode_sync_buffer_len;
            (*ctx).decode_sync_buffer_len = (*ctx).decode_sync_buffer_len.wrapping_add(1);
            (*ctx).decode_sync_buffer[fresh4 as usize] = *input.offset(fresh3 as isize);
        }
    }
    *written = opos;
    return ipos;
}
#[inline]
pub unsafe fn aptx_decode_sync_finish(mut ctx: *mut aptx_context) -> size_t {
    let dropped: uint8_t = (*ctx).decode_sync_buffer_len;
    aptx_reset(ctx);
    return dropped as size_t;
}

extern "C" fn run_static_initializers() { unsafe {
    all_tables = [
        [
            aptx_tables {
                quantize_intervals: &raw const quantize_intervals_LF as *const int32_t,
                invert_quantize_dither_factors: &raw const invert_quantize_dither_factors_LF
                    as *const int32_t,
                quantize_dither_factors: &raw const quantize_dither_factors_LF as *const int32_t,
                quantize_factor_select_offset: &raw const quantize_factor_select_offset_LF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 65]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x11ff as int32_t,
                prediction_order: 24 as c_int,
            },
            aptx_tables {
                quantize_intervals: &raw const quantize_intervals_MLF as *const int32_t,
                invert_quantize_dither_factors: &raw const invert_quantize_dither_factors_MLF
                    as *const int32_t,
                quantize_dither_factors: &raw const quantize_dither_factors_MLF as *const int32_t,
                quantize_factor_select_offset: &raw const quantize_factor_select_offset_MLF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 9]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x14ff as int32_t,
                prediction_order: 12 as c_int,
            },
            aptx_tables {
                quantize_intervals: &raw const quantize_intervals_MHF as *const int32_t,
                invert_quantize_dither_factors: &raw const invert_quantize_dither_factors_MHF
                    as *const int32_t,
                quantize_dither_factors: &raw const quantize_dither_factors_MHF as *const int32_t,
                quantize_factor_select_offset: &raw const quantize_factor_select_offset_MHF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 3]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x16ff as int32_t,
                prediction_order: 6 as c_int,
            },
            aptx_tables {
                quantize_intervals: &raw const quantize_intervals_HF as *const int32_t,
                invert_quantize_dither_factors: &raw const invert_quantize_dither_factors_HF
                    as *const int32_t,
                quantize_dither_factors: &raw const quantize_dither_factors_HF as *const int32_t,
                quantize_factor_select_offset: &raw const quantize_factor_select_offset_HF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 5]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x15ff as int32_t,
                prediction_order: 12 as c_int,
            },
        ],
        [
            aptx_tables {
                quantize_intervals: &raw const hd_quantize_intervals_LF as *const int32_t,
                invert_quantize_dither_factors: &raw const hd_invert_quantize_dither_factors_LF
                    as *const int32_t,
                quantize_dither_factors: &raw const hd_quantize_dither_factors_LF as *const int32_t,
                quantize_factor_select_offset: &raw const hd_quantize_factor_select_offset_LF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 257]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x11ff as int32_t,
                prediction_order: 24 as c_int,
            },
            aptx_tables {
                quantize_intervals: &raw const hd_quantize_intervals_MLF as *const int32_t,
                invert_quantize_dither_factors: &raw const hd_invert_quantize_dither_factors_MLF
                    as *const int32_t,
                quantize_dither_factors: &raw const hd_quantize_dither_factors_MLF
                    as *const int32_t,
                quantize_factor_select_offset: &raw const hd_quantize_factor_select_offset_MLF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 33]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x14ff as int32_t,
                prediction_order: 12 as c_int,
            },
            aptx_tables {
                quantize_intervals: &raw const hd_quantize_intervals_MHF as *const int32_t,
                invert_quantize_dither_factors: &raw const hd_invert_quantize_dither_factors_MHF
                    as *const int32_t,
                quantize_dither_factors: &raw const hd_quantize_dither_factors_MHF
                    as *const int32_t,
                quantize_factor_select_offset: &raw const hd_quantize_factor_select_offset_MHF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 9]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x16ff as int32_t,
                prediction_order: 6 as c_int,
            },
            aptx_tables {
                quantize_intervals: &raw const hd_quantize_intervals_HF as *const int32_t,
                invert_quantize_dither_factors: &raw const hd_invert_quantize_dither_factors_HF
                    as *const int32_t,
                quantize_dither_factors: &raw const hd_quantize_dither_factors_HF as *const int32_t,
                quantize_factor_select_offset: &raw const hd_quantize_factor_select_offset_HF
                    as *const int16_t,
                tables_size: (::core::mem::size_of::<[int32_t; 17]>() as usize)
                    .wrapping_div(::core::mem::size_of::<int32_t>() as usize)
                    as c_int,
                factor_max: 0x15ff as int32_t,
                prediction_order: 12 as c_int,
            },
        ],
    ];
} }
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
