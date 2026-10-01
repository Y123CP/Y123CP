#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
use core::ffi::*;
use ::libxml2_cleaned::src::ffi::*;
use ::libxml2_cleaned::src::c_consts::*;
use ::libxml2_cleaned::src::c_structs::*;
use ::libxml2_cleaned::src::c_types::*;
use ::libxml2_cleaned::src::c_extern_types::*;
#[allow(unused_imports)]
use ::libxml2_cleaned;
use ::libxml2_cleaned::src::catalog::xmlACatalogAdd;
use ::libxml2_cleaned::src::catalog::xmlACatalogDump;
use ::libxml2_cleaned::src::catalog::xmlACatalogRemove;
use ::libxml2_cleaned::src::catalog::xmlCatalogAdd;
use ::libxml2_cleaned::src::catalog::xmlCatalogConvert;
use ::libxml2_cleaned::src::catalog::xmlCatalogDump;
use ::libxml2_cleaned::src::catalog::xmlCatalogIsEmpty;
use ::libxml2_cleaned::src::catalog::xmlCatalogRemove;
use ::libxml2_cleaned::src::catalog::xmlCatalogResolve;
use ::libxml2_cleaned::src::catalog::xmlCatalogResolvePublic;
use ::libxml2_cleaned::src::catalog::xmlCatalogResolveSystem;
use ::libxml2_cleaned::src::catalog::xmlCatalogResolveURI;
use ::libxml2_cleaned::src::catalog::xmlCatalogSetDebug;
use ::libxml2_cleaned::src::parserInternals::xmlCheckVersion;
use ::libxml2_cleaned::src::threads::xmlCleanupParser;
use ::libxml2_cleaned::src::catalog::xmlFreeCatalog;
use ::libxml2_cleaned::src::uri::xmlFreeURI;
use ::libxml2_cleaned::src::catalog::xmlInitializeCatalog;
use ::libxml2_cleaned::src::catalog::xmlLoadCatalog;
use ::libxml2_cleaned::src::catalog::xmlLoadSGMLSuperCatalog;
use ::libxml2_cleaned::src::catalog::xmlNewCatalog;
use ::libxml2_cleaned::src::uri::xmlParseURI;
pub use libxml2_cleaned::src::catalog::_xmlCatalog;

pub type xmlCatalog = _xmlCatalog;
pub type xmlCatalogPtr = *mut xmlCatalog;

static mut shell: c_int = 0 as c_int;
static mut sgml: c_int = 0 as c_int;
static mut noout: c_int = 0 as c_int;
static mut create: c_int = 0 as c_int;
static mut add: c_int = 0 as c_int;
static mut del: c_int = 0 as c_int;
static mut convert: c_int = 0 as c_int;
static mut no_super_update: c_int = 0 as c_int;
static mut verbose: c_int = 0 as c_int;
static mut filename: *mut c_char =
    ::core::ptr::null::<c_char>() as *mut c_char;
unsafe extern "C" fn xmlShellReadline(
    mut prompt: *const c_char,
) -> *mut c_char {
    let mut line_read: [c_char; 501] = [0; 501];
    let mut ret: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut len: c_int = 0;
    if !prompt.is_null() {
        fprintf(
            stdout,
            b"%s\0" as *const u8 as *const c_char,
            prompt,
        );
    }
    fflush(stdout);
    if fgets(
        &raw mut line_read as *mut c_char,
        500 as c_int,
        stdin,
    )
    .is_null()
    {
        return ::core::ptr::null_mut::<c_char>();
    }
    line_read[500 as c_int as usize] = 0 as c_char;
    len = strlen(&raw mut line_read as *mut c_char) as c_int;
    ret = malloc((len + 1 as c_int) as size_t) as *mut c_char;
    if !ret.is_null() {
        memcpy(
            ret as *mut c_void,
            &raw mut line_read as *mut c_char as *const c_void,
            (len + 1 as c_int) as size_t,
        );
    }
    return ret;
}
extern "C" fn usershell() { unsafe {
    let mut cmdline: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut cur: *mut c_char = ::core::ptr::null_mut::<c_char>();
    let mut nbargs: c_int = 0;
    let mut command: [c_char; 100] = [0; 100];
    let mut arg: [c_char; 400] = [0; 400];
    let mut argv: [*mut c_char; 20] =
        [::core::ptr::null_mut::<c_char>(); 20];
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    let mut ans: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
    loop {
        cmdline = xmlShellReadline(b"> \0" as *const u8 as *const c_char);
        if cmdline.is_null() {
            return;
        }
        cur = cmdline;
        nbargs = 0 as c_int;
        while *cur as c_int == ' ' as i32 || *cur as c_int == '\t' as i32
        {
            cur = cur.offset(1);
        }
        i = 0 as c_int;
        while *cur as c_int != ' ' as i32
            && *cur as c_int != '\t' as i32
            && *cur as c_int != '\n' as i32
            && *cur as c_int != '\r' as i32
        {
            if *cur as c_int == 0 as c_int {
                break;
            }
            let fresh0 = cur;
            cur = cur.offset(1);
            let fresh1 = i;
            i = i + 1;
            command[fresh1 as usize] = *fresh0;
        }
        command[i as usize] = 0 as c_char;
        if i == 0 as c_int {
            free(cmdline as *mut c_void);
        } else {
            memset(
                &raw mut arg as *mut c_char as *mut c_void,
                0 as c_int,
                ::core::mem::size_of::<[c_char; 400]>() as size_t,
            );
            while *cur as c_int == ' ' as i32
                || *cur as c_int == '\t' as i32
            {
                cur = cur.offset(1);
            }
            i = 0 as c_int;
            while *cur as c_int != '\n' as i32
                && *cur as c_int != '\r' as i32
                && *cur as c_int != 0 as c_int
            {
                if *cur as c_int == 0 as c_int {
                    break;
                }
                let fresh2 = cur;
                cur = cur.offset(1);
                let fresh3 = i;
                i = i + 1;
                arg[fresh3 as usize] = *fresh2;
            }
            arg[i as usize] = 0 as c_char;
            i = 0 as c_int;
            nbargs = 0 as c_int;
            cur = &raw mut arg as *mut c_char;
            memset(
                &raw mut argv as *mut *mut c_char as *mut c_void,
                0 as c_int,
                ::core::mem::size_of::<[*mut c_char; 20]>() as size_t,
            );
            while *cur as c_int != 0 as c_int {
                while *cur as c_int == ' ' as i32
                    || *cur as c_int == '\t' as i32
                {
                    cur = cur.offset(1);
                }
                if *cur as c_int == '\'' as i32 {
                    cur = cur.offset(1);
                    argv[i as usize] = cur;
                    while *cur as c_int != 0 as c_int
                        && *cur as c_int != '\'' as i32
                    {
                        cur = cur.offset(1);
                    }
                    if *cur as c_int == '\'' as i32 {
                        *cur = 0 as c_char;
                        nbargs += 1;
                        i += 1;
                        cur = cur.offset(1);
                    }
                } else if *cur as c_int == '"' as i32 {
                    cur = cur.offset(1);
                    argv[i as usize] = cur;
                    while *cur as c_int != 0 as c_int
                        && *cur as c_int != '"' as i32
                    {
                        cur = cur.offset(1);
                    }
                    if *cur as c_int == '"' as i32 {
                        *cur = 0 as c_char;
                        nbargs += 1;
                        i += 1;
                        cur = cur.offset(1);
                    }
                } else {
                    argv[i as usize] = cur;
                    while *cur as c_int != 0 as c_int
                        && *cur as c_int != ' ' as i32
                        && *cur as c_int != '\t' as i32
                    {
                        cur = cur.offset(1);
                    }
                    *cur = 0 as c_char;
                    nbargs += 1;
                    i += 1;
                    cur = cur.offset(1);
                }
            }
            if strcmp(
                &raw mut command as *mut c_char,
                b"exit\0" as *const u8 as *const c_char,
            ) == 0
                || strcmp(
                    &raw mut command as *mut c_char,
                    b"quit\0" as *const u8 as *const c_char,
                ) == 0
                || strcmp(
                    &raw mut command as *mut c_char,
                    b"bye\0" as *const u8 as *const c_char,
                ) == 0
            {
                free(cmdline as *mut c_void);
                break;
            } else {
                if strcmp(
                    &raw mut command as *mut c_char,
                    b"public\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 1 as c_int {
                        printf(
                            b"public requires 1 arguments\n\0" as *const u8
                                as *const c_char,
                        );
                    } else {
                        ans = xmlCatalogResolvePublic(
                            argv[0 as c_int as usize] as *const xmlChar,
                        );
                        if ans.is_null() {
                            printf(
                                b"No entry for PUBLIC %s\n\0" as *const u8
                                    as *const c_char,
                                argv[0 as c_int as usize],
                            );
                        } else {
                            printf(
                                b"%s\n\0" as *const u8 as *const c_char,
                                ans as *mut c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                ans as *mut c_void,
                            );
                        }
                    }
                } else if strcmp(
                    &raw mut command as *mut c_char,
                    b"system\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 1 as c_int {
                        printf(
                            b"system requires 1 arguments\n\0" as *const u8
                                as *const c_char,
                        );
                    } else {
                        ans = xmlCatalogResolveSystem(
                            argv[0 as c_int as usize] as *const xmlChar,
                        );
                        if ans.is_null() {
                            printf(
                                b"No entry for SYSTEM %s\n\0" as *const u8
                                    as *const c_char,
                                argv[0 as c_int as usize],
                            );
                        } else {
                            printf(
                                b"%s\n\0" as *const u8 as *const c_char,
                                ans as *mut c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                ans as *mut c_void,
                            );
                        }
                    }
                } else if strcmp(
                    &raw mut command as *mut c_char,
                    b"add\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 3 as c_int && nbargs != 2 as c_int {
                        printf(
                            b"add requires 2 or 3 arguments\n\0" as *const u8
                                as *const c_char,
                        );
                    } else {
                        if argv[2 as c_int as usize].is_null() {
                            ret = xmlCatalogAdd(
                                argv[0 as c_int as usize] as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                                argv[1 as c_int as usize] as *mut xmlChar,
                            );
                        } else {
                            ret = xmlCatalogAdd(
                                argv[0 as c_int as usize] as *mut xmlChar,
                                argv[1 as c_int as usize] as *mut xmlChar,
                                argv[2 as c_int as usize] as *mut xmlChar,
                            );
                        }
                        if ret != 0 as c_int {
                            printf(
                                b"add command failed\n\0" as *const u8
                                    as *const c_char,
                            );
                        }
                    }
                } else if strcmp(
                    &raw mut command as *mut c_char,
                    b"del\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 1 as c_int {
                        printf(b"del requires 1\n\0" as *const u8 as *const c_char);
                    } else {
                        ret = xmlCatalogRemove(
                            argv[0 as c_int as usize] as *mut xmlChar,
                        );
                        if ret <= 0 as c_int {
                            printf(
                                b"del command failed\n\0" as *const u8
                                    as *const c_char,
                            );
                        }
                    }
                } else if strcmp(
                    &raw mut command as *mut c_char,
                    b"resolve\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 2 as c_int {
                        printf(
                            b"resolve requires 2 arguments\n\0" as *const u8
                                as *const c_char,
                        );
                    } else {
                        ans = xmlCatalogResolve(
                            argv[0 as c_int as usize] as *mut xmlChar,
                            argv[1 as c_int as usize] as *mut xmlChar,
                        );
                        if ans.is_null() {
                            printf(
                                b"Resolver failed to find an answer\n\0" as *const u8
                                    as *const c_char,
                            );
                        } else {
                            printf(
                                b"%s\n\0" as *const u8 as *const c_char,
                                ans as *mut c_char,
                            );
                            xmlFree.expect("non-null function pointer")(
                                ans as *mut c_void,
                            );
                        }
                    }
                } else if strcmp(
                    &raw mut command as *mut c_char,
                    b"dump\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 0 as c_int {
                        printf(
                            b"dump has no arguments\n\0" as *const u8 as *const c_char,
                        );
                    } else {
                        xmlCatalogDump(stdout);
                    }
                } else if strcmp(
                    &raw mut command as *mut c_char,
                    b"debug\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 0 as c_int {
                        printf(
                            b"debug has no arguments\n\0" as *const u8
                                as *const c_char,
                        );
                    } else {
                        verbose += 1;
                        xmlCatalogSetDebug(verbose);
                    }
                } else if strcmp(
                    &raw mut command as *mut c_char,
                    b"quiet\0" as *const u8 as *const c_char,
                ) == 0
                {
                    if nbargs != 0 as c_int {
                        printf(
                            b"quiet has no arguments\n\0" as *const u8
                                as *const c_char,
                        );
                    } else {
                        if verbose > 0 as c_int {
                            verbose -= 1;
                        }
                        xmlCatalogSetDebug(verbose);
                    }
                } else {
                    if strcmp(
                        &raw mut command as *mut c_char,
                        b"help\0" as *const u8 as *const c_char,
                    ) != 0
                    {
                        printf(
                            b"Unrecognized command %s\n\0" as *const u8
                                as *const c_char,
                            &raw mut command as *mut c_char,
                        );
                    }
                    printf(b"Commands available:\n\0" as *const u8 as *const c_char);
                    printf(
                        b"\tpublic PublicID: make a PUBLIC identifier lookup\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\tsystem SystemID: make a SYSTEM identifier lookup\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\tresolve PublicID SystemID: do a full resolver lookup\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\tadd 'type' 'orig' 'replace' : add an entry\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\tdel 'values' : remove values\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\tdump: print the current catalog state\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\tdebug: increase the verbosity level\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\tquiet: decrease the verbosity level\n\0" as *const u8
                            as *const c_char,
                    );
                    printf(
                        b"\texit:  quit the shell\n\0" as *const u8 as *const c_char,
                    );
                }
                free(cmdline as *mut c_void);
            }
        }
    }
} }
unsafe extern "C" fn usage(mut name: *const c_char) {
    printf(
        b"Usage : %s [options] catalogfile entities...\n\tParse the catalog file (void specification possibly expressed as \"\"\n\tappoints the default system one) and query it for the entities\n\t--sgml : handle SGML Super catalogs for --add and --del\n\t--shell : run a shell allowing interactive queries\n\t--create : create a new catalog\n\t--add 'type' 'orig' 'replace' : add an XML entry\n\t--add 'entry' : add an SGML entry\n\0"
            as *const u8 as *const c_char,
        name,
    );
    printf(
        b"\t--del 'values' : remove values\n\t--noout: avoid dumping the result on stdout\n\t         used with --add or --del, it saves the catalog changes\n\t         and with --sgml it automatically updates the super catalog\n\t--no-super-update: do not update the SGML super catalog\n\t-v --verbose : provide debug information\n\0"
            as *const u8 as *const c_char,
    );
}
unsafe fn main_0(
    mut argc: c_int,
    mut argv: *mut *mut c_char,
) -> c_int {
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    let mut exit_value: c_int = 0 as c_int;
    if argc <= 1 as c_int {
        usage(*argv.offset(0 as c_int as isize));
        return 1 as c_int;
    }
    xmlCheckVersion(21205 as c_int);
    i = 1 as c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-\0" as *const u8 as *const c_char,
        ) == 0
        {
            break;
        }
        if *(*argv.offset(i as isize)).offset(0 as c_int as isize)
            as c_int
            != '-' as i32
        {
            break;
        }
        if strcmp(
            *argv.offset(i as isize),
            b"-verbose\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"-v\0" as *const u8 as *const c_char,
            ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--verbose\0" as *const u8 as *const c_char,
            ) == 0
        {
            verbose += 1;
            xmlCatalogSetDebug(verbose);
        } else if strcmp(
            *argv.offset(i as isize),
            b"-noout\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--noout\0" as *const u8 as *const c_char,
            ) == 0
        {
            noout = 1 as c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-shell\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--shell\0" as *const u8 as *const c_char,
            ) == 0
        {
            shell += 1;
            noout = 1 as c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-sgml\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--sgml\0" as *const u8 as *const c_char,
            ) == 0
        {
            sgml += 1;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-create\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--create\0" as *const u8 as *const c_char,
            ) == 0
        {
            create += 1;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-convert\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--convert\0" as *const u8 as *const c_char,
            ) == 0
        {
            convert += 1;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-no-super-update\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--no-super-update\0" as *const u8 as *const c_char,
            ) == 0
        {
            no_super_update += 1;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-add\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--add\0" as *const u8 as *const c_char,
            ) == 0
        {
            if sgml != 0 {
                i += 2 as c_int;
            } else {
                i += 3 as c_int;
            }
            add += 1;
        } else if strcmp(
            *argv.offset(i as isize),
            b"-del\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--del\0" as *const u8 as *const c_char,
            ) == 0
        {
            i += 1 as c_int;
            del += 1;
        } else {
            fprintf(
                stderr,
                b"Unknown option %s\n\0" as *const u8 as *const c_char,
                *argv.offset(i as isize),
            );
            usage(*argv.offset(0 as c_int as isize));
            return 1 as c_int;
        }
        i += 1;
    }
    i = 1 as c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-add\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--add\0" as *const u8 as *const c_char,
            ) == 0
        {
            if sgml != 0 {
                i += 2 as c_int;
            } else {
                i += 3 as c_int;
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-del\0" as *const u8 as *const c_char,
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                b"--del\0" as *const u8 as *const c_char,
            ) == 0
        {
            i += 1 as c_int;
            if i == argc || sgml != 0 && i + 1 as c_int == argc {
                fprintf(
                    stderr,
                    b"No catalog entry specified to remove from\n\0" as *const u8
                        as *const c_char,
                );
                usage(*argv.offset(0 as c_int as isize));
                return 1 as c_int;
            }
        } else if !(*(*argv.offset(i as isize)).offset(0 as c_int as isize)
            as c_int
            == '-' as i32)
        {
            if filename.is_null()
                && *(*argv.offset(i as isize)).offset(0 as c_int as isize)
                    as c_int
                    == '\0' as i32
            {
                xmlInitializeCatalog();
            } else {
                filename = *argv.offset(i as isize);
                ret = xmlLoadCatalog(*argv.offset(i as isize));
                if ret < 0 as c_int && create != 0 {
                    xmlCatalogAdd(
                        b"catalog\0" as *const u8 as *const c_char as *mut xmlChar,
                        *argv.offset(i as isize) as *mut xmlChar,
                        ::core::ptr::null::<xmlChar>(),
                    );
                }
            }
            break;
        }
        i += 1;
    }
    if convert != 0 {
        ret = xmlCatalogConvert();
    }
    if add != 0 || del != 0 {
        i = 1 as c_int;
        while i < argc {
            if strcmp(
                *argv.offset(i as isize),
                b"-\0" as *const u8 as *const c_char,
            ) == 0
            {
                break;
            }
            if !(*(*argv.offset(i as isize)).offset(0 as c_int as isize)
                as c_int
                != '-' as i32)
            {
                if !(strcmp(
                    *argv.offset(i as isize),
                    b"-add\0" as *const u8 as *const c_char,
                ) != 0
                    && strcmp(
                        *argv.offset(i as isize),
                        b"--add\0" as *const u8 as *const c_char,
                    ) != 0
                    && strcmp(
                        *argv.offset(i as isize),
                        b"-del\0" as *const u8 as *const c_char,
                    ) != 0
                    && strcmp(
                        *argv.offset(i as isize),
                        b"--del\0" as *const u8 as *const c_char,
                    ) != 0)
                {
                    if sgml != 0 {
                        let mut catal: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
                        let mut super_0: xmlCatalogPtr = ::core::ptr::null_mut::<xmlCatalog>();
                        catal = xmlLoadSGMLSuperCatalog(
                            *argv.offset((i + 1 as c_int) as isize),
                        );
                        if strcmp(
                            *argv.offset(i as isize),
                            b"-add\0" as *const u8 as *const c_char,
                        ) == 0
                            || strcmp(
                                *argv.offset(i as isize),
                                b"--add\0" as *const u8 as *const c_char,
                            ) == 0
                        {
                            if catal.is_null() {
                                catal = xmlNewCatalog(1 as c_int);
                            }
                            xmlACatalogAdd(
                                catal,
                                b"CATALOG\0" as *const u8 as *const c_char
                                    as *mut xmlChar,
                                *argv.offset((i + 2 as c_int) as isize)
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                            );
                            if no_super_update == 0 {
                                super_0 = xmlLoadSGMLSuperCatalog(
                                    b"/usr/local/etc/sgml/catalog\0" as *const u8
                                        as *const c_char,
                                );
                                if super_0.is_null() {
                                    super_0 = xmlNewCatalog(1 as c_int);
                                }
                                xmlACatalogAdd(
                                    super_0,
                                    b"CATALOG\0" as *const u8 as *const c_char
                                        as *mut xmlChar,
                                    *argv.offset((i + 1 as c_int) as isize)
                                        as *mut xmlChar,
                                    ::core::ptr::null::<xmlChar>(),
                                );
                            }
                        } else {
                            if !catal.is_null() {
                                ret = xmlACatalogRemove(
                                    catal,
                                    *argv.offset((i + 2 as c_int) as isize)
                                        as *mut xmlChar,
                                );
                            } else {
                                ret = -(1 as c_int);
                            }
                            if ret < 0 as c_int {
                                fprintf(
                                    stderr,
                                    b"Failed to remove entry from %s\n\0" as *const u8
                                        as *const c_char,
                                    *argv.offset((i + 1 as c_int) as isize),
                                );
                                exit_value = 1 as c_int;
                            }
                            if no_super_update == 0
                                && noout != 0
                                && !catal.is_null()
                                && xmlCatalogIsEmpty(catal) != 0
                            {
                                super_0 = xmlLoadSGMLSuperCatalog(
                                    b"/usr/local/etc/sgml/catalog\0" as *const u8
                                        as *const c_char,
                                );
                                if !super_0.is_null() {
                                    ret = xmlACatalogRemove(
                                        super_0,
                                        *argv.offset((i + 1 as c_int) as isize)
                                            as *mut xmlChar,
                                    );
                                    if ret < 0 as c_int {
                                        fprintf(
                                            stderr,
                                            b"Failed to remove entry from %s\n\0" as *const u8
                                                as *const c_char,
                                            b"/usr/local/etc/sgml/catalog\0" as *const u8
                                                as *const c_char,
                                        );
                                        exit_value = 1 as c_int;
                                    }
                                }
                            }
                        }
                        if noout != 0 {
                            let mut out: *mut FILE = ::core::ptr::null_mut::<FILE>();
                            if xmlCatalogIsEmpty(catal) != 0 {
                                remove(*argv.offset((i + 1 as c_int) as isize));
                            } else {
                                out = fopen(
                                    *argv.offset((i + 1 as c_int) as isize),
                                    b"w\0" as *const u8 as *const c_char,
                                );
                                if out.is_null() {
                                    fprintf(
                                        stderr,
                                        b"could not open %s for saving\n\0" as *const u8
                                            as *const c_char,
                                        *argv.offset((i + 1 as c_int) as isize),
                                    );
                                    exit_value = 2 as c_int;
                                    noout = 0 as c_int;
                                } else {
                                    xmlACatalogDump(catal, out);
                                    fclose(out);
                                }
                            }
                            if no_super_update == 0 && !super_0.is_null() {
                                if xmlCatalogIsEmpty(super_0) != 0 {
                                    remove(
                                        b"/usr/local/etc/sgml/catalog\0" as *const u8
                                            as *const c_char,
                                    );
                                } else {
                                    out = fopen(
                                        b"/usr/local/etc/sgml/catalog\0" as *const u8
                                            as *const c_char,
                                        b"w\0" as *const u8 as *const c_char,
                                    );
                                    if out.is_null() {
                                        fprintf(
                                            stderr,
                                            b"could not open %s for saving\n\0" as *const u8
                                                as *const c_char,
                                            b"/usr/local/etc/sgml/catalog\0" as *const u8
                                                as *const c_char,
                                        );
                                        exit_value = 2 as c_int;
                                        noout = 0 as c_int;
                                    } else {
                                        xmlACatalogDump(super_0, out);
                                        fclose(out);
                                    }
                                }
                            }
                        } else {
                            xmlACatalogDump(catal, stdout);
                        }
                        i += 2 as c_int;
                        xmlFreeCatalog(catal);
                        xmlFreeCatalog(super_0);
                    } else if strcmp(
                        *argv.offset(i as isize),
                        b"-add\0" as *const u8 as *const c_char,
                    ) == 0
                        || strcmp(
                            *argv.offset(i as isize),
                            b"--add\0" as *const u8 as *const c_char,
                        ) == 0
                    {
                        if (*argv.offset((i + 3 as c_int) as isize)).is_null()
                            || *(*argv.offset((i + 3 as c_int) as isize))
                                .offset(0 as c_int as isize)
                                as c_int
                                == 0 as c_int
                        {
                            ret = xmlCatalogAdd(
                                *argv.offset((i + 1 as c_int) as isize)
                                    as *mut xmlChar,
                                ::core::ptr::null::<xmlChar>(),
                                *argv.offset((i + 2 as c_int) as isize)
                                    as *mut xmlChar,
                            );
                        } else {
                            ret = xmlCatalogAdd(
                                *argv.offset((i + 1 as c_int) as isize)
                                    as *mut xmlChar,
                                *argv.offset((i + 2 as c_int) as isize)
                                    as *mut xmlChar,
                                *argv.offset((i + 3 as c_int) as isize)
                                    as *mut xmlChar,
                            );
                        }
                        if ret != 0 as c_int {
                            printf(
                                b"add command failed\n\0" as *const u8
                                    as *const c_char,
                            );
                            exit_value = 3 as c_int;
                        }
                        i += 3 as c_int;
                    } else if strcmp(
                        *argv.offset(i as isize),
                        b"-del\0" as *const u8 as *const c_char,
                    ) == 0
                        || strcmp(
                            *argv.offset(i as isize),
                            b"--del\0" as *const u8 as *const c_char,
                        ) == 0
                    {
                        ret =
                            xmlCatalogRemove(*argv.offset((i + 1 as c_int) as isize)
                                as *mut xmlChar);
                        if ret < 0 as c_int {
                            fprintf(
                                stderr,
                                b"Failed to remove entry %s\n\0" as *const u8
                                    as *const c_char,
                                *argv.offset((i + 1 as c_int) as isize),
                            );
                            exit_value = 1 as c_int;
                        }
                        i += 1 as c_int;
                    }
                }
            }
            i += 1;
        }
    } else if shell != 0 {
        usershell();
    } else {
        i += 1;
        while i < argc {
            let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
            let mut ans: *mut xmlChar = ::core::ptr::null_mut::<xmlChar>();
            uri = xmlParseURI(*argv.offset(i as isize));
            if uri.is_null() {
                ans = xmlCatalogResolvePublic(*argv.offset(i as isize) as *const xmlChar);
                if ans.is_null() {
                    printf(
                        b"No entry for PUBLIC %s\n\0" as *const u8 as *const c_char,
                        *argv.offset(i as isize),
                    );
                    exit_value = 4 as c_int;
                } else {
                    printf(
                        b"%s\n\0" as *const u8 as *const c_char,
                        ans as *mut c_char,
                    );
                    xmlFree.expect("non-null function pointer")(ans as *mut c_void);
                }
            } else {
                xmlFreeURI(uri);
                ans = xmlCatalogResolveSystem(*argv.offset(i as isize) as *const xmlChar);
                if ans.is_null() {
                    printf(
                        b"No entry for SYSTEM %s\n\0" as *const u8 as *const c_char,
                        *argv.offset(i as isize),
                    );
                    ans = xmlCatalogResolveURI(*argv.offset(i as isize) as *const xmlChar);
                    if ans.is_null() {
                        printf(
                            b"No entry for URI %s\n\0" as *const u8 as *const c_char,
                            *argv.offset(i as isize),
                        );
                        exit_value = 4 as c_int;
                    } else {
                        printf(
                            b"%s\n\0" as *const u8 as *const c_char,
                            ans as *mut c_char,
                        );
                        xmlFree.expect("non-null function pointer")(
                            ans as *mut c_void,
                        );
                    }
                } else {
                    printf(
                        b"%s\n\0" as *const u8 as *const c_char,
                        ans as *mut c_char,
                    );
                    xmlFree.expect("non-null function pointer")(ans as *mut c_void);
                }
            }
            i += 1;
        }
    }
    if sgml == 0 && (add != 0 || del != 0 || create != 0 || convert != 0) {
        if noout != 0 && !filename.is_null() && *filename as c_int != 0 {
            let mut out_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
            out_0 = fopen(filename, b"w\0" as *const u8 as *const c_char);
            if out_0.is_null() {
                fprintf(
                    stderr,
                    b"could not open %s for saving\n\0" as *const u8 as *const c_char,
                    filename,
                );
                exit_value = 2 as c_int;
                noout = 0 as c_int;
            } else {
                xmlCatalogDump(out_0);
            }
        } else {
            xmlCatalogDump(stdout);
        }
    }
    xmlCleanupParser();
    return exit_value;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as c_int,
            args_ptrs.as_mut_ptr() as *mut *mut c_char,
        ) as i32)
    }
}
