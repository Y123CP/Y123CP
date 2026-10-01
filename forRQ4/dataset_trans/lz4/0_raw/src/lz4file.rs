extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LZ4F_cctx_s;
    pub type LZ4F_dctx_s;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn LZ4F_isError(code: LZ4F_errorCode_t) -> ::core::ffi::c_uint;
    fn LZ4F_createCompressionContext(
        cctxPtr: *mut *mut LZ4F_cctx,
        version: ::core::ffi::c_uint,
    ) -> LZ4F_errorCode_t;
    fn LZ4F_freeCompressionContext(cctx: *mut LZ4F_cctx) -> LZ4F_errorCode_t;
    fn LZ4F_compressBegin(
        cctx: *mut LZ4F_cctx,
        dstBuffer: *mut ::core::ffi::c_void,
        dstCapacity: size_t,
        prefsPtr: *const LZ4F_preferences_t,
    ) -> size_t;
    fn LZ4F_compressBound(srcSize: size_t, prefsPtr: *const LZ4F_preferences_t) -> size_t;
    fn LZ4F_compressUpdate(
        cctx: *mut LZ4F_cctx,
        dstBuffer: *mut ::core::ffi::c_void,
        dstCapacity: size_t,
        srcBuffer: *const ::core::ffi::c_void,
        srcSize: size_t,
        cOptPtr: *const LZ4F_compressOptions_t,
    ) -> size_t;
    fn LZ4F_compressEnd(
        cctx: *mut LZ4F_cctx,
        dstBuffer: *mut ::core::ffi::c_void,
        dstCapacity: size_t,
        cOptPtr: *const LZ4F_compressOptions_t,
    ) -> size_t;
    fn LZ4F_createDecompressionContext(
        dctxPtr: *mut *mut LZ4F_dctx,
        version: ::core::ffi::c_uint,
    ) -> LZ4F_errorCode_t;
    fn LZ4F_freeDecompressionContext(dctx: *mut LZ4F_dctx) -> LZ4F_errorCode_t;
    fn LZ4F_getFrameInfo(
        dctx: *mut LZ4F_dctx,
        frameInfoPtr: *mut LZ4F_frameInfo_t,
        srcBuffer: *const ::core::ffi::c_void,
        srcSizePtr: *mut size_t,
    ) -> size_t;
    fn LZ4F_decompress(
        dctx: *mut LZ4F_dctx,
        dstBuffer: *mut ::core::ffi::c_void,
        dstSizePtr: *mut size_t,
        srcBuffer: *const ::core::ffi::c_void,
        srcSizePtr: *mut size_t,
        dOptPtr: *const LZ4F_decompressOptions_t,
    ) -> size_t;
    fn LZ4F_getBlockSize(blockSizeID: LZ4F_blockSizeID_t) -> size_t;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type ptrdiff_t = isize;
pub type LZ4_byte = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type LZ4F_errorCode_t = size_t;
pub type LZ4F_blockSizeID_t = ::core::ffi::c_uint;
pub const LZ4F_max4MB: LZ4F_blockSizeID_t = 7;
pub const LZ4F_max1MB: LZ4F_blockSizeID_t = 6;
pub const LZ4F_max256KB: LZ4F_blockSizeID_t = 5;
pub const LZ4F_max64KB: LZ4F_blockSizeID_t = 4;
pub const LZ4F_default: LZ4F_blockSizeID_t = 0;
pub type LZ4F_blockMode_t = ::core::ffi::c_uint;
pub const LZ4F_blockIndependent: LZ4F_blockMode_t = 1;
pub const LZ4F_blockLinked: LZ4F_blockMode_t = 0;
pub type LZ4F_contentChecksum_t = ::core::ffi::c_uint;
pub const LZ4F_contentChecksumEnabled: LZ4F_contentChecksum_t = 1;
pub const LZ4F_noContentChecksum: LZ4F_contentChecksum_t = 0;
pub type LZ4F_blockChecksum_t = ::core::ffi::c_uint;
pub const LZ4F_blockChecksumEnabled: LZ4F_blockChecksum_t = 1;
pub const LZ4F_noBlockChecksum: LZ4F_blockChecksum_t = 0;
pub type LZ4F_frameType_t = ::core::ffi::c_uint;
pub const LZ4F_skippableFrame: LZ4F_frameType_t = 1;
pub const LZ4F_frame: LZ4F_frameType_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_frameInfo_t {
    pub blockSizeID: LZ4F_blockSizeID_t,
    pub blockMode: LZ4F_blockMode_t,
    pub contentChecksumFlag: LZ4F_contentChecksum_t,
    pub frameType: LZ4F_frameType_t,
    pub contentSize: ::core::ffi::c_ulonglong,
    pub dictID: ::core::ffi::c_uint,
    pub blockChecksumFlag: LZ4F_blockChecksum_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_preferences_t {
    pub frameInfo: LZ4F_frameInfo_t,
    pub compressionLevel: ::core::ffi::c_int,
    pub autoFlush: ::core::ffi::c_uint,
    pub favorDecSpeed: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 3],
}
pub type LZ4F_cctx = LZ4F_cctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_compressOptions_t {
    pub stableSrc: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 3],
}
pub type LZ4F_dctx = LZ4F_dctx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4F_decompressOptions_t {
    pub stableDst: ::core::ffi::c_uint,
    pub skipChecksums: ::core::ffi::c_uint,
    pub reserved1: ::core::ffi::c_uint,
    pub reserved0: ::core::ffi::c_uint,
}
pub type LZ4F_errorCodes = ::core::ffi::c_uint;
pub const _LZ4F_dummy_error_enum_for_c89_never_used: LZ4F_errorCodes = 25;
pub const LZ4F_ERROR_maxCode: LZ4F_errorCodes = 24;
pub const LZ4F_ERROR_io_read: LZ4F_errorCodes = 23;
pub const LZ4F_ERROR_io_write: LZ4F_errorCodes = 22;
pub const LZ4F_ERROR_parameter_null: LZ4F_errorCodes = 21;
pub const LZ4F_ERROR_compressionState_uninitialized: LZ4F_errorCodes = 20;
pub const LZ4F_ERROR_frameDecoding_alreadyStarted: LZ4F_errorCodes = 19;
pub const LZ4F_ERROR_contentChecksum_invalid: LZ4F_errorCodes = 18;
pub const LZ4F_ERROR_headerChecksum_invalid: LZ4F_errorCodes = 17;
pub const LZ4F_ERROR_decompressionFailed: LZ4F_errorCodes = 16;
pub const LZ4F_ERROR_srcPtr_wrong: LZ4F_errorCodes = 15;
pub const LZ4F_ERROR_frameSize_wrong: LZ4F_errorCodes = 14;
pub const LZ4F_ERROR_frameType_unknown: LZ4F_errorCodes = 13;
pub const LZ4F_ERROR_frameHeader_incomplete: LZ4F_errorCodes = 12;
pub const LZ4F_ERROR_dstMaxSize_tooSmall: LZ4F_errorCodes = 11;
pub const LZ4F_ERROR_srcSize_tooLarge: LZ4F_errorCodes = 10;
pub const LZ4F_ERROR_allocation_failed: LZ4F_errorCodes = 9;
pub const LZ4F_ERROR_reservedFlag_set: LZ4F_errorCodes = 8;
pub const LZ4F_ERROR_blockChecksum_invalid: LZ4F_errorCodes = 7;
pub const LZ4F_ERROR_headerVersion_wrong: LZ4F_errorCodes = 6;
pub const LZ4F_ERROR_compressionLevel_invalid: LZ4F_errorCodes = 5;
pub const LZ4F_ERROR_parameter_invalid: LZ4F_errorCodes = 4;
pub const LZ4F_ERROR_blockMode_invalid: LZ4F_errorCodes = 3;
pub const LZ4F_ERROR_maxBlockSize_invalid: LZ4F_errorCodes = 2;
pub const LZ4F_ERROR_GENERIC: LZ4F_errorCodes = 1;
pub const LZ4F_OK_NoError: LZ4F_errorCodes = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4_readFile_s {
    pub dctxPtr: *mut LZ4F_dctx,
    pub fp: *mut FILE,
    pub srcBuf: *mut LZ4_byte,
    pub srcBufNext: size_t,
    pub srcBufSize: size_t,
    pub srcBufMaxSize: size_t,
}
pub type LZ4_readFile_t = LZ4_readFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LZ4_writeFile_s {
    pub cctxPtr: *mut LZ4F_cctx,
    pub fp: *mut FILE,
    pub dstBuf: *mut LZ4_byte,
    pub maxWriteSize: size_t,
    pub dstBufMaxSize: size_t,
    pub errCode: LZ4F_errorCode_t,
}
pub type LZ4_writeFile_t = LZ4_writeFile_s;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const LZ4F_VERSION: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const LZ4F_HEADER_SIZE_MIN: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LZ4F_HEADER_SIZE_MAX: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const LZ4F_ENDMARK_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn returnErrorCode(mut code: LZ4F_errorCodes) -> LZ4F_errorCode_t {
    return -(code as ptrdiff_t) as LZ4F_errorCode_t;
}
unsafe extern "C" fn freeReadFileResources(mut lz4fRead: *mut LZ4_readFile_t) {
    if lz4fRead.is_null() {
        return;
    }
    LZ4F_freeDecompressionContext((*lz4fRead).dctxPtr);
    free((*lz4fRead).srcBuf as *mut ::core::ffi::c_void);
    free(lz4fRead as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn freeAndNullReadFile(mut statePtr: *mut *mut LZ4_readFile_t) {
    '_c2rust_label: {
        if !statePtr.is_null() {
        } else {
            __assert_fail(
                b"statePtr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"lib/lz4file.c\0" as *const u8 as *const ::core::ffi::c_char,
                70 as ::core::ffi::c_uint,
                b"void freeAndNullReadFile(LZ4_readFile_t **)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    freeReadFileResources(*statePtr);
    *statePtr = ::core::ptr::null_mut::<LZ4_readFile_t>();
}
unsafe extern "C" fn readAndParseHeader(
    mut readFile: *mut LZ4_readFile_t,
    mut fp: *mut FILE,
) -> LZ4F_errorCode_t {
    let mut headerBuf: [::core::ffi::c_char; 19] = [0; 19];
    let mut frameInfo: LZ4F_frameInfo_t = LZ4F_frameInfo_t {
        blockSizeID: LZ4F_default,
        blockMode: LZ4F_blockLinked,
        contentChecksumFlag: LZ4F_noContentChecksum,
        frameType: LZ4F_frame,
        contentSize: 0,
        dictID: 0,
        blockChecksumFlag: LZ4F_noBlockChecksum,
    };
    let mut consumedSize: size_t = 0;
    let bytesRead: size_t = fread(
        &raw mut headerBuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        1 as size_t,
        ::core::mem::size_of::<[::core::ffi::c_char; 19]>() as size_t,
        fp,
    ) as size_t;
    if bytesRead < (LZ4F_HEADER_SIZE_MIN + LZ4F_ENDMARK_SIZE) as size_t {
        return returnErrorCode(LZ4F_ERROR_io_read);
    }
    consumedSize = bytesRead;
    let result: LZ4F_errorCode_t = LZ4F_getFrameInfo(
        (*readFile).dctxPtr,
        &raw mut frameInfo,
        &raw mut headerBuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        &raw mut consumedSize,
    ) as LZ4F_errorCode_t;
    if LZ4F_isError(result) != 0 {
        return result;
    }
    let blockSize: size_t = LZ4F_getBlockSize(frameInfo.blockSizeID) as size_t;
    if blockSize == 0 as size_t {
        return returnErrorCode(LZ4F_ERROR_maxBlockSize_invalid);
    }
    (*readFile).srcBufMaxSize = blockSize;
    '_c2rust_label: {
        if (*readFile).srcBuf.is_null() {
        } else {
            __assert_fail(
                b"readFile->srcBuf == NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"lib/lz4file.c\0" as *const u8 as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_uint,
                b"LZ4F_errorCode_t readAndParseHeader(LZ4_readFile_t *, FILE *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*readFile).srcBuf = malloc((*readFile).srcBufMaxSize) as *mut LZ4_byte;
    if (*readFile).srcBuf.is_null() {
        return returnErrorCode(LZ4F_ERROR_allocation_failed);
    }
    (*readFile).srcBufSize = bytesRead.wrapping_sub(consumedSize);
    if (*readFile).srcBufSize > 0 as size_t {
        memcpy(
            (*readFile).srcBuf as *mut ::core::ffi::c_void,
            (&raw mut headerBuf as *mut ::core::ffi::c_char).offset(consumedSize as isize)
                as *const ::core::ffi::c_void,
            (*readFile).srcBufSize,
        );
    }
    (*readFile).srcBufNext = 0 as size_t;
    return LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_readOpen(
    mut lz4fRead: *mut *mut LZ4_readFile_t,
    mut fp: *mut FILE,
) -> LZ4F_errorCode_t {
    let mut readFile: *mut LZ4_readFile_t = ::core::ptr::null_mut::<LZ4_readFile_t>();
    if fp.is_null() || lz4fRead.is_null() {
        return returnErrorCode(LZ4F_ERROR_parameter_null);
    }
    readFile = calloc(
        1 as size_t,
        ::core::mem::size_of::<LZ4_readFile_t>() as size_t,
    ) as *mut LZ4_readFile_t;
    if readFile.is_null() {
        return returnErrorCode(LZ4F_ERROR_allocation_failed);
    }
    (*readFile).fp = fp;
    let result: LZ4F_errorCode_t = LZ4F_createDecompressionContext(
        &raw mut (*readFile).dctxPtr,
        LZ4F_VERSION as ::core::ffi::c_uint,
    ) as LZ4F_errorCode_t;
    if LZ4F_isError(result) != 0 {
        freeAndNullReadFile(&raw mut readFile);
        return result;
    }
    let result_0: LZ4F_errorCode_t = readAndParseHeader(readFile, fp) as LZ4F_errorCode_t;
    if LZ4F_isError(result_0) != 0 {
        freeAndNullReadFile(&raw mut readFile);
        return result_0;
    }
    *lz4fRead = readFile;
    return LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_read(
    mut lz4fRead: *mut LZ4_readFile_t,
    mut buf: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> size_t {
    let mut outPtr: *mut LZ4_byte = buf as *mut LZ4_byte;
    let mut totalBytesRead: size_t = 0 as size_t;
    if lz4fRead.is_null() || buf.is_null() {
        return returnErrorCode(LZ4F_ERROR_parameter_null) as size_t;
    }
    while totalBytesRead < size {
        let mut srcBytes: size_t = (*lz4fRead).srcBufSize.wrapping_sub((*lz4fRead).srcBufNext);
        let mut dstBytes: size_t = size.wrapping_sub(totalBytesRead);
        if srcBytes == 0 as size_t {
            let bytesRead: size_t = fread(
                (*lz4fRead).srcBuf as *mut ::core::ffi::c_void,
                1 as size_t,
                (*lz4fRead).srcBufMaxSize,
                (*lz4fRead).fp,
            ) as size_t;
            if bytesRead == 0 as size_t {
                if ferror((*lz4fRead).fp) != 0 {
                    return returnErrorCode(LZ4F_ERROR_io_read) as size_t;
                }
                break;
            } else {
                (*lz4fRead).srcBufSize = bytesRead;
                srcBytes = (*lz4fRead).srcBufSize;
                (*lz4fRead).srcBufNext = 0 as size_t;
            }
        }
        let decStatus: size_t = LZ4F_decompress(
            (*lz4fRead).dctxPtr,
            outPtr as *mut ::core::ffi::c_void,
            &raw mut dstBytes,
            (*lz4fRead).srcBuf.offset((*lz4fRead).srcBufNext as isize)
                as *const ::core::ffi::c_void,
            &raw mut srcBytes,
            ::core::ptr::null::<LZ4F_decompressOptions_t>(),
        ) as size_t;
        if LZ4F_isError(decStatus as LZ4F_errorCode_t) != 0 {
            return decStatus;
        }
        (*lz4fRead).srcBufNext = (*lz4fRead).srcBufNext.wrapping_add(srcBytes);
        totalBytesRead = totalBytesRead.wrapping_add(dstBytes);
        outPtr = outPtr.offset(dstBytes as isize);
    }
    return totalBytesRead;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_readClose(mut lz4fRead: *mut LZ4_readFile_t) -> LZ4F_errorCode_t {
    if lz4fRead.is_null() {
        return returnErrorCode(LZ4F_ERROR_parameter_null);
    }
    freeReadFileResources(lz4fRead);
    return LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t;
}
unsafe extern "C" fn freeWriteFileResources(mut state: *mut LZ4_writeFile_t) {
    if state.is_null() {
        return;
    }
    LZ4F_freeCompressionContext((*state).cctxPtr);
    free((*state).dstBuf as *mut ::core::ffi::c_void);
    free(state as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn freeAndNullWriteFile(mut statePtr: *mut *mut LZ4_writeFile_t) {
    '_c2rust_label: {
        if !statePtr.is_null() {
        } else {
            __assert_fail(
                b"statePtr != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"lib/lz4file.c\0" as *const u8 as *const ::core::ffi::c_char,
                227 as ::core::ffi::c_uint,
                b"void freeAndNullWriteFile(LZ4_writeFile_t **)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    freeWriteFileResources(*statePtr);
    *statePtr = ::core::ptr::null_mut::<LZ4_writeFile_t>();
}
unsafe extern "C" fn writeHeader(
    mut writeFile: *mut LZ4_writeFile_t,
    mut fp: *mut FILE,
    mut prefsPtr: *const LZ4F_preferences_t,
) -> LZ4F_errorCode_t {
    let mut headerBuf: [LZ4_byte; 19] = [0; 19];
    let headerSize: LZ4F_errorCode_t = LZ4F_compressBegin(
        (*writeFile).cctxPtr,
        &raw mut headerBuf as *mut LZ4_byte as *mut ::core::ffi::c_void,
        LZ4F_HEADER_SIZE_MAX as size_t,
        prefsPtr,
    ) as LZ4F_errorCode_t;
    if LZ4F_isError(headerSize) != 0 {
        return headerSize;
    }
    if headerSize
        != fwrite(
            &raw mut headerBuf as *mut LZ4_byte as *const ::core::ffi::c_void,
            1 as size_t,
            headerSize as size_t,
            fp,
        ) as LZ4F_errorCode_t
    {
        return returnErrorCode(LZ4F_ERROR_io_write);
    }
    return LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_writeOpen(
    mut lz4fWrite: *mut *mut LZ4_writeFile_t,
    mut fp: *mut FILE,
    mut prefsPtr: *const LZ4F_preferences_t,
) -> LZ4F_errorCode_t {
    let mut writeFile: *mut LZ4_writeFile_t = ::core::ptr::null_mut::<LZ4_writeFile_t>();
    let mut blockSize: size_t = 0;
    if fp.is_null() || lz4fWrite.is_null() {
        return returnErrorCode(LZ4F_ERROR_parameter_null);
    }
    let blockSizeID: LZ4F_blockSizeID_t = (if !prefsPtr.is_null() {
        (*prefsPtr).frameInfo.blockSizeID as ::core::ffi::c_uint
    } else {
        LZ4F_default as ::core::ffi::c_int as ::core::ffi::c_uint
    }) as LZ4F_blockSizeID_t;
    blockSize = LZ4F_getBlockSize(blockSizeID);
    if blockSize == 0 as size_t {
        return returnErrorCode(LZ4F_ERROR_maxBlockSize_invalid);
    }
    writeFile = calloc(
        1 as size_t,
        ::core::mem::size_of::<LZ4_writeFile_t>() as size_t,
    ) as *mut LZ4_writeFile_t;
    if writeFile.is_null() {
        return returnErrorCode(LZ4F_ERROR_allocation_failed);
    }
    (*writeFile).fp = fp;
    (*writeFile).errCode = LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t;
    (*writeFile).maxWriteSize = blockSize;
    (*writeFile).dstBufMaxSize = LZ4F_compressBound(blockSize, prefsPtr);
    (*writeFile).dstBuf = malloc((*writeFile).dstBufMaxSize) as *mut LZ4_byte;
    if (*writeFile).dstBuf.is_null() {
        freeAndNullWriteFile(&raw mut writeFile);
        return returnErrorCode(LZ4F_ERROR_allocation_failed);
    }
    let status: LZ4F_errorCode_t = LZ4F_createCompressionContext(
        &raw mut (*writeFile).cctxPtr,
        LZ4F_VERSION as ::core::ffi::c_uint,
    ) as LZ4F_errorCode_t;
    if LZ4F_isError(status) != 0 {
        freeAndNullWriteFile(lz4fWrite);
        return status;
    }
    let writeStatus: LZ4F_errorCode_t = writeHeader(writeFile, fp, prefsPtr) as LZ4F_errorCode_t;
    if LZ4F_isError(writeStatus) != 0 {
        freeAndNullWriteFile(&raw mut writeFile);
        return writeStatus;
    }
    *lz4fWrite = writeFile;
    return LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_write(
    mut lz4fWrite: *mut LZ4_writeFile_t,
    mut buf: *const ::core::ffi::c_void,
    mut size: size_t,
) -> size_t {
    let mut p: *const LZ4_byte = buf as *const LZ4_byte;
    let mut remainingBytes: size_t = size;
    if lz4fWrite.is_null() || buf.is_null() {
        return returnErrorCode(LZ4F_ERROR_parameter_null) as size_t;
    }
    while remainingBytes != 0 {
        let chunkSize: size_t = if remainingBytes > (*lz4fWrite).maxWriteSize {
            (*lz4fWrite).maxWriteSize
        } else {
            remainingBytes
        };
        let mut cSize: size_t = LZ4F_compressUpdate(
            (*lz4fWrite).cctxPtr,
            (*lz4fWrite).dstBuf as *mut ::core::ffi::c_void,
            (*lz4fWrite).dstBufMaxSize,
            p as *const ::core::ffi::c_void,
            chunkSize,
            ::core::ptr::null::<LZ4F_compressOptions_t>(),
        );
        if LZ4F_isError(cSize as LZ4F_errorCode_t) != 0 {
            (*lz4fWrite).errCode = cSize as LZ4F_errorCode_t;
            return cSize;
        }
        if cSize
            != fwrite(
                (*lz4fWrite).dstBuf as *const ::core::ffi::c_void,
                1 as size_t,
                cSize,
                (*lz4fWrite).fp,
            ) as size_t
        {
            (*lz4fWrite).errCode = returnErrorCode(LZ4F_ERROR_io_write);
            return returnErrorCode(LZ4F_ERROR_io_write) as size_t;
        }
        p = p.offset(chunkSize as isize);
        remainingBytes = remainingBytes.wrapping_sub(chunkSize);
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn LZ4F_writeClose(mut lz4fWrite: *mut LZ4_writeFile_t) -> LZ4F_errorCode_t {
    let mut ret: LZ4F_errorCode_t = LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t;
    if lz4fWrite.is_null() {
        return returnErrorCode(LZ4F_ERROR_parameter_null);
    }
    if (*lz4fWrite).errCode == LZ4F_OK_NoError as ::core::ffi::c_int as LZ4F_errorCode_t {
        ret = LZ4F_compressEnd(
            (*lz4fWrite).cctxPtr,
            (*lz4fWrite).dstBuf as *mut ::core::ffi::c_void,
            (*lz4fWrite).dstBufMaxSize,
            ::core::ptr::null::<LZ4F_compressOptions_t>(),
        ) as LZ4F_errorCode_t;
        if !(LZ4F_isError(ret) != 0) {
            if ret
                != fwrite(
                    (*lz4fWrite).dstBuf as *const ::core::ffi::c_void,
                    1 as size_t,
                    ret as size_t,
                    (*lz4fWrite).fp,
                ) as LZ4F_errorCode_t
            {
                ret = returnErrorCode(LZ4F_ERROR_io_write);
            }
        }
    }
    freeWriteFileResources(lz4fWrite);
    return ret;
}
