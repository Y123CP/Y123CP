extern "C" {
    pub type json_object;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn mc_set_debug(debug: ::core::ffi::c_int);
    fn json_object_put(obj: *mut json_object) -> ::core::ffi::c_int;
    fn json_object_to_json_string(obj: *mut json_object) -> *const ::core::ffi::c_char;
    fn json_tokener_parse(str: *const ::core::ffi::c_char) -> *mut json_object;
}
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut new_obj: *mut json_object = ::core::ptr::null_mut::<json_object>();
    new_obj = json_tokener_parse(
        b"/* more difficult test case */{ \"glossary\": { \"title\": \"example glossary\", \"GlossDiv\": { \"title\": \"S\", \"GlossList\": [ { \"ID\": \"SGML\", \"SortAs\": \"SGML\", \"GlossTerm\": \"Standard Generalized Markup Language\", \"Acronym\": \"SGML\", \"Abbrev\": \"ISO 8879:1986\", \"GlossDef\": \"A meta-markup language, used to create markup languages such as DocBook.\", \"GlossSeeAlso\": [\"GML\", \"XML\", \"markup\"] } ] } } }\0"
            as *const u8 as *const ::core::ffi::c_char,
    ) as *mut json_object;
    printf(
        b"new_obj.to_string()=%s\n\0" as *const u8 as *const ::core::ffi::c_char,
        json_object_to_json_string(new_obj as *mut json_object),
    );
    json_object_put(new_obj as *mut json_object);
    return EXIT_SUCCESS;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
