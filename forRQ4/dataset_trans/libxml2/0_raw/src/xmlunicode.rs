extern "C" {
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn xmlCharInRange(
        val: ::core::ffi::c_uint,
        group: *const xmlChRangeGroup,
    ) -> ::core::ffi::c_int;
}
pub type xmlIntFunc = unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlUnicodeNameTable {
    pub table: *const xmlUnicodeRange,
    pub numentries: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlUnicodeRange {
    pub rangename: *const ::core::ffi::c_char,
    pub func: Option<xmlIntFunc>,
}
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: ::core::ffi::c_int,
    pub nbLongRange: ::core::ffi::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: ::core::ffi::c_uint,
    pub high: ::core::ffi::c_uint,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: ::core::ffi::c_ushort,
    pub high: ::core::ffi::c_ushort,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut xmlUnicodeBlocks: [xmlUnicodeRange; 128] = unsafe {
    [
        xmlUnicodeRange {
            rangename: b"AegeanNumbers\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsAegeanNumbers
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"AlphabeticPresentationForms\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsAlphabeticPresentationForms
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Arabic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsArabic as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"ArabicPresentationForms-A\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsArabicPresentationFormsA
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"ArabicPresentationForms-B\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsArabicPresentationFormsB
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Armenian\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsArmenian as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Arrows\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsArrows as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"BasicLatin\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBasicLatin
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Bengali\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBengali as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"BlockElements\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBlockElements
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Bopomofo\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBopomofo as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"BopomofoExtended\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBopomofoExtended
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"BoxDrawing\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBoxDrawing
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"BraillePatterns\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBraillePatterns
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Buhid\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsBuhid as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"ByzantineMusicalSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsByzantineMusicalSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKCompatibility\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKCompatibility
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKCompatibilityForms\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKCompatibilityForms
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKCompatibilityIdeographs\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKCompatibilityIdeographs
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKCompatibilityIdeographsSupplement\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKCompatibilityIdeographsSupplement
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKRadicalsSupplement\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKRadicalsSupplement
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKSymbolsandPunctuation\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKSymbolsandPunctuation
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKUnifiedIdeographs\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKUnifiedIdeographs
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKUnifiedIdeographsExtensionA\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKUnifiedIdeographsExtensionA
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CJKUnifiedIdeographsExtensionB\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCJKUnifiedIdeographsExtensionB
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Cherokee\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCherokee as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CombiningDiacriticalMarks\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCombiningDiacriticalMarks
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CombiningDiacriticalMarksforSymbols\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCombiningDiacriticalMarksforSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CombiningHalfMarks\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCombiningHalfMarks
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CombiningMarksforSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCombiningMarksforSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"ControlPictures\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsControlPictures
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CurrencySymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCurrencySymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CypriotSyllabary\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCypriotSyllabary
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Cyrillic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCyrillic as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"CyrillicSupplement\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCyrillicSupplement
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Deseret\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsDeseret as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Devanagari\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsDevanagari
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Dingbats\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsDingbats as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"EnclosedAlphanumerics\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsEnclosedAlphanumerics
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"EnclosedCJKLettersandMonths\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsEnclosedCJKLettersandMonths
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Ethiopic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsEthiopic as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"GeneralPunctuation\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGeneralPunctuation
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"GeometricShapes\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGeometricShapes
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Georgian\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGeorgian as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Gothic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGothic as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Greek\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGreek as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"GreekExtended\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGreekExtended
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"GreekandCoptic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGreekandCoptic
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Gujarati\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGujarati as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Gurmukhi\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsGurmukhi as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"HalfwidthandFullwidthForms\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHalfwidthandFullwidthForms
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"HangulCompatibilityJamo\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHangulCompatibilityJamo
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"HangulJamo\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHangulJamo
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"HangulSyllables\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHangulSyllables
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Hanunoo\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHanunoo as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Hebrew\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHebrew as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"HighPrivateUseSurrogates\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHighPrivateUseSurrogates
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"HighSurrogates\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHighSurrogates
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Hiragana\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsHiragana as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"IPAExtensions\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsIPAExtensions
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"IdeographicDescriptionCharacters\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsIdeographicDescriptionCharacters
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Kanbun\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsKanbun as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"KangxiRadicals\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsKangxiRadicals
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Kannada\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsKannada as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Katakana\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsKatakana as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"KatakanaPhoneticExtensions\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsKatakanaPhoneticExtensions
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Khmer\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsKhmer as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"KhmerSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsKhmerSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Lao\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLao as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Latin-1Supplement\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLatin1Supplement
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"LatinExtended-A\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLatinExtendedA
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"LatinExtended-B\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLatinExtendedB
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"LatinExtendedAdditional\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLatinExtendedAdditional
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"LetterlikeSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLetterlikeSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Limbu\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLimbu as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"LinearBIdeograms\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLinearBIdeograms
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"LinearBSyllabary\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLinearBSyllabary
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"LowSurrogates\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsLowSurrogates
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Malayalam\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMalayalam as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MathematicalAlphanumericSymbols\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMathematicalAlphanumericSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MathematicalOperators\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMathematicalOperators
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MiscellaneousMathematicalSymbols-A\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMiscellaneousMathematicalSymbolsA
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MiscellaneousMathematicalSymbols-B\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMiscellaneousMathematicalSymbolsB
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MiscellaneousSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMiscellaneousSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MiscellaneousSymbolsandArrows\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMiscellaneousSymbolsandArrows
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MiscellaneousTechnical\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMiscellaneousTechnical
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Mongolian\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMongolian as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"MusicalSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMusicalSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Myanmar\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsMyanmar as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"NumberForms\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsNumberForms
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Ogham\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsOgham as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"OldItalic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsOldItalic as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"OpticalCharacterRecognition\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsOpticalCharacterRecognition
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Oriya\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsOriya as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Osmanya\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsOsmanya as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"PhoneticExtensions\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsPhoneticExtensions
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"PrivateUse\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsPrivateUse
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"PrivateUseArea\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsPrivateUseArea
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Runic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsRunic as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Shavian\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsShavian as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Sinhala\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSinhala as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SmallFormVariants\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSmallFormVariants
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SpacingModifierLetters\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSpacingModifierLetters
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Specials\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSpecials as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SuperscriptsandSubscripts\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSuperscriptsandSubscripts
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SupplementalArrows-A\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSupplementalArrowsA
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SupplementalArrows-B\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSupplementalArrowsB
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SupplementalMathematicalOperators\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSupplementalMathematicalOperators
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SupplementaryPrivateUseArea-A\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSupplementaryPrivateUseAreaA
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"SupplementaryPrivateUseArea-B\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSupplementaryPrivateUseAreaB
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Syriac\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsSyriac as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Tagalog\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTagalog as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Tagbanwa\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTagbanwa as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Tags\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTags as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"TaiLe\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTaiLe as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"TaiXuanJingSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTaiXuanJingSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Tamil\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTamil as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Telugu\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTelugu as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Thaana\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsThaana as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Thai\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsThai as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Tibetan\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsTibetan as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Ugaritic\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsUgaritic as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"UnifiedCanadianAboriginalSyllabics\0" as *const u8
                as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsUnifiedCanadianAboriginalSyllabics
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"VariationSelectors\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsVariationSelectors
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"VariationSelectorsSupplement\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsVariationSelectorsSupplement
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"YiRadicals\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsYiRadicals
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"YiSyllables\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsYiSyllables
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"YijingHexagramSymbols\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsYijingHexagramSymbols
                    as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
    ]
};
static mut xmlUnicodeCats: [xmlUnicodeRange; 36] = unsafe {
    [
        xmlUnicodeRange {
            rangename: b"C\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatC as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Cc\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatCc as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Cf\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatCf as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Co\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatCo as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Cs\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatCs as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"L\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatL as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Ll\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatLl as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Lm\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatLm as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Lo\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatLo as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Lt\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatLt as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Lu\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatLu as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"M\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatM as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Mc\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatMc as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Me\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatMe as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Mn\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatMn as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"N\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatN as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Nd\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatNd as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Nl\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatNl as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"No\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatNo as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"P\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatP as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Pc\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatPc as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Pd\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatPd as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Pe\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatPe as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Pf\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatPf as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Pi\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatPi as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Po\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatPo as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Ps\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatPs as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"S\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatS as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Sc\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatSc as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Sk\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatSk as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Sm\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatSm as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"So\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatSo as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Z\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatZ as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Zl\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatZl as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Zp\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatZp as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
        xmlUnicodeRange {
            rangename: b"Zs\0" as *const u8 as *const ::core::ffi::c_char,
            func: Some(
                xmlUCSIsCatZs as unsafe extern "C" fn(::core::ffi::c_int) -> ::core::ffi::c_int,
            ),
        },
    ]
};
static mut xmlCS: [xmlChSRange; 18] = [
    _xmlChSRange {
        low: 0 as ::core::ffi::c_ushort,
        high: 0x1f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7f as ::core::ffi::c_ushort,
        high: 0x9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xad as ::core::ffi::c_ushort,
        high: 0xad as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x600 as ::core::ffi::c_ushort,
        high: 0x603 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6dd as ::core::ffi::c_ushort,
        high: 0x6dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x70f as ::core::ffi::c_ushort,
        high: 0x70f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17b4 as ::core::ffi::c_ushort,
        high: 0x17b5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x200b as ::core::ffi::c_ushort,
        high: 0x200f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x202a as ::core::ffi::c_ushort,
        high: 0x202e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2060 as ::core::ffi::c_ushort,
        high: 0x2063 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x206a as ::core::ffi::c_ushort,
        high: 0x206f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd800 as ::core::ffi::c_ushort,
        high: 0xd800 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdb7f as ::core::ffi::c_ushort,
        high: 0xdb80 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdbff as ::core::ffi::c_ushort,
        high: 0xdc00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdfff as ::core::ffi::c_ushort,
        high: 0xe000 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf8ff as ::core::ffi::c_ushort,
        high: 0xf8ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfeff as ::core::ffi::c_ushort,
        high: 0xfeff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfff9 as ::core::ffi::c_ushort,
        high: 0xfffb as ::core::ffi::c_ushort,
    },
];
static mut xmlCL: [xmlChLRange; 7] = [
    _xmlChLRange {
        low: 0x1d173 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d17a as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xe0001 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xe0001 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xe0020 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xe007f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xf0000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xf0000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xffffd as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xffffd as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x100000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x100000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10fffd as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10fffd as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlCG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 18 as ::core::ffi::c_int,
        nbLongRange: 7 as ::core::ffi::c_int,
        shortRange: &raw const xmlCS as *const xmlChSRange,
        longRange: &raw const xmlCL as *const xmlChLRange,
    }
};
static mut xmlCfS: [xmlChSRange; 11] = [
    _xmlChSRange {
        low: 0xad as ::core::ffi::c_ushort,
        high: 0xad as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x600 as ::core::ffi::c_ushort,
        high: 0x603 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6dd as ::core::ffi::c_ushort,
        high: 0x6dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x70f as ::core::ffi::c_ushort,
        high: 0x70f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17b4 as ::core::ffi::c_ushort,
        high: 0x17b5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x200b as ::core::ffi::c_ushort,
        high: 0x200f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x202a as ::core::ffi::c_ushort,
        high: 0x202e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2060 as ::core::ffi::c_ushort,
        high: 0x2063 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x206a as ::core::ffi::c_ushort,
        high: 0x206f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfeff as ::core::ffi::c_ushort,
        high: 0xfeff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfff9 as ::core::ffi::c_ushort,
        high: 0xfffb as ::core::ffi::c_ushort,
    },
];
static mut xmlCfL: [xmlChLRange; 3] = [
    _xmlChLRange {
        low: 0x1d173 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d17a as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xe0001 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xe0001 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xe0020 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xe007f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlCfG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 11 as ::core::ffi::c_int,
        nbLongRange: 3 as ::core::ffi::c_int,
        shortRange: &raw const xmlCfS as *const xmlChSRange,
        longRange: &raw const xmlCfL as *const xmlChLRange,
    }
};
static mut xmlLS: [xmlChSRange; 279] = [
    _xmlChSRange {
        low: 0x41 as ::core::ffi::c_ushort,
        high: 0x5a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x61 as ::core::ffi::c_ushort,
        high: 0x7a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaa as ::core::ffi::c_ushort,
        high: 0xaa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5 as ::core::ffi::c_ushort,
        high: 0xb5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba as ::core::ffi::c_ushort,
        high: 0xba as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc0 as ::core::ffi::c_ushort,
        high: 0xd6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd8 as ::core::ffi::c_ushort,
        high: 0xf6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf8 as ::core::ffi::c_ushort,
        high: 0x236 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x250 as ::core::ffi::c_ushort,
        high: 0x2c1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2c6 as ::core::ffi::c_ushort,
        high: 0x2d1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e0 as ::core::ffi::c_ushort,
        high: 0x2e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2ee as ::core::ffi::c_ushort,
        high: 0x2ee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x37a as ::core::ffi::c_ushort,
        high: 0x37a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x386 as ::core::ffi::c_ushort,
        high: 0x386 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x388 as ::core::ffi::c_ushort,
        high: 0x38a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x38c as ::core::ffi::c_ushort,
        high: 0x38c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x38e as ::core::ffi::c_ushort,
        high: 0x3a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3a3 as ::core::ffi::c_ushort,
        high: 0x3ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3d0 as ::core::ffi::c_ushort,
        high: 0x3f5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f7 as ::core::ffi::c_ushort,
        high: 0x3fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x400 as ::core::ffi::c_ushort,
        high: 0x481 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x48a as ::core::ffi::c_ushort,
        high: 0x4ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d0 as ::core::ffi::c_ushort,
        high: 0x4f5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f8 as ::core::ffi::c_ushort,
        high: 0x4f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x500 as ::core::ffi::c_ushort,
        high: 0x50f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x531 as ::core::ffi::c_ushort,
        high: 0x556 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x559 as ::core::ffi::c_ushort,
        high: 0x559 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x561 as ::core::ffi::c_ushort,
        high: 0x587 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5d0 as ::core::ffi::c_ushort,
        high: 0x5ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5f0 as ::core::ffi::c_ushort,
        high: 0x5f2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x621 as ::core::ffi::c_ushort,
        high: 0x63a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x640 as ::core::ffi::c_ushort,
        high: 0x64a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x66e as ::core::ffi::c_ushort,
        high: 0x66f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x671 as ::core::ffi::c_ushort,
        high: 0x6d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d5 as ::core::ffi::c_ushort,
        high: 0x6d5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e5 as ::core::ffi::c_ushort,
        high: 0x6e6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ee as ::core::ffi::c_ushort,
        high: 0x6ef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6fa as ::core::ffi::c_ushort,
        high: 0x6fc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ff as ::core::ffi::c_ushort,
        high: 0x6ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x710 as ::core::ffi::c_ushort,
        high: 0x710 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x712 as ::core::ffi::c_ushort,
        high: 0x72f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x74d as ::core::ffi::c_ushort,
        high: 0x74f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x780 as ::core::ffi::c_ushort,
        high: 0x7a5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7b1 as ::core::ffi::c_ushort,
        high: 0x7b1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x904 as ::core::ffi::c_ushort,
        high: 0x939 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93d as ::core::ffi::c_ushort,
        high: 0x93d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x950 as ::core::ffi::c_ushort,
        high: 0x950 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x958 as ::core::ffi::c_ushort,
        high: 0x961 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x985 as ::core::ffi::c_ushort,
        high: 0x98c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x98f as ::core::ffi::c_ushort,
        high: 0x990 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x993 as ::core::ffi::c_ushort,
        high: 0x9a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9aa as ::core::ffi::c_ushort,
        high: 0x9b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9b2 as ::core::ffi::c_ushort,
        high: 0x9b2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9b6 as ::core::ffi::c_ushort,
        high: 0x9b9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9bd as ::core::ffi::c_ushort,
        high: 0x9bd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9dc as ::core::ffi::c_ushort,
        high: 0x9dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9df as ::core::ffi::c_ushort,
        high: 0x9e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9f0 as ::core::ffi::c_ushort,
        high: 0x9f1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa05 as ::core::ffi::c_ushort,
        high: 0xa0a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa0f as ::core::ffi::c_ushort,
        high: 0xa10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa13 as ::core::ffi::c_ushort,
        high: 0xa28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa2a as ::core::ffi::c_ushort,
        high: 0xa30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa32 as ::core::ffi::c_ushort,
        high: 0xa33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa35 as ::core::ffi::c_ushort,
        high: 0xa36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa38 as ::core::ffi::c_ushort,
        high: 0xa39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa59 as ::core::ffi::c_ushort,
        high: 0xa5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa5e as ::core::ffi::c_ushort,
        high: 0xa5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa72 as ::core::ffi::c_ushort,
        high: 0xa74 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa85 as ::core::ffi::c_ushort,
        high: 0xa8d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa8f as ::core::ffi::c_ushort,
        high: 0xa91 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa93 as ::core::ffi::c_ushort,
        high: 0xaa8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaaa as ::core::ffi::c_ushort,
        high: 0xab0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xab2 as ::core::ffi::c_ushort,
        high: 0xab3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xab5 as ::core::ffi::c_ushort,
        high: 0xab9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabd as ::core::ffi::c_ushort,
        high: 0xabd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xad0 as ::core::ffi::c_ushort,
        high: 0xad0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae0 as ::core::ffi::c_ushort,
        high: 0xae1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb05 as ::core::ffi::c_ushort,
        high: 0xb0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb0f as ::core::ffi::c_ushort,
        high: 0xb10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb13 as ::core::ffi::c_ushort,
        high: 0xb28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb2a as ::core::ffi::c_ushort,
        high: 0xb30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb32 as ::core::ffi::c_ushort,
        high: 0xb33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb35 as ::core::ffi::c_ushort,
        high: 0xb39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3d as ::core::ffi::c_ushort,
        high: 0xb3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5c as ::core::ffi::c_ushort,
        high: 0xb5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5f as ::core::ffi::c_ushort,
        high: 0xb61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb71 as ::core::ffi::c_ushort,
        high: 0xb71 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb83 as ::core::ffi::c_ushort,
        high: 0xb83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb85 as ::core::ffi::c_ushort,
        high: 0xb8a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb8e as ::core::ffi::c_ushort,
        high: 0xb90 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb92 as ::core::ffi::c_ushort,
        high: 0xb95 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb99 as ::core::ffi::c_ushort,
        high: 0xb9a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9c as ::core::ffi::c_ushort,
        high: 0xb9c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9e as ::core::ffi::c_ushort,
        high: 0xb9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba3 as ::core::ffi::c_ushort,
        high: 0xba4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba8 as ::core::ffi::c_ushort,
        high: 0xbaa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbae as ::core::ffi::c_ushort,
        high: 0xbb5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbb7 as ::core::ffi::c_ushort,
        high: 0xbb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc05 as ::core::ffi::c_ushort,
        high: 0xc0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc0e as ::core::ffi::c_ushort,
        high: 0xc10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc12 as ::core::ffi::c_ushort,
        high: 0xc28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc2a as ::core::ffi::c_ushort,
        high: 0xc33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc35 as ::core::ffi::c_ushort,
        high: 0xc39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc60 as ::core::ffi::c_ushort,
        high: 0xc61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc85 as ::core::ffi::c_ushort,
        high: 0xc8c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc8e as ::core::ffi::c_ushort,
        high: 0xc90 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc92 as ::core::ffi::c_ushort,
        high: 0xca8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcaa as ::core::ffi::c_ushort,
        high: 0xcb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcb5 as ::core::ffi::c_ushort,
        high: 0xcb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbd as ::core::ffi::c_ushort,
        high: 0xcbd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcde as ::core::ffi::c_ushort,
        high: 0xcde as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xce0 as ::core::ffi::c_ushort,
        high: 0xce1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd05 as ::core::ffi::c_ushort,
        high: 0xd0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd0e as ::core::ffi::c_ushort,
        high: 0xd10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd12 as ::core::ffi::c_ushort,
        high: 0xd28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd2a as ::core::ffi::c_ushort,
        high: 0xd39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd60 as ::core::ffi::c_ushort,
        high: 0xd61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd85 as ::core::ffi::c_ushort,
        high: 0xd96 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd9a as ::core::ffi::c_ushort,
        high: 0xdb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdb3 as ::core::ffi::c_ushort,
        high: 0xdbb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdbd as ::core::ffi::c_ushort,
        high: 0xdbd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdc0 as ::core::ffi::c_ushort,
        high: 0xdc6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe01 as ::core::ffi::c_ushort,
        high: 0xe30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe32 as ::core::ffi::c_ushort,
        high: 0xe33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe40 as ::core::ffi::c_ushort,
        high: 0xe46 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe81 as ::core::ffi::c_ushort,
        high: 0xe82 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe84 as ::core::ffi::c_ushort,
        high: 0xe84 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe87 as ::core::ffi::c_ushort,
        high: 0xe88 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe8a as ::core::ffi::c_ushort,
        high: 0xe8a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe8d as ::core::ffi::c_ushort,
        high: 0xe8d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe94 as ::core::ffi::c_ushort,
        high: 0xe97 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe99 as ::core::ffi::c_ushort,
        high: 0xe9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea1 as ::core::ffi::c_ushort,
        high: 0xea3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea5 as ::core::ffi::c_ushort,
        high: 0xea5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea7 as ::core::ffi::c_ushort,
        high: 0xea7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeaa as ::core::ffi::c_ushort,
        high: 0xeab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xead as ::core::ffi::c_ushort,
        high: 0xeb0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb2 as ::core::ffi::c_ushort,
        high: 0xeb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xebd as ::core::ffi::c_ushort,
        high: 0xebd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec0 as ::core::ffi::c_ushort,
        high: 0xec4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec6 as ::core::ffi::c_ushort,
        high: 0xec6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xedc as ::core::ffi::c_ushort,
        high: 0xedd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf00 as ::core::ffi::c_ushort,
        high: 0xf00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf40 as ::core::ffi::c_ushort,
        high: 0xf47 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf49 as ::core::ffi::c_ushort,
        high: 0xf6a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf88 as ::core::ffi::c_ushort,
        high: 0xf8b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1000 as ::core::ffi::c_ushort,
        high: 0x1021 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1023 as ::core::ffi::c_ushort,
        high: 0x1027 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1029 as ::core::ffi::c_ushort,
        high: 0x102a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1050 as ::core::ffi::c_ushort,
        high: 0x1055 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10a0 as ::core::ffi::c_ushort,
        high: 0x10c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10d0 as ::core::ffi::c_ushort,
        high: 0x10f8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1100 as ::core::ffi::c_ushort,
        high: 0x1159 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x115f as ::core::ffi::c_ushort,
        high: 0x11a2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11a8 as ::core::ffi::c_ushort,
        high: 0x11f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1200 as ::core::ffi::c_ushort,
        high: 0x1206 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1208 as ::core::ffi::c_ushort,
        high: 0x1246 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1248 as ::core::ffi::c_ushort,
        high: 0x1248 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x124a as ::core::ffi::c_ushort,
        high: 0x124d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1250 as ::core::ffi::c_ushort,
        high: 0x1256 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1258 as ::core::ffi::c_ushort,
        high: 0x1258 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x125a as ::core::ffi::c_ushort,
        high: 0x125d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1260 as ::core::ffi::c_ushort,
        high: 0x1286 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1288 as ::core::ffi::c_ushort,
        high: 0x1288 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x128a as ::core::ffi::c_ushort,
        high: 0x128d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1290 as ::core::ffi::c_ushort,
        high: 0x12ae as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12b0 as ::core::ffi::c_ushort,
        high: 0x12b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12b2 as ::core::ffi::c_ushort,
        high: 0x12b5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12b8 as ::core::ffi::c_ushort,
        high: 0x12be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12c0 as ::core::ffi::c_ushort,
        high: 0x12c0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12c2 as ::core::ffi::c_ushort,
        high: 0x12c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12c8 as ::core::ffi::c_ushort,
        high: 0x12ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12d0 as ::core::ffi::c_ushort,
        high: 0x12d6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12d8 as ::core::ffi::c_ushort,
        high: 0x12ee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12f0 as ::core::ffi::c_ushort,
        high: 0x130e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1310 as ::core::ffi::c_ushort,
        high: 0x1310 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1312 as ::core::ffi::c_ushort,
        high: 0x1315 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1318 as ::core::ffi::c_ushort,
        high: 0x131e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1320 as ::core::ffi::c_ushort,
        high: 0x1346 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1348 as ::core::ffi::c_ushort,
        high: 0x135a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13a0 as ::core::ffi::c_ushort,
        high: 0x13f4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1401 as ::core::ffi::c_ushort,
        high: 0x166c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x166f as ::core::ffi::c_ushort,
        high: 0x1676 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1681 as ::core::ffi::c_ushort,
        high: 0x169a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16a0 as ::core::ffi::c_ushort,
        high: 0x16ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1700 as ::core::ffi::c_ushort,
        high: 0x170c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x170e as ::core::ffi::c_ushort,
        high: 0x1711 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1720 as ::core::ffi::c_ushort,
        high: 0x1731 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1740 as ::core::ffi::c_ushort,
        high: 0x1751 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1760 as ::core::ffi::c_ushort,
        high: 0x176c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x176e as ::core::ffi::c_ushort,
        high: 0x1770 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1780 as ::core::ffi::c_ushort,
        high: 0x17b3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17d7 as ::core::ffi::c_ushort,
        high: 0x17d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17dc as ::core::ffi::c_ushort,
        high: 0x17dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1820 as ::core::ffi::c_ushort,
        high: 0x1877 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1880 as ::core::ffi::c_ushort,
        high: 0x18a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1900 as ::core::ffi::c_ushort,
        high: 0x191c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1950 as ::core::ffi::c_ushort,
        high: 0x196d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1970 as ::core::ffi::c_ushort,
        high: 0x1974 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d00 as ::core::ffi::c_ushort,
        high: 0x1d6b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e00 as ::core::ffi::c_ushort,
        high: 0x1e9b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea0 as ::core::ffi::c_ushort,
        high: 0x1ef9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f00 as ::core::ffi::c_ushort,
        high: 0x1f15 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f18 as ::core::ffi::c_ushort,
        high: 0x1f1d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f20 as ::core::ffi::c_ushort,
        high: 0x1f45 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f48 as ::core::ffi::c_ushort,
        high: 0x1f4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f50 as ::core::ffi::c_ushort,
        high: 0x1f57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f59 as ::core::ffi::c_ushort,
        high: 0x1f59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5b as ::core::ffi::c_ushort,
        high: 0x1f5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5d as ::core::ffi::c_ushort,
        high: 0x1f5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5f as ::core::ffi::c_ushort,
        high: 0x1f7d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f80 as ::core::ffi::c_ushort,
        high: 0x1fb4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fb6 as ::core::ffi::c_ushort,
        high: 0x1fbc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbe as ::core::ffi::c_ushort,
        high: 0x1fbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc2 as ::core::ffi::c_ushort,
        high: 0x1fc4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc6 as ::core::ffi::c_ushort,
        high: 0x1fcc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd0 as ::core::ffi::c_ushort,
        high: 0x1fd3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd6 as ::core::ffi::c_ushort,
        high: 0x1fdb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fe0 as ::core::ffi::c_ushort,
        high: 0x1fec as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff2 as ::core::ffi::c_ushort,
        high: 0x1ff4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff6 as ::core::ffi::c_ushort,
        high: 0x1ffc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2071 as ::core::ffi::c_ushort,
        high: 0x2071 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207f as ::core::ffi::c_ushort,
        high: 0x207f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2102 as ::core::ffi::c_ushort,
        high: 0x2102 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2107 as ::core::ffi::c_ushort,
        high: 0x2107 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x210a as ::core::ffi::c_ushort,
        high: 0x2113 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2115 as ::core::ffi::c_ushort,
        high: 0x2115 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2119 as ::core::ffi::c_ushort,
        high: 0x211d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2124 as ::core::ffi::c_ushort,
        high: 0x2124 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2126 as ::core::ffi::c_ushort,
        high: 0x2126 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2128 as ::core::ffi::c_ushort,
        high: 0x2128 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212a as ::core::ffi::c_ushort,
        high: 0x212d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212f as ::core::ffi::c_ushort,
        high: 0x2131 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2133 as ::core::ffi::c_ushort,
        high: 0x2139 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x213d as ::core::ffi::c_ushort,
        high: 0x213f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2145 as ::core::ffi::c_ushort,
        high: 0x2149 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3005 as ::core::ffi::c_ushort,
        high: 0x3006 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3031 as ::core::ffi::c_ushort,
        high: 0x3035 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x303b as ::core::ffi::c_ushort,
        high: 0x303c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3041 as ::core::ffi::c_ushort,
        high: 0x3096 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x309d as ::core::ffi::c_ushort,
        high: 0x309f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30a1 as ::core::ffi::c_ushort,
        high: 0x30fa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30fc as ::core::ffi::c_ushort,
        high: 0x30ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3105 as ::core::ffi::c_ushort,
        high: 0x312c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3131 as ::core::ffi::c_ushort,
        high: 0x318e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x31a0 as ::core::ffi::c_ushort,
        high: 0x31b7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x31f0 as ::core::ffi::c_ushort,
        high: 0x31ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3400 as ::core::ffi::c_ushort,
        high: 0x3400 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4db5 as ::core::ffi::c_ushort,
        high: 0x4db5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e00 as ::core::ffi::c_ushort,
        high: 0x4e00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9fa5 as ::core::ffi::c_ushort,
        high: 0x9fa5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa000 as ::core::ffi::c_ushort,
        high: 0xa48c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac00 as ::core::ffi::c_ushort,
        high: 0xac00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd7a3 as ::core::ffi::c_ushort,
        high: 0xd7a3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf900 as ::core::ffi::c_ushort,
        high: 0xfa2d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfa30 as ::core::ffi::c_ushort,
        high: 0xfa6a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb00 as ::core::ffi::c_ushort,
        high: 0xfb06 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb13 as ::core::ffi::c_ushort,
        high: 0xfb17 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1d as ::core::ffi::c_ushort,
        high: 0xfb1d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1f as ::core::ffi::c_ushort,
        high: 0xfb28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb2a as ::core::ffi::c_ushort,
        high: 0xfb36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb38 as ::core::ffi::c_ushort,
        high: 0xfb3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb3e as ::core::ffi::c_ushort,
        high: 0xfb3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb40 as ::core::ffi::c_ushort,
        high: 0xfb41 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb43 as ::core::ffi::c_ushort,
        high: 0xfb44 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb46 as ::core::ffi::c_ushort,
        high: 0xfbb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfbd3 as ::core::ffi::c_ushort,
        high: 0xfd3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfd50 as ::core::ffi::c_ushort,
        high: 0xfd8f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfd92 as ::core::ffi::c_ushort,
        high: 0xfdc7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfdf0 as ::core::ffi::c_ushort,
        high: 0xfdfb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe70 as ::core::ffi::c_ushort,
        high: 0xfe74 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe76 as ::core::ffi::c_ushort,
        high: 0xfefc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff21 as ::core::ffi::c_ushort,
        high: 0xff3a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff41 as ::core::ffi::c_ushort,
        high: 0xff5a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff66 as ::core::ffi::c_ushort,
        high: 0xffbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffc2 as ::core::ffi::c_ushort,
        high: 0xffc7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffca as ::core::ffi::c_ushort,
        high: 0xffcf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffd2 as ::core::ffi::c_ushort,
        high: 0xffd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffda as ::core::ffi::c_ushort,
        high: 0xffdc as ::core::ffi::c_ushort,
    },
];
static mut xmlLL: [xmlChLRange; 50] = [
    _xmlChLRange {
        low: 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1000b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1000d as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10026 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10028 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1003a as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1003c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1003d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1003f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1004d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10050 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1005d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10080 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x100fa as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10300 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1031e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10330 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10349 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10380 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1039d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10400 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1049d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10800 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10805 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10808 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10808 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1080a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10835 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10837 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10838 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1083c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1083c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1083f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1083f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d400 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d454 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d456 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d49c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d49e as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d49f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4a2 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4a2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4a5 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4a6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4ac as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4ae as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4b9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4bb as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4bb as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4bd as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4c3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4c5 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d505 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d507 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d50a as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d50d as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d514 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d516 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d51c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d51e as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d539 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d53b as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d53e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d540 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d544 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d546 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d546 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d54a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d550 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d552 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6a3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6a8 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6c0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6c2 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6da as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6dc as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6fa as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6fc as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d714 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d716 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d734 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d736 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d74e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d750 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d76e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d770 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d788 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d78a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7a8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7aa as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7c2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7c4 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7c9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x20000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x20000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x2a6d6 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x2a6d6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x2f800 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x2fa1d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlLG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 279 as ::core::ffi::c_int,
        nbLongRange: 50 as ::core::ffi::c_int,
        shortRange: &raw const xmlLS as *const xmlChSRange,
        longRange: &raw const xmlLL as *const xmlChLRange,
    }
};
static mut xmlLlS: [xmlChSRange; 396] = [
    _xmlChSRange {
        low: 0x61 as ::core::ffi::c_ushort,
        high: 0x7a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaa as ::core::ffi::c_ushort,
        high: 0xaa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5 as ::core::ffi::c_ushort,
        high: 0xb5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba as ::core::ffi::c_ushort,
        high: 0xba as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdf as ::core::ffi::c_ushort,
        high: 0xf6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf8 as ::core::ffi::c_ushort,
        high: 0xff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x101 as ::core::ffi::c_ushort,
        high: 0x101 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x103 as ::core::ffi::c_ushort,
        high: 0x103 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x105 as ::core::ffi::c_ushort,
        high: 0x105 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x107 as ::core::ffi::c_ushort,
        high: 0x107 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x109 as ::core::ffi::c_ushort,
        high: 0x109 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10b as ::core::ffi::c_ushort,
        high: 0x10b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10d as ::core::ffi::c_ushort,
        high: 0x10d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10f as ::core::ffi::c_ushort,
        high: 0x10f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x111 as ::core::ffi::c_ushort,
        high: 0x111 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x113 as ::core::ffi::c_ushort,
        high: 0x113 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x115 as ::core::ffi::c_ushort,
        high: 0x115 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x117 as ::core::ffi::c_ushort,
        high: 0x117 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x119 as ::core::ffi::c_ushort,
        high: 0x119 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11b as ::core::ffi::c_ushort,
        high: 0x11b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11d as ::core::ffi::c_ushort,
        high: 0x11d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11f as ::core::ffi::c_ushort,
        high: 0x11f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x121 as ::core::ffi::c_ushort,
        high: 0x121 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x123 as ::core::ffi::c_ushort,
        high: 0x123 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x125 as ::core::ffi::c_ushort,
        high: 0x125 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x127 as ::core::ffi::c_ushort,
        high: 0x127 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x129 as ::core::ffi::c_ushort,
        high: 0x129 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12b as ::core::ffi::c_ushort,
        high: 0x12b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12d as ::core::ffi::c_ushort,
        high: 0x12d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12f as ::core::ffi::c_ushort,
        high: 0x12f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x131 as ::core::ffi::c_ushort,
        high: 0x131 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x133 as ::core::ffi::c_ushort,
        high: 0x133 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x135 as ::core::ffi::c_ushort,
        high: 0x135 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x137 as ::core::ffi::c_ushort,
        high: 0x138 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13a as ::core::ffi::c_ushort,
        high: 0x13a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13c as ::core::ffi::c_ushort,
        high: 0x13c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13e as ::core::ffi::c_ushort,
        high: 0x13e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x140 as ::core::ffi::c_ushort,
        high: 0x140 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x142 as ::core::ffi::c_ushort,
        high: 0x142 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x144 as ::core::ffi::c_ushort,
        high: 0x144 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x146 as ::core::ffi::c_ushort,
        high: 0x146 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x148 as ::core::ffi::c_ushort,
        high: 0x149 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x14b as ::core::ffi::c_ushort,
        high: 0x14b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x14d as ::core::ffi::c_ushort,
        high: 0x14d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x14f as ::core::ffi::c_ushort,
        high: 0x14f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x151 as ::core::ffi::c_ushort,
        high: 0x151 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x153 as ::core::ffi::c_ushort,
        high: 0x153 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x155 as ::core::ffi::c_ushort,
        high: 0x155 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x157 as ::core::ffi::c_ushort,
        high: 0x157 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x159 as ::core::ffi::c_ushort,
        high: 0x159 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x15b as ::core::ffi::c_ushort,
        high: 0x15b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x15d as ::core::ffi::c_ushort,
        high: 0x15d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x15f as ::core::ffi::c_ushort,
        high: 0x15f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x161 as ::core::ffi::c_ushort,
        high: 0x161 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x163 as ::core::ffi::c_ushort,
        high: 0x163 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x165 as ::core::ffi::c_ushort,
        high: 0x165 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x167 as ::core::ffi::c_ushort,
        high: 0x167 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x169 as ::core::ffi::c_ushort,
        high: 0x169 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16b as ::core::ffi::c_ushort,
        high: 0x16b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16d as ::core::ffi::c_ushort,
        high: 0x16d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16f as ::core::ffi::c_ushort,
        high: 0x16f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x171 as ::core::ffi::c_ushort,
        high: 0x171 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x173 as ::core::ffi::c_ushort,
        high: 0x173 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x175 as ::core::ffi::c_ushort,
        high: 0x175 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x177 as ::core::ffi::c_ushort,
        high: 0x177 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17a as ::core::ffi::c_ushort,
        high: 0x17a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17c as ::core::ffi::c_ushort,
        high: 0x17c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17e as ::core::ffi::c_ushort,
        high: 0x180 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x183 as ::core::ffi::c_ushort,
        high: 0x183 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x185 as ::core::ffi::c_ushort,
        high: 0x185 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x188 as ::core::ffi::c_ushort,
        high: 0x188 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x18c as ::core::ffi::c_ushort,
        high: 0x18d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x192 as ::core::ffi::c_ushort,
        high: 0x192 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x195 as ::core::ffi::c_ushort,
        high: 0x195 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x199 as ::core::ffi::c_ushort,
        high: 0x19b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x19e as ::core::ffi::c_ushort,
        high: 0x19e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a1 as ::core::ffi::c_ushort,
        high: 0x1a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a3 as ::core::ffi::c_ushort,
        high: 0x1a3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a5 as ::core::ffi::c_ushort,
        high: 0x1a5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a8 as ::core::ffi::c_ushort,
        high: 0x1a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1aa as ::core::ffi::c_ushort,
        high: 0x1ab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ad as ::core::ffi::c_ushort,
        high: 0x1ad as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1b0 as ::core::ffi::c_ushort,
        high: 0x1b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1b4 as ::core::ffi::c_ushort,
        high: 0x1b4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1b6 as ::core::ffi::c_ushort,
        high: 0x1b6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1b9 as ::core::ffi::c_ushort,
        high: 0x1ba as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1bd as ::core::ffi::c_ushort,
        high: 0x1bf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1c6 as ::core::ffi::c_ushort,
        high: 0x1c6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1c9 as ::core::ffi::c_ushort,
        high: 0x1c9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1cc as ::core::ffi::c_ushort,
        high: 0x1cc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ce as ::core::ffi::c_ushort,
        high: 0x1ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d0 as ::core::ffi::c_ushort,
        high: 0x1d0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d2 as ::core::ffi::c_ushort,
        high: 0x1d2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d4 as ::core::ffi::c_ushort,
        high: 0x1d4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d6 as ::core::ffi::c_ushort,
        high: 0x1d6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d8 as ::core::ffi::c_ushort,
        high: 0x1d8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1da as ::core::ffi::c_ushort,
        high: 0x1da as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1dc as ::core::ffi::c_ushort,
        high: 0x1dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1df as ::core::ffi::c_ushort,
        high: 0x1df as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e1 as ::core::ffi::c_ushort,
        high: 0x1e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e3 as ::core::ffi::c_ushort,
        high: 0x1e3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e5 as ::core::ffi::c_ushort,
        high: 0x1e5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e7 as ::core::ffi::c_ushort,
        high: 0x1e7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e9 as ::core::ffi::c_ushort,
        high: 0x1e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb as ::core::ffi::c_ushort,
        high: 0x1eb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed as ::core::ffi::c_ushort,
        high: 0x1ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef as ::core::ffi::c_ushort,
        high: 0x1f0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f3 as ::core::ffi::c_ushort,
        high: 0x1f3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5 as ::core::ffi::c_ushort,
        high: 0x1f5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f9 as ::core::ffi::c_ushort,
        high: 0x1f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fb as ::core::ffi::c_ushort,
        high: 0x1fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd as ::core::ffi::c_ushort,
        high: 0x1fd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff as ::core::ffi::c_ushort,
        high: 0x1ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x201 as ::core::ffi::c_ushort,
        high: 0x201 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x203 as ::core::ffi::c_ushort,
        high: 0x203 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x205 as ::core::ffi::c_ushort,
        high: 0x205 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207 as ::core::ffi::c_ushort,
        high: 0x207 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x209 as ::core::ffi::c_ushort,
        high: 0x209 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20b as ::core::ffi::c_ushort,
        high: 0x20b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20d as ::core::ffi::c_ushort,
        high: 0x20d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20f as ::core::ffi::c_ushort,
        high: 0x20f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x211 as ::core::ffi::c_ushort,
        high: 0x211 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x213 as ::core::ffi::c_ushort,
        high: 0x213 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x215 as ::core::ffi::c_ushort,
        high: 0x215 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x217 as ::core::ffi::c_ushort,
        high: 0x217 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x219 as ::core::ffi::c_ushort,
        high: 0x219 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21b as ::core::ffi::c_ushort,
        high: 0x21b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21d as ::core::ffi::c_ushort,
        high: 0x21d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21f as ::core::ffi::c_ushort,
        high: 0x21f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x221 as ::core::ffi::c_ushort,
        high: 0x221 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x223 as ::core::ffi::c_ushort,
        high: 0x223 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x225 as ::core::ffi::c_ushort,
        high: 0x225 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x227 as ::core::ffi::c_ushort,
        high: 0x227 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x229 as ::core::ffi::c_ushort,
        high: 0x229 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x22b as ::core::ffi::c_ushort,
        high: 0x22b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x22d as ::core::ffi::c_ushort,
        high: 0x22d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x22f as ::core::ffi::c_ushort,
        high: 0x22f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x231 as ::core::ffi::c_ushort,
        high: 0x231 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x233 as ::core::ffi::c_ushort,
        high: 0x236 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x250 as ::core::ffi::c_ushort,
        high: 0x2af as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x390 as ::core::ffi::c_ushort,
        high: 0x390 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3ac as ::core::ffi::c_ushort,
        high: 0x3ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3d0 as ::core::ffi::c_ushort,
        high: 0x3d1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3d5 as ::core::ffi::c_ushort,
        high: 0x3d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3d9 as ::core::ffi::c_ushort,
        high: 0x3d9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3db as ::core::ffi::c_ushort,
        high: 0x3db as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3dd as ::core::ffi::c_ushort,
        high: 0x3dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3df as ::core::ffi::c_ushort,
        high: 0x3df as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e1 as ::core::ffi::c_ushort,
        high: 0x3e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e3 as ::core::ffi::c_ushort,
        high: 0x3e3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e5 as ::core::ffi::c_ushort,
        high: 0x3e5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e7 as ::core::ffi::c_ushort,
        high: 0x3e7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e9 as ::core::ffi::c_ushort,
        high: 0x3e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3eb as ::core::ffi::c_ushort,
        high: 0x3eb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3ed as ::core::ffi::c_ushort,
        high: 0x3ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3ef as ::core::ffi::c_ushort,
        high: 0x3f3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f5 as ::core::ffi::c_ushort,
        high: 0x3f5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f8 as ::core::ffi::c_ushort,
        high: 0x3f8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3fb as ::core::ffi::c_ushort,
        high: 0x3fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x430 as ::core::ffi::c_ushort,
        high: 0x45f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x461 as ::core::ffi::c_ushort,
        high: 0x461 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x463 as ::core::ffi::c_ushort,
        high: 0x463 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x465 as ::core::ffi::c_ushort,
        high: 0x465 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x467 as ::core::ffi::c_ushort,
        high: 0x467 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x469 as ::core::ffi::c_ushort,
        high: 0x469 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x46b as ::core::ffi::c_ushort,
        high: 0x46b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x46d as ::core::ffi::c_ushort,
        high: 0x46d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x46f as ::core::ffi::c_ushort,
        high: 0x46f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x471 as ::core::ffi::c_ushort,
        high: 0x471 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x473 as ::core::ffi::c_ushort,
        high: 0x473 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x475 as ::core::ffi::c_ushort,
        high: 0x475 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x477 as ::core::ffi::c_ushort,
        high: 0x477 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x479 as ::core::ffi::c_ushort,
        high: 0x479 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x47b as ::core::ffi::c_ushort,
        high: 0x47b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x47d as ::core::ffi::c_ushort,
        high: 0x47d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x47f as ::core::ffi::c_ushort,
        high: 0x47f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x481 as ::core::ffi::c_ushort,
        high: 0x481 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x48b as ::core::ffi::c_ushort,
        high: 0x48b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x48d as ::core::ffi::c_ushort,
        high: 0x48d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x48f as ::core::ffi::c_ushort,
        high: 0x48f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x491 as ::core::ffi::c_ushort,
        high: 0x491 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x493 as ::core::ffi::c_ushort,
        high: 0x493 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x495 as ::core::ffi::c_ushort,
        high: 0x495 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x497 as ::core::ffi::c_ushort,
        high: 0x497 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x499 as ::core::ffi::c_ushort,
        high: 0x499 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x49b as ::core::ffi::c_ushort,
        high: 0x49b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x49d as ::core::ffi::c_ushort,
        high: 0x49d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x49f as ::core::ffi::c_ushort,
        high: 0x49f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a1 as ::core::ffi::c_ushort,
        high: 0x4a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a3 as ::core::ffi::c_ushort,
        high: 0x4a3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a5 as ::core::ffi::c_ushort,
        high: 0x4a5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a7 as ::core::ffi::c_ushort,
        high: 0x4a7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a9 as ::core::ffi::c_ushort,
        high: 0x4a9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ab as ::core::ffi::c_ushort,
        high: 0x4ab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ad as ::core::ffi::c_ushort,
        high: 0x4ad as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4af as ::core::ffi::c_ushort,
        high: 0x4af as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b1 as ::core::ffi::c_ushort,
        high: 0x4b1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b3 as ::core::ffi::c_ushort,
        high: 0x4b3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b5 as ::core::ffi::c_ushort,
        high: 0x4b5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b7 as ::core::ffi::c_ushort,
        high: 0x4b7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b9 as ::core::ffi::c_ushort,
        high: 0x4b9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4bb as ::core::ffi::c_ushort,
        high: 0x4bb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4bd as ::core::ffi::c_ushort,
        high: 0x4bd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4bf as ::core::ffi::c_ushort,
        high: 0x4bf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c2 as ::core::ffi::c_ushort,
        high: 0x4c2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c4 as ::core::ffi::c_ushort,
        high: 0x4c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c6 as ::core::ffi::c_ushort,
        high: 0x4c6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c8 as ::core::ffi::c_ushort,
        high: 0x4c8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ca as ::core::ffi::c_ushort,
        high: 0x4ca as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4cc as ::core::ffi::c_ushort,
        high: 0x4cc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ce as ::core::ffi::c_ushort,
        high: 0x4ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d1 as ::core::ffi::c_ushort,
        high: 0x4d1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d3 as ::core::ffi::c_ushort,
        high: 0x4d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d5 as ::core::ffi::c_ushort,
        high: 0x4d5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d7 as ::core::ffi::c_ushort,
        high: 0x4d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d9 as ::core::ffi::c_ushort,
        high: 0x4d9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4db as ::core::ffi::c_ushort,
        high: 0x4db as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4dd as ::core::ffi::c_ushort,
        high: 0x4dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4df as ::core::ffi::c_ushort,
        high: 0x4df as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e1 as ::core::ffi::c_ushort,
        high: 0x4e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e3 as ::core::ffi::c_ushort,
        high: 0x4e3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e5 as ::core::ffi::c_ushort,
        high: 0x4e5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e7 as ::core::ffi::c_ushort,
        high: 0x4e7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e9 as ::core::ffi::c_ushort,
        high: 0x4e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4eb as ::core::ffi::c_ushort,
        high: 0x4eb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ed as ::core::ffi::c_ushort,
        high: 0x4ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ef as ::core::ffi::c_ushort,
        high: 0x4ef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f1 as ::core::ffi::c_ushort,
        high: 0x4f1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f3 as ::core::ffi::c_ushort,
        high: 0x4f3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f5 as ::core::ffi::c_ushort,
        high: 0x4f5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f9 as ::core::ffi::c_ushort,
        high: 0x4f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x501 as ::core::ffi::c_ushort,
        high: 0x501 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x503 as ::core::ffi::c_ushort,
        high: 0x503 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x505 as ::core::ffi::c_ushort,
        high: 0x505 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x507 as ::core::ffi::c_ushort,
        high: 0x507 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x509 as ::core::ffi::c_ushort,
        high: 0x509 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x50b as ::core::ffi::c_ushort,
        high: 0x50b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x50d as ::core::ffi::c_ushort,
        high: 0x50d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x50f as ::core::ffi::c_ushort,
        high: 0x50f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x561 as ::core::ffi::c_ushort,
        high: 0x587 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d00 as ::core::ffi::c_ushort,
        high: 0x1d2b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d62 as ::core::ffi::c_ushort,
        high: 0x1d6b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e01 as ::core::ffi::c_ushort,
        high: 0x1e01 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e03 as ::core::ffi::c_ushort,
        high: 0x1e03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e05 as ::core::ffi::c_ushort,
        high: 0x1e05 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e07 as ::core::ffi::c_ushort,
        high: 0x1e07 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e09 as ::core::ffi::c_ushort,
        high: 0x1e09 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e0b as ::core::ffi::c_ushort,
        high: 0x1e0b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e0d as ::core::ffi::c_ushort,
        high: 0x1e0d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e0f as ::core::ffi::c_ushort,
        high: 0x1e0f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e11 as ::core::ffi::c_ushort,
        high: 0x1e11 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e13 as ::core::ffi::c_ushort,
        high: 0x1e13 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e15 as ::core::ffi::c_ushort,
        high: 0x1e15 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e17 as ::core::ffi::c_ushort,
        high: 0x1e17 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e19 as ::core::ffi::c_ushort,
        high: 0x1e19 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e1b as ::core::ffi::c_ushort,
        high: 0x1e1b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e1d as ::core::ffi::c_ushort,
        high: 0x1e1d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e1f as ::core::ffi::c_ushort,
        high: 0x1e1f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e21 as ::core::ffi::c_ushort,
        high: 0x1e21 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e23 as ::core::ffi::c_ushort,
        high: 0x1e23 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e25 as ::core::ffi::c_ushort,
        high: 0x1e25 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e27 as ::core::ffi::c_ushort,
        high: 0x1e27 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e29 as ::core::ffi::c_ushort,
        high: 0x1e29 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e2b as ::core::ffi::c_ushort,
        high: 0x1e2b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e2d as ::core::ffi::c_ushort,
        high: 0x1e2d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e2f as ::core::ffi::c_ushort,
        high: 0x1e2f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e31 as ::core::ffi::c_ushort,
        high: 0x1e31 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e33 as ::core::ffi::c_ushort,
        high: 0x1e33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e35 as ::core::ffi::c_ushort,
        high: 0x1e35 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e37 as ::core::ffi::c_ushort,
        high: 0x1e37 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e39 as ::core::ffi::c_ushort,
        high: 0x1e39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e3b as ::core::ffi::c_ushort,
        high: 0x1e3b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e3d as ::core::ffi::c_ushort,
        high: 0x1e3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e3f as ::core::ffi::c_ushort,
        high: 0x1e3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e41 as ::core::ffi::c_ushort,
        high: 0x1e41 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e43 as ::core::ffi::c_ushort,
        high: 0x1e43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e45 as ::core::ffi::c_ushort,
        high: 0x1e45 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e47 as ::core::ffi::c_ushort,
        high: 0x1e47 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e49 as ::core::ffi::c_ushort,
        high: 0x1e49 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e4b as ::core::ffi::c_ushort,
        high: 0x1e4b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e4d as ::core::ffi::c_ushort,
        high: 0x1e4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e4f as ::core::ffi::c_ushort,
        high: 0x1e4f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e51 as ::core::ffi::c_ushort,
        high: 0x1e51 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e53 as ::core::ffi::c_ushort,
        high: 0x1e53 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e55 as ::core::ffi::c_ushort,
        high: 0x1e55 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e57 as ::core::ffi::c_ushort,
        high: 0x1e57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e59 as ::core::ffi::c_ushort,
        high: 0x1e59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e5b as ::core::ffi::c_ushort,
        high: 0x1e5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e5d as ::core::ffi::c_ushort,
        high: 0x1e5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e5f as ::core::ffi::c_ushort,
        high: 0x1e5f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e61 as ::core::ffi::c_ushort,
        high: 0x1e61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e63 as ::core::ffi::c_ushort,
        high: 0x1e63 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e65 as ::core::ffi::c_ushort,
        high: 0x1e65 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e67 as ::core::ffi::c_ushort,
        high: 0x1e67 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e69 as ::core::ffi::c_ushort,
        high: 0x1e69 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e6b as ::core::ffi::c_ushort,
        high: 0x1e6b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e6d as ::core::ffi::c_ushort,
        high: 0x1e6d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e6f as ::core::ffi::c_ushort,
        high: 0x1e6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e71 as ::core::ffi::c_ushort,
        high: 0x1e71 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e73 as ::core::ffi::c_ushort,
        high: 0x1e73 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e75 as ::core::ffi::c_ushort,
        high: 0x1e75 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e77 as ::core::ffi::c_ushort,
        high: 0x1e77 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e79 as ::core::ffi::c_ushort,
        high: 0x1e79 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e7b as ::core::ffi::c_ushort,
        high: 0x1e7b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e7d as ::core::ffi::c_ushort,
        high: 0x1e7d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e7f as ::core::ffi::c_ushort,
        high: 0x1e7f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e81 as ::core::ffi::c_ushort,
        high: 0x1e81 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e83 as ::core::ffi::c_ushort,
        high: 0x1e83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e85 as ::core::ffi::c_ushort,
        high: 0x1e85 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e87 as ::core::ffi::c_ushort,
        high: 0x1e87 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e89 as ::core::ffi::c_ushort,
        high: 0x1e89 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e8b as ::core::ffi::c_ushort,
        high: 0x1e8b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e8d as ::core::ffi::c_ushort,
        high: 0x1e8d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e8f as ::core::ffi::c_ushort,
        high: 0x1e8f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e91 as ::core::ffi::c_ushort,
        high: 0x1e91 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e93 as ::core::ffi::c_ushort,
        high: 0x1e93 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e95 as ::core::ffi::c_ushort,
        high: 0x1e9b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea1 as ::core::ffi::c_ushort,
        high: 0x1ea1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea3 as ::core::ffi::c_ushort,
        high: 0x1ea3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea5 as ::core::ffi::c_ushort,
        high: 0x1ea5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea7 as ::core::ffi::c_ushort,
        high: 0x1ea7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea9 as ::core::ffi::c_ushort,
        high: 0x1ea9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eab as ::core::ffi::c_ushort,
        high: 0x1eab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ead as ::core::ffi::c_ushort,
        high: 0x1ead as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eaf as ::core::ffi::c_ushort,
        high: 0x1eaf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb1 as ::core::ffi::c_ushort,
        high: 0x1eb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb3 as ::core::ffi::c_ushort,
        high: 0x1eb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb5 as ::core::ffi::c_ushort,
        high: 0x1eb5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb7 as ::core::ffi::c_ushort,
        high: 0x1eb7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb9 as ::core::ffi::c_ushort,
        high: 0x1eb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ebb as ::core::ffi::c_ushort,
        high: 0x1ebb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ebd as ::core::ffi::c_ushort,
        high: 0x1ebd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ebf as ::core::ffi::c_ushort,
        high: 0x1ebf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec1 as ::core::ffi::c_ushort,
        high: 0x1ec1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec3 as ::core::ffi::c_ushort,
        high: 0x1ec3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec5 as ::core::ffi::c_ushort,
        high: 0x1ec5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec7 as ::core::ffi::c_ushort,
        high: 0x1ec7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec9 as ::core::ffi::c_ushort,
        high: 0x1ec9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ecb as ::core::ffi::c_ushort,
        high: 0x1ecb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ecd as ::core::ffi::c_ushort,
        high: 0x1ecd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ecf as ::core::ffi::c_ushort,
        high: 0x1ecf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed1 as ::core::ffi::c_ushort,
        high: 0x1ed1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed3 as ::core::ffi::c_ushort,
        high: 0x1ed3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed5 as ::core::ffi::c_ushort,
        high: 0x1ed5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed7 as ::core::ffi::c_ushort,
        high: 0x1ed7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed9 as ::core::ffi::c_ushort,
        high: 0x1ed9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1edb as ::core::ffi::c_ushort,
        high: 0x1edb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1edd as ::core::ffi::c_ushort,
        high: 0x1edd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1edf as ::core::ffi::c_ushort,
        high: 0x1edf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee1 as ::core::ffi::c_ushort,
        high: 0x1ee1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee3 as ::core::ffi::c_ushort,
        high: 0x1ee3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee5 as ::core::ffi::c_ushort,
        high: 0x1ee5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee7 as ::core::ffi::c_ushort,
        high: 0x1ee7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee9 as ::core::ffi::c_ushort,
        high: 0x1ee9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eeb as ::core::ffi::c_ushort,
        high: 0x1eeb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eed as ::core::ffi::c_ushort,
        high: 0x1eed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eef as ::core::ffi::c_ushort,
        high: 0x1eef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef1 as ::core::ffi::c_ushort,
        high: 0x1ef1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef3 as ::core::ffi::c_ushort,
        high: 0x1ef3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef5 as ::core::ffi::c_ushort,
        high: 0x1ef5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef7 as ::core::ffi::c_ushort,
        high: 0x1ef7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef9 as ::core::ffi::c_ushort,
        high: 0x1ef9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f00 as ::core::ffi::c_ushort,
        high: 0x1f07 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f10 as ::core::ffi::c_ushort,
        high: 0x1f15 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f20 as ::core::ffi::c_ushort,
        high: 0x1f27 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f30 as ::core::ffi::c_ushort,
        high: 0x1f37 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f40 as ::core::ffi::c_ushort,
        high: 0x1f45 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f50 as ::core::ffi::c_ushort,
        high: 0x1f57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f60 as ::core::ffi::c_ushort,
        high: 0x1f67 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f70 as ::core::ffi::c_ushort,
        high: 0x1f7d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f80 as ::core::ffi::c_ushort,
        high: 0x1f87 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f90 as ::core::ffi::c_ushort,
        high: 0x1f97 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fa0 as ::core::ffi::c_ushort,
        high: 0x1fa7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fb0 as ::core::ffi::c_ushort,
        high: 0x1fb4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fb6 as ::core::ffi::c_ushort,
        high: 0x1fb7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbe as ::core::ffi::c_ushort,
        high: 0x1fbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc2 as ::core::ffi::c_ushort,
        high: 0x1fc4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc6 as ::core::ffi::c_ushort,
        high: 0x1fc7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd0 as ::core::ffi::c_ushort,
        high: 0x1fd3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd6 as ::core::ffi::c_ushort,
        high: 0x1fd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fe0 as ::core::ffi::c_ushort,
        high: 0x1fe7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff2 as ::core::ffi::c_ushort,
        high: 0x1ff4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff6 as ::core::ffi::c_ushort,
        high: 0x1ff7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2071 as ::core::ffi::c_ushort,
        high: 0x2071 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207f as ::core::ffi::c_ushort,
        high: 0x207f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x210a as ::core::ffi::c_ushort,
        high: 0x210a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x210e as ::core::ffi::c_ushort,
        high: 0x210f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2113 as ::core::ffi::c_ushort,
        high: 0x2113 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212f as ::core::ffi::c_ushort,
        high: 0x212f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2134 as ::core::ffi::c_ushort,
        high: 0x2134 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2139 as ::core::ffi::c_ushort,
        high: 0x2139 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x213d as ::core::ffi::c_ushort,
        high: 0x213d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2146 as ::core::ffi::c_ushort,
        high: 0x2149 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb00 as ::core::ffi::c_ushort,
        high: 0xfb06 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb13 as ::core::ffi::c_ushort,
        high: 0xfb17 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff41 as ::core::ffi::c_ushort,
        high: 0xff5a as ::core::ffi::c_ushort,
    },
];
static mut xmlLlL: [xmlChLRange; 28] = [
    _xmlChLRange {
        low: 0x10428 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1044f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d41a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d433 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d44e as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d454 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d456 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d467 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d482 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d49b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4b6 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4b9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4bb as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4bb as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4bd as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4c3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4c5 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4cf as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4ea as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d503 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d51e as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d537 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d552 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d56b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d586 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d59f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d5ba as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d5d3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d5ee as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d607 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d622 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d63b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d656 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d66f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d68a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6a3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6c2 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6da as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6dc as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6e1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6fc as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d714 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d716 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d71b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d736 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d74e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d750 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d755 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d770 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d788 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d78a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d78f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7aa as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7c2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7c4 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7c9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlLlG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 396 as ::core::ffi::c_int,
        nbLongRange: 28 as ::core::ffi::c_int,
        shortRange: &raw const xmlLlS as *const xmlChSRange,
        longRange: &raw const xmlLlL as *const xmlChLRange,
    }
};
static mut xmlLmS: [xmlChSRange; 20] = [
    _xmlChSRange {
        low: 0x2b0 as ::core::ffi::c_ushort,
        high: 0x2c1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2c6 as ::core::ffi::c_ushort,
        high: 0x2d1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e0 as ::core::ffi::c_ushort,
        high: 0x2e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2ee as ::core::ffi::c_ushort,
        high: 0x2ee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x37a as ::core::ffi::c_ushort,
        high: 0x37a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x559 as ::core::ffi::c_ushort,
        high: 0x559 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x640 as ::core::ffi::c_ushort,
        high: 0x640 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e5 as ::core::ffi::c_ushort,
        high: 0x6e6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe46 as ::core::ffi::c_ushort,
        high: 0xe46 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec6 as ::core::ffi::c_ushort,
        high: 0xec6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17d7 as ::core::ffi::c_ushort,
        high: 0x17d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1843 as ::core::ffi::c_ushort,
        high: 0x1843 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d2c as ::core::ffi::c_ushort,
        high: 0x1d61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3005 as ::core::ffi::c_ushort,
        high: 0x3005 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3031 as ::core::ffi::c_ushort,
        high: 0x3035 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x303b as ::core::ffi::c_ushort,
        high: 0x303b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x309d as ::core::ffi::c_ushort,
        high: 0x309e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30fc as ::core::ffi::c_ushort,
        high: 0x30fe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff70 as ::core::ffi::c_ushort,
        high: 0xff70 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff9e as ::core::ffi::c_ushort,
        high: 0xff9f as ::core::ffi::c_ushort,
    },
];
static mut xmlLmG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 20 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlLmS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlLoS: [xmlChSRange; 211] = [
    _xmlChSRange {
        low: 0x1bb as ::core::ffi::c_ushort,
        high: 0x1bb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1c0 as ::core::ffi::c_ushort,
        high: 0x1c3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5d0 as ::core::ffi::c_ushort,
        high: 0x5ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5f0 as ::core::ffi::c_ushort,
        high: 0x5f2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x621 as ::core::ffi::c_ushort,
        high: 0x63a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x641 as ::core::ffi::c_ushort,
        high: 0x64a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x66e as ::core::ffi::c_ushort,
        high: 0x66f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x671 as ::core::ffi::c_ushort,
        high: 0x6d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d5 as ::core::ffi::c_ushort,
        high: 0x6d5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ee as ::core::ffi::c_ushort,
        high: 0x6ef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6fa as ::core::ffi::c_ushort,
        high: 0x6fc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ff as ::core::ffi::c_ushort,
        high: 0x6ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x710 as ::core::ffi::c_ushort,
        high: 0x710 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x712 as ::core::ffi::c_ushort,
        high: 0x72f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x74d as ::core::ffi::c_ushort,
        high: 0x74f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x780 as ::core::ffi::c_ushort,
        high: 0x7a5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7b1 as ::core::ffi::c_ushort,
        high: 0x7b1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x904 as ::core::ffi::c_ushort,
        high: 0x939 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93d as ::core::ffi::c_ushort,
        high: 0x93d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x950 as ::core::ffi::c_ushort,
        high: 0x950 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x958 as ::core::ffi::c_ushort,
        high: 0x961 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x985 as ::core::ffi::c_ushort,
        high: 0x98c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x98f as ::core::ffi::c_ushort,
        high: 0x990 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x993 as ::core::ffi::c_ushort,
        high: 0x9a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9aa as ::core::ffi::c_ushort,
        high: 0x9b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9b2 as ::core::ffi::c_ushort,
        high: 0x9b2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9b6 as ::core::ffi::c_ushort,
        high: 0x9b9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9bd as ::core::ffi::c_ushort,
        high: 0x9bd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9dc as ::core::ffi::c_ushort,
        high: 0x9dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9df as ::core::ffi::c_ushort,
        high: 0x9e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9f0 as ::core::ffi::c_ushort,
        high: 0x9f1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa05 as ::core::ffi::c_ushort,
        high: 0xa0a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa0f as ::core::ffi::c_ushort,
        high: 0xa10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa13 as ::core::ffi::c_ushort,
        high: 0xa28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa2a as ::core::ffi::c_ushort,
        high: 0xa30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa32 as ::core::ffi::c_ushort,
        high: 0xa33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa35 as ::core::ffi::c_ushort,
        high: 0xa36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa38 as ::core::ffi::c_ushort,
        high: 0xa39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa59 as ::core::ffi::c_ushort,
        high: 0xa5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa5e as ::core::ffi::c_ushort,
        high: 0xa5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa72 as ::core::ffi::c_ushort,
        high: 0xa74 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa85 as ::core::ffi::c_ushort,
        high: 0xa8d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa8f as ::core::ffi::c_ushort,
        high: 0xa91 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa93 as ::core::ffi::c_ushort,
        high: 0xaa8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaaa as ::core::ffi::c_ushort,
        high: 0xab0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xab2 as ::core::ffi::c_ushort,
        high: 0xab3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xab5 as ::core::ffi::c_ushort,
        high: 0xab9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabd as ::core::ffi::c_ushort,
        high: 0xabd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xad0 as ::core::ffi::c_ushort,
        high: 0xad0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae0 as ::core::ffi::c_ushort,
        high: 0xae1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb05 as ::core::ffi::c_ushort,
        high: 0xb0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb0f as ::core::ffi::c_ushort,
        high: 0xb10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb13 as ::core::ffi::c_ushort,
        high: 0xb28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb2a as ::core::ffi::c_ushort,
        high: 0xb30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb32 as ::core::ffi::c_ushort,
        high: 0xb33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb35 as ::core::ffi::c_ushort,
        high: 0xb39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3d as ::core::ffi::c_ushort,
        high: 0xb3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5c as ::core::ffi::c_ushort,
        high: 0xb5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb5f as ::core::ffi::c_ushort,
        high: 0xb61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb71 as ::core::ffi::c_ushort,
        high: 0xb71 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb83 as ::core::ffi::c_ushort,
        high: 0xb83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb85 as ::core::ffi::c_ushort,
        high: 0xb8a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb8e as ::core::ffi::c_ushort,
        high: 0xb90 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb92 as ::core::ffi::c_ushort,
        high: 0xb95 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb99 as ::core::ffi::c_ushort,
        high: 0xb9a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9c as ::core::ffi::c_ushort,
        high: 0xb9c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9e as ::core::ffi::c_ushort,
        high: 0xb9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba3 as ::core::ffi::c_ushort,
        high: 0xba4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xba8 as ::core::ffi::c_ushort,
        high: 0xbaa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbae as ::core::ffi::c_ushort,
        high: 0xbb5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbb7 as ::core::ffi::c_ushort,
        high: 0xbb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc05 as ::core::ffi::c_ushort,
        high: 0xc0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc0e as ::core::ffi::c_ushort,
        high: 0xc10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc12 as ::core::ffi::c_ushort,
        high: 0xc28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc2a as ::core::ffi::c_ushort,
        high: 0xc33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc35 as ::core::ffi::c_ushort,
        high: 0xc39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc60 as ::core::ffi::c_ushort,
        high: 0xc61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc85 as ::core::ffi::c_ushort,
        high: 0xc8c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc8e as ::core::ffi::c_ushort,
        high: 0xc90 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc92 as ::core::ffi::c_ushort,
        high: 0xca8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcaa as ::core::ffi::c_ushort,
        high: 0xcb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcb5 as ::core::ffi::c_ushort,
        high: 0xcb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbd as ::core::ffi::c_ushort,
        high: 0xcbd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcde as ::core::ffi::c_ushort,
        high: 0xcde as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xce0 as ::core::ffi::c_ushort,
        high: 0xce1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd05 as ::core::ffi::c_ushort,
        high: 0xd0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd0e as ::core::ffi::c_ushort,
        high: 0xd10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd12 as ::core::ffi::c_ushort,
        high: 0xd28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd2a as ::core::ffi::c_ushort,
        high: 0xd39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd60 as ::core::ffi::c_ushort,
        high: 0xd61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd85 as ::core::ffi::c_ushort,
        high: 0xd96 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd9a as ::core::ffi::c_ushort,
        high: 0xdb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdb3 as ::core::ffi::c_ushort,
        high: 0xdbb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdbd as ::core::ffi::c_ushort,
        high: 0xdbd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdc0 as ::core::ffi::c_ushort,
        high: 0xdc6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe01 as ::core::ffi::c_ushort,
        high: 0xe30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe32 as ::core::ffi::c_ushort,
        high: 0xe33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe40 as ::core::ffi::c_ushort,
        high: 0xe45 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe81 as ::core::ffi::c_ushort,
        high: 0xe82 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe84 as ::core::ffi::c_ushort,
        high: 0xe84 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe87 as ::core::ffi::c_ushort,
        high: 0xe88 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe8a as ::core::ffi::c_ushort,
        high: 0xe8a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe8d as ::core::ffi::c_ushort,
        high: 0xe8d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe94 as ::core::ffi::c_ushort,
        high: 0xe97 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe99 as ::core::ffi::c_ushort,
        high: 0xe9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea1 as ::core::ffi::c_ushort,
        high: 0xea3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea5 as ::core::ffi::c_ushort,
        high: 0xea5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xea7 as ::core::ffi::c_ushort,
        high: 0xea7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeaa as ::core::ffi::c_ushort,
        high: 0xeab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xead as ::core::ffi::c_ushort,
        high: 0xeb0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb2 as ::core::ffi::c_ushort,
        high: 0xeb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xebd as ::core::ffi::c_ushort,
        high: 0xebd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec0 as ::core::ffi::c_ushort,
        high: 0xec4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xedc as ::core::ffi::c_ushort,
        high: 0xedd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf00 as ::core::ffi::c_ushort,
        high: 0xf00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf40 as ::core::ffi::c_ushort,
        high: 0xf47 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf49 as ::core::ffi::c_ushort,
        high: 0xf6a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf88 as ::core::ffi::c_ushort,
        high: 0xf8b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1000 as ::core::ffi::c_ushort,
        high: 0x1021 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1023 as ::core::ffi::c_ushort,
        high: 0x1027 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1029 as ::core::ffi::c_ushort,
        high: 0x102a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1050 as ::core::ffi::c_ushort,
        high: 0x1055 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10d0 as ::core::ffi::c_ushort,
        high: 0x10f8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1100 as ::core::ffi::c_ushort,
        high: 0x1159 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x115f as ::core::ffi::c_ushort,
        high: 0x11a2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11a8 as ::core::ffi::c_ushort,
        high: 0x11f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1200 as ::core::ffi::c_ushort,
        high: 0x1206 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1208 as ::core::ffi::c_ushort,
        high: 0x1246 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1248 as ::core::ffi::c_ushort,
        high: 0x1248 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x124a as ::core::ffi::c_ushort,
        high: 0x124d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1250 as ::core::ffi::c_ushort,
        high: 0x1256 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1258 as ::core::ffi::c_ushort,
        high: 0x1258 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x125a as ::core::ffi::c_ushort,
        high: 0x125d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1260 as ::core::ffi::c_ushort,
        high: 0x1286 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1288 as ::core::ffi::c_ushort,
        high: 0x1288 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x128a as ::core::ffi::c_ushort,
        high: 0x128d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1290 as ::core::ffi::c_ushort,
        high: 0x12ae as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12b0 as ::core::ffi::c_ushort,
        high: 0x12b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12b2 as ::core::ffi::c_ushort,
        high: 0x12b5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12b8 as ::core::ffi::c_ushort,
        high: 0x12be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12c0 as ::core::ffi::c_ushort,
        high: 0x12c0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12c2 as ::core::ffi::c_ushort,
        high: 0x12c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12c8 as ::core::ffi::c_ushort,
        high: 0x12ce as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12d0 as ::core::ffi::c_ushort,
        high: 0x12d6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12d8 as ::core::ffi::c_ushort,
        high: 0x12ee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12f0 as ::core::ffi::c_ushort,
        high: 0x130e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1310 as ::core::ffi::c_ushort,
        high: 0x1310 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1312 as ::core::ffi::c_ushort,
        high: 0x1315 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1318 as ::core::ffi::c_ushort,
        high: 0x131e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1320 as ::core::ffi::c_ushort,
        high: 0x1346 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1348 as ::core::ffi::c_ushort,
        high: 0x135a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13a0 as ::core::ffi::c_ushort,
        high: 0x13f4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1401 as ::core::ffi::c_ushort,
        high: 0x166c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x166f as ::core::ffi::c_ushort,
        high: 0x1676 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1681 as ::core::ffi::c_ushort,
        high: 0x169a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16a0 as ::core::ffi::c_ushort,
        high: 0x16ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1700 as ::core::ffi::c_ushort,
        high: 0x170c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x170e as ::core::ffi::c_ushort,
        high: 0x1711 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1720 as ::core::ffi::c_ushort,
        high: 0x1731 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1740 as ::core::ffi::c_ushort,
        high: 0x1751 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1760 as ::core::ffi::c_ushort,
        high: 0x176c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x176e as ::core::ffi::c_ushort,
        high: 0x1770 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1780 as ::core::ffi::c_ushort,
        high: 0x17b3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17dc as ::core::ffi::c_ushort,
        high: 0x17dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1820 as ::core::ffi::c_ushort,
        high: 0x1842 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1844 as ::core::ffi::c_ushort,
        high: 0x1877 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1880 as ::core::ffi::c_ushort,
        high: 0x18a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1900 as ::core::ffi::c_ushort,
        high: 0x191c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1950 as ::core::ffi::c_ushort,
        high: 0x196d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1970 as ::core::ffi::c_ushort,
        high: 0x1974 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2135 as ::core::ffi::c_ushort,
        high: 0x2138 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3006 as ::core::ffi::c_ushort,
        high: 0x3006 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x303c as ::core::ffi::c_ushort,
        high: 0x303c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3041 as ::core::ffi::c_ushort,
        high: 0x3096 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x309f as ::core::ffi::c_ushort,
        high: 0x309f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30a1 as ::core::ffi::c_ushort,
        high: 0x30fa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30ff as ::core::ffi::c_ushort,
        high: 0x30ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3105 as ::core::ffi::c_ushort,
        high: 0x312c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3131 as ::core::ffi::c_ushort,
        high: 0x318e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x31a0 as ::core::ffi::c_ushort,
        high: 0x31b7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x31f0 as ::core::ffi::c_ushort,
        high: 0x31ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3400 as ::core::ffi::c_ushort,
        high: 0x3400 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4db5 as ::core::ffi::c_ushort,
        high: 0x4db5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e00 as ::core::ffi::c_ushort,
        high: 0x4e00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9fa5 as ::core::ffi::c_ushort,
        high: 0x9fa5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa000 as ::core::ffi::c_ushort,
        high: 0xa48c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac00 as ::core::ffi::c_ushort,
        high: 0xac00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd7a3 as ::core::ffi::c_ushort,
        high: 0xd7a3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf900 as ::core::ffi::c_ushort,
        high: 0xfa2d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfa30 as ::core::ffi::c_ushort,
        high: 0xfa6a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1d as ::core::ffi::c_ushort,
        high: 0xfb1d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1f as ::core::ffi::c_ushort,
        high: 0xfb28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb2a as ::core::ffi::c_ushort,
        high: 0xfb36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb38 as ::core::ffi::c_ushort,
        high: 0xfb3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb3e as ::core::ffi::c_ushort,
        high: 0xfb3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb40 as ::core::ffi::c_ushort,
        high: 0xfb41 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb43 as ::core::ffi::c_ushort,
        high: 0xfb44 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb46 as ::core::ffi::c_ushort,
        high: 0xfbb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfbd3 as ::core::ffi::c_ushort,
        high: 0xfd3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfd50 as ::core::ffi::c_ushort,
        high: 0xfd8f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfd92 as ::core::ffi::c_ushort,
        high: 0xfdc7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfdf0 as ::core::ffi::c_ushort,
        high: 0xfdfb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe70 as ::core::ffi::c_ushort,
        high: 0xfe74 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe76 as ::core::ffi::c_ushort,
        high: 0xfefc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff66 as ::core::ffi::c_ushort,
        high: 0xff6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff71 as ::core::ffi::c_ushort,
        high: 0xff9d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffa0 as ::core::ffi::c_ushort,
        high: 0xffbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffc2 as ::core::ffi::c_ushort,
        high: 0xffc7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffca as ::core::ffi::c_ushort,
        high: 0xffcf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffd2 as ::core::ffi::c_ushort,
        high: 0xffd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffda as ::core::ffi::c_ushort,
        high: 0xffdc as ::core::ffi::c_ushort,
    },
];
static mut xmlLoL: [xmlChLRange; 20] = [
    _xmlChLRange {
        low: 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1000b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1000d as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10026 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10028 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1003a as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1003c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1003d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1003f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1004d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10050 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1005d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10080 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x100fa as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10300 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1031e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10330 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10349 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10380 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1039d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10450 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1049d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10800 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10805 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10808 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10808 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1080a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10835 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10837 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10838 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1083c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1083c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1083f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1083f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x20000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x20000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x2a6d6 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x2a6d6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x2f800 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x2fa1d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlLoG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 211 as ::core::ffi::c_int,
        nbLongRange: 20 as ::core::ffi::c_int,
        shortRange: &raw const xmlLoS as *const xmlChSRange,
        longRange: &raw const xmlLoL as *const xmlChLRange,
    }
};
static mut xmlLtS: [xmlChSRange; 10] = [
    _xmlChSRange {
        low: 0x1c5 as ::core::ffi::c_ushort,
        high: 0x1c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1c8 as ::core::ffi::c_ushort,
        high: 0x1c8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1cb as ::core::ffi::c_ushort,
        high: 0x1cb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f2 as ::core::ffi::c_ushort,
        high: 0x1f2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f88 as ::core::ffi::c_ushort,
        high: 0x1f8f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f98 as ::core::ffi::c_ushort,
        high: 0x1f9f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fa8 as ::core::ffi::c_ushort,
        high: 0x1faf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbc as ::core::ffi::c_ushort,
        high: 0x1fbc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fcc as ::core::ffi::c_ushort,
        high: 0x1fcc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ffc as ::core::ffi::c_ushort,
        high: 0x1ffc as ::core::ffi::c_ushort,
    },
];
static mut xmlLtG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 10 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlLtS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlLuS: [xmlChSRange; 390] = [
    _xmlChSRange {
        low: 0x41 as ::core::ffi::c_ushort,
        high: 0x5a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc0 as ::core::ffi::c_ushort,
        high: 0xd6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd8 as ::core::ffi::c_ushort,
        high: 0xde as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x100 as ::core::ffi::c_ushort,
        high: 0x100 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x102 as ::core::ffi::c_ushort,
        high: 0x102 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x104 as ::core::ffi::c_ushort,
        high: 0x104 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x106 as ::core::ffi::c_ushort,
        high: 0x106 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x108 as ::core::ffi::c_ushort,
        high: 0x108 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10a as ::core::ffi::c_ushort,
        high: 0x10a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10c as ::core::ffi::c_ushort,
        high: 0x10c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10e as ::core::ffi::c_ushort,
        high: 0x10e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x110 as ::core::ffi::c_ushort,
        high: 0x110 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x112 as ::core::ffi::c_ushort,
        high: 0x112 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x114 as ::core::ffi::c_ushort,
        high: 0x114 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x116 as ::core::ffi::c_ushort,
        high: 0x116 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x118 as ::core::ffi::c_ushort,
        high: 0x118 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11a as ::core::ffi::c_ushort,
        high: 0x11a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11c as ::core::ffi::c_ushort,
        high: 0x11c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x11e as ::core::ffi::c_ushort,
        high: 0x11e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x120 as ::core::ffi::c_ushort,
        high: 0x120 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x122 as ::core::ffi::c_ushort,
        high: 0x122 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x124 as ::core::ffi::c_ushort,
        high: 0x124 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x126 as ::core::ffi::c_ushort,
        high: 0x126 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x128 as ::core::ffi::c_ushort,
        high: 0x128 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12a as ::core::ffi::c_ushort,
        high: 0x12a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12c as ::core::ffi::c_ushort,
        high: 0x12c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x12e as ::core::ffi::c_ushort,
        high: 0x12e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x130 as ::core::ffi::c_ushort,
        high: 0x130 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x132 as ::core::ffi::c_ushort,
        high: 0x132 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x134 as ::core::ffi::c_ushort,
        high: 0x134 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x136 as ::core::ffi::c_ushort,
        high: 0x136 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x139 as ::core::ffi::c_ushort,
        high: 0x139 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13b as ::core::ffi::c_ushort,
        high: 0x13b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13d as ::core::ffi::c_ushort,
        high: 0x13d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x13f as ::core::ffi::c_ushort,
        high: 0x13f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x141 as ::core::ffi::c_ushort,
        high: 0x141 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x143 as ::core::ffi::c_ushort,
        high: 0x143 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x145 as ::core::ffi::c_ushort,
        high: 0x145 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x147 as ::core::ffi::c_ushort,
        high: 0x147 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x14a as ::core::ffi::c_ushort,
        high: 0x14a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x14c as ::core::ffi::c_ushort,
        high: 0x14c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x14e as ::core::ffi::c_ushort,
        high: 0x14e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x150 as ::core::ffi::c_ushort,
        high: 0x150 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x152 as ::core::ffi::c_ushort,
        high: 0x152 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x154 as ::core::ffi::c_ushort,
        high: 0x154 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x156 as ::core::ffi::c_ushort,
        high: 0x156 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x158 as ::core::ffi::c_ushort,
        high: 0x158 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x15a as ::core::ffi::c_ushort,
        high: 0x15a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x15c as ::core::ffi::c_ushort,
        high: 0x15c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x15e as ::core::ffi::c_ushort,
        high: 0x15e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x160 as ::core::ffi::c_ushort,
        high: 0x160 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x162 as ::core::ffi::c_ushort,
        high: 0x162 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x164 as ::core::ffi::c_ushort,
        high: 0x164 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x166 as ::core::ffi::c_ushort,
        high: 0x166 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x168 as ::core::ffi::c_ushort,
        high: 0x168 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16a as ::core::ffi::c_ushort,
        high: 0x16a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16c as ::core::ffi::c_ushort,
        high: 0x16c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16e as ::core::ffi::c_ushort,
        high: 0x16e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x170 as ::core::ffi::c_ushort,
        high: 0x170 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x172 as ::core::ffi::c_ushort,
        high: 0x172 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x174 as ::core::ffi::c_ushort,
        high: 0x174 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x176 as ::core::ffi::c_ushort,
        high: 0x176 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x178 as ::core::ffi::c_ushort,
        high: 0x179 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17b as ::core::ffi::c_ushort,
        high: 0x17b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17d as ::core::ffi::c_ushort,
        high: 0x17d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x181 as ::core::ffi::c_ushort,
        high: 0x182 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x184 as ::core::ffi::c_ushort,
        high: 0x184 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x186 as ::core::ffi::c_ushort,
        high: 0x187 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x189 as ::core::ffi::c_ushort,
        high: 0x18b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x18e as ::core::ffi::c_ushort,
        high: 0x191 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x193 as ::core::ffi::c_ushort,
        high: 0x194 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x196 as ::core::ffi::c_ushort,
        high: 0x198 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x19c as ::core::ffi::c_ushort,
        high: 0x19d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x19f as ::core::ffi::c_ushort,
        high: 0x1a0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a2 as ::core::ffi::c_ushort,
        high: 0x1a2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a4 as ::core::ffi::c_ushort,
        high: 0x1a4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a6 as ::core::ffi::c_ushort,
        high: 0x1a7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1a9 as ::core::ffi::c_ushort,
        high: 0x1a9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ac as ::core::ffi::c_ushort,
        high: 0x1ac as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ae as ::core::ffi::c_ushort,
        high: 0x1af as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1b1 as ::core::ffi::c_ushort,
        high: 0x1b3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1b5 as ::core::ffi::c_ushort,
        high: 0x1b5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1b7 as ::core::ffi::c_ushort,
        high: 0x1b8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1bc as ::core::ffi::c_ushort,
        high: 0x1bc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1c4 as ::core::ffi::c_ushort,
        high: 0x1c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1c7 as ::core::ffi::c_ushort,
        high: 0x1c7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ca as ::core::ffi::c_ushort,
        high: 0x1ca as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1cd as ::core::ffi::c_ushort,
        high: 0x1cd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1cf as ::core::ffi::c_ushort,
        high: 0x1cf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d1 as ::core::ffi::c_ushort,
        high: 0x1d1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d3 as ::core::ffi::c_ushort,
        high: 0x1d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d5 as ::core::ffi::c_ushort,
        high: 0x1d5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d7 as ::core::ffi::c_ushort,
        high: 0x1d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1d9 as ::core::ffi::c_ushort,
        high: 0x1d9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1db as ::core::ffi::c_ushort,
        high: 0x1db as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1de as ::core::ffi::c_ushort,
        high: 0x1de as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e0 as ::core::ffi::c_ushort,
        high: 0x1e0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e2 as ::core::ffi::c_ushort,
        high: 0x1e2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e4 as ::core::ffi::c_ushort,
        high: 0x1e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e6 as ::core::ffi::c_ushort,
        high: 0x1e6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e8 as ::core::ffi::c_ushort,
        high: 0x1e8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea as ::core::ffi::c_ushort,
        high: 0x1ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec as ::core::ffi::c_ushort,
        high: 0x1ec as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee as ::core::ffi::c_ushort,
        high: 0x1ee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f1 as ::core::ffi::c_ushort,
        high: 0x1f1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f4 as ::core::ffi::c_ushort,
        high: 0x1f4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f6 as ::core::ffi::c_ushort,
        high: 0x1f8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fa as ::core::ffi::c_ushort,
        high: 0x1fa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc as ::core::ffi::c_ushort,
        high: 0x1fc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fe as ::core::ffi::c_ushort,
        high: 0x1fe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x200 as ::core::ffi::c_ushort,
        high: 0x200 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x202 as ::core::ffi::c_ushort,
        high: 0x202 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x204 as ::core::ffi::c_ushort,
        high: 0x204 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x206 as ::core::ffi::c_ushort,
        high: 0x206 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x208 as ::core::ffi::c_ushort,
        high: 0x208 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20a as ::core::ffi::c_ushort,
        high: 0x20a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20c as ::core::ffi::c_ushort,
        high: 0x20c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20e as ::core::ffi::c_ushort,
        high: 0x20e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x210 as ::core::ffi::c_ushort,
        high: 0x210 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212 as ::core::ffi::c_ushort,
        high: 0x212 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x214 as ::core::ffi::c_ushort,
        high: 0x214 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x216 as ::core::ffi::c_ushort,
        high: 0x216 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x218 as ::core::ffi::c_ushort,
        high: 0x218 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21a as ::core::ffi::c_ushort,
        high: 0x21a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21c as ::core::ffi::c_ushort,
        high: 0x21c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21e as ::core::ffi::c_ushort,
        high: 0x21e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x220 as ::core::ffi::c_ushort,
        high: 0x220 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x222 as ::core::ffi::c_ushort,
        high: 0x222 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x224 as ::core::ffi::c_ushort,
        high: 0x224 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x226 as ::core::ffi::c_ushort,
        high: 0x226 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x228 as ::core::ffi::c_ushort,
        high: 0x228 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x22a as ::core::ffi::c_ushort,
        high: 0x22a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x22c as ::core::ffi::c_ushort,
        high: 0x22c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x22e as ::core::ffi::c_ushort,
        high: 0x22e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x230 as ::core::ffi::c_ushort,
        high: 0x230 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x232 as ::core::ffi::c_ushort,
        high: 0x232 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x386 as ::core::ffi::c_ushort,
        high: 0x386 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x388 as ::core::ffi::c_ushort,
        high: 0x38a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x38c as ::core::ffi::c_ushort,
        high: 0x38c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x38e as ::core::ffi::c_ushort,
        high: 0x38f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x391 as ::core::ffi::c_ushort,
        high: 0x3a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3a3 as ::core::ffi::c_ushort,
        high: 0x3ab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3d2 as ::core::ffi::c_ushort,
        high: 0x3d4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3d8 as ::core::ffi::c_ushort,
        high: 0x3d8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3da as ::core::ffi::c_ushort,
        high: 0x3da as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3dc as ::core::ffi::c_ushort,
        high: 0x3dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3de as ::core::ffi::c_ushort,
        high: 0x3de as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e0 as ::core::ffi::c_ushort,
        high: 0x3e0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e2 as ::core::ffi::c_ushort,
        high: 0x3e2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e4 as ::core::ffi::c_ushort,
        high: 0x3e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e6 as ::core::ffi::c_ushort,
        high: 0x3e6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3e8 as ::core::ffi::c_ushort,
        high: 0x3e8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3ea as ::core::ffi::c_ushort,
        high: 0x3ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3ec as ::core::ffi::c_ushort,
        high: 0x3ec as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3ee as ::core::ffi::c_ushort,
        high: 0x3ee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f4 as ::core::ffi::c_ushort,
        high: 0x3f4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f7 as ::core::ffi::c_ushort,
        high: 0x3f7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f9 as ::core::ffi::c_ushort,
        high: 0x3fa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x400 as ::core::ffi::c_ushort,
        high: 0x42f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x460 as ::core::ffi::c_ushort,
        high: 0x460 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x462 as ::core::ffi::c_ushort,
        high: 0x462 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x464 as ::core::ffi::c_ushort,
        high: 0x464 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x466 as ::core::ffi::c_ushort,
        high: 0x466 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x468 as ::core::ffi::c_ushort,
        high: 0x468 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x46a as ::core::ffi::c_ushort,
        high: 0x46a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x46c as ::core::ffi::c_ushort,
        high: 0x46c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x46e as ::core::ffi::c_ushort,
        high: 0x46e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x470 as ::core::ffi::c_ushort,
        high: 0x470 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x472 as ::core::ffi::c_ushort,
        high: 0x472 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x474 as ::core::ffi::c_ushort,
        high: 0x474 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x476 as ::core::ffi::c_ushort,
        high: 0x476 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x478 as ::core::ffi::c_ushort,
        high: 0x478 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x47a as ::core::ffi::c_ushort,
        high: 0x47a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x47c as ::core::ffi::c_ushort,
        high: 0x47c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x47e as ::core::ffi::c_ushort,
        high: 0x47e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x480 as ::core::ffi::c_ushort,
        high: 0x480 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x48a as ::core::ffi::c_ushort,
        high: 0x48a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x48c as ::core::ffi::c_ushort,
        high: 0x48c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x48e as ::core::ffi::c_ushort,
        high: 0x48e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x490 as ::core::ffi::c_ushort,
        high: 0x490 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x492 as ::core::ffi::c_ushort,
        high: 0x492 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x494 as ::core::ffi::c_ushort,
        high: 0x494 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x496 as ::core::ffi::c_ushort,
        high: 0x496 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x498 as ::core::ffi::c_ushort,
        high: 0x498 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x49a as ::core::ffi::c_ushort,
        high: 0x49a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x49c as ::core::ffi::c_ushort,
        high: 0x49c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x49e as ::core::ffi::c_ushort,
        high: 0x49e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a0 as ::core::ffi::c_ushort,
        high: 0x4a0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a2 as ::core::ffi::c_ushort,
        high: 0x4a2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a4 as ::core::ffi::c_ushort,
        high: 0x4a4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a6 as ::core::ffi::c_ushort,
        high: 0x4a6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4a8 as ::core::ffi::c_ushort,
        high: 0x4a8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4aa as ::core::ffi::c_ushort,
        high: 0x4aa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ac as ::core::ffi::c_ushort,
        high: 0x4ac as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ae as ::core::ffi::c_ushort,
        high: 0x4ae as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b0 as ::core::ffi::c_ushort,
        high: 0x4b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b2 as ::core::ffi::c_ushort,
        high: 0x4b2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b4 as ::core::ffi::c_ushort,
        high: 0x4b4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b6 as ::core::ffi::c_ushort,
        high: 0x4b6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4b8 as ::core::ffi::c_ushort,
        high: 0x4b8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ba as ::core::ffi::c_ushort,
        high: 0x4ba as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4bc as ::core::ffi::c_ushort,
        high: 0x4bc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4be as ::core::ffi::c_ushort,
        high: 0x4be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c0 as ::core::ffi::c_ushort,
        high: 0x4c1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c3 as ::core::ffi::c_ushort,
        high: 0x4c3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c5 as ::core::ffi::c_ushort,
        high: 0x4c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c7 as ::core::ffi::c_ushort,
        high: 0x4c7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4c9 as ::core::ffi::c_ushort,
        high: 0x4c9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4cb as ::core::ffi::c_ushort,
        high: 0x4cb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4cd as ::core::ffi::c_ushort,
        high: 0x4cd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d0 as ::core::ffi::c_ushort,
        high: 0x4d0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d2 as ::core::ffi::c_ushort,
        high: 0x4d2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d4 as ::core::ffi::c_ushort,
        high: 0x4d4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d6 as ::core::ffi::c_ushort,
        high: 0x4d6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4d8 as ::core::ffi::c_ushort,
        high: 0x4d8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4da as ::core::ffi::c_ushort,
        high: 0x4da as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4dc as ::core::ffi::c_ushort,
        high: 0x4dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4de as ::core::ffi::c_ushort,
        high: 0x4de as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e0 as ::core::ffi::c_ushort,
        high: 0x4e0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e2 as ::core::ffi::c_ushort,
        high: 0x4e2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e4 as ::core::ffi::c_ushort,
        high: 0x4e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e6 as ::core::ffi::c_ushort,
        high: 0x4e6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4e8 as ::core::ffi::c_ushort,
        high: 0x4e8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ea as ::core::ffi::c_ushort,
        high: 0x4ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ec as ::core::ffi::c_ushort,
        high: 0x4ec as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4ee as ::core::ffi::c_ushort,
        high: 0x4ee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f0 as ::core::ffi::c_ushort,
        high: 0x4f0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f2 as ::core::ffi::c_ushort,
        high: 0x4f2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f4 as ::core::ffi::c_ushort,
        high: 0x4f4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4f8 as ::core::ffi::c_ushort,
        high: 0x4f8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x500 as ::core::ffi::c_ushort,
        high: 0x500 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x502 as ::core::ffi::c_ushort,
        high: 0x502 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x504 as ::core::ffi::c_ushort,
        high: 0x504 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x506 as ::core::ffi::c_ushort,
        high: 0x506 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x508 as ::core::ffi::c_ushort,
        high: 0x508 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x50a as ::core::ffi::c_ushort,
        high: 0x50a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x50c as ::core::ffi::c_ushort,
        high: 0x50c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x50e as ::core::ffi::c_ushort,
        high: 0x50e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x531 as ::core::ffi::c_ushort,
        high: 0x556 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10a0 as ::core::ffi::c_ushort,
        high: 0x10c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e00 as ::core::ffi::c_ushort,
        high: 0x1e00 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e02 as ::core::ffi::c_ushort,
        high: 0x1e02 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e04 as ::core::ffi::c_ushort,
        high: 0x1e04 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e06 as ::core::ffi::c_ushort,
        high: 0x1e06 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e08 as ::core::ffi::c_ushort,
        high: 0x1e08 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e0a as ::core::ffi::c_ushort,
        high: 0x1e0a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e0c as ::core::ffi::c_ushort,
        high: 0x1e0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e0e as ::core::ffi::c_ushort,
        high: 0x1e0e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e10 as ::core::ffi::c_ushort,
        high: 0x1e10 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e12 as ::core::ffi::c_ushort,
        high: 0x1e12 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e14 as ::core::ffi::c_ushort,
        high: 0x1e14 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e16 as ::core::ffi::c_ushort,
        high: 0x1e16 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e18 as ::core::ffi::c_ushort,
        high: 0x1e18 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e1a as ::core::ffi::c_ushort,
        high: 0x1e1a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e1c as ::core::ffi::c_ushort,
        high: 0x1e1c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e1e as ::core::ffi::c_ushort,
        high: 0x1e1e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e20 as ::core::ffi::c_ushort,
        high: 0x1e20 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e22 as ::core::ffi::c_ushort,
        high: 0x1e22 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e24 as ::core::ffi::c_ushort,
        high: 0x1e24 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e26 as ::core::ffi::c_ushort,
        high: 0x1e26 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e28 as ::core::ffi::c_ushort,
        high: 0x1e28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e2a as ::core::ffi::c_ushort,
        high: 0x1e2a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e2c as ::core::ffi::c_ushort,
        high: 0x1e2c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e2e as ::core::ffi::c_ushort,
        high: 0x1e2e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e30 as ::core::ffi::c_ushort,
        high: 0x1e30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e32 as ::core::ffi::c_ushort,
        high: 0x1e32 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e34 as ::core::ffi::c_ushort,
        high: 0x1e34 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e36 as ::core::ffi::c_ushort,
        high: 0x1e36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e38 as ::core::ffi::c_ushort,
        high: 0x1e38 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e3a as ::core::ffi::c_ushort,
        high: 0x1e3a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e3c as ::core::ffi::c_ushort,
        high: 0x1e3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e3e as ::core::ffi::c_ushort,
        high: 0x1e3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e40 as ::core::ffi::c_ushort,
        high: 0x1e40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e42 as ::core::ffi::c_ushort,
        high: 0x1e42 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e44 as ::core::ffi::c_ushort,
        high: 0x1e44 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e46 as ::core::ffi::c_ushort,
        high: 0x1e46 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e48 as ::core::ffi::c_ushort,
        high: 0x1e48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e4a as ::core::ffi::c_ushort,
        high: 0x1e4a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e4c as ::core::ffi::c_ushort,
        high: 0x1e4c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e4e as ::core::ffi::c_ushort,
        high: 0x1e4e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e50 as ::core::ffi::c_ushort,
        high: 0x1e50 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e52 as ::core::ffi::c_ushort,
        high: 0x1e52 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e54 as ::core::ffi::c_ushort,
        high: 0x1e54 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e56 as ::core::ffi::c_ushort,
        high: 0x1e56 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e58 as ::core::ffi::c_ushort,
        high: 0x1e58 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e5a as ::core::ffi::c_ushort,
        high: 0x1e5a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e5c as ::core::ffi::c_ushort,
        high: 0x1e5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e5e as ::core::ffi::c_ushort,
        high: 0x1e5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e60 as ::core::ffi::c_ushort,
        high: 0x1e60 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e62 as ::core::ffi::c_ushort,
        high: 0x1e62 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e64 as ::core::ffi::c_ushort,
        high: 0x1e64 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e66 as ::core::ffi::c_ushort,
        high: 0x1e66 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e68 as ::core::ffi::c_ushort,
        high: 0x1e68 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e6a as ::core::ffi::c_ushort,
        high: 0x1e6a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e6c as ::core::ffi::c_ushort,
        high: 0x1e6c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e6e as ::core::ffi::c_ushort,
        high: 0x1e6e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e70 as ::core::ffi::c_ushort,
        high: 0x1e70 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e72 as ::core::ffi::c_ushort,
        high: 0x1e72 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e74 as ::core::ffi::c_ushort,
        high: 0x1e74 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e76 as ::core::ffi::c_ushort,
        high: 0x1e76 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e78 as ::core::ffi::c_ushort,
        high: 0x1e78 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e7a as ::core::ffi::c_ushort,
        high: 0x1e7a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e7c as ::core::ffi::c_ushort,
        high: 0x1e7c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e7e as ::core::ffi::c_ushort,
        high: 0x1e7e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e80 as ::core::ffi::c_ushort,
        high: 0x1e80 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e82 as ::core::ffi::c_ushort,
        high: 0x1e82 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e84 as ::core::ffi::c_ushort,
        high: 0x1e84 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e86 as ::core::ffi::c_ushort,
        high: 0x1e86 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e88 as ::core::ffi::c_ushort,
        high: 0x1e88 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e8a as ::core::ffi::c_ushort,
        high: 0x1e8a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e8c as ::core::ffi::c_ushort,
        high: 0x1e8c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e8e as ::core::ffi::c_ushort,
        high: 0x1e8e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e90 as ::core::ffi::c_ushort,
        high: 0x1e90 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e92 as ::core::ffi::c_ushort,
        high: 0x1e92 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1e94 as ::core::ffi::c_ushort,
        high: 0x1e94 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea0 as ::core::ffi::c_ushort,
        high: 0x1ea0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea2 as ::core::ffi::c_ushort,
        high: 0x1ea2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea4 as ::core::ffi::c_ushort,
        high: 0x1ea4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea6 as ::core::ffi::c_ushort,
        high: 0x1ea6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ea8 as ::core::ffi::c_ushort,
        high: 0x1ea8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eaa as ::core::ffi::c_ushort,
        high: 0x1eaa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eac as ::core::ffi::c_ushort,
        high: 0x1eac as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eae as ::core::ffi::c_ushort,
        high: 0x1eae as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb0 as ::core::ffi::c_ushort,
        high: 0x1eb0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb2 as ::core::ffi::c_ushort,
        high: 0x1eb2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb4 as ::core::ffi::c_ushort,
        high: 0x1eb4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb6 as ::core::ffi::c_ushort,
        high: 0x1eb6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eb8 as ::core::ffi::c_ushort,
        high: 0x1eb8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eba as ::core::ffi::c_ushort,
        high: 0x1eba as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ebc as ::core::ffi::c_ushort,
        high: 0x1ebc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ebe as ::core::ffi::c_ushort,
        high: 0x1ebe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec0 as ::core::ffi::c_ushort,
        high: 0x1ec0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec2 as ::core::ffi::c_ushort,
        high: 0x1ec2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec4 as ::core::ffi::c_ushort,
        high: 0x1ec4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec6 as ::core::ffi::c_ushort,
        high: 0x1ec6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ec8 as ::core::ffi::c_ushort,
        high: 0x1ec8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eca as ::core::ffi::c_ushort,
        high: 0x1eca as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ecc as ::core::ffi::c_ushort,
        high: 0x1ecc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ece as ::core::ffi::c_ushort,
        high: 0x1ece as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed0 as ::core::ffi::c_ushort,
        high: 0x1ed0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed2 as ::core::ffi::c_ushort,
        high: 0x1ed2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed4 as ::core::ffi::c_ushort,
        high: 0x1ed4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed6 as ::core::ffi::c_ushort,
        high: 0x1ed6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ed8 as ::core::ffi::c_ushort,
        high: 0x1ed8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eda as ::core::ffi::c_ushort,
        high: 0x1eda as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1edc as ::core::ffi::c_ushort,
        high: 0x1edc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ede as ::core::ffi::c_ushort,
        high: 0x1ede as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee0 as ::core::ffi::c_ushort,
        high: 0x1ee0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee2 as ::core::ffi::c_ushort,
        high: 0x1ee2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee4 as ::core::ffi::c_ushort,
        high: 0x1ee4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee6 as ::core::ffi::c_ushort,
        high: 0x1ee6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ee8 as ::core::ffi::c_ushort,
        high: 0x1ee8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eea as ::core::ffi::c_ushort,
        high: 0x1eea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eec as ::core::ffi::c_ushort,
        high: 0x1eec as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1eee as ::core::ffi::c_ushort,
        high: 0x1eee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef0 as ::core::ffi::c_ushort,
        high: 0x1ef0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef2 as ::core::ffi::c_ushort,
        high: 0x1ef2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef4 as ::core::ffi::c_ushort,
        high: 0x1ef4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef6 as ::core::ffi::c_ushort,
        high: 0x1ef6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ef8 as ::core::ffi::c_ushort,
        high: 0x1ef8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f08 as ::core::ffi::c_ushort,
        high: 0x1f0f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f18 as ::core::ffi::c_ushort,
        high: 0x1f1d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f28 as ::core::ffi::c_ushort,
        high: 0x1f2f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f38 as ::core::ffi::c_ushort,
        high: 0x1f3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f48 as ::core::ffi::c_ushort,
        high: 0x1f4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f59 as ::core::ffi::c_ushort,
        high: 0x1f59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5b as ::core::ffi::c_ushort,
        high: 0x1f5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5d as ::core::ffi::c_ushort,
        high: 0x1f5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f5f as ::core::ffi::c_ushort,
        high: 0x1f5f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1f68 as ::core::ffi::c_ushort,
        high: 0x1f6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fb8 as ::core::ffi::c_ushort,
        high: 0x1fbb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fc8 as ::core::ffi::c_ushort,
        high: 0x1fcb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fd8 as ::core::ffi::c_ushort,
        high: 0x1fdb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fe8 as ::core::ffi::c_ushort,
        high: 0x1fec as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ff8 as ::core::ffi::c_ushort,
        high: 0x1ffb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2102 as ::core::ffi::c_ushort,
        high: 0x2102 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2107 as ::core::ffi::c_ushort,
        high: 0x2107 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x210b as ::core::ffi::c_ushort,
        high: 0x210d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2110 as ::core::ffi::c_ushort,
        high: 0x2112 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2115 as ::core::ffi::c_ushort,
        high: 0x2115 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2119 as ::core::ffi::c_ushort,
        high: 0x211d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2124 as ::core::ffi::c_ushort,
        high: 0x2124 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2126 as ::core::ffi::c_ushort,
        high: 0x2126 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2128 as ::core::ffi::c_ushort,
        high: 0x2128 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212a as ::core::ffi::c_ushort,
        high: 0x212d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2130 as ::core::ffi::c_ushort,
        high: 0x2131 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2133 as ::core::ffi::c_ushort,
        high: 0x2133 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x213e as ::core::ffi::c_ushort,
        high: 0x213f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2145 as ::core::ffi::c_ushort,
        high: 0x2145 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff21 as ::core::ffi::c_ushort,
        high: 0xff3a as ::core::ffi::c_ushort,
    },
];
static mut xmlLuL: [xmlChLRange; 31] = [
    _xmlChLRange {
        low: 0x10400 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10427 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d400 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d419 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d434 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d44d as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d468 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d481 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d49c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d49c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d49e as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d49f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4a2 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4a2 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4a5 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4a6 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4ac as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4ae as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4b5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d4d0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d4e9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d504 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d505 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d507 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d50a as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d50d as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d514 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d516 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d51c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d538 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d539 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d53b as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d53e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d540 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d544 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d546 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d546 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d54a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d550 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d56c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d585 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d5a0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d5b9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d5d4 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d5ed as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d608 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d621 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d63c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d655 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d670 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d689 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6a8 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6c0 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6e2 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6fa as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d71c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d734 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d756 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d76e as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d790 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7a8 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlLuG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 390 as ::core::ffi::c_int,
        nbLongRange: 31 as ::core::ffi::c_int,
        shortRange: &raw const xmlLuS as *const xmlChSRange,
        longRange: &raw const xmlLuL as *const xmlChLRange,
    }
};
static mut xmlMS: [xmlChSRange; 113] = [
    _xmlChSRange {
        low: 0x300 as ::core::ffi::c_ushort,
        high: 0x357 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x35d as ::core::ffi::c_ushort,
        high: 0x36f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x483 as ::core::ffi::c_ushort,
        high: 0x486 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x488 as ::core::ffi::c_ushort,
        high: 0x489 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x591 as ::core::ffi::c_ushort,
        high: 0x5a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5a3 as ::core::ffi::c_ushort,
        high: 0x5b9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5bb as ::core::ffi::c_ushort,
        high: 0x5bd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5bf as ::core::ffi::c_ushort,
        high: 0x5bf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c1 as ::core::ffi::c_ushort,
        high: 0x5c2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c4 as ::core::ffi::c_ushort,
        high: 0x5c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x610 as ::core::ffi::c_ushort,
        high: 0x615 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x64b as ::core::ffi::c_ushort,
        high: 0x658 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x670 as ::core::ffi::c_ushort,
        high: 0x670 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d6 as ::core::ffi::c_ushort,
        high: 0x6dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6de as ::core::ffi::c_ushort,
        high: 0x6e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e7 as ::core::ffi::c_ushort,
        high: 0x6e8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ea as ::core::ffi::c_ushort,
        high: 0x6ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x711 as ::core::ffi::c_ushort,
        high: 0x711 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x730 as ::core::ffi::c_ushort,
        high: 0x74a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7a6 as ::core::ffi::c_ushort,
        high: 0x7b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x901 as ::core::ffi::c_ushort,
        high: 0x903 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93c as ::core::ffi::c_ushort,
        high: 0x93c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93e as ::core::ffi::c_ushort,
        high: 0x94d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x951 as ::core::ffi::c_ushort,
        high: 0x954 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x962 as ::core::ffi::c_ushort,
        high: 0x963 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x981 as ::core::ffi::c_ushort,
        high: 0x983 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9bc as ::core::ffi::c_ushort,
        high: 0x9bc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9be as ::core::ffi::c_ushort,
        high: 0x9c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9c7 as ::core::ffi::c_ushort,
        high: 0x9c8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9cb as ::core::ffi::c_ushort,
        high: 0x9cd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9d7 as ::core::ffi::c_ushort,
        high: 0x9d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9e2 as ::core::ffi::c_ushort,
        high: 0x9e3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa01 as ::core::ffi::c_ushort,
        high: 0xa03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa3c as ::core::ffi::c_ushort,
        high: 0xa3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa3e as ::core::ffi::c_ushort,
        high: 0xa42 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa47 as ::core::ffi::c_ushort,
        high: 0xa48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa4b as ::core::ffi::c_ushort,
        high: 0xa4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa70 as ::core::ffi::c_ushort,
        high: 0xa71 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa81 as ::core::ffi::c_ushort,
        high: 0xa83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabc as ::core::ffi::c_ushort,
        high: 0xabc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabe as ::core::ffi::c_ushort,
        high: 0xac5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac7 as ::core::ffi::c_ushort,
        high: 0xac9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xacb as ::core::ffi::c_ushort,
        high: 0xacd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae2 as ::core::ffi::c_ushort,
        high: 0xae3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb01 as ::core::ffi::c_ushort,
        high: 0xb03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3c as ::core::ffi::c_ushort,
        high: 0xb3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3e as ::core::ffi::c_ushort,
        high: 0xb43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb47 as ::core::ffi::c_ushort,
        high: 0xb48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb4b as ::core::ffi::c_ushort,
        high: 0xb4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb56 as ::core::ffi::c_ushort,
        high: 0xb57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb82 as ::core::ffi::c_ushort,
        high: 0xb82 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbbe as ::core::ffi::c_ushort,
        high: 0xbc2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbc6 as ::core::ffi::c_ushort,
        high: 0xbc8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbca as ::core::ffi::c_ushort,
        high: 0xbcd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbd7 as ::core::ffi::c_ushort,
        high: 0xbd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc01 as ::core::ffi::c_ushort,
        high: 0xc03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc3e as ::core::ffi::c_ushort,
        high: 0xc44 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc46 as ::core::ffi::c_ushort,
        high: 0xc48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc4a as ::core::ffi::c_ushort,
        high: 0xc4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc55 as ::core::ffi::c_ushort,
        high: 0xc56 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc82 as ::core::ffi::c_ushort,
        high: 0xc83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbc as ::core::ffi::c_ushort,
        high: 0xcbc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbe as ::core::ffi::c_ushort,
        high: 0xcc4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcc6 as ::core::ffi::c_ushort,
        high: 0xcc8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcca as ::core::ffi::c_ushort,
        high: 0xccd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcd5 as ::core::ffi::c_ushort,
        high: 0xcd6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd02 as ::core::ffi::c_ushort,
        high: 0xd03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd3e as ::core::ffi::c_ushort,
        high: 0xd43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd46 as ::core::ffi::c_ushort,
        high: 0xd48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd4a as ::core::ffi::c_ushort,
        high: 0xd4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd57 as ::core::ffi::c_ushort,
        high: 0xd57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd82 as ::core::ffi::c_ushort,
        high: 0xd83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdca as ::core::ffi::c_ushort,
        high: 0xdca as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdcf as ::core::ffi::c_ushort,
        high: 0xdd4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdd6 as ::core::ffi::c_ushort,
        high: 0xdd6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdd8 as ::core::ffi::c_ushort,
        high: 0xddf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdf2 as ::core::ffi::c_ushort,
        high: 0xdf3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe31 as ::core::ffi::c_ushort,
        high: 0xe31 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe34 as ::core::ffi::c_ushort,
        high: 0xe3a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe47 as ::core::ffi::c_ushort,
        high: 0xe4e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb1 as ::core::ffi::c_ushort,
        high: 0xeb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb4 as ::core::ffi::c_ushort,
        high: 0xeb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xebb as ::core::ffi::c_ushort,
        high: 0xebc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec8 as ::core::ffi::c_ushort,
        high: 0xecd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf18 as ::core::ffi::c_ushort,
        high: 0xf19 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf35 as ::core::ffi::c_ushort,
        high: 0xf35 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf37 as ::core::ffi::c_ushort,
        high: 0xf37 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf39 as ::core::ffi::c_ushort,
        high: 0xf39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3e as ::core::ffi::c_ushort,
        high: 0xf3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf71 as ::core::ffi::c_ushort,
        high: 0xf84 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf86 as ::core::ffi::c_ushort,
        high: 0xf87 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf90 as ::core::ffi::c_ushort,
        high: 0xf97 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf99 as ::core::ffi::c_ushort,
        high: 0xfbc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfc6 as ::core::ffi::c_ushort,
        high: 0xfc6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x102c as ::core::ffi::c_ushort,
        high: 0x1032 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1036 as ::core::ffi::c_ushort,
        high: 0x1039 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1056 as ::core::ffi::c_ushort,
        high: 0x1059 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1712 as ::core::ffi::c_ushort,
        high: 0x1714 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1732 as ::core::ffi::c_ushort,
        high: 0x1734 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1752 as ::core::ffi::c_ushort,
        high: 0x1753 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1772 as ::core::ffi::c_ushort,
        high: 0x1773 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17b6 as ::core::ffi::c_ushort,
        high: 0x17d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17dd as ::core::ffi::c_ushort,
        high: 0x17dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x180b as ::core::ffi::c_ushort,
        high: 0x180d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x18a9 as ::core::ffi::c_ushort,
        high: 0x18a9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1920 as ::core::ffi::c_ushort,
        high: 0x192b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1930 as ::core::ffi::c_ushort,
        high: 0x193b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20d0 as ::core::ffi::c_ushort,
        high: 0x20ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x302a as ::core::ffi::c_ushort,
        high: 0x302f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3099 as ::core::ffi::c_ushort,
        high: 0x309a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1e as ::core::ffi::c_ushort,
        high: 0xfb1e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe00 as ::core::ffi::c_ushort,
        high: 0xfe0f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe20 as ::core::ffi::c_ushort,
        high: 0xfe23 as ::core::ffi::c_ushort,
    },
];
static mut xmlML: [xmlChLRange; 6] = [
    _xmlChLRange {
        low: 0x1d165 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d169 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d16d as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d172 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d17b as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d182 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d185 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d18b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d1aa as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d1ad as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xe0100 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xe01ef as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlMG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 113 as ::core::ffi::c_int,
        nbLongRange: 6 as ::core::ffi::c_int,
        shortRange: &raw const xmlMS as *const xmlChSRange,
        longRange: &raw const xmlML as *const xmlChLRange,
    }
};
static mut xmlMcS: [xmlChSRange; 55] = [
    _xmlChSRange {
        low: 0x903 as ::core::ffi::c_ushort,
        high: 0x903 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93e as ::core::ffi::c_ushort,
        high: 0x940 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x949 as ::core::ffi::c_ushort,
        high: 0x94c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x982 as ::core::ffi::c_ushort,
        high: 0x983 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9be as ::core::ffi::c_ushort,
        high: 0x9c0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9c7 as ::core::ffi::c_ushort,
        high: 0x9c8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9cb as ::core::ffi::c_ushort,
        high: 0x9cc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9d7 as ::core::ffi::c_ushort,
        high: 0x9d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa03 as ::core::ffi::c_ushort,
        high: 0xa03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa3e as ::core::ffi::c_ushort,
        high: 0xa40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa83 as ::core::ffi::c_ushort,
        high: 0xa83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabe as ::core::ffi::c_ushort,
        high: 0xac0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac9 as ::core::ffi::c_ushort,
        high: 0xac9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xacb as ::core::ffi::c_ushort,
        high: 0xacc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb02 as ::core::ffi::c_ushort,
        high: 0xb03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3e as ::core::ffi::c_ushort,
        high: 0xb3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb40 as ::core::ffi::c_ushort,
        high: 0xb40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb47 as ::core::ffi::c_ushort,
        high: 0xb48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb4b as ::core::ffi::c_ushort,
        high: 0xb4c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb57 as ::core::ffi::c_ushort,
        high: 0xb57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbbe as ::core::ffi::c_ushort,
        high: 0xbbf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbc1 as ::core::ffi::c_ushort,
        high: 0xbc2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbc6 as ::core::ffi::c_ushort,
        high: 0xbc8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbca as ::core::ffi::c_ushort,
        high: 0xbcc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbd7 as ::core::ffi::c_ushort,
        high: 0xbd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc01 as ::core::ffi::c_ushort,
        high: 0xc03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc41 as ::core::ffi::c_ushort,
        high: 0xc44 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc82 as ::core::ffi::c_ushort,
        high: 0xc83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbe as ::core::ffi::c_ushort,
        high: 0xcbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcc0 as ::core::ffi::c_ushort,
        high: 0xcc4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcc7 as ::core::ffi::c_ushort,
        high: 0xcc8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcca as ::core::ffi::c_ushort,
        high: 0xccb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcd5 as ::core::ffi::c_ushort,
        high: 0xcd6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd02 as ::core::ffi::c_ushort,
        high: 0xd03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd3e as ::core::ffi::c_ushort,
        high: 0xd40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd46 as ::core::ffi::c_ushort,
        high: 0xd48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd4a as ::core::ffi::c_ushort,
        high: 0xd4c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd57 as ::core::ffi::c_ushort,
        high: 0xd57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd82 as ::core::ffi::c_ushort,
        high: 0xd83 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdcf as ::core::ffi::c_ushort,
        high: 0xdd1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdd8 as ::core::ffi::c_ushort,
        high: 0xddf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdf2 as ::core::ffi::c_ushort,
        high: 0xdf3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3e as ::core::ffi::c_ushort,
        high: 0xf3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf7f as ::core::ffi::c_ushort,
        high: 0xf7f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x102c as ::core::ffi::c_ushort,
        high: 0x102c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1031 as ::core::ffi::c_ushort,
        high: 0x1031 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1038 as ::core::ffi::c_ushort,
        high: 0x1038 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1056 as ::core::ffi::c_ushort,
        high: 0x1057 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17b6 as ::core::ffi::c_ushort,
        high: 0x17b6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17be as ::core::ffi::c_ushort,
        high: 0x17c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17c7 as ::core::ffi::c_ushort,
        high: 0x17c8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1923 as ::core::ffi::c_ushort,
        high: 0x1926 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1929 as ::core::ffi::c_ushort,
        high: 0x192b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1930 as ::core::ffi::c_ushort,
        high: 0x1931 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1933 as ::core::ffi::c_ushort,
        high: 0x1938 as ::core::ffi::c_ushort,
    },
];
static mut xmlMcL: [xmlChLRange; 2] = [
    _xmlChLRange {
        low: 0x1d165 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d166 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d16d as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d172 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlMcG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 55 as ::core::ffi::c_int,
        nbLongRange: 2 as ::core::ffi::c_int,
        shortRange: &raw const xmlMcS as *const xmlChSRange,
        longRange: &raw const xmlMcL as *const xmlChLRange,
    }
};
static mut xmlMnS: [xmlChSRange; 108] = [
    _xmlChSRange {
        low: 0x300 as ::core::ffi::c_ushort,
        high: 0x357 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x35d as ::core::ffi::c_ushort,
        high: 0x36f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x483 as ::core::ffi::c_ushort,
        high: 0x486 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x591 as ::core::ffi::c_ushort,
        high: 0x5a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5a3 as ::core::ffi::c_ushort,
        high: 0x5b9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5bb as ::core::ffi::c_ushort,
        high: 0x5bd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5bf as ::core::ffi::c_ushort,
        high: 0x5bf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c1 as ::core::ffi::c_ushort,
        high: 0x5c2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c4 as ::core::ffi::c_ushort,
        high: 0x5c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x610 as ::core::ffi::c_ushort,
        high: 0x615 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x64b as ::core::ffi::c_ushort,
        high: 0x658 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x670 as ::core::ffi::c_ushort,
        high: 0x670 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d6 as ::core::ffi::c_ushort,
        high: 0x6dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6df as ::core::ffi::c_ushort,
        high: 0x6e4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e7 as ::core::ffi::c_ushort,
        high: 0x6e8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6ea as ::core::ffi::c_ushort,
        high: 0x6ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x711 as ::core::ffi::c_ushort,
        high: 0x711 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x730 as ::core::ffi::c_ushort,
        high: 0x74a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7a6 as ::core::ffi::c_ushort,
        high: 0x7b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x901 as ::core::ffi::c_ushort,
        high: 0x902 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x93c as ::core::ffi::c_ushort,
        high: 0x93c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x941 as ::core::ffi::c_ushort,
        high: 0x948 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x94d as ::core::ffi::c_ushort,
        high: 0x94d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x951 as ::core::ffi::c_ushort,
        high: 0x954 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x962 as ::core::ffi::c_ushort,
        high: 0x963 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x981 as ::core::ffi::c_ushort,
        high: 0x981 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9bc as ::core::ffi::c_ushort,
        high: 0x9bc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9c1 as ::core::ffi::c_ushort,
        high: 0x9c4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9cd as ::core::ffi::c_ushort,
        high: 0x9cd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9e2 as ::core::ffi::c_ushort,
        high: 0x9e3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa01 as ::core::ffi::c_ushort,
        high: 0xa02 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa3c as ::core::ffi::c_ushort,
        high: 0xa3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa41 as ::core::ffi::c_ushort,
        high: 0xa42 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa47 as ::core::ffi::c_ushort,
        high: 0xa48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa4b as ::core::ffi::c_ushort,
        high: 0xa4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa70 as ::core::ffi::c_ushort,
        high: 0xa71 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa81 as ::core::ffi::c_ushort,
        high: 0xa82 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xabc as ::core::ffi::c_ushort,
        high: 0xabc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac1 as ::core::ffi::c_ushort,
        high: 0xac5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac7 as ::core::ffi::c_ushort,
        high: 0xac8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xacd as ::core::ffi::c_ushort,
        high: 0xacd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae2 as ::core::ffi::c_ushort,
        high: 0xae3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb01 as ::core::ffi::c_ushort,
        high: 0xb01 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3c as ::core::ffi::c_ushort,
        high: 0xb3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb3f as ::core::ffi::c_ushort,
        high: 0xb3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb41 as ::core::ffi::c_ushort,
        high: 0xb43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb4d as ::core::ffi::c_ushort,
        high: 0xb4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb56 as ::core::ffi::c_ushort,
        high: 0xb56 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb82 as ::core::ffi::c_ushort,
        high: 0xb82 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbc0 as ::core::ffi::c_ushort,
        high: 0xbc0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbcd as ::core::ffi::c_ushort,
        high: 0xbcd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc3e as ::core::ffi::c_ushort,
        high: 0xc40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc46 as ::core::ffi::c_ushort,
        high: 0xc48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc4a as ::core::ffi::c_ushort,
        high: 0xc4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc55 as ::core::ffi::c_ushort,
        high: 0xc56 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbc as ::core::ffi::c_ushort,
        high: 0xcbc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcbf as ::core::ffi::c_ushort,
        high: 0xcbf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xcc6 as ::core::ffi::c_ushort,
        high: 0xcc6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xccc as ::core::ffi::c_ushort,
        high: 0xccd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd41 as ::core::ffi::c_ushort,
        high: 0xd43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd4d as ::core::ffi::c_ushort,
        high: 0xd4d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdca as ::core::ffi::c_ushort,
        high: 0xdca as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdd2 as ::core::ffi::c_ushort,
        high: 0xdd4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdd6 as ::core::ffi::c_ushort,
        high: 0xdd6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe31 as ::core::ffi::c_ushort,
        high: 0xe31 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe34 as ::core::ffi::c_ushort,
        high: 0xe3a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe47 as ::core::ffi::c_ushort,
        high: 0xe4e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb1 as ::core::ffi::c_ushort,
        high: 0xeb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xeb4 as ::core::ffi::c_ushort,
        high: 0xeb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xebb as ::core::ffi::c_ushort,
        high: 0xebc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xec8 as ::core::ffi::c_ushort,
        high: 0xecd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf18 as ::core::ffi::c_ushort,
        high: 0xf19 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf35 as ::core::ffi::c_ushort,
        high: 0xf35 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf37 as ::core::ffi::c_ushort,
        high: 0xf37 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf39 as ::core::ffi::c_ushort,
        high: 0xf39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf71 as ::core::ffi::c_ushort,
        high: 0xf7e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf80 as ::core::ffi::c_ushort,
        high: 0xf84 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf86 as ::core::ffi::c_ushort,
        high: 0xf87 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf90 as ::core::ffi::c_ushort,
        high: 0xf97 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf99 as ::core::ffi::c_ushort,
        high: 0xfbc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfc6 as ::core::ffi::c_ushort,
        high: 0xfc6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x102d as ::core::ffi::c_ushort,
        high: 0x1030 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1032 as ::core::ffi::c_ushort,
        high: 0x1032 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1036 as ::core::ffi::c_ushort,
        high: 0x1037 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1039 as ::core::ffi::c_ushort,
        high: 0x1039 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1058 as ::core::ffi::c_ushort,
        high: 0x1059 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1712 as ::core::ffi::c_ushort,
        high: 0x1714 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1732 as ::core::ffi::c_ushort,
        high: 0x1734 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1752 as ::core::ffi::c_ushort,
        high: 0x1753 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1772 as ::core::ffi::c_ushort,
        high: 0x1773 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17b7 as ::core::ffi::c_ushort,
        high: 0x17bd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17c6 as ::core::ffi::c_ushort,
        high: 0x17c6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17c9 as ::core::ffi::c_ushort,
        high: 0x17d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17dd as ::core::ffi::c_ushort,
        high: 0x17dd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x180b as ::core::ffi::c_ushort,
        high: 0x180d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x18a9 as ::core::ffi::c_ushort,
        high: 0x18a9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1920 as ::core::ffi::c_ushort,
        high: 0x1922 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1927 as ::core::ffi::c_ushort,
        high: 0x1928 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1932 as ::core::ffi::c_ushort,
        high: 0x1932 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1939 as ::core::ffi::c_ushort,
        high: 0x193b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20d0 as ::core::ffi::c_ushort,
        high: 0x20dc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20e1 as ::core::ffi::c_ushort,
        high: 0x20e1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20e5 as ::core::ffi::c_ushort,
        high: 0x20ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x302a as ::core::ffi::c_ushort,
        high: 0x302f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3099 as ::core::ffi::c_ushort,
        high: 0x309a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb1e as ::core::ffi::c_ushort,
        high: 0xfb1e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe00 as ::core::ffi::c_ushort,
        high: 0xfe0f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe20 as ::core::ffi::c_ushort,
        high: 0xfe23 as ::core::ffi::c_ushort,
    },
];
static mut xmlMnL: [xmlChLRange; 5] = [
    _xmlChLRange {
        low: 0x1d167 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d169 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d17b as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d182 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d185 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d18b as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d1aa as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d1ad as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0xe0100 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0xe01ef as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlMnG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 108 as ::core::ffi::c_int,
        nbLongRange: 5 as ::core::ffi::c_int,
        shortRange: &raw const xmlMnS as *const xmlChSRange,
        longRange: &raw const xmlMnL as *const xmlChLRange,
    }
};
static mut xmlNS: [xmlChSRange; 42] = [
    _xmlChSRange {
        low: 0x30 as ::core::ffi::c_ushort,
        high: 0x39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb2 as ::core::ffi::c_ushort,
        high: 0xb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9 as ::core::ffi::c_ushort,
        high: 0xb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbc as ::core::ffi::c_ushort,
        high: 0xbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x660 as ::core::ffi::c_ushort,
        high: 0x669 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6f0 as ::core::ffi::c_ushort,
        high: 0x6f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x966 as ::core::ffi::c_ushort,
        high: 0x96f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9e6 as ::core::ffi::c_ushort,
        high: 0x9ef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9f4 as ::core::ffi::c_ushort,
        high: 0x9f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa66 as ::core::ffi::c_ushort,
        high: 0xa6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae6 as ::core::ffi::c_ushort,
        high: 0xaef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb66 as ::core::ffi::c_ushort,
        high: 0xb6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbe7 as ::core::ffi::c_ushort,
        high: 0xbf2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc66 as ::core::ffi::c_ushort,
        high: 0xc6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xce6 as ::core::ffi::c_ushort,
        high: 0xcef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd66 as ::core::ffi::c_ushort,
        high: 0xd6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe50 as ::core::ffi::c_ushort,
        high: 0xe59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xed0 as ::core::ffi::c_ushort,
        high: 0xed9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf20 as ::core::ffi::c_ushort,
        high: 0xf33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1040 as ::core::ffi::c_ushort,
        high: 0x1049 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1369 as ::core::ffi::c_ushort,
        high: 0x137c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16ee as ::core::ffi::c_ushort,
        high: 0x16f0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17e0 as ::core::ffi::c_ushort,
        high: 0x17e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17f0 as ::core::ffi::c_ushort,
        high: 0x17f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1810 as ::core::ffi::c_ushort,
        high: 0x1819 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1946 as ::core::ffi::c_ushort,
        high: 0x194f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2070 as ::core::ffi::c_ushort,
        high: 0x2070 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2074 as ::core::ffi::c_ushort,
        high: 0x2079 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2080 as ::core::ffi::c_ushort,
        high: 0x2089 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2153 as ::core::ffi::c_ushort,
        high: 0x2183 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2460 as ::core::ffi::c_ushort,
        high: 0x249b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x24ea as ::core::ffi::c_ushort,
        high: 0x24ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2776 as ::core::ffi::c_ushort,
        high: 0x2793 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3007 as ::core::ffi::c_ushort,
        high: 0x3007 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3021 as ::core::ffi::c_ushort,
        high: 0x3029 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3038 as ::core::ffi::c_ushort,
        high: 0x303a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3192 as ::core::ffi::c_ushort,
        high: 0x3195 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3220 as ::core::ffi::c_ushort,
        high: 0x3229 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3251 as ::core::ffi::c_ushort,
        high: 0x325f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3280 as ::core::ffi::c_ushort,
        high: 0x3289 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x32b1 as ::core::ffi::c_ushort,
        high: 0x32bf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff10 as ::core::ffi::c_ushort,
        high: 0xff19 as ::core::ffi::c_ushort,
    },
];
static mut xmlNL: [xmlChLRange; 5] = [
    _xmlChLRange {
        low: 0x10107 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10133 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10320 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10323 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1034a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1034a as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x104a0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x104a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7ce as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7ff as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlNG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 42 as ::core::ffi::c_int,
        nbLongRange: 5 as ::core::ffi::c_int,
        shortRange: &raw const xmlNS as *const xmlChSRange,
        longRange: &raw const xmlNL as *const xmlChLRange,
    }
};
static mut xmlNdS: [xmlChSRange; 21] = [
    _xmlChSRange {
        low: 0x30 as ::core::ffi::c_ushort,
        high: 0x39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x660 as ::core::ffi::c_ushort,
        high: 0x669 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6f0 as ::core::ffi::c_ushort,
        high: 0x6f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x966 as ::core::ffi::c_ushort,
        high: 0x96f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9e6 as ::core::ffi::c_ushort,
        high: 0x9ef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa66 as ::core::ffi::c_ushort,
        high: 0xa6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae6 as ::core::ffi::c_ushort,
        high: 0xaef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb66 as ::core::ffi::c_ushort,
        high: 0xb6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbe7 as ::core::ffi::c_ushort,
        high: 0xbef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xc66 as ::core::ffi::c_ushort,
        high: 0xc6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xce6 as ::core::ffi::c_ushort,
        high: 0xcef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd66 as ::core::ffi::c_ushort,
        high: 0xd6f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe50 as ::core::ffi::c_ushort,
        high: 0xe59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xed0 as ::core::ffi::c_ushort,
        high: 0xed9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf20 as ::core::ffi::c_ushort,
        high: 0xf29 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1040 as ::core::ffi::c_ushort,
        high: 0x1049 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1369 as ::core::ffi::c_ushort,
        high: 0x1371 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17e0 as ::core::ffi::c_ushort,
        high: 0x17e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1810 as ::core::ffi::c_ushort,
        high: 0x1819 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1946 as ::core::ffi::c_ushort,
        high: 0x194f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff10 as ::core::ffi::c_ushort,
        high: 0xff19 as ::core::ffi::c_ushort,
    },
];
static mut xmlNdL: [xmlChLRange; 2] = [
    _xmlChLRange {
        low: 0x104a0 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x104a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7ce as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7ff as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlNdG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 21 as ::core::ffi::c_int,
        nbLongRange: 2 as ::core::ffi::c_int,
        shortRange: &raw const xmlNdS as *const xmlChSRange,
        longRange: &raw const xmlNdL as *const xmlChLRange,
    }
};
static mut xmlNoS: [xmlChSRange; 20] = [
    _xmlChSRange {
        low: 0xb2 as ::core::ffi::c_ushort,
        high: 0xb3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb9 as ::core::ffi::c_ushort,
        high: 0xb9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbc as ::core::ffi::c_ushort,
        high: 0xbe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9f4 as ::core::ffi::c_ushort,
        high: 0x9f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbf0 as ::core::ffi::c_ushort,
        high: 0xbf2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf2a as ::core::ffi::c_ushort,
        high: 0xf33 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1372 as ::core::ffi::c_ushort,
        high: 0x137c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17f0 as ::core::ffi::c_ushort,
        high: 0x17f9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2070 as ::core::ffi::c_ushort,
        high: 0x2070 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2074 as ::core::ffi::c_ushort,
        high: 0x2079 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2080 as ::core::ffi::c_ushort,
        high: 0x2089 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2153 as ::core::ffi::c_ushort,
        high: 0x215f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2460 as ::core::ffi::c_ushort,
        high: 0x249b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x24ea as ::core::ffi::c_ushort,
        high: 0x24ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2776 as ::core::ffi::c_ushort,
        high: 0x2793 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3192 as ::core::ffi::c_ushort,
        high: 0x3195 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3220 as ::core::ffi::c_ushort,
        high: 0x3229 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3251 as ::core::ffi::c_ushort,
        high: 0x325f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3280 as ::core::ffi::c_ushort,
        high: 0x3289 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x32b1 as ::core::ffi::c_ushort,
        high: 0x32bf as ::core::ffi::c_ushort,
    },
];
static mut xmlNoL: [xmlChLRange; 2] = [
    _xmlChLRange {
        low: 0x10107 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10133 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10320 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10323 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlNoG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 20 as ::core::ffi::c_int,
        nbLongRange: 2 as ::core::ffi::c_int,
        shortRange: &raw const xmlNoS as *const xmlChSRange,
        longRange: &raw const xmlNoL as *const xmlChLRange,
    }
};
static mut xmlPS: [xmlChSRange; 84] = [
    _xmlChSRange {
        low: 0x21 as ::core::ffi::c_ushort,
        high: 0x23 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x25 as ::core::ffi::c_ushort,
        high: 0x2a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2c as ::core::ffi::c_ushort,
        high: 0x2f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3a as ::core::ffi::c_ushort,
        high: 0x3b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f as ::core::ffi::c_ushort,
        high: 0x40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5b as ::core::ffi::c_ushort,
        high: 0x5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5f as ::core::ffi::c_ushort,
        high: 0x5f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7b as ::core::ffi::c_ushort,
        high: 0x7b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7d as ::core::ffi::c_ushort,
        high: 0x7d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa1 as ::core::ffi::c_ushort,
        high: 0xa1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xab as ::core::ffi::c_ushort,
        high: 0xab as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb7 as ::core::ffi::c_ushort,
        high: 0xb7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbb as ::core::ffi::c_ushort,
        high: 0xbb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbf as ::core::ffi::c_ushort,
        high: 0xbf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x37e as ::core::ffi::c_ushort,
        high: 0x37e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x387 as ::core::ffi::c_ushort,
        high: 0x387 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x55a as ::core::ffi::c_ushort,
        high: 0x55f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x589 as ::core::ffi::c_ushort,
        high: 0x58a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5be as ::core::ffi::c_ushort,
        high: 0x5be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c0 as ::core::ffi::c_ushort,
        high: 0x5c0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c3 as ::core::ffi::c_ushort,
        high: 0x5c3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5f3 as ::core::ffi::c_ushort,
        high: 0x5f4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x60c as ::core::ffi::c_ushort,
        high: 0x60d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x61b as ::core::ffi::c_ushort,
        high: 0x61b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x61f as ::core::ffi::c_ushort,
        high: 0x61f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x66a as ::core::ffi::c_ushort,
        high: 0x66d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d4 as ::core::ffi::c_ushort,
        high: 0x6d4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x700 as ::core::ffi::c_ushort,
        high: 0x70d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x964 as ::core::ffi::c_ushort,
        high: 0x965 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x970 as ::core::ffi::c_ushort,
        high: 0x970 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdf4 as ::core::ffi::c_ushort,
        high: 0xdf4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe4f as ::core::ffi::c_ushort,
        high: 0xe4f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe5a as ::core::ffi::c_ushort,
        high: 0xe5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf04 as ::core::ffi::c_ushort,
        high: 0xf12 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3a as ::core::ffi::c_ushort,
        high: 0xf3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf85 as ::core::ffi::c_ushort,
        high: 0xf85 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x104a as ::core::ffi::c_ushort,
        high: 0x104f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10fb as ::core::ffi::c_ushort,
        high: 0x10fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1361 as ::core::ffi::c_ushort,
        high: 0x1368 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x166d as ::core::ffi::c_ushort,
        high: 0x166e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x169b as ::core::ffi::c_ushort,
        high: 0x169c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16eb as ::core::ffi::c_ushort,
        high: 0x16ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1735 as ::core::ffi::c_ushort,
        high: 0x1736 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17d4 as ::core::ffi::c_ushort,
        high: 0x17d6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17d8 as ::core::ffi::c_ushort,
        high: 0x17da as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1800 as ::core::ffi::c_ushort,
        high: 0x180a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1944 as ::core::ffi::c_ushort,
        high: 0x1945 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2010 as ::core::ffi::c_ushort,
        high: 0x2027 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2030 as ::core::ffi::c_ushort,
        high: 0x2043 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2045 as ::core::ffi::c_ushort,
        high: 0x2051 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2053 as ::core::ffi::c_ushort,
        high: 0x2054 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2057 as ::core::ffi::c_ushort,
        high: 0x2057 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207d as ::core::ffi::c_ushort,
        high: 0x207e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x208d as ::core::ffi::c_ushort,
        high: 0x208e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2329 as ::core::ffi::c_ushort,
        high: 0x232a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x23b4 as ::core::ffi::c_ushort,
        high: 0x23b6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2768 as ::core::ffi::c_ushort,
        high: 0x2775 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27e6 as ::core::ffi::c_ushort,
        high: 0x27eb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2983 as ::core::ffi::c_ushort,
        high: 0x2998 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29d8 as ::core::ffi::c_ushort,
        high: 0x29db as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29fc as ::core::ffi::c_ushort,
        high: 0x29fd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3001 as ::core::ffi::c_ushort,
        high: 0x3003 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3008 as ::core::ffi::c_ushort,
        high: 0x3011 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3014 as ::core::ffi::c_ushort,
        high: 0x301f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3030 as ::core::ffi::c_ushort,
        high: 0x3030 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x303d as ::core::ffi::c_ushort,
        high: 0x303d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30a0 as ::core::ffi::c_ushort,
        high: 0x30a0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30fb as ::core::ffi::c_ushort,
        high: 0x30fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfd3e as ::core::ffi::c_ushort,
        high: 0xfd3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe30 as ::core::ffi::c_ushort,
        high: 0xfe52 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe54 as ::core::ffi::c_ushort,
        high: 0xfe61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe63 as ::core::ffi::c_ushort,
        high: 0xfe63 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe68 as ::core::ffi::c_ushort,
        high: 0xfe68 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe6a as ::core::ffi::c_ushort,
        high: 0xfe6b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff01 as ::core::ffi::c_ushort,
        high: 0xff03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff05 as ::core::ffi::c_ushort,
        high: 0xff0a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff0c as ::core::ffi::c_ushort,
        high: 0xff0f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff1a as ::core::ffi::c_ushort,
        high: 0xff1b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff1f as ::core::ffi::c_ushort,
        high: 0xff20 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff3b as ::core::ffi::c_ushort,
        high: 0xff3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff3f as ::core::ffi::c_ushort,
        high: 0xff3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5b as ::core::ffi::c_ushort,
        high: 0xff5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5d as ::core::ffi::c_ushort,
        high: 0xff5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5f as ::core::ffi::c_ushort,
        high: 0xff65 as ::core::ffi::c_ushort,
    },
];
static mut xmlPL: [xmlChLRange; 2] = [
    _xmlChLRange {
        low: 0x10100 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10101 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1039f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1039f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlPG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 84 as ::core::ffi::c_int,
        nbLongRange: 2 as ::core::ffi::c_int,
        shortRange: &raw const xmlPS as *const xmlChSRange,
        longRange: &raw const xmlPL as *const xmlChLRange,
    }
};
static mut xmlPdS: [xmlChSRange; 11] = [
    _xmlChSRange {
        low: 0x2d as ::core::ffi::c_ushort,
        high: 0x2d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x58a as ::core::ffi::c_ushort,
        high: 0x58a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1806 as ::core::ffi::c_ushort,
        high: 0x1806 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2010 as ::core::ffi::c_ushort,
        high: 0x2015 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x301c as ::core::ffi::c_ushort,
        high: 0x301c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3030 as ::core::ffi::c_ushort,
        high: 0x3030 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x30a0 as ::core::ffi::c_ushort,
        high: 0x30a0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe31 as ::core::ffi::c_ushort,
        high: 0xfe32 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe58 as ::core::ffi::c_ushort,
        high: 0xfe58 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe63 as ::core::ffi::c_ushort,
        high: 0xfe63 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff0d as ::core::ffi::c_ushort,
        high: 0xff0d as ::core::ffi::c_ushort,
    },
];
static mut xmlPdG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 11 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlPdS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlPeS: [xmlChSRange; 63] = [
    _xmlChSRange {
        low: 0x29 as ::core::ffi::c_ushort,
        high: 0x29 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5d as ::core::ffi::c_ushort,
        high: 0x5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7d as ::core::ffi::c_ushort,
        high: 0x7d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3b as ::core::ffi::c_ushort,
        high: 0xf3b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3d as ::core::ffi::c_ushort,
        high: 0xf3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x169c as ::core::ffi::c_ushort,
        high: 0x169c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2046 as ::core::ffi::c_ushort,
        high: 0x2046 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207e as ::core::ffi::c_ushort,
        high: 0x207e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x208e as ::core::ffi::c_ushort,
        high: 0x208e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x232a as ::core::ffi::c_ushort,
        high: 0x232a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x23b5 as ::core::ffi::c_ushort,
        high: 0x23b5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2769 as ::core::ffi::c_ushort,
        high: 0x2769 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x276b as ::core::ffi::c_ushort,
        high: 0x276b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x276d as ::core::ffi::c_ushort,
        high: 0x276d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x276f as ::core::ffi::c_ushort,
        high: 0x276f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2771 as ::core::ffi::c_ushort,
        high: 0x2771 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2773 as ::core::ffi::c_ushort,
        high: 0x2773 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2775 as ::core::ffi::c_ushort,
        high: 0x2775 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27e7 as ::core::ffi::c_ushort,
        high: 0x27e7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27e9 as ::core::ffi::c_ushort,
        high: 0x27e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27eb as ::core::ffi::c_ushort,
        high: 0x27eb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2984 as ::core::ffi::c_ushort,
        high: 0x2984 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2986 as ::core::ffi::c_ushort,
        high: 0x2986 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2988 as ::core::ffi::c_ushort,
        high: 0x2988 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x298a as ::core::ffi::c_ushort,
        high: 0x298a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x298c as ::core::ffi::c_ushort,
        high: 0x298c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x298e as ::core::ffi::c_ushort,
        high: 0x298e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2990 as ::core::ffi::c_ushort,
        high: 0x2990 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2992 as ::core::ffi::c_ushort,
        high: 0x2992 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2994 as ::core::ffi::c_ushort,
        high: 0x2994 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2996 as ::core::ffi::c_ushort,
        high: 0x2996 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2998 as ::core::ffi::c_ushort,
        high: 0x2998 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29d9 as ::core::ffi::c_ushort,
        high: 0x29d9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29db as ::core::ffi::c_ushort,
        high: 0x29db as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29fd as ::core::ffi::c_ushort,
        high: 0x29fd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3009 as ::core::ffi::c_ushort,
        high: 0x3009 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x300b as ::core::ffi::c_ushort,
        high: 0x300b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x300d as ::core::ffi::c_ushort,
        high: 0x300d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x300f as ::core::ffi::c_ushort,
        high: 0x300f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3011 as ::core::ffi::c_ushort,
        high: 0x3011 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3015 as ::core::ffi::c_ushort,
        high: 0x3015 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3017 as ::core::ffi::c_ushort,
        high: 0x3017 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3019 as ::core::ffi::c_ushort,
        high: 0x3019 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x301b as ::core::ffi::c_ushort,
        high: 0x301b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x301e as ::core::ffi::c_ushort,
        high: 0x301f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfd3f as ::core::ffi::c_ushort,
        high: 0xfd3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe36 as ::core::ffi::c_ushort,
        high: 0xfe36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe38 as ::core::ffi::c_ushort,
        high: 0xfe38 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe3a as ::core::ffi::c_ushort,
        high: 0xfe3a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe3c as ::core::ffi::c_ushort,
        high: 0xfe3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe3e as ::core::ffi::c_ushort,
        high: 0xfe3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe40 as ::core::ffi::c_ushort,
        high: 0xfe40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe42 as ::core::ffi::c_ushort,
        high: 0xfe42 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe44 as ::core::ffi::c_ushort,
        high: 0xfe44 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe48 as ::core::ffi::c_ushort,
        high: 0xfe48 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe5a as ::core::ffi::c_ushort,
        high: 0xfe5a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe5c as ::core::ffi::c_ushort,
        high: 0xfe5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe5e as ::core::ffi::c_ushort,
        high: 0xfe5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff09 as ::core::ffi::c_ushort,
        high: 0xff09 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff3d as ::core::ffi::c_ushort,
        high: 0xff3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5d as ::core::ffi::c_ushort,
        high: 0xff5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff60 as ::core::ffi::c_ushort,
        high: 0xff60 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff63 as ::core::ffi::c_ushort,
        high: 0xff63 as ::core::ffi::c_ushort,
    },
];
static mut xmlPeG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 63 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlPeS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlPoS: [xmlChSRange; 72] = [
    _xmlChSRange {
        low: 0x21 as ::core::ffi::c_ushort,
        high: 0x23 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x25 as ::core::ffi::c_ushort,
        high: 0x27 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2a as ::core::ffi::c_ushort,
        high: 0x2a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2c as ::core::ffi::c_ushort,
        high: 0x2c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e as ::core::ffi::c_ushort,
        high: 0x2f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3a as ::core::ffi::c_ushort,
        high: 0x3b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f as ::core::ffi::c_ushort,
        high: 0x40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c as ::core::ffi::c_ushort,
        high: 0x5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa1 as ::core::ffi::c_ushort,
        high: 0xa1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb7 as ::core::ffi::c_ushort,
        high: 0xb7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbf as ::core::ffi::c_ushort,
        high: 0xbf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x37e as ::core::ffi::c_ushort,
        high: 0x37e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x387 as ::core::ffi::c_ushort,
        high: 0x387 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x55a as ::core::ffi::c_ushort,
        high: 0x55f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x589 as ::core::ffi::c_ushort,
        high: 0x589 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5be as ::core::ffi::c_ushort,
        high: 0x5be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c0 as ::core::ffi::c_ushort,
        high: 0x5c0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5c3 as ::core::ffi::c_ushort,
        high: 0x5c3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5f3 as ::core::ffi::c_ushort,
        high: 0x5f4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x60c as ::core::ffi::c_ushort,
        high: 0x60d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x61b as ::core::ffi::c_ushort,
        high: 0x61b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x61f as ::core::ffi::c_ushort,
        high: 0x61f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x66a as ::core::ffi::c_ushort,
        high: 0x66d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6d4 as ::core::ffi::c_ushort,
        high: 0x6d4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x700 as ::core::ffi::c_ushort,
        high: 0x70d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x964 as ::core::ffi::c_ushort,
        high: 0x965 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x970 as ::core::ffi::c_ushort,
        high: 0x970 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xdf4 as ::core::ffi::c_ushort,
        high: 0xdf4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe4f as ::core::ffi::c_ushort,
        high: 0xe4f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe5a as ::core::ffi::c_ushort,
        high: 0xe5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf04 as ::core::ffi::c_ushort,
        high: 0xf12 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf85 as ::core::ffi::c_ushort,
        high: 0xf85 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x104a as ::core::ffi::c_ushort,
        high: 0x104f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x10fb as ::core::ffi::c_ushort,
        high: 0x10fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1361 as ::core::ffi::c_ushort,
        high: 0x1368 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x166d as ::core::ffi::c_ushort,
        high: 0x166e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x16eb as ::core::ffi::c_ushort,
        high: 0x16ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1735 as ::core::ffi::c_ushort,
        high: 0x1736 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17d4 as ::core::ffi::c_ushort,
        high: 0x17d6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17d8 as ::core::ffi::c_ushort,
        high: 0x17da as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1800 as ::core::ffi::c_ushort,
        high: 0x1805 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1807 as ::core::ffi::c_ushort,
        high: 0x180a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1944 as ::core::ffi::c_ushort,
        high: 0x1945 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2016 as ::core::ffi::c_ushort,
        high: 0x2017 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2020 as ::core::ffi::c_ushort,
        high: 0x2027 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2030 as ::core::ffi::c_ushort,
        high: 0x2038 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x203b as ::core::ffi::c_ushort,
        high: 0x203e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2041 as ::core::ffi::c_ushort,
        high: 0x2043 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2047 as ::core::ffi::c_ushort,
        high: 0x2051 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2053 as ::core::ffi::c_ushort,
        high: 0x2053 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2057 as ::core::ffi::c_ushort,
        high: 0x2057 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x23b6 as ::core::ffi::c_ushort,
        high: 0x23b6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3001 as ::core::ffi::c_ushort,
        high: 0x3003 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x303d as ::core::ffi::c_ushort,
        high: 0x303d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe30 as ::core::ffi::c_ushort,
        high: 0xfe30 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe45 as ::core::ffi::c_ushort,
        high: 0xfe46 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe49 as ::core::ffi::c_ushort,
        high: 0xfe4c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe50 as ::core::ffi::c_ushort,
        high: 0xfe52 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe54 as ::core::ffi::c_ushort,
        high: 0xfe57 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe5f as ::core::ffi::c_ushort,
        high: 0xfe61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe68 as ::core::ffi::c_ushort,
        high: 0xfe68 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe6a as ::core::ffi::c_ushort,
        high: 0xfe6b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff01 as ::core::ffi::c_ushort,
        high: 0xff03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff05 as ::core::ffi::c_ushort,
        high: 0xff07 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff0a as ::core::ffi::c_ushort,
        high: 0xff0a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff0c as ::core::ffi::c_ushort,
        high: 0xff0c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff0e as ::core::ffi::c_ushort,
        high: 0xff0f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff1a as ::core::ffi::c_ushort,
        high: 0xff1b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff1f as ::core::ffi::c_ushort,
        high: 0xff20 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff3c as ::core::ffi::c_ushort,
        high: 0xff3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff61 as ::core::ffi::c_ushort,
        high: 0xff61 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff64 as ::core::ffi::c_ushort,
        high: 0xff64 as ::core::ffi::c_ushort,
    },
];
static mut xmlPoL: [xmlChLRange; 2] = [
    _xmlChLRange {
        low: 0x10100 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10101 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1039f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1039f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlPoG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 72 as ::core::ffi::c_int,
        nbLongRange: 2 as ::core::ffi::c_int,
        shortRange: &raw const xmlPoS as *const xmlChSRange,
        longRange: &raw const xmlPoL as *const xmlChLRange,
    }
};
static mut xmlPsS: [xmlChSRange; 65] = [
    _xmlChSRange {
        low: 0x28 as ::core::ffi::c_ushort,
        high: 0x28 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5b as ::core::ffi::c_ushort,
        high: 0x5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7b as ::core::ffi::c_ushort,
        high: 0x7b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3a as ::core::ffi::c_ushort,
        high: 0xf3a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf3c as ::core::ffi::c_ushort,
        high: 0xf3c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x169b as ::core::ffi::c_ushort,
        high: 0x169b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x201a as ::core::ffi::c_ushort,
        high: 0x201a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x201e as ::core::ffi::c_ushort,
        high: 0x201e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2045 as ::core::ffi::c_ushort,
        high: 0x2045 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207d as ::core::ffi::c_ushort,
        high: 0x207d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x208d as ::core::ffi::c_ushort,
        high: 0x208d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2329 as ::core::ffi::c_ushort,
        high: 0x2329 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x23b4 as ::core::ffi::c_ushort,
        high: 0x23b4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2768 as ::core::ffi::c_ushort,
        high: 0x2768 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x276a as ::core::ffi::c_ushort,
        high: 0x276a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x276c as ::core::ffi::c_ushort,
        high: 0x276c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x276e as ::core::ffi::c_ushort,
        high: 0x276e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2770 as ::core::ffi::c_ushort,
        high: 0x2770 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2772 as ::core::ffi::c_ushort,
        high: 0x2772 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2774 as ::core::ffi::c_ushort,
        high: 0x2774 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27e6 as ::core::ffi::c_ushort,
        high: 0x27e6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27e8 as ::core::ffi::c_ushort,
        high: 0x27e8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27ea as ::core::ffi::c_ushort,
        high: 0x27ea as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2983 as ::core::ffi::c_ushort,
        high: 0x2983 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2985 as ::core::ffi::c_ushort,
        high: 0x2985 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2987 as ::core::ffi::c_ushort,
        high: 0x2987 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2989 as ::core::ffi::c_ushort,
        high: 0x2989 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x298b as ::core::ffi::c_ushort,
        high: 0x298b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x298d as ::core::ffi::c_ushort,
        high: 0x298d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x298f as ::core::ffi::c_ushort,
        high: 0x298f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2991 as ::core::ffi::c_ushort,
        high: 0x2991 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2993 as ::core::ffi::c_ushort,
        high: 0x2993 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2995 as ::core::ffi::c_ushort,
        high: 0x2995 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2997 as ::core::ffi::c_ushort,
        high: 0x2997 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29d8 as ::core::ffi::c_ushort,
        high: 0x29d8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29da as ::core::ffi::c_ushort,
        high: 0x29da as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29fc as ::core::ffi::c_ushort,
        high: 0x29fc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3008 as ::core::ffi::c_ushort,
        high: 0x3008 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x300a as ::core::ffi::c_ushort,
        high: 0x300a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x300c as ::core::ffi::c_ushort,
        high: 0x300c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x300e as ::core::ffi::c_ushort,
        high: 0x300e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3010 as ::core::ffi::c_ushort,
        high: 0x3010 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3014 as ::core::ffi::c_ushort,
        high: 0x3014 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3016 as ::core::ffi::c_ushort,
        high: 0x3016 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3018 as ::core::ffi::c_ushort,
        high: 0x3018 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x301a as ::core::ffi::c_ushort,
        high: 0x301a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x301d as ::core::ffi::c_ushort,
        high: 0x301d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfd3e as ::core::ffi::c_ushort,
        high: 0xfd3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe35 as ::core::ffi::c_ushort,
        high: 0xfe35 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe37 as ::core::ffi::c_ushort,
        high: 0xfe37 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe39 as ::core::ffi::c_ushort,
        high: 0xfe39 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe3b as ::core::ffi::c_ushort,
        high: 0xfe3b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe3d as ::core::ffi::c_ushort,
        high: 0xfe3d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe3f as ::core::ffi::c_ushort,
        high: 0xfe3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe41 as ::core::ffi::c_ushort,
        high: 0xfe41 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe43 as ::core::ffi::c_ushort,
        high: 0xfe43 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe47 as ::core::ffi::c_ushort,
        high: 0xfe47 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe59 as ::core::ffi::c_ushort,
        high: 0xfe59 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe5b as ::core::ffi::c_ushort,
        high: 0xfe5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe5d as ::core::ffi::c_ushort,
        high: 0xfe5d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff08 as ::core::ffi::c_ushort,
        high: 0xff08 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff3b as ::core::ffi::c_ushort,
        high: 0xff3b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5b as ::core::ffi::c_ushort,
        high: 0xff5b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5f as ::core::ffi::c_ushort,
        high: 0xff5f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff62 as ::core::ffi::c_ushort,
        high: 0xff62 as ::core::ffi::c_ushort,
    },
];
static mut xmlPsG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 65 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlPsS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlSS: [xmlChSRange; 133] = [
    _xmlChSRange {
        low: 0x24 as ::core::ffi::c_ushort,
        high: 0x24 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2b as ::core::ffi::c_ushort,
        high: 0x2b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3c as ::core::ffi::c_ushort,
        high: 0x3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x5e as ::core::ffi::c_ushort,
        high: 0x5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x60 as ::core::ffi::c_ushort,
        high: 0x60 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7c as ::core::ffi::c_ushort,
        high: 0x7c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7e as ::core::ffi::c_ushort,
        high: 0x7e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa2 as ::core::ffi::c_ushort,
        high: 0xa9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac as ::core::ffi::c_ushort,
        high: 0xac as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae as ::core::ffi::c_ushort,
        high: 0xb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb4 as ::core::ffi::c_ushort,
        high: 0xb4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb6 as ::core::ffi::c_ushort,
        high: 0xb6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb8 as ::core::ffi::c_ushort,
        high: 0xb8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd7 as ::core::ffi::c_ushort,
        high: 0xd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf7 as ::core::ffi::c_ushort,
        high: 0xf7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2c2 as ::core::ffi::c_ushort,
        high: 0x2c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2d2 as ::core::ffi::c_ushort,
        high: 0x2df as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e5 as ::core::ffi::c_ushort,
        high: 0x2ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2ef as ::core::ffi::c_ushort,
        high: 0x2ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x374 as ::core::ffi::c_ushort,
        high: 0x375 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x384 as ::core::ffi::c_ushort,
        high: 0x385 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f6 as ::core::ffi::c_ushort,
        high: 0x3f6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x482 as ::core::ffi::c_ushort,
        high: 0x482 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x60e as ::core::ffi::c_ushort,
        high: 0x60f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e9 as ::core::ffi::c_ushort,
        high: 0x6e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6fd as ::core::ffi::c_ushort,
        high: 0x6fe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9f2 as ::core::ffi::c_ushort,
        high: 0x9f3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9fa as ::core::ffi::c_ushort,
        high: 0x9fa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaf1 as ::core::ffi::c_ushort,
        high: 0xaf1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb70 as ::core::ffi::c_ushort,
        high: 0xb70 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbf3 as ::core::ffi::c_ushort,
        high: 0xbfa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe3f as ::core::ffi::c_ushort,
        high: 0xe3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf01 as ::core::ffi::c_ushort,
        high: 0xf03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf13 as ::core::ffi::c_ushort,
        high: 0xf17 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf1a as ::core::ffi::c_ushort,
        high: 0xf1f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf34 as ::core::ffi::c_ushort,
        high: 0xf34 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf36 as ::core::ffi::c_ushort,
        high: 0xf36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf38 as ::core::ffi::c_ushort,
        high: 0xf38 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfbe as ::core::ffi::c_ushort,
        high: 0xfc5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfc7 as ::core::ffi::c_ushort,
        high: 0xfcc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfcf as ::core::ffi::c_ushort,
        high: 0xfcf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17db as ::core::ffi::c_ushort,
        high: 0x17db as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1940 as ::core::ffi::c_ushort,
        high: 0x1940 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x19e0 as ::core::ffi::c_ushort,
        high: 0x19ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbd as ::core::ffi::c_ushort,
        high: 0x1fbd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbf as ::core::ffi::c_ushort,
        high: 0x1fc1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fcd as ::core::ffi::c_ushort,
        high: 0x1fcf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fdd as ::core::ffi::c_ushort,
        high: 0x1fdf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fed as ::core::ffi::c_ushort,
        high: 0x1fef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ffd as ::core::ffi::c_ushort,
        high: 0x1ffe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2044 as ::core::ffi::c_ushort,
        high: 0x2044 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2052 as ::core::ffi::c_ushort,
        high: 0x2052 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207a as ::core::ffi::c_ushort,
        high: 0x207c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x208a as ::core::ffi::c_ushort,
        high: 0x208c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20a0 as ::core::ffi::c_ushort,
        high: 0x20b1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2100 as ::core::ffi::c_ushort,
        high: 0x2101 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2103 as ::core::ffi::c_ushort,
        high: 0x2106 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2108 as ::core::ffi::c_ushort,
        high: 0x2109 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2114 as ::core::ffi::c_ushort,
        high: 0x2114 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2116 as ::core::ffi::c_ushort,
        high: 0x2118 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x211e as ::core::ffi::c_ushort,
        high: 0x2123 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2125 as ::core::ffi::c_ushort,
        high: 0x2125 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2127 as ::core::ffi::c_ushort,
        high: 0x2127 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2129 as ::core::ffi::c_ushort,
        high: 0x2129 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212e as ::core::ffi::c_ushort,
        high: 0x212e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2132 as ::core::ffi::c_ushort,
        high: 0x2132 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x213a as ::core::ffi::c_ushort,
        high: 0x213b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2140 as ::core::ffi::c_ushort,
        high: 0x2144 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x214a as ::core::ffi::c_ushort,
        high: 0x214b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2190 as ::core::ffi::c_ushort,
        high: 0x2328 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x232b as ::core::ffi::c_ushort,
        high: 0x23b3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x23b7 as ::core::ffi::c_ushort,
        high: 0x23d0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2400 as ::core::ffi::c_ushort,
        high: 0x2426 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2440 as ::core::ffi::c_ushort,
        high: 0x244a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x249c as ::core::ffi::c_ushort,
        high: 0x24e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2500 as ::core::ffi::c_ushort,
        high: 0x2617 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2619 as ::core::ffi::c_ushort,
        high: 0x267d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2680 as ::core::ffi::c_ushort,
        high: 0x2691 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x26a0 as ::core::ffi::c_ushort,
        high: 0x26a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2701 as ::core::ffi::c_ushort,
        high: 0x2704 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2706 as ::core::ffi::c_ushort,
        high: 0x2709 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x270c as ::core::ffi::c_ushort,
        high: 0x2727 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2729 as ::core::ffi::c_ushort,
        high: 0x274b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x274d as ::core::ffi::c_ushort,
        high: 0x274d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x274f as ::core::ffi::c_ushort,
        high: 0x2752 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2756 as ::core::ffi::c_ushort,
        high: 0x2756 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2758 as ::core::ffi::c_ushort,
        high: 0x275e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2761 as ::core::ffi::c_ushort,
        high: 0x2767 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2794 as ::core::ffi::c_ushort,
        high: 0x2794 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2798 as ::core::ffi::c_ushort,
        high: 0x27af as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27b1 as ::core::ffi::c_ushort,
        high: 0x27be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27d0 as ::core::ffi::c_ushort,
        high: 0x27e5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27f0 as ::core::ffi::c_ushort,
        high: 0x2982 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2999 as ::core::ffi::c_ushort,
        high: 0x29d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29dc as ::core::ffi::c_ushort,
        high: 0x29fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29fe as ::core::ffi::c_ushort,
        high: 0x2b0d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e80 as ::core::ffi::c_ushort,
        high: 0x2e99 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e9b as ::core::ffi::c_ushort,
        high: 0x2ef3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2f00 as ::core::ffi::c_ushort,
        high: 0x2fd5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2ff0 as ::core::ffi::c_ushort,
        high: 0x2ffb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3004 as ::core::ffi::c_ushort,
        high: 0x3004 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3012 as ::core::ffi::c_ushort,
        high: 0x3013 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3020 as ::core::ffi::c_ushort,
        high: 0x3020 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3036 as ::core::ffi::c_ushort,
        high: 0x3037 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x303e as ::core::ffi::c_ushort,
        high: 0x303f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x309b as ::core::ffi::c_ushort,
        high: 0x309c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3190 as ::core::ffi::c_ushort,
        high: 0x3191 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3196 as ::core::ffi::c_ushort,
        high: 0x319f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3200 as ::core::ffi::c_ushort,
        high: 0x321e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x322a as ::core::ffi::c_ushort,
        high: 0x3243 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3250 as ::core::ffi::c_ushort,
        high: 0x3250 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3260 as ::core::ffi::c_ushort,
        high: 0x327d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x327f as ::core::ffi::c_ushort,
        high: 0x327f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x328a as ::core::ffi::c_ushort,
        high: 0x32b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x32c0 as ::core::ffi::c_ushort,
        high: 0x32fe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3300 as ::core::ffi::c_ushort,
        high: 0x33ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4dc0 as ::core::ffi::c_ushort,
        high: 0x4dff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa490 as ::core::ffi::c_ushort,
        high: 0xa4c6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb29 as ::core::ffi::c_ushort,
        high: 0xfb29 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfdfc as ::core::ffi::c_ushort,
        high: 0xfdfd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe62 as ::core::ffi::c_ushort,
        high: 0xfe62 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe64 as ::core::ffi::c_ushort,
        high: 0xfe66 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe69 as ::core::ffi::c_ushort,
        high: 0xfe69 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff04 as ::core::ffi::c_ushort,
        high: 0xff04 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff0b as ::core::ffi::c_ushort,
        high: 0xff0b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff1c as ::core::ffi::c_ushort,
        high: 0xff1e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff3e as ::core::ffi::c_ushort,
        high: 0xff3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff40 as ::core::ffi::c_ushort,
        high: 0xff40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5c as ::core::ffi::c_ushort,
        high: 0xff5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5e as ::core::ffi::c_ushort,
        high: 0xff5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe0 as ::core::ffi::c_ushort,
        high: 0xffe6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe8 as ::core::ffi::c_ushort,
        high: 0xffee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfffc as ::core::ffi::c_ushort,
        high: 0xfffd as ::core::ffi::c_ushort,
    },
];
static mut xmlSL: [xmlChLRange; 20] = [
    _xmlChLRange {
        low: 0x10102 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10102 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10137 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1013f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d0f5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d100 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d126 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d12a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d164 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d16a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d16c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d183 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d184 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d18c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d1a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d1ae as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d1dd as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d300 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d356 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6c1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6c1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6db as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6db as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6fb as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6fb as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d715 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d715 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d735 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d735 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d74f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d74f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d76f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d76f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d789 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d789 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7c3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7c3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlSG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 133 as ::core::ffi::c_int,
        nbLongRange: 20 as ::core::ffi::c_int,
        shortRange: &raw const xmlSS as *const xmlChSRange,
        longRange: &raw const xmlSL as *const xmlChLRange,
    }
};
static mut xmlScS: [xmlChSRange; 13] = [
    _xmlChSRange {
        low: 0x24 as ::core::ffi::c_ushort,
        high: 0x24 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa2 as ::core::ffi::c_ushort,
        high: 0xa5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9f2 as ::core::ffi::c_ushort,
        high: 0x9f3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaf1 as ::core::ffi::c_ushort,
        high: 0xaf1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbf9 as ::core::ffi::c_ushort,
        high: 0xbf9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xe3f as ::core::ffi::c_ushort,
        high: 0xe3f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x17db as ::core::ffi::c_ushort,
        high: 0x17db as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x20a0 as ::core::ffi::c_ushort,
        high: 0x20b1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfdfc as ::core::ffi::c_ushort,
        high: 0xfdfc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe69 as ::core::ffi::c_ushort,
        high: 0xfe69 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff04 as ::core::ffi::c_ushort,
        high: 0xff04 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe0 as ::core::ffi::c_ushort,
        high: 0xffe1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe5 as ::core::ffi::c_ushort,
        high: 0xffe6 as ::core::ffi::c_ushort,
    },
];
static mut xmlScG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 13 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlScS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlSkS: [xmlChSRange; 22] = [
    _xmlChSRange {
        low: 0x5e as ::core::ffi::c_ushort,
        high: 0x5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x60 as ::core::ffi::c_ushort,
        high: 0x60 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa8 as ::core::ffi::c_ushort,
        high: 0xa8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xaf as ::core::ffi::c_ushort,
        high: 0xaf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb4 as ::core::ffi::c_ushort,
        high: 0xb4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb8 as ::core::ffi::c_ushort,
        high: 0xb8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2c2 as ::core::ffi::c_ushort,
        high: 0x2c5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2d2 as ::core::ffi::c_ushort,
        high: 0x2df as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e5 as ::core::ffi::c_ushort,
        high: 0x2ed as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2ef as ::core::ffi::c_ushort,
        high: 0x2ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x374 as ::core::ffi::c_ushort,
        high: 0x375 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x384 as ::core::ffi::c_ushort,
        high: 0x385 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbd as ::core::ffi::c_ushort,
        high: 0x1fbd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fbf as ::core::ffi::c_ushort,
        high: 0x1fc1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fcd as ::core::ffi::c_ushort,
        high: 0x1fcf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fdd as ::core::ffi::c_ushort,
        high: 0x1fdf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1fed as ::core::ffi::c_ushort,
        high: 0x1fef as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1ffd as ::core::ffi::c_ushort,
        high: 0x1ffe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x309b as ::core::ffi::c_ushort,
        high: 0x309c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff3e as ::core::ffi::c_ushort,
        high: 0xff3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff40 as ::core::ffi::c_ushort,
        high: 0xff40 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe3 as ::core::ffi::c_ushort,
        high: 0xffe3 as ::core::ffi::c_ushort,
    },
];
static mut xmlSkG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 22 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlSkS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlSmS: [xmlChSRange; 48] = [
    _xmlChSRange {
        low: 0x2b as ::core::ffi::c_ushort,
        high: 0x2b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3c as ::core::ffi::c_ushort,
        high: 0x3e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7c as ::core::ffi::c_ushort,
        high: 0x7c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x7e as ::core::ffi::c_ushort,
        high: 0x7e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xac as ::core::ffi::c_ushort,
        high: 0xac as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb1 as ::core::ffi::c_ushort,
        high: 0xb1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xd7 as ::core::ffi::c_ushort,
        high: 0xd7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf7 as ::core::ffi::c_ushort,
        high: 0xf7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3f6 as ::core::ffi::c_ushort,
        high: 0x3f6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2044 as ::core::ffi::c_ushort,
        high: 0x2044 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2052 as ::core::ffi::c_ushort,
        high: 0x2052 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x207a as ::core::ffi::c_ushort,
        high: 0x207c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x208a as ::core::ffi::c_ushort,
        high: 0x208c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2140 as ::core::ffi::c_ushort,
        high: 0x2144 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x214b as ::core::ffi::c_ushort,
        high: 0x214b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2190 as ::core::ffi::c_ushort,
        high: 0x2194 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x219a as ::core::ffi::c_ushort,
        high: 0x219b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21a0 as ::core::ffi::c_ushort,
        high: 0x21a0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21a3 as ::core::ffi::c_ushort,
        high: 0x21a3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21a6 as ::core::ffi::c_ushort,
        high: 0x21a6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21ae as ::core::ffi::c_ushort,
        high: 0x21ae as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21ce as ::core::ffi::c_ushort,
        high: 0x21cf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21d2 as ::core::ffi::c_ushort,
        high: 0x21d2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21d4 as ::core::ffi::c_ushort,
        high: 0x21d4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21f4 as ::core::ffi::c_ushort,
        high: 0x22ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2308 as ::core::ffi::c_ushort,
        high: 0x230b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2320 as ::core::ffi::c_ushort,
        high: 0x2321 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x237c as ::core::ffi::c_ushort,
        high: 0x237c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x239b as ::core::ffi::c_ushort,
        high: 0x23b3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x25b7 as ::core::ffi::c_ushort,
        high: 0x25b7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x25c1 as ::core::ffi::c_ushort,
        high: 0x25c1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x25f8 as ::core::ffi::c_ushort,
        high: 0x25ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x266f as ::core::ffi::c_ushort,
        high: 0x266f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27d0 as ::core::ffi::c_ushort,
        high: 0x27e5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27f0 as ::core::ffi::c_ushort,
        high: 0x27ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2900 as ::core::ffi::c_ushort,
        high: 0x2982 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2999 as ::core::ffi::c_ushort,
        high: 0x29d7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29dc as ::core::ffi::c_ushort,
        high: 0x29fb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x29fe as ::core::ffi::c_ushort,
        high: 0x2aff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfb29 as ::core::ffi::c_ushort,
        high: 0xfb29 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe62 as ::core::ffi::c_ushort,
        high: 0xfe62 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfe64 as ::core::ffi::c_ushort,
        high: 0xfe66 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff0b as ::core::ffi::c_ushort,
        high: 0xff0b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff1c as ::core::ffi::c_ushort,
        high: 0xff1e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5c as ::core::ffi::c_ushort,
        high: 0xff5c as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xff5e as ::core::ffi::c_ushort,
        high: 0xff5e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe2 as ::core::ffi::c_ushort,
        high: 0xffe2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe9 as ::core::ffi::c_ushort,
        high: 0xffec as ::core::ffi::c_ushort,
    },
];
static mut xmlSmL: [xmlChLRange; 10] = [
    _xmlChLRange {
        low: 0x1d6c1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6c1 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6db as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6db as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d6fb as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d6fb as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d715 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d715 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d735 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d735 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d74f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d74f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d76f as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d76f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d789 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d789 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d7c3 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d7c3 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlSmG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 48 as ::core::ffi::c_int,
        nbLongRange: 10 as ::core::ffi::c_int,
        shortRange: &raw const xmlSmS as *const xmlChSRange,
        longRange: &raw const xmlSmL as *const xmlChLRange,
    }
};
static mut xmlSoS: [xmlChSRange; 103] = [
    _xmlChSRange {
        low: 0xa6 as ::core::ffi::c_ushort,
        high: 0xa7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa9 as ::core::ffi::c_ushort,
        high: 0xa9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xae as ::core::ffi::c_ushort,
        high: 0xae as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb0 as ::core::ffi::c_ushort,
        high: 0xb0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb6 as ::core::ffi::c_ushort,
        high: 0xb6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x482 as ::core::ffi::c_ushort,
        high: 0x482 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x60e as ::core::ffi::c_ushort,
        high: 0x60f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6e9 as ::core::ffi::c_ushort,
        high: 0x6e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x6fd as ::core::ffi::c_ushort,
        high: 0x6fe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x9fa as ::core::ffi::c_ushort,
        high: 0x9fa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xb70 as ::core::ffi::c_ushort,
        high: 0xb70 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbf3 as ::core::ffi::c_ushort,
        high: 0xbf8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xbfa as ::core::ffi::c_ushort,
        high: 0xbfa as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf01 as ::core::ffi::c_ushort,
        high: 0xf03 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf13 as ::core::ffi::c_ushort,
        high: 0xf17 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf1a as ::core::ffi::c_ushort,
        high: 0xf1f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf34 as ::core::ffi::c_ushort,
        high: 0xf34 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf36 as ::core::ffi::c_ushort,
        high: 0xf36 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xf38 as ::core::ffi::c_ushort,
        high: 0xf38 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfbe as ::core::ffi::c_ushort,
        high: 0xfc5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfc7 as ::core::ffi::c_ushort,
        high: 0xfcc as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfcf as ::core::ffi::c_ushort,
        high: 0xfcf as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1940 as ::core::ffi::c_ushort,
        high: 0x1940 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x19e0 as ::core::ffi::c_ushort,
        high: 0x19ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2100 as ::core::ffi::c_ushort,
        high: 0x2101 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2103 as ::core::ffi::c_ushort,
        high: 0x2106 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2108 as ::core::ffi::c_ushort,
        high: 0x2109 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2114 as ::core::ffi::c_ushort,
        high: 0x2114 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2116 as ::core::ffi::c_ushort,
        high: 0x2118 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x211e as ::core::ffi::c_ushort,
        high: 0x2123 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2125 as ::core::ffi::c_ushort,
        high: 0x2125 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2127 as ::core::ffi::c_ushort,
        high: 0x2127 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2129 as ::core::ffi::c_ushort,
        high: 0x2129 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x212e as ::core::ffi::c_ushort,
        high: 0x212e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2132 as ::core::ffi::c_ushort,
        high: 0x2132 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x213a as ::core::ffi::c_ushort,
        high: 0x213b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x214a as ::core::ffi::c_ushort,
        high: 0x214a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2195 as ::core::ffi::c_ushort,
        high: 0x2199 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x219c as ::core::ffi::c_ushort,
        high: 0x219f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21a1 as ::core::ffi::c_ushort,
        high: 0x21a2 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21a4 as ::core::ffi::c_ushort,
        high: 0x21a5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21a7 as ::core::ffi::c_ushort,
        high: 0x21ad as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21af as ::core::ffi::c_ushort,
        high: 0x21cd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21d0 as ::core::ffi::c_ushort,
        high: 0x21d1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21d3 as ::core::ffi::c_ushort,
        high: 0x21d3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x21d5 as ::core::ffi::c_ushort,
        high: 0x21f3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2300 as ::core::ffi::c_ushort,
        high: 0x2307 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x230c as ::core::ffi::c_ushort,
        high: 0x231f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2322 as ::core::ffi::c_ushort,
        high: 0x2328 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x232b as ::core::ffi::c_ushort,
        high: 0x237b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x237d as ::core::ffi::c_ushort,
        high: 0x239a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x23b7 as ::core::ffi::c_ushort,
        high: 0x23d0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2400 as ::core::ffi::c_ushort,
        high: 0x2426 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2440 as ::core::ffi::c_ushort,
        high: 0x244a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x249c as ::core::ffi::c_ushort,
        high: 0x24e9 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2500 as ::core::ffi::c_ushort,
        high: 0x25b6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x25b8 as ::core::ffi::c_ushort,
        high: 0x25c0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x25c2 as ::core::ffi::c_ushort,
        high: 0x25f7 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2600 as ::core::ffi::c_ushort,
        high: 0x2617 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2619 as ::core::ffi::c_ushort,
        high: 0x266e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2670 as ::core::ffi::c_ushort,
        high: 0x267d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2680 as ::core::ffi::c_ushort,
        high: 0x2691 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x26a0 as ::core::ffi::c_ushort,
        high: 0x26a1 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2701 as ::core::ffi::c_ushort,
        high: 0x2704 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2706 as ::core::ffi::c_ushort,
        high: 0x2709 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x270c as ::core::ffi::c_ushort,
        high: 0x2727 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2729 as ::core::ffi::c_ushort,
        high: 0x274b as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x274d as ::core::ffi::c_ushort,
        high: 0x274d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x274f as ::core::ffi::c_ushort,
        high: 0x2752 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2756 as ::core::ffi::c_ushort,
        high: 0x2756 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2758 as ::core::ffi::c_ushort,
        high: 0x275e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2761 as ::core::ffi::c_ushort,
        high: 0x2767 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2794 as ::core::ffi::c_ushort,
        high: 0x2794 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2798 as ::core::ffi::c_ushort,
        high: 0x27af as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x27b1 as ::core::ffi::c_ushort,
        high: 0x27be as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2800 as ::core::ffi::c_ushort,
        high: 0x28ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2b00 as ::core::ffi::c_ushort,
        high: 0x2b0d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e80 as ::core::ffi::c_ushort,
        high: 0x2e99 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2e9b as ::core::ffi::c_ushort,
        high: 0x2ef3 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2f00 as ::core::ffi::c_ushort,
        high: 0x2fd5 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2ff0 as ::core::ffi::c_ushort,
        high: 0x2ffb as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3004 as ::core::ffi::c_ushort,
        high: 0x3004 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3012 as ::core::ffi::c_ushort,
        high: 0x3013 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3020 as ::core::ffi::c_ushort,
        high: 0x3020 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3036 as ::core::ffi::c_ushort,
        high: 0x3037 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x303e as ::core::ffi::c_ushort,
        high: 0x303f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3190 as ::core::ffi::c_ushort,
        high: 0x3191 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3196 as ::core::ffi::c_ushort,
        high: 0x319f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3200 as ::core::ffi::c_ushort,
        high: 0x321e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x322a as ::core::ffi::c_ushort,
        high: 0x3243 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3250 as ::core::ffi::c_ushort,
        high: 0x3250 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3260 as ::core::ffi::c_ushort,
        high: 0x327d as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x327f as ::core::ffi::c_ushort,
        high: 0x327f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x328a as ::core::ffi::c_ushort,
        high: 0x32b0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x32c0 as ::core::ffi::c_ushort,
        high: 0x32fe as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3300 as ::core::ffi::c_ushort,
        high: 0x33ff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x4dc0 as ::core::ffi::c_ushort,
        high: 0x4dff as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa490 as ::core::ffi::c_ushort,
        high: 0xa4c6 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfdfd as ::core::ffi::c_ushort,
        high: 0xfdfd as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe4 as ::core::ffi::c_ushort,
        high: 0xffe4 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffe8 as ::core::ffi::c_ushort,
        high: 0xffe8 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xffed as ::core::ffi::c_ushort,
        high: 0xffee as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xfffc as ::core::ffi::c_ushort,
        high: 0xfffd as ::core::ffi::c_ushort,
    },
];
static mut xmlSoL: [xmlChLRange; 10] = [
    _xmlChLRange {
        low: 0x10102 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x10102 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x10137 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1013f as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d0f5 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d100 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d126 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d12a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d164 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d16a as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d16c as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d183 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d184 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d18c as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d1a9 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d1ae as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d1dd as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
    _xmlChLRange {
        low: 0x1d300 as ::core::ffi::c_int as ::core::ffi::c_uint,
        high: 0x1d356 as ::core::ffi::c_int as ::core::ffi::c_uint,
    },
];
static mut xmlSoG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 103 as ::core::ffi::c_int,
        nbLongRange: 10 as ::core::ffi::c_int,
        shortRange: &raw const xmlSoS as *const xmlChSRange,
        longRange: &raw const xmlSoL as *const xmlChLRange,
    }
};
static mut xmlZS: [xmlChSRange; 9] = [
    _xmlChSRange {
        low: 0x20 as ::core::ffi::c_ushort,
        high: 0x20 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0xa0 as ::core::ffi::c_ushort,
        high: 0xa0 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x1680 as ::core::ffi::c_ushort,
        high: 0x1680 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x180e as ::core::ffi::c_ushort,
        high: 0x180e as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2000 as ::core::ffi::c_ushort,
        high: 0x200a as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x2028 as ::core::ffi::c_ushort,
        high: 0x2029 as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x202f as ::core::ffi::c_ushort,
        high: 0x202f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x205f as ::core::ffi::c_ushort,
        high: 0x205f as ::core::ffi::c_ushort,
    },
    _xmlChSRange {
        low: 0x3000 as ::core::ffi::c_ushort,
        high: 0x3000 as ::core::ffi::c_ushort,
    },
];
static mut xmlZG: xmlChRangeGroup = unsafe {
    _xmlChRangeGroup {
        nbShortRange: 9 as ::core::ffi::c_int,
        nbLongRange: 0 as ::core::ffi::c_int,
        shortRange: &raw const xmlZS as *const xmlChSRange,
        longRange: ::core::ptr::null::<xmlChLRange>(),
    }
};
static mut xmlUnicodeBlockTbl: xmlUnicodeNameTable = unsafe {
    xmlUnicodeNameTable {
        table: &raw const xmlUnicodeBlocks as *const xmlUnicodeRange,
        numentries: 128 as ::core::ffi::c_int,
    }
};
static mut xmlUnicodeCatTbl: xmlUnicodeNameTable = unsafe {
    xmlUnicodeNameTable {
        table: &raw const xmlUnicodeCats as *const xmlUnicodeRange,
        numentries: 36 as ::core::ffi::c_int,
    }
};
unsafe extern "C" fn xmlUnicodeLookup(
    mut tptr: *const xmlUnicodeNameTable,
    mut tname: *const ::core::ffi::c_char,
) -> Option<xmlIntFunc> {
    let mut low: ::core::ffi::c_int = 0;
    let mut high: ::core::ffi::c_int = 0;
    let mut mid: ::core::ffi::c_int = 0;
    let mut cmp: ::core::ffi::c_int = 0;
    let mut sptr: *const xmlUnicodeRange = ::core::ptr::null::<xmlUnicodeRange>();
    if tptr.is_null() || tname.is_null() {
        return None;
    }
    low = 0 as ::core::ffi::c_int;
    high = (*tptr).numentries - 1 as ::core::ffi::c_int;
    sptr = (*tptr).table;
    while low <= high {
        mid = (low + high) / 2 as ::core::ffi::c_int;
        cmp = strcmp(tname, (*sptr.offset(mid as isize)).rangename);
        if cmp == 0 as ::core::ffi::c_int {
            return (*sptr.offset(mid as isize)).func;
        }
        if cmp < 0 as ::core::ffi::c_int {
            high = mid - 1 as ::core::ffi::c_int;
        } else {
            low = mid + 1 as ::core::ffi::c_int;
        }
    }
    return None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsAegeanNumbers(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10100 as ::core::ffi::c_int && code <= 0x1013f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsAlphabeticPresentationForms(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xfb00 as ::core::ffi::c_int && code <= 0xfb4f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsArabic(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x600 as ::core::ffi::c_int && code <= 0x6ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsArabicPresentationFormsA(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xfb50 as ::core::ffi::c_int && code <= 0xfdff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsArabicPresentationFormsB(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xfe70 as ::core::ffi::c_int && code <= 0xfeff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsArmenian(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x530 as ::core::ffi::c_int && code <= 0x58f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsArrows(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x2190 as ::core::ffi::c_int && code <= 0x21ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBasicLatin(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0 as ::core::ffi::c_int && code <= 0x7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBengali(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x980 as ::core::ffi::c_int && code <= 0x9ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBlockElements(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x2580 as ::core::ffi::c_int && code <= 0x259f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBopomofo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x3100 as ::core::ffi::c_int && code <= 0x312f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBopomofoExtended(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x31a0 as ::core::ffi::c_int && code <= 0x31bf as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBoxDrawing(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x2500 as ::core::ffi::c_int && code <= 0x257f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBraillePatterns(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2800 as ::core::ffi::c_int && code <= 0x28ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBuhid(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1740 as ::core::ffi::c_int && code <= 0x175f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsByzantineMusicalSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x1d000 as ::core::ffi::c_int && code <= 0x1d0ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKCompatibility(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x3300 as ::core::ffi::c_int && code <= 0x33ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKCompatibilityForms(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xfe30 as ::core::ffi::c_int && code <= 0xfe4f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKCompatibilityIdeographs(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xf900 as ::core::ffi::c_int && code <= 0xfaff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKCompatibilityIdeographsSupplement(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2f800 as ::core::ffi::c_int && code <= 0x2fa1f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKRadicalsSupplement(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2e80 as ::core::ffi::c_int && code <= 0x2eff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKSymbolsandPunctuation(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x3000 as ::core::ffi::c_int && code <= 0x303f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKUnifiedIdeographs(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x4e00 as ::core::ffi::c_int && code <= 0x9fff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKUnifiedIdeographsExtensionA(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x3400 as ::core::ffi::c_int && code <= 0x4dbf as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCJKUnifiedIdeographsExtensionB(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x20000 as ::core::ffi::c_int && code <= 0x2a6df as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCherokee(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x13a0 as ::core::ffi::c_int && code <= 0x13ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCombiningDiacriticalMarks(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x300 as ::core::ffi::c_int && code <= 0x36f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCombiningDiacriticalMarksforSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x20d0 as ::core::ffi::c_int && code <= 0x20ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCombiningHalfMarks(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xfe20 as ::core::ffi::c_int && code <= 0xfe2f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCombiningMarksforSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x20d0 as ::core::ffi::c_int && code <= 0x20ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsControlPictures(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2400 as ::core::ffi::c_int && code <= 0x243f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCurrencySymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x20a0 as ::core::ffi::c_int && code <= 0x20cf as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCypriotSyllabary(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x10800 as ::core::ffi::c_int && code <= 0x1083f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCyrillic(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x400 as ::core::ffi::c_int && code <= 0x4ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCyrillicSupplement(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x500 as ::core::ffi::c_int && code <= 0x52f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsDeseret(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10400 as ::core::ffi::c_int && code <= 0x1044f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsDevanagari(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x900 as ::core::ffi::c_int && code <= 0x97f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsDingbats(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x2700 as ::core::ffi::c_int && code <= 0x27bf as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsEnclosedAlphanumerics(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2460 as ::core::ffi::c_int && code <= 0x24ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsEnclosedCJKLettersandMonths(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x3200 as ::core::ffi::c_int && code <= 0x32ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsEthiopic(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1200 as ::core::ffi::c_int && code <= 0x137f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGeneralPunctuation(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2000 as ::core::ffi::c_int && code <= 0x206f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGeometricShapes(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x25a0 as ::core::ffi::c_int && code <= 0x25ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGeorgian(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10a0 as ::core::ffi::c_int && code <= 0x10ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGothic(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10330 as ::core::ffi::c_int && code <= 0x1034f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGreek(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x370 as ::core::ffi::c_int && code <= 0x3ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGreekExtended(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1f00 as ::core::ffi::c_int && code <= 0x1fff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGreekandCoptic(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x370 as ::core::ffi::c_int && code <= 0x3ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGujarati(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xa80 as ::core::ffi::c_int && code <= 0xaff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsGurmukhi(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xa00 as ::core::ffi::c_int && code <= 0xa7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHalfwidthandFullwidthForms(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xff00 as ::core::ffi::c_int && code <= 0xffef as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHangulCompatibilityJamo(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x3130 as ::core::ffi::c_int && code <= 0x318f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHangulJamo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1100 as ::core::ffi::c_int && code <= 0x11ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHangulSyllables(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xac00 as ::core::ffi::c_int && code <= 0xd7af as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHanunoo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1720 as ::core::ffi::c_int && code <= 0x173f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHebrew(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x590 as ::core::ffi::c_int && code <= 0x5ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHighPrivateUseSurrogates(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xdb80 as ::core::ffi::c_int && code <= 0xdbff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHighSurrogates(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xd800 as ::core::ffi::c_int && code <= 0xdb7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsHiragana(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x3040 as ::core::ffi::c_int && code <= 0x309f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsIPAExtensions(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x250 as ::core::ffi::c_int && code <= 0x2af as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsIdeographicDescriptionCharacters(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2ff0 as ::core::ffi::c_int && code <= 0x2fff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsKanbun(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x3190 as ::core::ffi::c_int && code <= 0x319f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsKangxiRadicals(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2f00 as ::core::ffi::c_int && code <= 0x2fdf as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsKannada(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xc80 as ::core::ffi::c_int && code <= 0xcff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsKatakana(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x30a0 as ::core::ffi::c_int && code <= 0x30ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsKatakanaPhoneticExtensions(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x31f0 as ::core::ffi::c_int && code <= 0x31ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsKhmer(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1780 as ::core::ffi::c_int && code <= 0x17ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsKhmerSymbols(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x19e0 as ::core::ffi::c_int && code <= 0x19ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLao(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xe80 as ::core::ffi::c_int && code <= 0xeff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLatin1Supplement(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x80 as ::core::ffi::c_int && code <= 0xff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLatinExtendedA(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x100 as ::core::ffi::c_int && code <= 0x17f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLatinExtendedB(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x180 as ::core::ffi::c_int && code <= 0x24f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLatinExtendedAdditional(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x1e00 as ::core::ffi::c_int && code <= 0x1eff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLetterlikeSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2100 as ::core::ffi::c_int && code <= 0x214f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLimbu(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1900 as ::core::ffi::c_int && code <= 0x194f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLinearBIdeograms(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x10080 as ::core::ffi::c_int && code <= 0x100ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLinearBSyllabary(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x10000 as ::core::ffi::c_int && code <= 0x1007f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsLowSurrogates(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xdc00 as ::core::ffi::c_int && code <= 0xdfff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMalayalam(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xd00 as ::core::ffi::c_int && code <= 0xd7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMathematicalAlphanumericSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x1d400 as ::core::ffi::c_int && code <= 0x1d7ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMathematicalOperators(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2200 as ::core::ffi::c_int && code <= 0x22ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMiscellaneousMathematicalSymbolsA(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x27c0 as ::core::ffi::c_int && code <= 0x27ef as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMiscellaneousMathematicalSymbolsB(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2980 as ::core::ffi::c_int && code <= 0x29ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMiscellaneousSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2600 as ::core::ffi::c_int && code <= 0x26ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMiscellaneousSymbolsandArrows(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2b00 as ::core::ffi::c_int && code <= 0x2bff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMiscellaneousTechnical(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2300 as ::core::ffi::c_int && code <= 0x23ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMongolian(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1800 as ::core::ffi::c_int && code <= 0x18af as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMusicalSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x1d100 as ::core::ffi::c_int && code <= 0x1d1ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsMyanmar(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1000 as ::core::ffi::c_int && code <= 0x109f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsNumberForms(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x2150 as ::core::ffi::c_int && code <= 0x218f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsOgham(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1680 as ::core::ffi::c_int && code <= 0x169f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsOldItalic(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10300 as ::core::ffi::c_int && code <= 0x1032f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsOpticalCharacterRecognition(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2440 as ::core::ffi::c_int && code <= 0x245f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsOriya(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xb00 as ::core::ffi::c_int && code <= 0xb7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsOsmanya(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10480 as ::core::ffi::c_int && code <= 0x104af as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsPhoneticExtensions(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x1d00 as ::core::ffi::c_int && code <= 0x1d7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsPrivateUse(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xe000 as ::core::ffi::c_int && code <= 0xf8ff as ::core::ffi::c_int
        || code >= 0xf0000 as ::core::ffi::c_int && code <= 0xfffff as ::core::ffi::c_int
        || code >= 0x100000 as ::core::ffi::c_int && code <= 0x10ffff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsPrivateUseArea(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xe000 as ::core::ffi::c_int && code <= 0xf8ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsRunic(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x16a0 as ::core::ffi::c_int && code <= 0x16ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsShavian(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10450 as ::core::ffi::c_int && code <= 0x1047f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSinhala(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xd80 as ::core::ffi::c_int && code <= 0xdff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSmallFormVariants(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xfe50 as ::core::ffi::c_int && code <= 0xfe6f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSpacingModifierLetters(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2b0 as ::core::ffi::c_int && code <= 0x2ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSpecials(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xfff0 as ::core::ffi::c_int && code <= 0xffff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSuperscriptsandSubscripts(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2070 as ::core::ffi::c_int && code <= 0x209f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSupplementalArrowsA(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x27f0 as ::core::ffi::c_int && code <= 0x27ff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSupplementalArrowsB(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2900 as ::core::ffi::c_int && code <= 0x297f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSupplementalMathematicalOperators(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x2a00 as ::core::ffi::c_int && code <= 0x2aff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSupplementaryPrivateUseAreaA(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xf0000 as ::core::ffi::c_int && code <= 0xfffff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSupplementaryPrivateUseAreaB(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x100000 as ::core::ffi::c_int && code <= 0x10ffff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsSyriac(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x700 as ::core::ffi::c_int && code <= 0x74f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTagalog(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1700 as ::core::ffi::c_int && code <= 0x171f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTagbanwa(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1760 as ::core::ffi::c_int && code <= 0x177f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTags(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xe0000 as ::core::ffi::c_int && code <= 0xe007f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTaiLe(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x1950 as ::core::ffi::c_int && code <= 0x197f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTaiXuanJingSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x1d300 as ::core::ffi::c_int && code <= 0x1d35f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTamil(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xb80 as ::core::ffi::c_int && code <= 0xbff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTelugu(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xc00 as ::core::ffi::c_int && code <= 0xc7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsThaana(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x780 as ::core::ffi::c_int && code <= 0x7bf as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsThai(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xe00 as ::core::ffi::c_int && code <= 0xe7f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsTibetan(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xf00 as ::core::ffi::c_int && code <= 0xfff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsUgaritic(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x10380 as ::core::ffi::c_int && code <= 0x1039f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsUnifiedCanadianAboriginalSyllabics(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x1400 as ::core::ffi::c_int && code <= 0x167f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsVariationSelectors(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xfe00 as ::core::ffi::c_int && code <= 0xfe0f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsVariationSelectorsSupplement(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0xe0100 as ::core::ffi::c_int && code <= 0xe01ef as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsYiRadicals(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xa490 as ::core::ffi::c_int && code <= 0xa4cf as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsYiSyllables(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0xa000 as ::core::ffi::c_int && code <= 0xa48f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsYijingHexagramSymbols(
    mut code: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (code >= 0x4dc0 as ::core::ffi::c_int && code <= 0x4dff as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsBlock(
    mut code: ::core::ffi::c_int,
    mut block: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut func: Option<xmlIntFunc> = None;
    func = xmlUnicodeLookup(&raw const xmlUnicodeBlockTbl, block);
    if func.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    return func.expect("non-null function pointer")(code);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatC(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlCG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatCc(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0 as ::core::ffi::c_int && code <= 0x1f as ::core::ffi::c_int
        || code >= 0x7f as ::core::ffi::c_int && code <= 0x9f as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatCf(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlCfG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatCo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0xe000 as ::core::ffi::c_int
        || code == 0xf8ff as ::core::ffi::c_int
        || code == 0xf0000 as ::core::ffi::c_int
        || code == 0xffffd as ::core::ffi::c_int
        || code == 0x100000 as ::core::ffi::c_int
        || code == 0x10fffd as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatCs(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0xd800 as ::core::ffi::c_int
        || code >= 0xdb7f as ::core::ffi::c_int && code <= 0xdb80 as ::core::ffi::c_int
        || code >= 0xdbff as ::core::ffi::c_int && code <= 0xdc00 as ::core::ffi::c_int
        || code == 0xdfff as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatL(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlLG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatLl(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlLlG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatLm(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlLmG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatLo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlLoG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatLt(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlLtG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatLu(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlLuG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatM(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlMG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatMc(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlMcG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatMe(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x488 as ::core::ffi::c_int && code <= 0x489 as ::core::ffi::c_int
        || code == 0x6de as ::core::ffi::c_int
        || code >= 0x20dd as ::core::ffi::c_int && code <= 0x20e0 as ::core::ffi::c_int
        || code >= 0x20e2 as ::core::ffi::c_int && code <= 0x20e4 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatMn(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlMnG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatN(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlNG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatNd(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlNdG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatNl(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code >= 0x16ee as ::core::ffi::c_int && code <= 0x16f0 as ::core::ffi::c_int
        || code >= 0x2160 as ::core::ffi::c_int && code <= 0x2183 as ::core::ffi::c_int
        || code == 0x3007 as ::core::ffi::c_int
        || code >= 0x3021 as ::core::ffi::c_int && code <= 0x3029 as ::core::ffi::c_int
        || code >= 0x3038 as ::core::ffi::c_int && code <= 0x303a as ::core::ffi::c_int
        || code == 0x1034a as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatNo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlNoG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatP(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlPG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatPc(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0x5f as ::core::ffi::c_int
        || code >= 0x203f as ::core::ffi::c_int && code <= 0x2040 as ::core::ffi::c_int
        || code == 0x2054 as ::core::ffi::c_int
        || code == 0x30fb as ::core::ffi::c_int
        || code >= 0xfe33 as ::core::ffi::c_int && code <= 0xfe34 as ::core::ffi::c_int
        || code >= 0xfe4d as ::core::ffi::c_int && code <= 0xfe4f as ::core::ffi::c_int
        || code == 0xff3f as ::core::ffi::c_int
        || code == 0xff65 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatPd(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlPdG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatPe(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlPeG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatPf(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0xbb as ::core::ffi::c_int
        || code == 0x2019 as ::core::ffi::c_int
        || code == 0x201d as ::core::ffi::c_int
        || code == 0x203a as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatPi(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0xab as ::core::ffi::c_int
        || code == 0x2018 as ::core::ffi::c_int
        || code >= 0x201b as ::core::ffi::c_int && code <= 0x201c as ::core::ffi::c_int
        || code == 0x201f as ::core::ffi::c_int
        || code == 0x2039 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatPo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlPoG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatPs(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlPsG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatS(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlSG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatSc(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlScG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatSk(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlSkG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatSm(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlSmG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatSo(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlSoG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatZ(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return xmlCharInRange(code as ::core::ffi::c_uint, &raw const xmlZG);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatZl(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0x2028 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatZp(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0x2029 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCatZs(mut code: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (code == 0x20 as ::core::ffi::c_int
        || code == 0xa0 as ::core::ffi::c_int
        || code == 0x1680 as ::core::ffi::c_int
        || code == 0x180e as ::core::ffi::c_int
        || code >= 0x2000 as ::core::ffi::c_int && code <= 0x200a as ::core::ffi::c_int
        || code == 0x202f as ::core::ffi::c_int
        || code == 0x205f as ::core::ffi::c_int
        || code == 0x3000 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUCSIsCat(
    mut code: ::core::ffi::c_int,
    mut cat: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut func: Option<xmlIntFunc> = None;
    func = xmlUnicodeLookup(&raw const xmlUnicodeCatTbl, cat);
    if func.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    return func.expect("non-null function pointer")(code);
}
