// ============================================================================
                                                                      
                                                            
                                                                  
                                                               
                            
                                                                       
                                                         
                                                 
// ============================================================================

use core::ffi::{c_char, c_int, c_void};
use std::process::exit;

#[inline]
fn digest64(seed: u64, p: &[u8]) -> u64 {
    let mut l0 = seed ^ 0xcbf2_9ce4_8422_2325u64;
    let mut l1 = 0x8422_2325_cbf2_9ce4u64;
    let mut l2 = 0x9ce4_8422_2325_cbf2u64;
    let mut l3 = 0x2325_cbf2_9ce4_8422u64;
    let n = p.len();
    let mut i = 0usize;
    while i + 32 <= n {
        let w0 = u64::from_ne_bytes(p[i..i + 8].try_into().unwrap());
        let w1 = u64::from_ne_bytes(p[i + 8..i + 16].try_into().unwrap());
        let w2 = u64::from_ne_bytes(p[i + 16..i + 24].try_into().unwrap());
        let w3 = u64::from_ne_bytes(p[i + 24..i + 32].try_into().unwrap());
        l0 = (l0 ^ w0).rotate_left(17);
        l1 = (l1 ^ w1).rotate_left(19);
        l2 = (l2 ^ w2).rotate_left(23);
        l3 = (l3 ^ w3).rotate_left(29);
        i += 32;
    }
    while i < n {
        l0 = (l0 ^ p[i] as u64).rotate_left(11);
        i += 1;
    }
    let mut h = l0.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h ^= l1.rotate_left(1);
    h = h.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h ^= l2.rotate_left(2);
    h = h.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h ^= l3.rotate_left(3);
    h = h.wrapping_mul(0x9e37_79b9_7f4a_7c15u64);
    h
}

                                                              
#[inline]
fn u64_bytes(s: &[u64]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, std::mem::size_of_val(s)) }
}

                                                     
unsafe fn cstr_bytes<'a>(p: *const u8) -> &'a [u8] {
    std::ffi::CStr::from_ptr(p as *const c_char).to_bytes()
}

                                                        
unsafe fn raw_bytes<'a>(p: *const u8, n: usize) -> &'a [u8] {
    if n == 0 { &[] } else { std::slice::from_raw_parts(p, n) }
}

                                                          
fn read_file(path: &str) -> (Vec<u8>, usize) {
    let mut buf = std::fs::read(path).unwrap_or_else(|_| {
        eprintln!("cannot open {}", path);
        exit(2);
    });
    let n = buf.len();
    buf.push(0);
    (buf, n)
}

// ---------------------------------------------------------------------------
                                                        
                                                              
// ---------------------------------------------------------------------------
type LineFn = unsafe fn(u64, *const c_char, *const c_char) -> u64;

fn for_each_line(listpath: &str, mut h: u64, f: LineFn) -> (u64, u64) {
    let (mut buf, n) = read_file(listpath);
    let mut count = 0u64;
    let mut p = 0usize;
    while p < n {
        let nl = buf[p..n].iter().position(|&b| b == b'\n').map(|k| p + k).unwrap_or(n);
        buf[nl] = 0;                       
        if buf[p] != 0 {
            let sp = buf[p..nl].iter().position(|&b| b == b' ').map(|k| p + k);
            let arg2: *const c_char = match sp {
                Some(s) => {
                    buf[s] = 0;
                    buf[s + 1..].as_ptr() as *const c_char
                }
                None => std::ptr::null(),
            };
            h = unsafe { f(h, buf[p..].as_ptr() as *const c_char, arg2) };
            count += 1;
        }
        p = nl + 1;
        if nl == n {
            break;
        }
    }
    (h, count)
}

// ---------------------------------------------------------------------------
                                                       
                                          
// ---------------------------------------------------------------------------
unsafe fn fold_node(mut h: u64, mut node: *mut libcall::XmlNode) -> u64 {
    while !node.is_null() {
        let t = (*node).type_0 as u64;
        h = digest64(h, &t.to_ne_bytes());
        let ty = (*node).type_0 as u32;
        if ty == libcall::XML_ELEMENT_NODE as u32
            || ty == libcall::XML_PI_NODE as u32
            || ty == libcall::XML_ENTITY_REF_NODE as u32
        {
            if !(*node).name.is_null() {
                h = digest64(h, cstr_bytes((*node).name));
            }
        }
        if (ty == libcall::XML_TEXT_NODE as u32
            || ty == libcall::XML_CDATA_SECTION_NODE as u32
            || ty == libcall::XML_COMMENT_NODE as u32)
            && !(*node).content.is_null()
        {
            h = digest64(h, cstr_bytes((*node).content));
        }
        if ty == libcall::XML_ELEMENT_NODE as u32 {
            let mut a = (*node).properties;
            while !a.is_null() {
                h = digest64(h, cstr_bytes((*a).name));
                let v = libcall::xmlGetProp(node, (*a).name);
                if !v.is_null() {
                    h = digest64(h, cstr_bytes(v));
                    libcall::xmlFree(v as *mut c_void);
                }
                a = (*a).next;
            }
            h = fold_node(h, (*node).children);
        }
        node = (*node).next;
    }
    h
}

// ---------------------------------------------------------------------------
                        
// ---------------------------------------------------------------------------
unsafe fn parse_one(mut h: u64, path: *const c_char, _arg2: *const c_char) -> u64 {
    let doc = libcall::xmlReadFile(
        path,
        std::ptr::null(),
        libcall::XML_PARSE_NOERROR as c_int | libcall::XML_PARSE_NOWARNING as c_int,
    );
    let ok: u64 = (!doc.is_null()) as u64;
    h = digest64(h, &ok.to_ne_bytes());
    if doc.is_null() {
        return h;
    }
    h = fold_node(h, libcall::xmlDocGetRootElement(doc));
    let mut dump: *mut u8 = std::ptr::null_mut();
    let mut dump_n: c_int = 0;
    libcall::xmlDocDumpFormatMemory(doc, &mut dump, &mut dump_n, 1);
    if !dump.is_null() {
        h = digest64(h, raw_bytes(dump, dump_n as usize));
        libcall::xmlFree(dump as *mut c_void);
    }
    let xb = libcall::xmlBufferCreate();
    let sc = libcall::xmlSaveToBuffer(xb, b"UTF-8\0".as_ptr() as *const c_char, 0);
    if !sc.is_null() {
        libcall::xmlSaveDoc(sc, doc);
        libcall::xmlSaveClose(sc);
        h = digest64(
            h,
            raw_bytes(libcall::xmlBufferContent(xb), libcall::xmlBufferLength(xb) as usize),
        );
    }
    libcall::xmlBufferFree(xb);
    libcall::xmlFreeDoc(doc);
    h
}

unsafe fn html_one(mut h: u64, path: *const c_char, _arg2: *const c_char) -> u64 {
    let doc = libcall::htmlReadFile(
        path,
        std::ptr::null(),
        libcall::HTML_PARSE_NOERROR as c_int | libcall::HTML_PARSE_NOWARNING as c_int,
    );
    let ok: u64 = (!doc.is_null()) as u64;
    h = digest64(h, &ok.to_ne_bytes());
    if doc.is_null() {
        return h;
    }
    h = fold_node(h, libcall::xmlDocGetRootElement(doc));
    let mut dump: *mut u8 = std::ptr::null_mut();
    let mut dump_n: c_int = 0;
    libcall::htmlDocDumpMemory(doc, &mut dump, &mut dump_n);
    if !dump.is_null() {
        h = digest64(h, raw_bytes(dump, dump_n as usize));
        libcall::xmlFree(dump as *mut c_void);
    }
    libcall::xmlFreeDoc(doc);
    h
}

                                                             
#[repr(C)]
struct SaxAcc {
    elems: u64,
    chars: u64,
    h: u64,
}

unsafe extern "C" fn sax_start(ctx: *mut c_void, name: *const u8, attrs: *mut *const u8) {
    let a = ctx as *mut SaxAcc;
    (*a).elems += 1;
    (*a).h = digest64((*a).h, cstr_bytes(name));
    if !attrs.is_null() {
        let mut i = 0isize;
        while !(*attrs.offset(i)).is_null() {
            (*a).h = digest64((*a).h, cstr_bytes(*attrs.offset(i)));
            i += 2;
        }
    }
}

unsafe extern "C" fn sax_end(ctx: *mut c_void, _name: *const u8) {
    (*(ctx as *mut SaxAcc)).elems += 1;
}

unsafe extern "C" fn sax_chars(ctx: *mut c_void, ch: *const u8, len: c_int) {
    let a = ctx as *mut SaxAcc;
    (*a).chars += len as u64;
    (*a).h = digest64((*a).h, raw_bytes(ch, len as usize));
}

unsafe fn sax_one(h: u64, path: *const c_char, _arg2: *const c_char) -> u64 {
    let mut sh: libcall::SaxHandler = std::mem::zeroed();
    sh.startElement = Some(sax_start as unsafe extern "C" fn(*mut c_void, *const u8, *mut *const u8));
    sh.endElement = Some(sax_end as unsafe extern "C" fn(*mut c_void, *const u8));
    sh.characters = Some(sax_chars as unsafe extern "C" fn(*mut c_void, *const u8, c_int));
    let mut a = SaxAcc { elems: 0, chars: 0, h };
    let r = libcall::xmlSAXUserParseFile(&mut sh, &mut a as *mut SaxAcc as *mut c_void, path);
    let meta: [u64; 3] = [r as i64 as u64, a.elems, a.chars];
    digest64(a.h, u64_bytes(&meta))
}

unsafe fn reader_one(mut h: u64, path: *const c_char, _arg2: *const c_char) -> u64 {
    let rd = libcall::xmlReaderForFile(
        path,
        std::ptr::null(),
        libcall::XML_PARSE_NOERROR as c_int | libcall::XML_PARSE_NOWARNING as c_int,
    );
    if rd.is_null() {
        return h;
    }
    loop {
        let r = libcall::xmlTextReaderRead(rd);
        if r != 1 {
            let rr = r as i64 as u64;
            h = digest64(h, &rr.to_ne_bytes());
            break;
        }
        let meta: [u64; 2] = [
            libcall::xmlTextReaderNodeType(rd) as i64 as u64,
            libcall::xmlTextReaderDepth(rd) as i64 as u64,
        ];
        h = digest64(h, u64_bytes(&meta));
        let nm = libcall::xmlTextReaderConstName(rd);
        if !nm.is_null() {
            h = digest64(h, cstr_bytes(nm));
        }
        let val = libcall::xmlTextReaderConstValue(rd);
        if !val.is_null() {
            h = digest64(h, cstr_bytes(val));
        }
    }
    libcall::xmlFreeTextReader(rd);
    h
}

unsafe fn valid_one(mut h: u64, path: *const c_char, _arg2: *const c_char) -> u64 {
    let doc = libcall::xmlReadFile(
        path,
        std::ptr::null(),
        libcall::XML_PARSE_DTDVALID as c_int
            | libcall::XML_PARSE_NOERROR as c_int
            | libcall::XML_PARSE_NOWARNING as c_int,
    );
    let ok: u64 = (!doc.is_null()) as u64;
    h = digest64(h, &ok.to_ne_bytes());
    if !doc.is_null() {
        h = fold_node(h, libcall::xmlDocGetRootElement(doc));
        libcall::xmlFreeDoc(doc);
    }
    h
}

unsafe fn schema_one(mut h: u64, xsd: *const c_char, xml: *const c_char) -> u64 {
    if xml.is_null() {
        return h;
    }
    let pc = libcall::xmlSchemaNewParserCtxt(xsd);
    if pc.is_null() {
        return h;
    }
    let schema = libcall::xmlSchemaParse(pc);
    libcall::xmlSchemaFreeParserCtxt(pc);
    let got: u64 = (!schema.is_null()) as u64;
    h = digest64(h, &got.to_ne_bytes());
    if schema.is_null() {
        return h;
    }
    let doc = libcall::xmlReadFile(
        xml,
        std::ptr::null(),
        libcall::XML_PARSE_NOERROR as c_int | libcall::XML_PARSE_NOWARNING as c_int,
    );
    if !doc.is_null() {
        let vc = libcall::xmlSchemaNewValidCtxt(schema);
        if !vc.is_null() {
            let v = libcall::xmlSchemaValidateDoc(vc, doc) as i64 as u64;
            h = digest64(h, &v.to_ne_bytes());
            libcall::xmlSchemaFreeValidCtxt(vc);
        }
        libcall::xmlFreeDoc(doc);
    }
    libcall::xmlSchemaFree(schema);
    h
}

unsafe fn relaxng_one(mut h: u64, rng: *const c_char, xml: *const c_char) -> u64 {
    if xml.is_null() {
        return h;
    }
    let pc = libcall::xmlRelaxNGNewParserCtxt(rng);
    if pc.is_null() {
        return h;
    }
    let rg = libcall::xmlRelaxNGParse(pc);
    libcall::xmlRelaxNGFreeParserCtxt(pc);
    let got: u64 = (!rg.is_null()) as u64;
    h = digest64(h, &got.to_ne_bytes());
    if rg.is_null() {
        return h;
    }
    let doc = libcall::xmlReadFile(
        xml,
        std::ptr::null(),
        libcall::XML_PARSE_NOERROR as c_int | libcall::XML_PARSE_NOWARNING as c_int,
    );
    if !doc.is_null() {
        let vc = libcall::xmlRelaxNGNewValidCtxt(rg);
        if !vc.is_null() {
            let v = libcall::xmlRelaxNGValidateDoc(vc, doc) as i64 as u64;
            h = digest64(h, &v.to_ne_bytes());
            libcall::xmlRelaxNGFreeValidCtxt(vc);
        }
        libcall::xmlFreeDoc(doc);
    }
    libcall::xmlRelaxNGFree(rg);
    h
}

fn run_suite(op: &str, listpath: &str, iters: i64, f: LineFn) {
    let mut h = 0u64;
    let mut count = 0u64;
    for _ in 0..iters {
        let (nh, nc) = for_each_line(listpath, h, f);
        h = nh;
        count = nc;
    }
    println!("op={} in=0 out={} iters={} digest={:016x}", op, count, iters, h);
}

// ---------------------------------------------------------------------------
// xpath_op
// ---------------------------------------------------------------------------
fn run_xpath(path: &str, iters: i64) {
    const EXPRS: [&[u8]; 10] = [
        b"//*\0",
        b"count(//*)\0",
        b"//@*\0",
        b"string(/*)\0",
        b"//*[position() mod 7 = 0]\0",
        b"//*[@*]\0",
        b"concat(name(/*), '-', count(//text()))\0",
        b"sum(//*[string-length(name()) > 3]/string-length(name()))\0",
        b"boolean(//comment())\0",
        b"normalize-space(string(/*))\0",
    ];
    let cpath = std::ffi::CString::new(path).unwrap();
    unsafe {
        let doc = libcall::xmlReadFile(
            cpath.as_ptr(),
            std::ptr::null(),
            libcall::XML_PARSE_NOERROR as c_int | libcall::XML_PARSE_NOWARNING as c_int,
        );
        if doc.is_null() {
            eprintln!("xpath parse failed");
            exit(3);
        }
        let mut h = 0u64;
        for _ in 0..iters {
            let ctx = libcall::xmlXPathNewContext(doc);
            for e in EXPRS {
                let obj = libcall::xmlXPathEvalExpression(e.as_ptr(), ctx);
                if obj.is_null() {
                    continue;
                }
                let t = (*obj).type_0 as u64;
                h = digest64(h, &t.to_ne_bytes());
                let ty = (*obj).type_0 as u32;
                if ty == libcall::XPATH_NODESET as u32 {
                    let sz: u64 = if !(*obj).nodesetval.is_null() {
                        (*(*obj).nodesetval).nodeNr as i64 as u64
                    } else {
                        0
                    };
                    h = digest64(h, &sz.to_ne_bytes());
                } else if ty == libcall::XPATH_NUMBER as u32 {
                    h = digest64(h, &(*obj).floatval.to_ne_bytes());
                } else if ty == libcall::XPATH_BOOLEAN as u32 {
                    let b = (*obj).boolval as i64 as u64;
                    h = digest64(h, &b.to_ne_bytes());
                } else if ty == libcall::XPATH_STRING as u32 {
                    if !(*obj).stringval.is_null() {
                        h = digest64(h, cstr_bytes((*obj).stringval));
                    }
                }
                libcall::xmlXPathFreeObject(obj);
            }
            libcall::xmlXPathFreeContext(ctx);
        }
        libcall::xmlFreeDoc(doc);
        println!("op=xpath_op in=0 out=0 iters={} digest={:016x}", iters, h);
    }
}

// ---------------------------------------------------------------------------
                                                    
// ---------------------------------------------------------------------------
fn run_writer(iters: i64) {
    let mut h = 0u64;
    unsafe {
        for _ in 0..iters {
            let xb = libcall::xmlBufferCreate();
            let w = libcall::xmlNewTextWriterMemory(xb, 0);
            if w.is_null() {
                eprintln!("writer failed");
                exit(3);
            }
            libcall::xmlTextWriterStartDocument(
                w,
                b"1.0\0".as_ptr() as *const c_char,
                b"UTF-8\0".as_ptr() as *const c_char,
                std::ptr::null(),
            );
            libcall::xmlTextWriterStartElement(w, b"catalog\0".as_ptr());
            libcall::xmlTextWriterWriteAttribute(w, b"version\0".as_ptr(), b"2.1\0".as_ptr());
            libcall::xmlTextWriterWriteComment(w, b"generated by validation driver\0".as_ptr());
            for k in 0..40 as c_int {
                libcall::xmlTextWriterStartElement(w, b"item\0".as_ptr());
                libcall::xmlTextWriterWriteFormatAttribute(
                    w,
                    b"id\0".as_ptr(),
                    b"i%04d\0".as_ptr() as *const c_char,
                    k,
                );
                libcall::xmlTextWriterWriteFormatElement(
                    w,
                    b"price\0".as_ptr(),
                    b"%d.%02d\0".as_ptr() as *const c_char,
                    k,
                    k % 100,
                );
                libcall::xmlTextWriterStartElement(w, b"desc\0".as_ptr());
                libcall::xmlTextWriterWriteString(w, b"plain & <escaped> text\0".as_ptr());
                libcall::xmlTextWriterEndElement(w);
                if k % 5 == 0 {
                    libcall::xmlTextWriterWriteCDATA(w, b"raw <cdata> content\0".as_ptr());
                    libcall::xmlTextWriterWritePI(w, b"proc\0".as_ptr(), b"inst\0".as_ptr());
                }
                libcall::xmlTextWriterEndElement(w);
            }
            libcall::xmlTextWriterEndElement(w);
            libcall::xmlTextWriterEndDocument(w);
            libcall::xmlFreeTextWriter(w);
            h = digest64(
                h,
                raw_bytes(libcall::xmlBufferContent(xb), libcall::xmlBufferLength(xb) as usize),
            );
            libcall::xmlBufferFree(xb);
        }
    }
    println!("op=writer_op in=0 out=0 iters={} digest={:016x}", iters, h);
}

// ---------------------------------------------------------------------------
                                                      
// ---------------------------------------------------------------------------
#[inline]
fn ch_is_char(c: u32) -> u64 {
    (((0x9 <= c) && (c <= 0xa)) || (c == 0xd) || (0x20 <= c)) as u64
}
#[inline]
fn ch_is_blank(c: u32) -> u64 {
    ((c == 0x20) || ((0x9 <= c) && (c <= 0xa)) || (c == 0xd)) as u64
}
#[inline]
fn ch_is_digit(c: u32) -> u64 {
    ((0x30 <= c) && (c <= 0x39)) as u64
}
#[inline]
fn ch_is_basechar(c: u32) -> u64 {
    (((0x41 <= c) && (c <= 0x5a))
        || ((0x61 <= c) && (c <= 0x7a))
        || ((0xc0 <= c) && (c <= 0xd6))
        || ((0xd8 <= c) && (c <= 0xf6))
        || (0xf8 <= c)) as u64
}

fn run_dict_uri_str(path: &str, iters: i64) {
    let (buf, n) = read_file(path);
    let mut h = 0u64;
    const URIS: [&[u8]; 5] = [
        b"http://user:pw@www.example.com:8080/a/b/c?x=1&y=2#frag\0",
        b"../relative/path/file.xml\0",
        b"urn:isbn:0451450523\0",
        b"https://[2001:db8::1]/ipv6\0",
        b"file:///tmp/x.xml\0",
    ];
    unsafe {
        for _ in 0..iters {
            // dict + hash over whitespace-separated words of the input
            let dict = libcall::xmlDictCreate();
            let ht = libcall::xmlHashCreate(64);
            let mut p = 0usize;
            let mut words = 0u64;
            while p < n && words < 5000 {
                while p < n
                    && (buf[p] == b' ' || buf[p] == b'\n' || buf[p] == b'\t' || buf[p] == b'\r')
                {
                    p += 1;
                }
                let w = p;
                while p < n
                    && buf[p] != b' '
                    && buf[p] != b'\n'
                    && buf[p] != b'\t'
                    && buf[p] != b'\r'
                {
                    p += 1;
                }
                if p > w {
                    let interned =
                        libcall::xmlDictLookup(dict, buf[w..].as_ptr(), (p - w) as c_int);
                    if !interned.is_null() {
                        libcall::xmlHashAddEntry(ht, interned, (words + 1) as usize as *mut c_void);
                        let found = libcall::xmlHashLookup(ht, interned);
                        let fv = found as usize as u64;
                        h = digest64(h, &fv.to_ne_bytes());
                    }
                    words += 1;
                }
            }
            let meta: [u64; 3] = [
                words,
                libcall::xmlDictSize(dict) as i64 as u64,
                libcall::xmlHashSize(ht) as i64 as u64,
            ];
            h = digest64(h, u64_bytes(&meta));
            libcall::xmlHashFree(ht);
            libcall::xmlDictFree(dict);

            // URI surface
            for u in URIS {
                let pu = libcall::xmlParseURI(u.as_ptr() as *const c_char);
                if !pu.is_null() {
                    let s = libcall::xmlSaveUri(pu);
                    if !s.is_null() {
                        h = digest64(h, cstr_bytes(s));
                        libcall::xmlFree(s as *mut c_void);
                    }
                    libcall::xmlFreeURI(pu);
                }
                let built = libcall::xmlBuildURI(
                    u.as_ptr(),
                    b"http://base.example.com/dir/\0".as_ptr(),
                );
                if !built.is_null() {
                    h = digest64(h, cstr_bytes(built));
                    libcall::xmlFree(built as *mut c_void);
                }
            }

            // string + chvalid surface
            let dup = libcall::xmlStrdup(b"hello libxml2 world\0".as_ptr());
            let cat = libcall::xmlStrcat(dup, b" & more\0".as_ptr());
            let acc: [u64; 6] = [
                libcall::xmlStrlen(cat) as i64 as u64,
                libcall::xmlStrcmp(cat, b"x\0".as_ptr()) as i64 as u64,
                if !libcall::xmlStrstr(cat, b"libxml\0".as_ptr()).is_null() { 1 } else { 0 },
                ch_is_char('A' as u32) + ch_is_blank(' ' as u32),
                ch_is_digit('7' as u32) + ch_is_basechar('z' as u32),
                libcall::xmlCharInRangeBaseCharGroup(0x4E2D) as i64 as u64,
            ];
            h = digest64(h, u64_bytes(&acc));
            libcall::xmlFree(cat as *mut c_void);
        }
    }
    println!("op=dict_uri_str in={} out=0 iters={} digest={:016x}", n, iters, h);
}

// ---------------------------------------------------------------------------
// c14n_op
// ---------------------------------------------------------------------------
fn run_c14n(path: &str, iters: i64) {
    let cpath = std::ffi::CString::new(path).unwrap();
    unsafe {
        let doc = libcall::xmlReadFile(
            cpath.as_ptr(),
            std::ptr::null(),
            libcall::XML_PARSE_NOERROR as c_int | libcall::XML_PARSE_NOWARNING as c_int,
        );
        if doc.is_null() {
            eprintln!("c14n parse failed");
            exit(3);
        }
        let mut h = 0u64;
        for _ in 0..iters {
            let mut out: *mut u8 = std::ptr::null_mut();
            let r = libcall::xmlC14NDocDumpMemory(
                doc,
                libcall::XML_C14N_1_0 as c_int,
                &mut out,
            );
            if r >= 0 && !out.is_null() {
                h = digest64(h, raw_bytes(out, r as usize));
                libcall::xmlFree(out as *mut c_void);
            }
            let rr = r as i64 as u64;
            h = digest64(h, &rr.to_ne_bytes());
        }
        libcall::xmlFreeDoc(doc);
        println!("op=c14n_op in=0 out=0 iters={} digest={:016x}", iters, h);
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------
const COVERAGE_ONLY_OPS: [&str; 17] = [
    "unicode_tables",
    "reader_full",
    "dom_build",
    "xpath_full",
    "writer_full",
    "push_parse",
    "encoding_op",
    "list_op",
    "error_op",
    "xinclude_suite",
    "xpointer_op",
    "schematron_suite",
    "gz_suite",
    "debug_op",
    "catalog_op",
    "api_storm",
    "api_variants",
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!(
            "usage: {} <op> <input> <iters>",
            args.get(0).map(|s| s.as_str()).unwrap_or("driver")
        );
        exit(1);
    }
    let op = args[1].as_str();
    let iters: i64 = args[3].parse().unwrap_or(0);
    if iters <= 0 {
        eprintln!("bad iters");
        exit(1);
    }
    unsafe {
        libcall::xmlInitParser();
    }

    match op {
        "parse_suite" => run_suite(op, &args[2], iters, parse_one),
        "html_suite" => run_suite(op, &args[2], iters, html_one),
        "sax_suite" => run_suite(op, &args[2], iters, sax_one),
        "reader_suite" => run_suite(op, &args[2], iters, reader_one),
        "valid_suite" => run_suite(op, &args[2], iters, valid_one),
        "schema_suite" => run_suite(op, &args[2], iters, schema_one),
        "relaxng_suite" => run_suite(op, &args[2], iters, relaxng_one),
        "xpath_op" => run_xpath(&args[2], iters),
        "writer_op" => run_writer(iters),
        "dict_uri_str" => run_dict_uri_str(&args[2], iters),
        "c14n_op" => run_c14n(&args[2], iters),
        _ if COVERAGE_ONLY_OPS.contains(&op) => {
            eprintln!("op {} is coverage_only (not timed in purebin)", op);
            exit(1);
        }
        _ => {
            eprintln!("unknown op {}", op);
            exit(1);
        }
    }

    unsafe {
        libcall::xmlCleanupParser();
    }
}
