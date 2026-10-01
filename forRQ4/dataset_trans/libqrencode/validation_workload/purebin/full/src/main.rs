                                                         
                                                                  
                                                              
                             
mod libcall {
    use core::ffi::{c_char, c_int, c_void};
    use qrlib::src::{qrencode as qe, qrinput as qi, split as sp};

    pub type Qr = qe::QRcode;
    pub type QrList = qe::QRcode_List;
    pub type In = qi::QRinput;
    pub type InStruct = qi::QRinput_Struct;

    #[inline] pub unsafe fn encode_string(s: *const c_char, v: i32, l: i32, m: i32, ci: i32) -> *mut Qr {
        qe::QRcode_encodeString(s, v, l as _, m as _, ci)
    }
    #[inline] pub unsafe fn encode_string_8bit(s: *const c_char, v: i32, l: i32) -> *mut Qr {
        qe::QRcode_encodeString8bit(s, v, l as _)
    }
    #[inline] pub unsafe fn encode_data(n: i32, d: *const u8, v: i32, l: i32) -> *mut Qr {
        qe::QRcode_encodeData(n, d, v, l as _)
    }
    #[inline] pub unsafe fn encode_string_mqr(s: *const c_char, v: i32, l: i32, m: i32, ci: i32) -> *mut Qr {
        qe::QRcode_encodeStringMQR(s, v, l as _, m as _, ci)
    }
    #[inline] pub unsafe fn encode_string_8bit_mqr(s: *const c_char, v: i32, l: i32) -> *mut Qr {
        qe::QRcode_encodeString8bitMQR(s, v, l as _)
    }
    #[inline] pub unsafe fn encode_data_mqr(n: i32, d: *const u8, v: i32, l: i32) -> *mut Qr {
        qe::QRcode_encodeDataMQR(n, d, v, l as _)
    }
    #[inline] pub unsafe fn encode_string_8bit_structured(s: *const c_char, v: i32, l: i32) -> *mut QrList {
        qe::QRcode_encodeString8bitStructured(s, v, l as _)
    }
    #[inline] pub unsafe fn encode_string_structured(s: *const c_char, v: i32, l: i32, m: i32, ci: i32) -> *mut QrList {
        qe::QRcode_encodeStringStructured(s, v, l as _, m as _, ci)
    }
    #[inline] pub unsafe fn encode_data_structured(n: i32, d: *const u8, v: i32, l: i32) -> *mut QrList {
        qe::QRcode_encodeDataStructured(n, d, v, l as _)
    }
    #[inline] pub unsafe fn encode_input(i: *mut In) -> *mut Qr {
        qe::QRcode_encodeInput(i as *mut _)
    }
    #[inline] pub unsafe fn encode_input_structured(s: *mut InStruct) -> *mut QrList {
        qe::QRcode_encodeInputStructured(s as *mut _)
    }
    #[inline] pub unsafe fn qr_free(q: *mut Qr) { qe::QRcode_free(q) }
    #[inline] pub unsafe fn list_size(l: *mut QrList) -> i32 { qe::QRcode_List_size(l) }
    #[inline] pub unsafe fn list_free(l: *mut QrList) { qe::QRcode_List_free(l) }
    #[inline] pub unsafe fn api_version(a: *mut i32, b: *mut i32, c: *mut i32) {
        qe::QRcode_APIVersion(a, b, c)
    }
    #[inline] pub unsafe fn api_version_string() -> *const c_char { qe::QRcode_APIVersionString() }
    #[inline] pub unsafe fn clear_cache() { qe::QRcode_clearCache() }

    #[inline] pub unsafe fn input_new() -> *mut In { qi::QRinput_new() }
    #[inline] pub unsafe fn input_new2(v: i32, l: i32) -> *mut In { qi::QRinput_new2(v, l as _) }
    #[inline] pub unsafe fn input_new_mqr(v: i32, l: i32) -> *mut In { qi::QRinput_newMQR(v, l as _) }
    #[inline] pub unsafe fn input_append(i: *mut In, m: i32, n: i32, d: *const u8) -> i32 {
        qi::QRinput_append(i, m as _, n, d)
    }
    #[inline] pub unsafe fn input_append_eci(i: *mut In, eci: u32) -> i32 {
        qi::QRinput_appendECIheader(i, eci)
    }
    #[inline] pub unsafe fn input_check(m: i32, n: i32, d: *const u8) -> i32 {
        qi::QRinput_check(m as _, n, d as *mut u8)
    }
    #[inline] pub unsafe fn estimate_bits_num(n: i32) -> i32 { qi::QRinput_estimateBitsModeNum(n) }
    #[inline] pub unsafe fn estimate_bits_an(n: i32) -> i32 { qi::QRinput_estimateBitsModeAn(n) }
    #[inline] pub unsafe fn estimate_bits_8(n: i32) -> i32 { qi::QRinput_estimateBitsMode8(n) }
    #[inline] pub unsafe fn estimate_bits_kanji(n: i32) -> i32 { qi::QRinput_estimateBitsModeKanji(n) }
    #[inline] pub unsafe fn input_set_ver_ecl(i: *mut In, v: i32, l: i32) -> i32 {
        qi::QRinput_setVersionAndErrorCorrectionLevel(i, v, l as _)
    }
    #[inline] pub unsafe fn input_set_version(i: *mut In, v: i32) -> i32 { qi::QRinput_setVersion(i, v) }
    #[inline] pub unsafe fn input_set_ecl(i: *mut In, l: i32) -> i32 { qi::QRinput_setErrorCorrectionLevel(i, l as _) }
    #[inline] pub unsafe fn input_get_version(i: *mut In) -> i32 { qi::QRinput_getVersion(i) }
    #[inline] pub unsafe fn input_get_ecl(i: *mut In) -> i32 { qi::QRinput_getErrorCorrectionLevel(i) as i32 }
    #[inline] pub unsafe fn input_dup(i: *mut In) -> *mut In { qi::QRinput_dup(i) }
    #[inline] pub unsafe fn input_free(i: *mut In) { qi::QRinput_free(i) }
    #[inline] pub unsafe fn input_set_fnc1_first(i: *mut In) -> i32 { qi::QRinput_setFNC1First(i) }
    #[inline] pub unsafe fn input_set_fnc1_second(i: *mut In, c: i8) -> i32 {
        qi::QRinput_setFNC1Second(i, c as u8)
    }
    #[inline] pub unsafe fn is_splittable(m: i32) -> i32 { qi::QRinput_isSplittableMode(m as _) }
    #[inline] pub unsafe fn input_get_byte_stream(i: *mut In) -> *mut u8 {
        qi::QRinput_getByteStream(i)
    }
    #[inline] pub unsafe fn input_split_to_struct(i: *mut In) -> *mut InStruct {
        qi::QRinput_splitQRinputToStruct(i)
    }
    #[inline] pub unsafe fn struct_set_parity(s: *mut InStruct, p: u8) {
        let _ = qi::QRinput_Struct_setParity(s, p);
    }
    #[inline] pub unsafe fn struct_insert_headers(s: *mut InStruct) -> i32 {
        qi::QRinput_Struct_insertStructuredAppendHeaders(s)
    }
    #[inline] pub unsafe fn struct_free(s: *mut InStruct) { qi::QRinput_Struct_free(s) }

    #[inline] pub unsafe fn split_string(s: *const c_char, i: *mut In, hint: i32, cs: i32) -> i32 {
        sp::Split_splitStringToQRinput(s, i as *mut _, hint as _, cs)
    }

    extern "C" {
        pub fn free(p: *mut c_void);
    }
}

include!("../../driver.rs");
