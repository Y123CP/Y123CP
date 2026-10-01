                                                                  
                                                    
                                                                 
                                                                               
                                              
mod libcall {
                                       
    pub use mzlib::src::miniz::{
        mz_adler32, mz_compress, mz_compress2, mz_compressBound, mz_crc32, mz_deflate,
        mz_deflateEnd, mz_deflateInit, mz_deflateReset, mz_error, mz_free, mz_inflate,
        mz_inflateEnd, mz_inflateInit, mz_inflateReset, mz_stream, mz_uncompress, mz_version,
        MZ_ADLER32_INIT, MZ_DEFAULT_LEVEL, MZ_FINISH, MZ_MEM_ERROR, MZ_NO_FLUSH, MZ_OK,
        MZ_STREAM_END,
    };
                                                                                
    pub use mzlib::src::miniz_tdef::{
        tdefl_compress_mem_to_heap, tdefl_compress_mem_to_mem, tdefl_compressor_alloc,
        tdefl_compressor_free, tdefl_get_prev_return_status, tdefl_init,
        tdefl_write_image_to_png_file_in_memory, tdefl_write_image_to_png_file_in_memory_ex,
        MZ_CRC32_INIT, TDEFL_FORCE_ALL_STATIC_BLOCKS, TDEFL_GREEDY_PARSING_FLAG,
        TDEFL_RLE_MATCHES, TDEFL_WRITE_ZLIB_HEADER,
    };
                                    
    pub use mzlib::src::miniz_tinfl::{
        tinfl_decompress_mem_to_callback, tinfl_decompress_mem_to_heap,
        tinfl_decompress_mem_to_mem, tinfl_decompressor_alloc, tinfl_decompressor_free,
    };
                                                                        
    pub use mzlib::src::miniz_zip::{
        mz_zip_add_mem_to_archive_file_in_place, mz_zip_add_mem_to_archive_file_in_place_v2,
        mz_zip_archive, mz_zip_archive_file_stat, mz_zip_clear_last_error, mz_zip_end,
        mz_zip_error, mz_zip_extract_archive_file_to_heap, mz_zip_extract_archive_file_to_heap_v2,
        mz_zip_get_archive_file_start_offset, mz_zip_get_archive_size,
        mz_zip_get_central_dir_size, mz_zip_get_cfile, mz_zip_get_error_string,
        mz_zip_get_last_error, mz_zip_get_mode, mz_zip_get_type, mz_zip_is_zip64,
        mz_zip_peek_last_error, mz_zip_read_archive_data, mz_zip_reader_end,
        mz_zip_reader_extract_file_iter_new, mz_zip_reader_extract_file_to_callback,
        mz_zip_reader_extract_file_to_cfile, mz_zip_reader_extract_file_to_file,
        mz_zip_reader_extract_file_to_heap, mz_zip_reader_extract_file_to_mem,
        mz_zip_reader_extract_file_to_mem_no_alloc, mz_zip_reader_extract_iter_free,
        mz_zip_reader_extract_iter_new, mz_zip_reader_extract_iter_read,
        mz_zip_reader_extract_to_callback, mz_zip_reader_extract_to_cfile,
        mz_zip_reader_extract_to_file, mz_zip_reader_extract_to_heap,
        mz_zip_reader_extract_to_mem, mz_zip_reader_extract_to_mem_no_alloc,
        mz_zip_reader_file_stat, mz_zip_reader_get_filename, mz_zip_reader_get_num_files,
        mz_zip_reader_init_cfile, mz_zip_reader_init_file, mz_zip_reader_init_mem,
        mz_zip_reader_is_file_a_directory, mz_zip_reader_locate_file, mz_zip_validate_archive,
        mz_zip_validate_file_archive, mz_zip_validate_mem_archive, mz_zip_writer_add_cfile,
        mz_zip_writer_add_file, mz_zip_writer_add_mem_ex_v2, mz_zip_writer_add_read_buf_callback,
        mz_zip_writer_end, mz_zip_writer_finalize_archive, mz_zip_writer_finalize_heap_archive,
        mz_zip_writer_init_cfile, mz_zip_writer_init_file, mz_zip_writer_init_from_reader_v2,
        mz_zip_writer_init_heap, mz_zip_zero_struct, time_t, FILE, MZ_BEST_SPEED,
        MZ_ZIP_NO_ERROR, MZ_ZIP_UNDEFINED_ERROR,
    };

                                                             
    use core::ffi::{c_char, c_int, c_long};
    extern "C" {
        pub fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
        pub fn fclose(f: *mut FILE) -> c_int;
        pub fn fseek(f: *mut FILE, off: c_long, whence: c_int) -> c_int;
        pub fn rewind(f: *mut FILE);
        pub fn remove(path: *const c_char) -> c_int;
    }
}

include!("../../driver.rs");
